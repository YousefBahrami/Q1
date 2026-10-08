"""Plan/run an explicitly private SSH-tunneled deployment. Never provisions hosts."""
import argparse
from collections import defaultdict
import hashlib
import json
import os
from pathlib import Path
import re
import secrets
import shlex
import sys

ROOT = Path(__file__).resolve().parents[2]
PROFILE = 'Q1_TESTNET_MULTI_HOST_V0'
ROLES = {'voter1':0,'voter2':1,'voter3':2,'producer_a':3,'producer_b':4,'ordinary':5}


def canonical(value): return json.dumps(value,sort_keys=True,separators=(',',':')).encode()


def validate(config):
    if set(config) != {'profile','chain_nonce','base_port','bootstrap_host','hosts','roles'}:
        raise ValueError('configuration fields')
    if config['profile'] != PROFILE or not re.fullmatch('[0-9a-f]{64}',config['chain_nonce']):
        raise ValueError('profile/chain nonce')
    if type(config['base_port']) is not int or not 1024<=config['base_port']<=65520:
        raise ValueError('base port')
    hosts=config['hosts']
    if not isinstance(hosts,dict) or not 1<=len(hosts)<=7: raise ValueError('host count')
    for name,h in hosts.items():
        if not re.fullmatch('[a-z][a-z0-9-]{0,31}',name):raise ValueError('host identifier')
        if set(h)!={'ssh_target','confirmed','failure_domain'}:raise ValueError('host fields')
        # SSH alias or user@hostname, not arbitrary option/command expansion.
        if not isinstance(h['ssh_target'],str) or not re.fullmatch('[A-Za-z0-9_][A-Za-z0-9_.@-]{0,127}',h['ssh_target']):raise ValueError('SSH target')
        if h['confirmed'] is not True:raise ValueError('human hardware confirmation missing')
        if not isinstance(h['failure_domain'],str) or not re.fullmatch('[a-z][a-z0-9-]{0,31}',h['failure_domain']):raise ValueError('failure domain')
    required=set(ROLES)-{'ordinary'}
    if not required<=set(config['roles'])<=set(ROLES):raise ValueError('required roles')
    if config['bootstrap_host'] not in hosts or any(h not in hosts for h in config['roles'].values()):raise ValueError('unknown host')
    return config


def load(path):
    with Path(path).open('rb') as f:raw=f.read(16385)
    if len(raw)>16384:raise ValueError('configuration bound')
    def unique(items):
        result={}
        for k,v in items:
            if k in result:raise ValueError('duplicate configuration field')
            result[k]=v
        return result
    return validate(json.loads(raw,object_pairs_hook=unique))


def plan(config):
    validate(config); commands={}; domains=defaultdict(list)
    for name,h in config['hosts'].items():
        domains[h['failure_domain']].append(name)
        by_remote=defaultdict(list)
        for role,remote in config['roles'].items():
            if remote!=name:
                port=config['base_port']+ROLES[role]
                by_remote[remote].extend(['-L',f'127.0.0.1:{port}:127.0.0.1:{port}'])
        commands[name]=[shlex.join(['ssh','-N','-T','-o','BatchMode=yes','-o','StrictHostKeyChecking=yes',
                    '-o','ExitOnForwardFailure=yes','-o','ServerAliveInterval=10','-o','ServerAliveCountMax=3',
                    *forwards,config['hosts'][remote]['ssh_target']]) for remote,forwards in sorted(by_remote.items())]
    risk=[]
    for domain,hosts in sorted(domains.items()):
        roles=[r for r,h in config['roles'].items() if h in hosts]
        voters=sum(r.startswith('voter') for r in roles)
        risk.append(dict(failure_domain=domain,roles_lost=roles,remaining_voters=3-voters,
                         quorum_survives=voters<=1,
                         producer_survives=not {'producer_a','producer_b'}<=set(roles)))
    return dict(profile=PROFILE,manifest_sha256=hashlib.sha256(canonical(config)).hexdigest(),
                commands=commands,failure_dependence=risk,
                note='Commands only. No SSH connection, host purchase, firewall edit or deployment performed.')


def module(config):
    # Chain context must be selected before importing the existing node module.
    os.environ['Q1_TESTNET_LAB_CHAIN']=config['chain_nonce']
    sys.path.insert(0,str(ROOT/'research/testnet_ledger_v0'))
    import node
    node.CHAIN=config['chain_nonce']
    return node


