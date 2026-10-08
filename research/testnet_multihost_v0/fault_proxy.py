"""Bounded loopback-only test proxy. Never changes host networking or Q1 bytes."""
import select
import socket
import socketserver
import threading
import time


class FaultProxy:
    def __init__(self, listen_port, target_port):
        self.target = target_port
        self.delay = 0.0
        self.drop_next = 0
        self.dropped = 0
        self.delayed = 0
        self.lock = threading.Lock()
        self.slots = threading.BoundedSemaphore(16)
        self.stopped = threading.Event()
        owner = self

        class Handler(socketserver.BaseRequestHandler):
            def handle(self):
                if not owner.slots.acquire(blocking=False):
                    return
                try:
                    with owner.lock:
                        if owner.drop_next:
                            owner.drop_next -= 1
                            owner.dropped += 1
                            return
                        delay = owner.delay
                    with socket.create_connection(('127.0.0.1', owner.target), timeout=2) as upstream:
                        peers = (self.request, upstream)
                        for peer in peers:
                            peer.settimeout(2)
                        first = True
                        total = 0
                        deadline = time.monotonic() + 15
                        while not owner.stopped.is_set() and time.monotonic() < deadline:
                            readable, _, _ = select.select(peers, [], [], .1)
                            for source in readable:
                                data = source.recv(4096)
                                if not data:
                                    return
                                total += len(data)
                                if total > 2 * (65536 + 4):
                                    return
                                if source is self.request and first:
                                    first = False
                                    if delay:
                                        owner.stopped.wait(delay)
                                        with owner.lock:
                                            owner.delayed += 1
                                destination = upstream if source is self.request else self.request
                                destination.sendall(data)
                except (OSError, ValueError):
                    pass
                finally:
                    owner.slots.release()

        class Server(socketserver.ThreadingTCPServer):
            allow_reuse_address = True
            daemon_threads = True

        self.server = Server(('127.0.0.1', listen_port), Handler)
        self.port = self.server.server_address[1]
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)

    def configure(self, *, delay=0.0, drop_next=0):
        if not 0 <= delay <= 1 or type(drop_next) is not int or not 0 <= drop_next <= 16:
            raise ValueError('bounded test fault only')
        with self.lock:
            self.delay, self.drop_next = delay, drop_next

    def start(self):
        self.thread.start()
        return self

    def close(self):
        self.stopped.set()
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(timeout=2)
