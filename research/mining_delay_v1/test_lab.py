import copy
import tempfile
import unittest
from pathlib import Path
from lab import (CHUNK, Rejected, Verifier, canonical, challenge, decode, digest,
                 full_recovery, manifest, proof, tree, verify)
from sybil import allocate, simulate

class EvidenceTests(unittest.TestCase):
    def setUp(self):
        # REPLAY FIXTURE ONLY, predictable bytes never a live challenge trial.
        self.data = [digest('fixture', bytes([i])) * (CHUNK//32) for i in range(64)]
        self.levels = tree(self.data)
        self.m = manifest('11'*32, self.levels[-1][0].hex(), 64, '22'*32)
        self.q = challenge(self.m, '33'*32, 1, 16)
        self.raw = proof(self.m, self.q, self.levels, self.data.__getitem__)

    def test_roundtrip_and_full_recovery(self):
        self.assertEqual(verify(self.raw, self.m, self.q), verify(self.raw, self.m, self.q))
        self.assertEqual(full_recovery(self.m, self.data.__getitem__), self.m['root'])

    def test_commit_before_fresh_challenge_and_single_terminal_result(self):
        v = Verifier(self.m)
        with self.assertRaisesRegex(Rejected, 'NOT_COMMITTED'): v.issue()
        v.commit(self.m)
        q = v.issue(16)
        with self.assertRaisesRegex(Rejected, 'OUTSTANDING'): v.issue()
        v.finish(proof(self.m, q, self.levels, self.data.__getitem__))
        with self.assertRaisesRegex(Rejected, 'REPLAY'): v.finish(self.raw)
        self.assertNotEqual(q['nonce'], v.issue(16)['nonce'])

    def test_substituted_assignment(self):
        v = Verifier(self.m)
        wrong = dict(self.m, root='00'*32)
        with self.assertRaisesRegex(Rejected, 'ASSIGNMENT'): v.commit(wrong)

    def test_wrong_deleted_partial_data(self):
        for reader in (lambda i: b'', lambda i: b'0'*CHUNK):
            with self.assertRaises(Rejected):
                verify(proof(self.m, self.q, self.levels, reader), self.m, self.q)
            with self.assertRaises(Rejected): full_recovery(self.m, reader)
        # A missing unsampled chunk passes sampling but cannot pass full retrieval.
        missing = next(i for i in range(64) if i not in self.q['indices'])
        reader = lambda i: b'' if i == missing else self.data[i]
        verify(proof(self.m, self.q, self.levels, reader), self.m, self.q)
        with self.assertRaises(Rejected): full_recovery(self.m, reader)

    def test_cross_challenge_context(self):
        for q in (challenge(self.m, '44'*32, 1, 16), challenge(self.m, '33'*32, 2, 16)):
            with self.assertRaises(Rejected): verify(self.raw, self.m, q)

    def test_corruption_index_path_and_length(self):
        original = decode(self.raw)
        for field, value in [('index', 999), ('data', '00'), ('path', []), ('index', True)]:
            p = copy.deepcopy(original)
            p['rows'][0][field] = value
            with self.assertRaises(Rejected): verify(canonical(p), self.m, self.q)
        p = copy.deepcopy(original)
        p['rows'][0]['data'] = '00'*CHUNK
        with self.assertRaisesRegex(Rejected, 'MEMBERSHIP'): verify(canonical(p), self.m, self.q)

    def test_canonical_and_parser_bounds(self):
        for raw in (self.raw+b' ', b'{"a":1,"a":2}', b'['*30+b'0'+b']'*30,
                    b'x'*(2*1024*1024+1), b'{"a":1.1}', b'{"a":true}'):
            with self.assertRaises(Rejected): decode(raw)

    def test_timeout_and_missing_response(self):
        now = [0]
        v = Verifier(self.m, 10, lambda: now[0])
        v.commit(self.m); v.issue(16); now[0] = 11
        with self.assertRaisesRegex(Rejected, 'TIMEOUT'): v.finish(self.raw)
        v.issue(16)
        with self.assertRaisesRegex(Rejected, 'MISSING_RESPONSE'): v.finish(None)

    def test_sybil_partitions_duplicates_rounding_and_authority(self):
        self.assertTrue(all(row['gain'] == 0 for row in simulate()['results']))
        with self.assertRaisesRegex(ValueError, 'UNAUTHORIZED_UNIT'):
            allocate([dict(owner='attacker', units=['a'])], {'a':'honest'})
        one = [dict(owner='a', units=['x','y']),dict(owner='b',units=['z'])]
        split = [dict(owner='a', units=['x']),dict(owner='a',units=['y'])]+one
        for budget in range(1, 100):
            self.assertEqual(allocate(one,{'x':'a','y':'a','z':'b'},budget),
                             allocate(split,{'x':'a','y':'a','z':'b'},budget))

if __name__ == '__main__': unittest.main()
