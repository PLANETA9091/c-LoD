#!/usr/bin/env python3
"""Type text into the X display via XTest (headless bench client).
Usage: type_text.py <text> [delay_per_key=0.06]"""
import sys
import time

from Xlib import X, display
from Xlib.ext import xtest

d = display.Display(":99")
text = sys.argv[1]
delay = float(sys.argv[2]) if len(sys.argv) > 2 else 0.06
for ch in text:
    if ch == ' ':
        ks = 0x20
    elif ch == '/':
        ks = 0x2F
    elif ch == ':':
        ks = 0x3A
    elif ch == '_':
        ks = 0x5F
    elif ch == '.':
        ks = 0x2E
    elif ch == '-':
        ks = 0x2D
    else:
        ks = ord(ch)
    kc = d.keysym_to_keycode(ks)
    if not kc:
        print(f"no keycode for {ch!r}", file=sys.stderr)
        continue
    xtest.fake_input(d, X.KeyPress, kc)
    d.sync()
    time.sleep(0.03)
    xtest.fake_input(d, X.KeyRelease, kc)
    d.sync()
    time.sleep(delay)
print(f"typed {len(text)} chars")
