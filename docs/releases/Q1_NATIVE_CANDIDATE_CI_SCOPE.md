# Native research candidate: reproducibility and CI scope

2026-10-08. Remote-CI candidate, not a completed remote run. Use the exact reviewed
candidate commit supplied with the source; the released LOCALNET tag remains unchanged.

From a fresh checkout outside any other Cargo workspace, install the pinned Rust,
Python 3.13.3, Node 20.17.0 and OpenSSL 3.x, then run:

```sh
python3 scripts/check_all.py
```

The GitHub workflow uses Ubuntu and the same complete script: Rust tests, fmt,
Clippy with warnings denied, all-target build, Rustdoc, protocol/LOCALNET/native
reference vectors, native Python integration/negative tests and actual loopback
LOCALNET/research acceptance. It needs process spawning, Unix sockets, loopback
TCP, file locks, temporary disk and OpenSSL; no secrets, SSH/VPN or private host
credentials. No native test is silently omitted. The timeout is 40 minutes;
failure remains a failure. Dependency/toolchain access and hosted-runner timing
still need the actual remote run, which has not been started by preparation.

Native-only checks after building:

```sh
python3 -m unittest discover -s research/testnet_native_v0 -p 'test_*.py'
python3 research/testnet_native_v0/vectors.py
python3 research/testnet_native_v0/run.py --output /tmp/q1-native-review-evidence
python3 research/testnet_native_v0/replay.py /tmp/q1-native-review-evidence
```

Use a new output directory. The runner terminates its own lab processes. Fresh
keys/genesis produce a different root each run; compare within-run roots and fixed
ledger semantics: height 5, sender 945, recipient 50, nonce 5, reward pool 5,
supply 1000. Original LOCALNET acceptance has its separate fixed fixture/root.

GitHub's single job cannot prove native traffic between the actual private hosts,
private-route loss/reconnect, three independent voter machines, power-loss recovery,
real customer use, permissionless resource economics or public-network safety.
Those gates stay open. Multiple local processes or hosted CI jobs are not substituted
for the prescribed physical-host experiment. Passing CI is a source-review signal,
not a public-testnet launch or security certification.
