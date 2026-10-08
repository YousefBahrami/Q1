"""Accounting hypothesis only: authorized useful-service jobs, never Q1 issuance.

Buyer authorization/assignment and service verification are external ORACLES here.
Passing this model does not solve permissionless resource identity or implement a
market, signature verifier, settlement protocol, reward rule or consensus weight.
"""
from fractions import Fraction
import json


def evaluate():
    attacks=('duplicate_commitments','shared_storage','outsourcing','leasing','reconstruction','useful_delivery')
    results=[]
    for model in attacks:
        for identities in (1,2,4,8):
            # One independently authorized unit of demand; aliases can submit the
            # same verified receipt, but cannot authorize additional buyer demand.
            settled=set();paid=0
            for alias in range(identities):
                receipt=('buyer-authorized-job-1','unit-1')
                if receipt not in settled:paid+=10;settled.add(receipt)
            assert paid==10
            results.append(dict(model=model,identities=identities,paid_units=paid,
                per_key_lottery_share=str(Fraction(identities,8+identities))))
    # A global budget alone still permits a larger Sybil share under key lotteries.
    assert Fraction(8,16)>Fraction(1,9)
    return dict(scope='ANALYTICAL ACCOUNTING MODEL ONLY',cases=results,
        dependency='externally authorized demand/assignment and verified receipt oracle',
        legitimate_extra_job='additional useful demand can earn another payment regardless of identity count',
        unresolved=['permissionless physical uniqueness','buyer Sybils','service verification','assignment fairness','correlated outsourcing failure','disputes'],
        consensus_voting_power_changed=False,real_issuance=False)

if __name__=='__main__':print(json.dumps(evaluate(),indent=2))
