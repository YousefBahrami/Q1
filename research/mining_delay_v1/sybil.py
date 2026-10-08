"""Resource attribution oracle, not permissionless proof of scarce capacity."""
from fractions import Fraction
import hashlib
import json


def allocate(records, assignments, budget=7200):
    """Pay stable atomic units before identity grouping; reject conflicting claims.

    assignments is an authoritative laboratory oracle, not inferred from roots.
    Each record maps a presentation key/commitment to assigned atomic units.
    """
    credited = {}
    for record in records:
        for unit in record['units']:
            if unit not in assignments or assignments[unit] != record['owner']:
                raise ValueError('UNAUTHORIZED_UNIT')
            credited[unit] = record['owner']
    units = sorted(credited)
    if not units or type(budget) is not int or budget < 0:
        raise ValueError('BUDGET')
    owners = {}
    for i, unit in enumerate(units):
        amount = budget // len(units) + (i < budget % len(units))
        owner = credited[unit]
        owners[owner] = owners.get(owner, 0) + amount
    return owners


def simulate(atomic_leaves=None):
    # Live runner supplies leaf hashes from the actual committed corpus.
    # Unit tests use explicit deterministic simulation fixtures.
    leaves = atomic_leaves or [hashlib.sha256(f'fixture-{i}'.encode()).hexdigest() for i in range(72)]
    if len(leaves) != 72 or len(set(leaves)) != 72:
        raise ValueError('ATOMIC_UNITS')
    underlying = dict(zip([f'a{i:03}' for i in range(64)]+[f'z{i:03}' for i in range(8)],leaves))
    def commit(units):
        return hashlib.sha256(json.dumps([underlying[u] for u in sorted(units)],separators=(',', ':')).encode()).hexdigest()
    assignments = {f'a{i:03}': 'large' for i in range(64)}
    assignments.update({f'z{i:03}': f'small{i}' for i in range(8)})
    base = [dict(key=f's{i}', commitment=f's{i}', owner=f'small{i}', units=[f'z{i:03}']) for i in range(8)]
    owned = [f'a{i:03}' for i in range(64)]
    cases = {}
    for keys in (1, 4, 64):
        cases[f'{keys}_keys'] = [dict(key=f'k{i}', commitment=commit(owned), owner='large', units=owned) for i in range(keys)]
    cases['split_commitments'] = [dict(key=f'k{i}', commitment=commit([u]), owner='large', units=[u]) for i, u in enumerate(owned)]
    cases['duplicated_commitments'] = [dict(key=f'k{i}', commitment=commit(owned), owner='large', units=owned) for i in range(64)]
    results = []
    for budget in (7, 71, 7200):
        baseline = allocate(base + cases['1_keys'], assignments, budget)['large']
        for name, records in cases.items():
            paid = allocate(base + records, assignments, budget)
            assert sum(paid.values()) == budget
            results.append(dict(case=name, budget=budget, large_credit=paid['large'],
                                gain=float(Fraction(paid['large'], baseline)-1),
                                presentation_commitments=[r['commitment'] for r in records],
                                naive_identity_share=len(records)/(8+len(records))))
    return dict(profile='SIMULATION_ONLY_NO_VALUE', results=results,
                actual_corpus_leaves=atomic_leaves is not None,
                underlying_commitment=commit(owned), atomic_unit_leaf_hashes=underlying,
                assumption='Trusted allocation oracle maps aliases/splits to the same atomic units. This is not proven permissionless deduplication.')
