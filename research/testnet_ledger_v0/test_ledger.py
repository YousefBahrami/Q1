"""Adversarial checks against the actual Rust execution/certificate backend."""
import copy
import json
import tempfile
import unittest
from pathlib import Path
import node as n

class LedgerTests(unittest.TestCase):
    def proposal(self,amount=10): return n.backend('propose',history=[],who=3,ballot=0,amount=amount)
    def certificate(self,v):
        return dict(height=1,ballot=0,value=v,votes=[n.signed(i,n.body('vote',height=1,ballot=0,value_id=n.hid(v))) for i in (0,1)])
    def test_accounting_and_replay(self):
        v=self.proposal(); r=n.backend('status',history=[self.certificate(v)])
        self.assertEqual((r['sender'],r['recipient'],r['reward_pool'],r['total_supply'],r['nonce']),('989','10','1','1000',1))
        self.assertEqual(n.backend('validate',history=[],value=v),r)
    def test_root_and_body_tampering(self):
        for field in ('state_root','block_body'):
            v=self.proposal(); b=copy.deepcopy(v['body']); p=json.loads(b['payload'])
            p[field]='00'*32; b['payload']=n.canonical(p).decode()
            with self.assertRaises(n.Rejected): n.backend('validate',history=[],value=n.signed(3,b))
    def test_certificate_authentication(self):
        c=self.certificate(self.proposal())
        variants=[dict(c,votes=c['votes'][:1]),dict(c,votes=[c['votes'][0]]*2)]
        forged=copy.deepcopy(c); forged['votes'][1]['signature']='00'*64;variants.append(forged)
        outsider=copy.deepcopy(c);outsider['votes'][1]=n.signed(5,c['votes'][1]['body']);variants.append(outsider)
        for variant in variants:
            with self.assertRaises(n.Rejected): n.backend('status',history=[variant])
    def test_chain_profile_ballot_and_balance(self):
        v=self.proposal()
        for key,val in [('chain','ff'*32),('profile','LOCALNET_V0'),('origin_ballot',2**64-1)]:
            b=dict(v['body']);b[key]=val
            with self.assertRaises(n.Rejected): n.backend('validate',history=[],value=n.signed(3,b))
        with self.assertRaises(n.Rejected): self.proposal(1000)
        with self.assertRaises(n.Rejected): self.proposal(0)
    def test_invalid_proposal_does_not_reserve_or_charge(self):
        with tempfile.TemporaryDirectory() as tmp:
            w=n.Worker(tmp,0,[0]*6)
            try:
                p=[n.signed(i,n.body('promise',height=1,ballot=0,accepted=None)) for i in (0,1)]
                v=self.proposal();b=dict(v['body']);payload=json.loads(b['payload']);payload['state_root']='00'*32;b['payload']=n.canonical(payload).decode()
                before=(Path(tmp)/'journal.json').read_bytes()
                req=n.signed(3,n.body('accept',seq=1,height=1,ballot=0,value=n.signed(3,b),promises=p))
                with self.assertRaises(n.Rejected): w.apply(req)
                self.assertEqual(before,(Path(tmp)/'journal.json').read_bytes())
                self.assertIsNone(w.state['accepted'])
            finally: w.close()
    def test_corrupt_persisted_root_fails_closed(self):
        with tempfile.TemporaryDirectory() as tmp:
            w=n.Worker(tmp,0,[0]*6);w.close();p=Path(tmp)/'journal.json'
            s=json.loads(p.read_text());s['state_root']='00'*32;p.write_bytes(n.canonical(s))
            with self.assertRaises(n.Rejected): n.Worker(tmp,0,[0]*6)

if __name__=='__main__': unittest.main()
