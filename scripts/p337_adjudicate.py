#!/usr/bin/env python3
"""p337_adjudicate.py — instant adjudication of the #337 coverage-sweeps feed
(and #338 diag fields) against the #336 baseline, per the (a)/(b)/(c) frame
recorded in worklog entry "#336 verdict + v6.26.0 COVERAGE SWEEPS".

Usage:
  python3 scripts/p337_adjudicate.py < parity.md        # feed table on stdin
  python3 scripts/p337_adjudicate.py parity.json        # diag-mode for #338
"""
import json
import re
import sys

# v6.25.7 four-point stable baseline (last = #336, run 42789151568)
BASE336 = {
    "00": 75.4, "01": 57.9, "02": 49.1, "03": 54.0, "04": 43.9, "05": 91.7,
    "06": 42.5, "07": 85.3, "08": 59.3, "09": 53.1, "10": 58.2, "11": 31.5,
    "12": 85.3,
}
# run-level variance band seen across 333..336 (stable vantages ±1pp;
# 06/08/09/10 carry run-level far-ring drift ±3..5pp, 06 up to ±24)
BAND = {"06": 24.0, "08": 5.0, "09": 5.0, "10": 6.0}
DEFAULT_BAND = 1.5

EAST_SOUTH = ["04", "02", "03", "01"]   # the never-eyed quadrants the sweeps feed
WALL = ["06", "10"]                     # the run-level far-ring wall pair


def parse_table(text):
    """Extract per-view exact% from a parity.md table. Returns dict view->float."""
    out = {}
    for line in text.splitlines():
        m = re.match(r"\s*\|?\s*(\d{2})\s*\|.*?(\d{1,3}\.\d)\s*%", line)
        if m:
            out[m.group(1)] = float(m.group(2))
    return out


def adjudicate(nums):
    print("=== per-view delta vs #336 baseline (ours) ===")
    flags = {}
    for v in sorted(BASE336):
        if v not in nums:
            print(f"  {v}: MISSING from feed — cannot adjudicate")
            continue
        d = nums[v] - BASE336[v]
        band = BAND.get(v, DEFAULT_BAND)
        mark = "JUMP" if d > band + 4 else ("up" if d > band else ("down" if d < -band else "flat"))
        if mark == "JUMP":
            flags[v] = d
        print(f"  {v}: {BASE336[v]:5.1f} -> {nums[v]:5.1f}  d={d:+5.1f}  [{mark}] (band ±{band})")

    print("\n=== frame (a)/(b)/(c) ===")
    jumps_es = [v for v in EAST_SOUTH if v in flags]
    wall_settled = all(v in nums and abs(nums[v] - BASE336[v]) <= BAND[v] + 3 for v in WALL)
    any_jump = bool(jumps_es)

    if len(jumps_es) >= 3 and wall_settled:
        print("(a) DATA-COVERAGE CONFIRMED: east/south quadrant jumps"
              f" {jumps_es}, 06/10 wall settled. Next reserve = palette/AO (class 3),"
              " then sky-wall/unfog. Sweeps stay in the tour permanently.")
    elif len(jumps_es) >= 3 and not wall_settled:
        print("(c) PARTIAL: east/south jumped but 06/10 still drifting ->"
              " wall = content/palette or paint-order class, NOT coverage."
              " Cross-check #338 diag edge_vs_flat on 06/10.")
    elif not any_jump:
        print("(b) NO JUMP on east/south -> sweeps did not feed decode:"
              " cone/network/budget choke. Check time-audit sweep wire/paint"
              " markers (wire 0p/0b during sweeps?) and settle timing.")
    else:
        print(f"MIXED: jumps {jumps_es} -> read partial attribution:"
              " which azimuths fed, which did not; compare against the 36-position grid.")

    gate = "GATE PASS" if all(nums.get(v, 0) >= BASE336.get(v, 0) for v in BASE336) else "gate still False (expected — attribution run)"
    print(f"\nGate: {gate}")


def diag_mode(path):
    with open(path) as f:
        data = json.load(f)
    pairs = data.get("pairs") or data.get("views") or []
    print("=== #338 diag fields per pair ===")
    for p in pairs:
        name = p.get("name") or p.get("view")
        diag = p.get("diag") or {}
        if not diag:
            continue
        dh = diag.get("delta_hist")
        evf = diag.get("edge_vs_flat")
        sky = diag.get("sky_share_hot")
        print(f"\n[{name}] sky_share_hot={sky}")
        if dh:
            print("  delta_hist:", json.dumps(dh))
        if evf:
            print("  edge_vs_flat:", json.dumps(evf))
    print("\nReading rules (worklog 'FACE-SHADE hypothesis'): edge_signed >> flat_signed"
          " on all vantages -> AO/face darkening in the march (fix a/b);"
          " 16+ buckets dominate on slopes -> content class.")


def main():
    if len(sys.argv) > 1 and sys.argv[1].endswith(".json"):
        diag_mode(sys.argv[1])
        return
    text = sys.stdin.read()
    nums = parse_table(text)
    if not nums:
        print("No per-view table found on stdin.")
        sys.exit(1)
    adjudicate(nums)


if __name__ == "__main__":
    main()
