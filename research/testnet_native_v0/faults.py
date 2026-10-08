"""Local test-only barriers: no wire-triggered authority, no production feature."""
import os
from pathlib import Path
import time


def barrier(stage):
    directory=os.environ.get('Q1_NATIVE_TEST_BARRIERS')
    if not directory:return
    root=Path(directory);trigger=root/(stage+'.arm')
    if not trigger.exists():return
    trigger.unlink()  # one shot
    (root/(stage+'.reached')).write_text(stage)
    end=time.monotonic()+10
    while not (root/(stage+'.release')).exists():
        if time.monotonic()>end:raise RuntimeError('TEST_BARRIER_TIMEOUT')
        time.sleep(.005)
