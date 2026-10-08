# Multi-host TESTNET deployment preparation

See the [runbook and failure matrix](../../docs/testnet/Q1_MULTI_HOST_V0_RUNBOOK.md).
The tool prints a validated SSH forwarding plan and launches the existing loopback
ledger workers. It does not provision machines, connect SSH automatically or expose
Q1 RPC externally. No consensus/Rust change. Hardware must be explicitly confirmed.

```sh
python3 -m unittest discover -s research/testnet_multihost_v0 -p 'test_*.py'
python3 research/testnet_multihost_v0/smoke.py
```

The smoke command executes six **local** processes via the new deployment CLI,
submits a real signed transfer, independently verifies status through Rust,
rejects malformed framing and untrusted duplicate-vote catch-up, then synchronizes
a learner and stops all workers. It validates wiring, not SSH transport or actual
host-loss behavior. Do not label its result as a multi-host acceptance pass.

`fault_proxy.py` adds a bounded loopback test-only delay/connection-drop helper.
It changes neither host networking nor Q1 bytes. Its socket tests run alongside
configuration tests. A dropped TCP stream is not a kernel packet-loss benchmark.
Native transport remains the separate
[TESTNET_NATIVE_TRANSPORT_V0 design proposal](../../docs/protocol/Q1_TESTNET_NATIVE_TRANSPORT_V0.md).