def serve(config,host,role,directory):
    if config['roles'].get(role)!=host:raise ValueError('role is not assigned to this host')
    n=module(config)
    ports=[config['base_port']+i for i in range(6)]
    n.server(directory,ROLES[role],ports)  # existing loopback binding stays intact


def control(config,args):
    n=module(config);local=config['bootstrap_host']
    if args.host!=local:raise ValueError('control runs on the declared bootstrap host')
    target=ROLES[args.role]
    if args.role not in config['roles']:raise ValueError('role is not deployed')
    # A durable controller counter shares the ordinary fixture signer: keep it
    # above worker outseq and do not run a second controller with the same state.
    import fcntl
    directory=Path(args.directory);directory.mkdir(parents=True,exist_ok=True)
    with (directory/'control.lock').open('a+b') as lock:
        fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        path=directory/'control.json'
        state=json.loads(path.read_text()) if path.exists() else {'chain':config['chain_nonce'],'seq':1000000}
        if state['chain']!=config['chain_nonce']:raise ValueError('controller chain mismatch')
        state['seq']+=1
        if not 1000000<state['seq']<2**32:raise ValueError('sequence bound')
        pending=directory/'control.pending'
        with pending.open('wb') as f:f.write(canonical(state));f.flush();os.fsync(f.fileno())
        os.replace(pending,path)
        fd=os.open(directory,os.O_RDONLY)
        try:os.fsync(fd)
        finally:os.close(fd)
        kw={}
        if args.action=='run':
            if args.ballot is None:raise ValueError('explicit ballot required; no timeout elects a leader')
            kw=dict(ballot=args.ballot,payload=args.amount,crash=0)
        if args.action=='sync':
            if not args.history:raise ValueError('--history required')
            with Path(args.history).open('rb') as f:raw=f.read(n.MAX_FRAME+1)
            if len(raw)>n.MAX_FRAME:raise ValueError('history bound')
            history=json.loads(raw)
            # Validate independently before sending to the target. Hostile-peer
            # acceptance checks also exercise the receiver directly.
            n.backend('status',history=history);kw=dict(history=history)
        request=n.signed(5,n.body(args.action,seq=state['seq'],**kw))
        response=n.rpc(config['base_port']+target,request)
        if response['signer']!=target:raise ValueError('wrong response signer')
        result=n.authentic(response)
        if args.action=='status':
            replay=n.backend('status',history=result['history'])
            if replay!=result['ledger']:raise ValueError('unverified status/root')
        print(json.dumps(result,indent=2))


def main():
    p=argparse.ArgumentParser();p.add_argument('action',choices=['template','plan','serve','status','run','sync'])
    p.add_argument('--config',type=Path);p.add_argument('--output',type=Path)
    p.add_argument('--host');p.add_argument('--role',choices=ROLES);p.add_argument('--directory',type=Path)
    p.add_argument('--ballot',type=int);p.add_argument('--amount',type=int,default=10);p.add_argument('--history',type=Path)
    args=p.parse_args()
    if args.action=='template':
        if not args.output:p.error('--output required')
        config=dict(profile=PROFILE,chain_nonce=secrets.token_hex(32),base_port=24000,
            bootstrap_host='host-a',hosts={f'host-{x}':dict(ssh_target=f'REPLACE-host-{x}',confirmed=False,failure_domain=f'host-{x}') for x in 'abc'},
            roles=dict(voter1='host-a',voter2='host-b',voter3='host-c',producer_a='host-a',producer_b='host-b',ordinary='host-c'))
        with args.output.open('x') as f:json.dump(config,f,indent=2);f.write('\n')
        return
    if not args.config:p.error('--config required')
    config=load(args.config)
    if args.action=='plan':print(json.dumps(plan(config),indent=2));return
    if not args.host or args.host not in config['hosts'] or not args.role or not args.directory:p.error('confirmed --host, --role and --directory required')
    if args.action=='serve':serve(config,args.host,args.role,args.directory)
    else:control(config,args)

if __name__=='__main__':
    try:main()
    except (ValueError,OSError,KeyError,TypeError) as error:raise SystemExit(str(error))
