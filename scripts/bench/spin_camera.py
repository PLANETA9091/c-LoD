#!/usr/bin/env python3
"""Spin the bench client's camera via XTest pointer motion (mouse-look).
Usage: spin_camera.py <duration_s> [step_px=140] [interval_s=0.12]
The game grabs the pointer; fake motion events act like real mouse moves.
Centers on 640,360 (the 1280x720 Xvfb screen), sweeps left<->right."""
import sys, time
from Xlib import display, X
from Xlib.ext import xtest

d = display.Display(":99")
dur = float(sys.argv[1]) if len(sys.argv) > 1 else 20.0
step = int(sys.argv[2]) if len(sys.argv) > 2 else 140
iv = float(sys.argv[3]) if len(sys.argv) > 3 else 0.12

t0 = time.time()
i = 0
# XTEST MotionNotify events here are RELATIVE (detail=1): the game turns the
# camera by per-event deltas. Sweep yaw only (dy=0) so pitch stays level —
# absolute-coordinate fakes made every event carry +360px of pitch.
while time.time() - t0 < dur:
    dx = step if (i // 6) % 2 == 0 else -step  # sweep direction every 6 steps
    xtest.fake_input(d, X.MotionNotify, detail=1, x=dx, y=0)
    d.sync()
    i += 1
    time.sleep(iv)
print(f"spun {i} motion events over {dur:.0f}s")
