import socket
import socketserver
import threading
import time
import unittest
from fault_proxy import FaultProxy


class ProxyTest(unittest.TestCase):
    def setUp(self):
        class Echo(socketserver.BaseRequestHandler):
            def handle(self):
                data = self.request.recv(128)
                if data:
                    self.request.sendall(data)
        self.server = socketserver.ThreadingTCPServer(('127.0.0.1', 0), Echo)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()
        self.proxy = FaultProxy(0, self.server.server_address[1]).start()

    def tearDown(self):
        self.proxy.close()
        self.server.shutdown()
        self.server.server_close()
        self.thread.join()

    def exchange(self):
        with socket.create_connection(('127.0.0.1', self.proxy.port), timeout=2) as s:
            s.sendall(b'unchanged signed-frame fixture')
            return s.recv(128)

    def test_delay_preserves_bytes_and_is_removable(self):
        self.proxy.configure(delay=.05)
        start = time.monotonic()
        self.assertEqual(self.exchange(), b'unchanged signed-frame fixture')
        self.assertGreaterEqual(time.monotonic() - start, .045)
        self.proxy.configure()
        self.assertEqual(self.exchange(), b'unchanged signed-frame fixture')

    def test_one_connection_drop_then_reconnect(self):
        self.proxy.configure(drop_next=1)
        try:
            self.assertEqual(self.exchange(), b'')
        except ConnectionResetError:
            pass
        self.assertEqual(self.exchange(), b'unchanged signed-frame fixture')
        self.assertEqual(self.proxy.dropped, 1)


if __name__ == '__main__':
    unittest.main()
