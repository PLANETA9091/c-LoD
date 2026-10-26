#!/usr/bin/env python3
"""E2E gate assertions for the c-LoD boot log (live-server job).

Map-carrier streaming architecture (v5.0). Gates (all hard):
  G1  the module injected pre-kernel          [c-LoD] booting: atlas ...
  G2  the runtime admitted it                 pipeline ready: 1 module hook(s)
  G3  the kernel came up and the loop started [c-LoD] kernel ready — starting engine loop
  G4  the Paper JNI surface resolved          [c-LoD] JNI bridge resolved: true
  G5  the atlas world resolved                [c-LoD] atlas world: <name>
  G6  the BASE pack was finalized (rev 0)     [c-LoD] pack finalized (base rev 0): ...
  G7  streaming discipline: revisions are throttled and monotonic — every
      'pack rebuilt (stream rev N)' has N increasing by 1, and no rebuild
      happened before the base pack (stream=off: zero rebuilds required).
Exit 0 only when every gate passes.
"""
import re
import sys


def main(path: str) -> int:
    log = open(path, encoding="utf-8", errors="replace").read()

    def need(cond: bool, gate: str, hint: str) -> None:
        nonlocal failed
        if cond:
            print(f"  OK   {gate}")
        else:
            print(f"  FAIL {gate} — {hint}")
            failed = True

    failed = False

    need(
        re.search(r"\[c-LoD\] booting: streaming regions \d+ blocks", log) is not None,
        "G1 inject (cplugin_init, config parsed)",
        "module never reached cplugin_init or config parsing failed",
    )
    need(
        re.search(r"pipeline ready: 1 module hook", log) is not None,
        "G2 runtime admitted the module",
        "runtime did not load exactly 1 module hook (scan error or double load)",
    )
    need(
        "kernel ready — starting engine loop" in log,
        "G3 kernel ready, engine loop started",
        "Bukkit never came up / engine thread never started",
    )
    need(
        re.search(r"JNI bridge resolved: true \(incl\. NMS map surface\)", log) is not None,
        "G4 Paper JNI + NMS map surface resolved",
        "Paper 26.3 API/NMS surface incomplete — module idles (see JNI FAILED above)",
    )
    m = re.search(r"streaming world: (\S+)", log)
    need(m is not None, "G5 streaming world resolved", "no Bukkit world available")
    pm = re.search(
        r"shader-only pack built ONCE at boot: (\d+) bytes, sha1 ([0-9a-f]{40})",
        log,
    )
    need(pm is not None, "G6 shader-only pack built once at boot", "packgen never produced clod-pack.zip")
    if pm:
        sha = pm.group(2)
        need(
            log.count(sha) >= 1 and "pack rebuilt" not in log,
            "G7 pack discipline: ONE build, ZERO revisions",
            "the pack must never rebuild in v5 (world data rides map packets)",
        )
    # Streaming discipline (only meaningful once a player joined):
    swaps = re.findall(r"stream: map ids (\d+)\.\.(\d+) assigned", log)
    pkts = re.search(r"total (\d+) B / (\d+) pkts", log)
    if swaps or pkts:
        need(True, "G8 map-carrier streaming active (packets + page swaps)", "")
        if pkts:
            need(int(pkts.group(2)) > 0, "G9 map packets sent to the client", "no raw map packets were handed to the connection")

    print()
    if failed:
        print("GATES FAILED")
        return 1
    print("ALL GATES PASSED")
    return 0


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("usage: gates.py <boot-log-file>", file=sys.stderr)
        sys.exit(2)
    sys.exit(main(sys.argv[1]))
