# Native CI rate-limit workload correction

2026-10-08. Classification: **PLATFORM/PORTABILITY DEFECT in the test workload**.
The [first public CI run](https://github.com/YousefBahrami/Q1/actions/runs/37720489659)
for source commit 05c60fe failed one of 30 native Python tests: the peer/frame-limit
case expected closure after 80 send-then-receive operations. Prior Rust, vectors,
LOCALNET and other research checks passed; the later native acceptance/docs stages
were not reached in that run. The original failure remains recorded.

Sending one request only after receiving its predecessor is limited by round-trip
latency. It does not guarantee exceeding a 32-frame/second window. A local diagnostic
with 40ms per client send reproduced the same old assertion failure: fewer than 32
requests/second need not trigger the limiter. The exact Linux latency cause was not
instrumented, so no kernel/TCP-specific root cause is asserted beyond the flawed
load-generation assumption. Existing Rust tests exercise the exact limiter boundary.

The correction sends the same 80 bounded duplicate frames in one pipelined burst,
then drains replies until rejection. It still requires closure before all replies,
checks successful first dispatch/replay rejection thereafter, treats a timeout as
failure, and additionally confirms another identity progresses after the burst.
No rate limit, production code, deadline or consensus rule is weakened or changed.
No test is skipped. A separate new commit preserves the original public history.
Local and remote outcomes for the correction must be recorded after actual execution;
this note itself is not evidence of a successful remote rerun.

Local validation: all ten recovery/pool tests passed after the correction. The
corrected case also passed with the same 40ms client pacing diagnostic that
reproduced the old failure. Corrective commit `735b3fe` passed the [full public CI rerun](https://github.com/YousefBahrami/Q1/actions/runs/37835065229), including all 30 native Python tests and one-host native acceptance. The original failed run remains visible.
