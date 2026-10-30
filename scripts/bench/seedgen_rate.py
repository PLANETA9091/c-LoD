#!/usr/bin/env python3
"""SEEDGEN fill-rate report from /tmp/server.log (v5.7 sprint helper).

Parses the engine diag:
  stream: tick T V/TOTAL cells, ... native Nc/Nch fail F (Ww q Q)
prints the fill timeline (cells + delta per 100-tick window) and a summary:
overall native cells/s, chunks/s, final coverage.

Usage: seedgen_rate.py [logfile]   (default /tmp/server.log)
"""
import re
import sys

RX = re.compile(
    r"stream: tick (\d+) (\d+)/(\d+) cells,.*?native (\d+)c/(\d+)ch fail (\d+) \((\d+)w q (\d+)\)"
)


def main() -> int:
    log = sys.argv[1] if len(sys.argv) > 1 else "/tmp/server.log"
    rows = []
    with open(log, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            m = RX.search(line)
            if m:
                rows.append(tuple(int(x) for x in m.groups()))
    if not rows:
        print("no stream: lines yet")
        return 0
    print(f"{'tick':>7} {'cells':>7} {'delta':>7} {'native c/ch':>13} {'fail':>5} {'q':>6}")
    prev = 0
    for tick, valid, total, nat_c, nat_ch, fail, workers, q in rows:
        print(
            f"{tick:>7} {valid:>7} {valid - prev:>7} {nat_c:>7}c/{nat_ch:<5} {fail:>5} {q:>6}"
        )
        prev = valid
    first, last = rows[0], rows[-1]
    dt_ticks = max(1, last[0] - first[0])
    # idle engine: ~1 tick/s; with players ~10 ticks/s — report both bounds
    print()
    print(f"coverage: {last[1]}/{last[2]} ({100.0 * last[1] / max(1, last[2]):.1f}%)")
    print(f"native chunks: {last[4]}  fail: {last[5]}  workers: {last[6]}")
    print(f"fill {last[1] - first[1]} cells in {dt_ticks} ticks")
    print(f"  -> {dt_ticks} cells/s at 1 tick/s (idle), {dt_ticks * 10} cells/s at 10 ticks/s")
    return 0


if __name__ == "__main__":
    sys.exit(main())
