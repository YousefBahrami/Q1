"""Failure reporting and process cleanup for the public reproduction wrapper."""
import contextlib
import io
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import validate_public_q1 as validator


class ValidationTests(unittest.TestCase):
    def run_failure(self, codes, malformed=False):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            if malformed:
                (output / "localnet").mkdir()
                (output / "localnet/acceptance.json").write_text('{}')
            with patch.object(validator, "capture", side_effect=["a" * 40, "", "rustc 1.97.1 (test)"]), \
                 patch.object(validator.shutil, "which", return_value="tool"), \
                 patch.object(validator, "run_stage", side_effect=codes) as runner, \
                 contextlib.redirect_stdout(io.StringIO()):
                code = validator.validate(output)
            return code, json.loads((output / "summary.json").read_text()), runner.call_count

    def test_failure_is_preserved_and_later_stages_are_skipped(self):
        code, report, count = self.run_failure([7])
        self.assertEqual(code, 1)
        self.assertEqual(report["stages"], {"build": "FAIL", "rust_tests": "SKIPPED", "localnet": "SKIPPED"})
        self.assertEqual(count, 1)
        self.assertFalse(report["localnet_completed"])

    def test_zero_exit_without_valid_evidence_is_failure(self):
        code, report, count = self.run_failure([0, 0, 0], malformed=True)
        self.assertEqual(code, 1)
        self.assertEqual(count, 3)
        self.assertEqual(report["failure_stage"], "localnet")
        self.assertEqual(report["result"], "FAIL")

    def test_interrupt_is_reported(self):
        code, report, _ = self.run_failure([KeyboardInterrupt()])
        self.assertEqual(code, 130)
        self.assertEqual(report["result"], "FAIL")

    def test_unsupported_has_no_false_pass(self):
        with tempfile.TemporaryDirectory() as directory, \
             patch.object(validator.platform, "system", return_value="Windows"), \
             contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(validator.validate(Path(directory)), 2)
            report = json.loads((Path(directory) / "summary.json").read_text())
            self.assertEqual(report["result"], "UNSUPPORTED")
            self.assertEqual(set(report["stages"].values()), {"SKIPPED"})

    def test_existing_directory_is_not_overwritten(self):
        with tempfile.TemporaryDirectory() as directory, \
             patch.object(sys, "argv", ["validate", "--output-dir", directory]), \
             contextlib.redirect_stderr(io.StringIO()):
            marker = Path(directory) / "summary.json"
            marker.write_text('original')
            self.assertEqual(validator.main(), 1)
            self.assertEqual(marker.read_text(), 'original')

    @unittest.skipUnless(os.name == "posix", "POSIX process groups required")
    def test_timeout_terminates_child(self):
        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory) / "timeout.log"
            command = [sys.executable, "-c", "import os,time; print(os.getpid(),flush=True); time.sleep(120)"]
            with self.assertRaises(subprocess.TimeoutExpired):
                validator.run_stage(command, log, timeout=1)
            pid = int(log.read_text().splitlines()[0])
            with self.assertRaises(ProcessLookupError):
                os.kill(pid, 0)


if __name__ == "__main__":
    unittest.main()
