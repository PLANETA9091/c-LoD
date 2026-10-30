#!/usr/bin/env python3
"""Scan-frontier rate report from /tmp/server.log (v5.5 sprint helper).

Parses the engine's rate-limited diagnostics:
  stream: tick T V/TOTAL cells, lap L, ... spawned S / failed F ...
prints the timeline (cells + delta per 100-tick window) and a summary:
overall cells/s, best window, current frontier %.

Usage: scan_rate.py [logfile] [--window N]   (default log /tmp/server.log)
"""
import re
import sys

RX = re.compile(
    r"stream: tick (\d+) (\d+)/(\d+) cells,.*?spawned (\d+) / failed (\d+)"
)


def main() -> int:
    log = sys.argv[1] if sys.argv and not sys.argv[1].startswith("--") else "/tmp/server.log"
    rows = []
    with open(log, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            m = RX.search(line)
            if m:
                rows.append(tuple(int(x) for x in m.groups()))
    if not rows:
        print("no stream: lines yet")
        return 0
    # Dedup: the same line can be seen twice on log rotation - keep last.
    seen, uniq = set(), []
    for r in rows:
        if r[0] not in seen:
            seen.add(r[0])
            uniq.append(r)
    rows = uniq
    total_cells = rows[0][2] or 1
    t0, v0 = rows[0][0], rows[0][1]
    print(f"{'tick':>7} {'cells':>7} {'d_cells':>7} {'d/s':>6} {'spawned':>9}")
    best = (0, 0)
    for t, v, tot, sp, _f in rows:
        dt = t - t0
        dv = v - v0
        # tick_ms=100 -> 10 ticks/s
        rate = dv / (dt / 10.0) if dt > 0 else 0.0
        if rate > best[0]:
            best = (rate, dv)
        print(f"{t:>7} {v:>7} {dv:>7} {rate:>6.1f} {sp:>9}")
        t0, v0 = t, v
    first, last = rows[0], rows[-1]
    dt_total = (last[0] - first[0]) / 10.0
    dv_total = last[1] - first[1]
    pct = 100.0 * last[1] / total_cells
    print("-" * 46)
    print(
        f"frontier: {last[1]}/{total_cells} ({pct:.1f}%) | "
        f"overall {dv_total / dt_total:.1f} cells/s | "
        f"best window {best[0]:.1f} cells/s | failed {last[4]}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
