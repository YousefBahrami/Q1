"""Private Unix-socket bridge to existing TESTNET coordinator; no TCP listener."""
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[1]/'testnet_ledger_v0'))
import node
from lab import call_native
from recovery import RecoverableWorker


def main(config_path):
    config_path = Path(config_path)
    c = json.loads(config_path.read_bytes()); who = c['who']
    # Ports are opaque peer indices to this unchanged coordinator interface.
    unavailable = set()
    def rpc(target, request):
        node.need(target not in unavailable, "NATIVE_PEER_UNAVAILABLE_THIS_OPERATION")
        p = call_native(config_path,target,request,node.authentic)
        if p.returncode:
            print(json.dumps(dict(event='outgoing_failure',peer=target,detail=p.stderr.decode()[:600])),file=sys.stderr,flush=True)
            unavailable.add(target)
            raise node.Rejected('NATIVE_TRANSPORT_UNCERTAIN')
        result = node.decode(p.stdout)
        node.need('error' not in result, result.get('error', 'REMOTE'))
        node.need(result['signer']==target, 'RESPONSE_SIGNER')
        node.authentic(result)
        return result
    node.rpc = rpc
    worker = RecoverableWorker(config_path.parent/'ledger', who, list(range(6)))
    stopped = False
    def stop(_sig, _frame):
        nonlocal stopped
        stopped = True
    signal.signal(signal.SIGTERM, stop)
    with socket.socket(socket.AF_UNIX) as server:
        server.bind(c['worker']); os.chmod(c['worker'], 0o600); server.listen(8); server.settimeout(.2)
        try:
            while not stopped:
                try: connection, _ = server.accept()
                except TimeoutError: continue
                with connection:
                    connection.settimeout(20)
                    unavailable.clear()
                    try:
                        request=node.receive(connection)
                        if set(request)=={'native_reconcile'}:answer=worker.reconcile(request['native_reconcile'])
                        else:answer=worker.apply(request)
                    except (node.Rejected, KeyError, TypeError, ValueError, AttributeError, IndexError) as e:
                        answer = dict(error=str(e) if isinstance(e, node.Rejected) else 'SCHEMA')
                    try:node.send(connection, answer)
                    except OSError:pass  # outcome is already journaled; caller can reconcile
        finally:
            worker.close(); Path(c['worker']).unlink(missing_ok=True)

if __name__ == '__main__': main(sys.argv[1])
