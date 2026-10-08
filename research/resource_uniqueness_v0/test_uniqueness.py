import copy
import unittest
import experiment as e

class AccountingTests(unittest.TestCase):
    def claims(self, n):
        levels=e.tree([e.content(i) for i in range(e.COUNT)])
        return [e.attempt(e.content,levels,f'key{i}','11'*32) for i in range(n)]
    def test_same_resource_more_keys_is_counterexample(self):
        c=self.claims(8);s=e.score(c)
        self.assertEqual(s['per_identity'],8)
        self.assertEqual(s['unique_data_commitment'],1)
        self.assertEqual(s['claimed_region'],8)
        self.assertEqual(s['self_issued_lease'],8)
        self.assertEqual(s['authorized_service_job'],1)
        self.assertEqual(s['self_issued_service_job'],8)
    def test_layout_alias_defeats_root_only_accounting(self):
        c=self.claims(2)
        order=list(reversed(range(e.COUNT)))
        levels=e.tree([e.content(i) for i in order])
        c[1]=e.attempt(lambda i:e.content(order[i]),levels,'alias','22'*32)
        self.assertEqual(e.score(c)['unique_data_commitment'],2)
    def test_failed_claims_and_repeated_key_do_not_count(self):
        c=self.claims(1); c.append(copy.deepcopy(c[0]))
        self.assertEqual(e.score(c)['per_identity'],1)
        for row in c:row['accepted']=False
        self.assertEqual(set(e.score(c).values()),{0})
    def test_compressed_reader_has_identical_content(self):
        raw=b''.join(e.content(i) for i in range(e.COUNT));compressed=e.zlib.compress(raw)
        self.assertLess(len(compressed),len(raw))
        self.assertEqual(e.zlib.decompress(compressed),raw)

if __name__=='__main__':unittest.main()
