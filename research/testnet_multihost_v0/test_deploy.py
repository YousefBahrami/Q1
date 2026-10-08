import copy
import json
from pathlib import Path
import tempfile
import unittest
import deploy as d

class DeploymentTests(unittest.TestCase):
    def config(self):
        return dict(profile=d.PROFILE,chain_nonce='11'*32,base_port=24000,bootstrap_host='a',
            hosts={x:dict(ssh_target='fixture-'+x,confirmed=True,failure_domain=x) for x in 'abc'},
            roles=dict(voter1='a',voter2='b',voter3='c',producer_a='a',producer_b='b',ordinary='c'))
    def test_placement_and_tunnel_safety(self):
        r=d.plan(self.config())
        self.assertEqual(len(r['commands']['a']),2)
        for commands in r['commands'].values():
            for cmd in commands:
                self.assertIn('StrictHostKeyChecking=yes',cmd)
                self.assertNotIn('0.0.0.0',cmd)
        self.assertTrue(all(x['quorum_survives'] and x['producer_survives'] for x in r['failure_dependence']))
    def test_common_host_loss_is_reported(self):
        c=self.config();c['hosts']['b']['failure_domain']='a'
        self.assertFalse(d.plan(c)['failure_dependence'][0]['quorum_survives'])
        self.assertFalse(d.plan(c)['failure_dependence'][0]['producer_survives'])
    def test_unknown_hardware_and_missing_roles_rejected(self):
        c=self.config();c['hosts']['a']['confirmed']=False
        with self.assertRaises(ValueError):d.plan(c)
        c=self.config();del c['roles']['voter2']
        with self.assertRaises(ValueError):d.plan(c)
    def test_shell_injection_and_bad_ports_rejected(self):
        for target in ('-oProxyCommand=evil','user@host;touch-file','$(cmd)','host space'):
            c=self.config();c['hosts']['a']['ssh_target']=target
            with self.assertRaises(ValueError):d.plan(c)
        c=self.config();c['base_port']=65535
        with self.assertRaises(ValueError):d.plan(c)
    def test_duplicate_config_keys_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)/'bad.json';p.write_text('{"profile":1,"profile":2}')
            with self.assertRaises(ValueError):d.load(p)

if __name__=='__main__':unittest.main()
