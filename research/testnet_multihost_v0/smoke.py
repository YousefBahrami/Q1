"""Exercise deployment CLI with six local processes; never a multi-host result."""
import argparse
import json
import os
from pathlib import Path
import socket
import struct
import subprocess
import sys
import tempfile
import time
import deploy


def run():
    with tempfile.TemporaryDirectory(prefix='q1-multihost-preflight-') as tmp:
        root=Path(tmp);held=[]
        for base in range(27000,28000,6):
            try:
                for i in range(6):
                    s=socket.socket();held.append(s);s.bind(('127.0.0.1',base+i))
                break
            except OSError:
                for s in held:s.close()
                held=[]
        else:raise RuntimeError('no six free local ports')
        for s in held:s.close()
        config=dict(profile=deploy.PROFILE,chain_nonce=os.urandom(32).hex(),base_port=base,
            bootstrap_host='local-fixture',hosts={'local-fixture':dict(ssh_target='unused-fixture',confirmed=True,failure_domain='same-machine')},
            roles={role:'local-fixture' for role in deploy.ROLES})
        path=root/'topology.json';path.write_bytes(deploy.canonical(config))
        n=deploy.module(config);processes=[];logs=[]
        command=[sys.executable,str(Path(deploy.__file__))]
        common=['--config',str(path),'--host','local-fixture']
        def control(action,role,*extra):
            r=subprocess.run(command+[action,*common,'--role',role,'--directory',str(root/'controller'),*extra],capture_output=True,text=True,timeout=30)
            if r.returncode:raise RuntimeError(r.stderr)
            return json.loads(r.stdout)
        try:
            for role in deploy.ROLES:
                log=(root/(role+'.log')).open('w');logs.append(log)
                processes.append(subprocess.Popen(command+['serve',*common,'--role',role,'--directory',str(root/role)],stdout=log,stderr=log))
            deadline=time.monotonic()+15
            for role in deploy.ROLES:
                while True:
                    if any(p.poll() is not None for p in processes):raise RuntimeError('worker exited')
                    try:control('status',role);break
                    except RuntimeError:
                        if time.monotonic()>deadline:raise
                        time.sleep(.05)
            control('run','producer_a','--ballot','0','--amount','10')
            statuses=[control('status',r) for r in deploy.ROLES]
            ledger=statuses[0]['ledger']
            assert all(s['ledger']==ledger for s in statuses)
            assert ledger['height']==1 and ledger['sender']=='989' and ledger['reward_pool']=='1'
            with socket.create_connection(('127.0.0.1',base),timeout=2) as s:
                s.sendall(struct.pack('!I',n.MAX_FRAME+1));reply=n.receive(s)
                assert reply['error']=='OVERSIZED_MESSAGE'
            bad=statuses[0]['history'];bad[0]['votes']=[bad[0]['votes'][0]]*2
            seq=json.loads((root/'controller/control.json').read_text())['seq']+1
            req=n.signed(5,n.body('sync',seq=seq,history=bad))
            try:n.rpc(base,req)
            except n.Rejected:pass
            else:raise AssertionError('untrusted invalid certificate accepted')
            assert control('status','voter1')['ledger']==ledger
            history=root/'history.json';history.write_bytes(n.canonical(statuses[1]['history']))
            control('sync','ordinary','--history',str(history))
            assert control('status','ordinary')['ledger']==ledger
            print(json.dumps(dict(scope='SINGLE_HOST_DEPLOYMENT_CLI_PREFLIGHT',processes=6,
                chain=config['chain_nonce'],ledger=ledger,history=statuses[1]['history'],
                signed_transfer=True,malformed_frame_rejected=True,untrusted_history_rejected=True,
                validated_sync=True,common_state_root=ledger['state_root'],host_loss_tested=False,
                ssh_transport_tested=False),indent=2))
        except BaseException:
            for log in root.glob('*.log'):print(log.name,log.read_text()[-1000:],file=sys.stderr)
            raise
        finally:
            for p in processes:
                if p.poll() is None:p.terminate()
            for p in processes:
                try:p.wait(timeout=5)
                except subprocess.TimeoutExpired:p.kill();p.wait()
            for log in logs:log.close()

if __name__=='__main__':run()
