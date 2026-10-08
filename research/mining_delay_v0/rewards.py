"""Deterministic, dimensionless reward scenarios. No tokens, keys or ledger API."""
from fractions import Fraction
from math import isqrt

MODELS = ("equal", "proportional", "capped", "diminishing")


def weights(units: list[int], model: str) -> list[int]:
    if not units or any(type(u) is not int or not 1 <= u <= 1000000 for u in units):
        raise ValueError("SIM_UNITS")
    if model == "equal":
        return [1] * len(units)
    if model == "proportional":
        return units.copy()
    if model == "capped":
        return [min(u, 4) for u in units]
    if model == "diminishing":
        return [isqrt(u * 1000000) for u in units]  # sqrt to 0.001 weight precision
    raise ValueError("SIM_MODEL")


def apportion(scores: list[int], budget: int) -> list[int]:
    """Exact largest-remainder budget allocation; index breaks ties in this lab."""
    if type(budget) is not int or budget < 0 or not scores or any(s <= 0 for s in scores):
        raise ValueError("SIM_BUDGET")
    total = sum(scores)
    result = [budget * s // total for s in scores]
    order = sorted(range(len(scores)), key=lambda i: (-(budget * scores[i] % total), i))
    for i in order[:budget - sum(result)]:
        result[i] += 1
    assert sum(result) == budget
    return result


def metrics(rewards: list[int], owners: list[str]) -> dict:
    totals = {}
    for owner, amount in zip(owners, rewards):
        totals[owner] = totals.get(owner, 0) + amount
    amounts = sorted(totals.values())
    total, n = sum(amounts), len(amounts)
    gini = sum((2 * i - n - 1) * value for i, value in enumerate(amounts, 1))
    return dict(operator_rewards=totals,
                top_operator_share=float(Fraction(max(amounts), total)),
                small_operator_share=float(Fraction(totals.get("small-0", 0), total)),
                hardware_operator_share=float(Fraction(totals.get("hardware", 0), total)),
                gini=float(Fraction(gini, n * total)),
                hhi=float(Fraction(sum(a * a for a in amounts), total * total)))


def simulate() -> dict:
    scenarios = [
        ("balanced", [1] * 8, [f"small-{i}" for i in range(8)]),
        ("hardware_64", [1] * 8 + [64], [f"small-{i}" for i in range(8)] + ["hardware"]),
        ("hardware_64_split", [1] * 72, [f"small-{i}" for i in range(8)] + ["hardware"] * 64),
        ("hardware_256", [1] * 8 + [256], [f"small-{i}" for i in range(8)] + ["hardware"]),
    ]
    # Same amount of assumed work with unsplit/split identities; no claim that
    # any prototype proof establishes these units, acceptance or operator IDs.
    rows = []
    for scenario, units, owners in scenarios:
        for model in MODELS:
            scores = weights(units, model)
            fixed = apportion(scores, 7200)
            # Open schedule: 100 units per weight; sqrt weight is scaled above.
            scale = 1000 if model == "diminishing" else 1
            open_rewards = [100 * s // scale for s in scores]
            for schedule, allocation in [("fixed_epoch_budget", fixed),
                                         ("open_per_weight", open_rewards)]:
                issue = sum(allocation)
                rows.append(dict(scenario=scenario, model=model, schedule=schedule,
                                 resource_units=units, owners=owners,
                                 accepted_records=len(units),
                                 assumptions="one accepted record per identity per epoch; no rejection/cost model",
                                 issued_per_epoch=issue,
                                 supply_by_epoch={str(t): 100000 + t * issue for t in (0, 1, 10, 100)},
                                 **metrics(allocation, owners)))
    return dict(profile="SIMULATION_ONLY_NO_Q1_UNITS", epoch_count=100,
                initial_supply=100000, fixed_epoch_budget=7200,
                parameters_are_illustrative=True, scenarios=rows,
                limitation="No conversion of MiB or measured latency into accepted units; no prices or economic viability inferred.")
