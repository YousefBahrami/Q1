# Q1 experimental native transport: recovery and bounded sessions

Draft for review — not published. Q1 has added explicit recovery for uncertain
requests, durable application receipts and bounded reuse of authenticated TLS
connections in its private TESTNET research implementation.

Local tests exercise interrupted requests before receive, after receive, after a
vote and after finalization before acknowledgment, plus sender and receiver restart.
Completed work returns its recorded result; unresolved intermediate work stops
rather than guessing a new transfer. Connection caps, timeouts, message/log rate
limits, backpressure and bounded network buffers have local tests. These are basic
resource controls, not exhaustive denial-of-service protection or a security audit.

A six-role, one-machine native experiment reproduces transfers, producer failover,
two-vote progress, stale-peer catch-up and disk restart with a common finalized
state and unchanged test supply. This is **not native inter-host proof**. The earlier
two-host experiment used SSH-carried traffic. Real native inter-host acceptance
awaits an approved private path; no public Q1 protocol ports were opened.

Public Testnet remains blocked. Mining/resource uniqueness is unresolved. There
is no Mainnet, monetary issuance or token sale. New native source has not yet been
published or verified by remote CI. Participation today: review the published
LOCALNET code and use [GitHub Issues](https://github.com/YousefBahrami/Q1/issues)
for public technical questions; do not post secrets or private customer records.
