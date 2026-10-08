# Q1 mining / delay experiment v0

Standalone Python standard-library research module, explicitly authorized on
2026-10-04. It is not imported by Rust, LOCALNET, genesis, networking or any
economic state transition. Removing this directory cannot change consensus.
There is no signing key, payout, token, RPC, automatic enrollment or background
process. The name describes a research question, not implemented mining.

See the [hypotheses and measured report](../../docs/research/Q1_MINING_DELAY_EXPERIMENT_V0.md)
and [recorded results](../../docs/reports/research/mining-delay-v0-2026-10-04/results.json).

## Reproduce

From the repository root, with Python 3.13 (tested 3.13.3), run:

```sh
python3 -m unittest discover -s research/mining_delay_v0 -p 'test_*.py' -v
python3 research/mining_delay_v0/run.py run --output /tmp/q1-resource-results
python3 research/mining_delay_v0/run.py verify --manifest /tmp/q1-resource-results/manifest.json --context /tmp/q1-resource-results/context.json --evidence /tmp/q1-resource-results/evidence.json
```

Choose a **new** output directory; an existing directory is rejected, never
overwritten. Default scratch space is the OS temporary directory. To run on an
available HDD later, explicitly pass `--scratch-parent /path/to/ordinary/test-dir`
for an existing directory on that volume. This does not prove device type.
Do not point at a raw device, production dataset or keys. No root permission,
cache flushing, direct I/O, stress workload, formatting or network service is
used. Ctrl-C exits and the temporary-directory context cleans its own dataset;
partial output is not a successful run. Abrupt process/host failure can leave a
small temporary directory, never a restartable daemon.

One benchmark run generates exactly 21 MiB (1 + 4 + 16 MiB) of dataset content,
one file at a time, and removes it. The parent volume must report at least
512 MiB free. The underlying module refuses datasets over 64 MiB; the runner
offers only the three smaller fixed sizes. Existing file/symlink overwrite is
rejected. Report files add less than a few MiB. Unit tests use tiny temporary
datasets. These are controlled writes with nonzero wear, not destructive tests.

## Exact laboratory choices

These instantiate historical dataset generation and random-read/transform/
commit ideas from [HDD §§14, 19, 26](../../docs/07_HDD_LAB_MODULE.md).
They do not allocate Q1 cryptographic domains or approve production primitives.

- Frame SHA-256 inputs as an eight-byte big-endian length followed by each
  component, including `Q1_RESOURCE_LAB_V0_NOT_CONSENSUS` and a purpose label.
- Expand the dataset seed/participant/index digest to a 4096-byte chunk with
  SHAKE-256. This is a reproducible lab instantiation of the draft's unspecified
  EXPAND operation, not a new proof-of-storage construction or formal VDF.
- Use power-of-two dataset/challenge sample counts to avoid inventing odd-tree
  semantics. Leaves bind index and chunk; internal nodes use separate labels.
  A manifest commits the participant, public seed, size and dataset root.
- Challenge binds an explicit lab chain, height, round, producer, nonce and
  manifest commitment. Fixed fixture nonces reproduce results; they do not
  simulate unpredictable live challenges.
- Select 64 indices by hash modulo chunk count; duplicate indices remain
  permitted as recommended in the recovered first experiment. Each response
  binds challenge, sample ordinal, chunk index and data, with a response root.
- Evidence includes chunk bytes and Merkle paths. Verification against **pinned
  external** manifest/context checks schema, bounds, deterministic generation,
  membership, selected indices and response commitment. It cannot establish that
  the rest of a dataset was ever stored or that a disk read occurred.
- JSON is a lab transport (not Q1 canonical CBOR). Output serialization is
  deterministic; verification rejects duplicate/extra fields, bad integer types,
  over-limit bytes/counts, malformed paths/data and wrong context. There is no
  application-level registry, admission, anti-replay reward ledger or protocol
  signature. Rechecking the same valid evidence under the same expected context
  is intentionally repeatable and must not be interpreted as new earned work.

## Measurements and adversarial substitution

Three trials per size/mode measure proof construction including serialization,
verification separately, process CPU, logical sampled bytes, evidence volume,
generation time, retained digest bytes and process peak RSS. Python allocations
add overhead beyond raw digest storage. OS input/output block counters are
recorded but are neither actual physical bytes nor trusted proof.

Modes: OS-buffered file reads immediately after generation; preloaded RAM; and
on-demand generation **after deleting the dataset file**, retaining Merkle hash
metadata. The evidence must be byte-identical across modes at each size. RAM
preload I/O is reported separately; dataset/tree construction costs are not
included in per-proof times. No cold-cache or network-latency claim is made.

`results.json` records source hashes, versions, raw trials and simulation inputs;
`manifest.json`, `context.json`, `evidence.json` form a standalone verification
fixture. `SHA256SUMS` covers these artifacts. Timing/UTC/RSS vary on rerun;
manifest, selected indices, evidence bytes and simulated outcomes must repeat
under identical inputs. Source hashes bind the measured implementation.

The verifier command runs in a separate process without a dataset, but uses the
same implementation. This is reproducibility, not independent cryptographic
review or an independent verifier implementation.

## Reward simulation

`rewards.py` uses artificial dimensionless resource and accounting units, never
Q1 balances. Four models compare weights per accepted identity-record: equal 1,
proportional units, cap at 4, and square root (integer precision 0.001).
All assume one accepted record per identity per epoch; these acceptance and
resource-unit assumptions are **not established by the proof**.

Scenarios: eight small operators; those eight plus a 64-unit operator; the same
64 units split into 64 identities of that operator; and a 256-unit operator.
Aggregation uses known simulation owners so splitting cannot hide concentration.
Two issuance schedules expose a separate decision: fixed 7200-unit epoch budget,
or 100 units per weight. Initial artificial supply 100000, 100 epochs. These
numbers are experimental controls, not proposed monetary policy.

Largest-remainder integer apportionment preserves the fixed budget, with index
tie-breaking for this laboratory. Open-schedule fractional weights are floored
per identity before issuance; rounding is reported in resulting totals. No real
treasury, burn, founder allocation or payout is added. Logical epochs have no
conversion to blocks or elapsed time. Results include owner shares, Gini, HHI,
small-operator participation and supply at epochs 0/1/10/100.

Tests include evidence forgery/context replay/corruption/schema limits,
regeneration substitution, no overwrite, deterministic results, exact budget/
supply and cap evasion by identity splitting. The normal check script runs
these tiny tests, not the benchmark.
