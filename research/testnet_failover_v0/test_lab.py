import copy
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch
from lab import (Worker, Rejected, signed, body, value, chosen_from, check_cert,
                 check_votes, canonical, decode, MAX_FRAME, hid)

class ProtocolTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory()
        self.workers=[Worker(Path(self.tmp.name)/str(i),i,[0]*6) for i in range(3)]
        self.seq=0
        self.v=value(3,1,0,'0'*64,'synthetic-work-A')
    def tearDown(self):
        for w in self.workers: w.close()
        self.tmp.cleanup()
    def req(self,who,kind,**kw):
        self.seq+=1
        return signed(who,body(kind,seq=self.seq,**kw))
    def prepare(self,ballot=0,who=3):
        return [w.apply(self.req(who,'prepare',height=1,ballot=ballot)) for w in self.workers]
    def accept(self,v=None,ballot=0,who=3,promises=None):
        promises=promises or self.prepare(ballot,who)
        return [w.apply(self.req(who,'accept',height=1,ballot=ballot,value=v or self.v,promises=promises)) for w in self.workers]
    def test_chosen_but_unannounced_adoption_after_restart(self):
        self.accept()
        for i,w in enumerate(self.workers):
            w.close(); self.workers[i]=Worker(Path(self.tmp.name)/str(i),i,[0]*6)
        promises=self.prepare(1,4)
        self.assertEqual(chosen_from(promises,1,1,'0'*64),self.v)
        conflict=value(4,1,1,'0'*64,'different')
        with self.assertRaisesRegex(Rejected,'ADOPTION_REQUIRED'):
            self.workers[0].apply(self.req(4,'accept',height=1,ballot=1,value=conflict,promises=promises))
        self.accept(ballot=1,who=4,promises=promises)
    def test_equivocation_and_late_producer(self):
        promises=self.prepare(); self.accept(promises=promises)
        conflict=value(3,1,0,'0'*64,'B')
        with self.assertRaisesRegex(Rejected,'EQUIVOCATION'):
            self.workers[0].apply(self.req(3,'accept',height=1,ballot=0,value=conflict,promises=promises))
        self.prepare(1,4)
        with self.assertRaisesRegex(Rejected,'STALE_BALLOT'):
            self.workers[0].apply(self.req(3,'prepare',height=1,ballot=0))
    def test_duplicate_unauthorized_invalid_signature_wrong_proposal(self):
        votes=self.accept()
        for changed,reason in [([votes[0],votes[0]],'DUPLICATE_VOTE'),
            ([votes[0],signed(5,votes[1]['body'])],'UNAUTHORIZED_VOTER'),
            ([votes[0],dict(votes[1],signature='00'*64)],'INVALID_SIGNATURE'),
            ([votes[0],signed(1,dict(votes[1]['body'],value_id='0'*64))],'DIFFERENT_PROPOSAL')]:
            with self.assertRaisesRegex(Rejected,reason): check_votes(changed,self.v,1,0)
    def test_stale_future_and_replay_survive_restart(self):
        for h in (0,2):
            with self.assertRaises(Rejected):
                self.workers[0].apply(self.req(3,'prepare',height=h,ballot=0))
        req=self.req(3,'prepare',height=1,ballot=0)
        self.workers[0].apply(req)
        self.workers[0].close(); self.workers[0]=Worker(Path(self.tmp.name)/'0',0,[0]*6)
        with self.assertRaisesRegex(Rejected,'REPLAYED_MESSAGE'): self.workers[0].apply(req)
    def test_malformed_certificate_oversize_noncanonical(self):
        for obj in ({},dict(value=self.v,height=1,ballot=0,votes=[1,2])):
            with self.assertRaises(Rejected): check_cert(obj,1,'0'*64)
        for raw in (b'x'*(MAX_FRAME+1),b'{"a":1,"a":2}',b'{} ',b'['*30+b'0'+b']'*30):
            with self.assertRaises(Rejected): decode(raw)
    def test_invalid_sync_rolls_back_and_double_finality_detected(self):
        votes=self.accept(); cert=dict(value=self.v,height=1,ballot=0,votes=votes)
        bad=copy.deepcopy(cert); bad['height']=2
        with self.assertRaises(Rejected):
            self.workers[0].apply(self.req(5,'sync',history=[cert,bad]))
        self.assertEqual(self.workers[0].height,1)
        self.workers[0].apply(self.req(5,'sync',history=[cert]))
        conflict=value(3,1,0,'0'*64,'B')
        # Malicious quorum fixture demonstrates detection, not Byzantine tolerance.
        cv=[signed(i,body('vote',height=1,ballot=0,value_id=hid(conflict))) for i in (0,1)]
        with self.assertRaisesRegex(Rejected,'CONFLICTING_FINALITY'):
            self.workers[0].apply(self.req(5,'sync',history=[dict(cert,value=conflict,votes=cv)]))
    def test_no_ack_on_failed_persistence_and_corrupt_store_rejected(self):
        with patch('lab.os.replace',side_effect=OSError('injected')):
            with self.assertRaises(OSError): self.workers[0].apply(self.req(3,'prepare',height=1,ballot=0))
        self.assertTrue(self.workers[0].poisoned)
        self.workers[0].close(); self.workers[0]=Worker(Path(self.tmp.name)/'0',0,[0]*6)
        self.assertEqual(self.workers[0].state['promised'],-1)
    def test_same_height_other_chain_rejected(self):
        req=self.req(3,'prepare',height=1,ballot=0)
        req=signed(3,dict(req['body'],chain='0'*64))
        with self.assertRaisesRegex(Rejected,'PROFILE'): self.workers[0].apply(req)

    def test_unchosen_minority_does_not_deadlock_new_majority(self):
        promises=self.prepare()
        self.workers[0].apply(self.req(3,'accept',height=1,ballot=0,value=self.v,promises=promises))
        next_promises=[w.apply(self.req(4,'prepare',height=1,ballot=1)) for w in self.workers[1:]]
        self.assertIsNone(chosen_from(next_promises,1,1,'0'*64))
        alternate=value(4,1,1,'0'*64,'B')
        for w in self.workers[1:]:
            w.apply(self.req(4,'accept',height=1,ballot=1,value=alternate,promises=next_promises))
        # Every subsequent majority intersects the now-chosen higher ballot.
        p2=self.prepare(2,3)
        for pair in ((0,1),(0,2),(1,2)):
            self.assertEqual(chosen_from([p2[i] for i in pair],1,2,'0'*64),alternate)
        self.accept(v=alternate,ballot=2,who=3,promises=p2)

    def test_corrupt_journal_fails_closed(self):
        self.workers[0].close()
        (Path(self.tmp.name)/'0/journal.json').write_bytes(b'{broken')
        with self.assertRaises(Rejected): Worker(Path(self.tmp.name)/'0',0,[0]*6)

if __name__=='__main__': unittest.main()
