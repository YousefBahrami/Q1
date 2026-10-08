import unittest
from service_accounting import evaluate

class AccountingBoundaryTests(unittest.TestCase):
    def test_all_six_models_and_four_key_counts(self):
        r=evaluate();self.assertEqual(len(r['cases']),24)
        self.assertEqual({x['paid_units'] for x in r['cases']},{10})
        self.assertFalse(r['consensus_voting_power_changed']);self.assertFalse(r['real_issuance'])
    def test_budget_cap_is_not_permissionless_uniqueness(self):
        r=evaluate();self.assertIn('assignment fairness',r['unresolved'])
        self.assertEqual(r['cases'][0]['per_key_lottery_share'],'1/9')
        self.assertEqual(r['cases'][3]['per_key_lottery_share'],'1/2')
