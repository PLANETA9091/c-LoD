#!/usr/bin/env python3
"""Kill the horizon tutorial 'Move with W, A, S and D' toast (run
44889619819's drift composite: the toast area = a solid changing blob, the
baseline never settles under it — drift 10-27k px on a frozen scene).

The Movement hint dismisses only after BOTH move_ticks > 40 (real WASD
input, tutorial.rs input()) and look_finished (look_ticks > 40, mouse());
neither ever fires on a bot that only teleports — the toast then animates
on screen forever and every A/B pair lies. XTest a 2.6 s W hold (40+ ticks)
and a 1.4 s camera sweep (the sticky look flag), the client steps the
tutorial to None (creative) on the next tick and the toast is gone for the
rest of the session (the state survives teleports; only a client relaunch
resets it — re-run after every join/rejoin).

Usage: walk_keys.py [walk_s=2.6]
"""
import sys
import time

from Xlib import display, X
from Xlib.ext import xtest

d = display.Display(":99")
walk_s = float(sys.argv[1]) if len(sys.argv) > 1 else 2.6

# evdev keymap: W = keycode 25. Press, hold past the 40-tick (2.0 s) gate,
# release.
W_KEY = 25
xtest.fake_input(d, X.KeyPress, detail=W_KEY)
d.sync()
time.sleep(walk_s)
xtest.fake_input(d, X.KeyRelease, detail=W_KEY)
d.sync()

# the look flag is sticky but needs 40 mouse ticks — sweep the camera like
# spin_camera.py does (relative MotionNotify events, pitch stays level).
t0 = time.time()
i = 0
while time.time() - t0 < 1.4:
    dx = 140 if (i // 6) % 2 == 0 else -140
    xtest.fake_input(d, X.MotionNotify, detail=1, x=dx, y=0)
    d.sync()
    i += 1
    time.sleep(0.12)
print(f"walked {walk_s:.1f}s + spun {i} events — tutorial toast dismissed")
