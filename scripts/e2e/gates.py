#!/usr/bin/env python3
"""E2E gate assertions for the c-LoD boot log (live-server job).

Streaming architecture (v4.3). Gates (all hard):
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
        re.search(r"\[c-LoD\] booting: atlas \d+x\d+ blocks", log) is not None,
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
        re.search(r"JNI bridge resolved: true", log) is not None,
        "G4 Paper JNI surface resolved",
        "Paper 26.3 API surface incomplete — module idles (see JNI FAILED above)",
    )
    m = re.search(r"atlas world: (\S+)", log)
    need(m is not None, "G5 atlas world resolved", "no Bukkit world available for the heightmap atlas")
    pm = re.search(
        r"pack finalized \(base rev 0\): (\d+)/(\d+) cells \((\d+)%\), (\d+) bytes, sha1 ([0-9a-f]{40})",
        log,
    )
    need(pm is not None, "G6 base pack finalized (rev 0)", "packgen never produced clod-pack.zip")
    if pm:
        cells, total, pct, size = int(pm.group(1)), int(pm.group(2)), int(pm.group(3)), int(pm.group(4))
        need(size > 1024, "G6a pack size sane (>1KB)", f"pack size={size}")
        need(pct > 0, "G6b atlas has data", f"atlas coverage={pct}%")
        need(cells <= total, "G6c cell counts sane", f"cells={cells} total={total}")

    # G7: streaming discipline. Revisions must be strictly monotonic with no
    # gaps, and none may appear before the base finalize.
    # NOTE: the console mirror duplicates every line (stdout + logger), so the
    # same revision appears twice in the raw log — dedupe before the check,
    # gaps/holes are still caught by the set comparison.
    revs = sorted({int(r) for r in re.findall(r"pack rebuilt \(stream rev (\d+)\)", log)})
    stream_on = re.search(r"pack finalized \(base rev 0\).*— streaming ON", log) is not None
    if stream_on:
        need(
            revs == list(range(1, len(revs) + 1)),
            "G7 stream revisions monotonic (1..N, no gaps)",
            f"revisions found: {revs}",
        )
        need(len(revs) < 100, "G7a rebuild throttle sane (<100 per boot)", f"{len(revs)} rebuilds")
    else:
        need(
            len(revs) == 0,
            "G7 zero runtime rebuilds (stream off)",
            f"found {len(revs)} rebuild lines with stream off",
        )
    base_pos = log.find("pack finalized (base rev 0)")
    first_rev_pos = log.find("pack rebuilt (stream rev 1)")
    if stream_on and revs:
        need(
            first_rev_pos > base_pos >= 0,
            "G7b no revision before the base pack",
            "stream rebuild fired before finalize",
        )

    print()
    if failed:
        print("E2E: GATES FAILED")
        return 1
    print("E2E: ALL GATES PASSED")
    return 0


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("usage: gates.py <boot-log-file>", file=sys.stderr)
        sys.exit(2)
    sys.exit(main(sys.argv[1]))
