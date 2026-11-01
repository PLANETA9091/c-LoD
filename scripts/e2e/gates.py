#!/usr/bin/env python3
"""E2E gate assertions for the c-LoD boot log (live-server job).

Map-carrier streaming architecture (v5.0). Gates (all hard):
  G1  the module injected pre-kernel          [c-LoD] booting: sky-grid disk ...
  G2  the runtime admitted it                 pipeline ready: 1 module hook(s)
  G3  the kernel came up and the loop started [c-LoD] kernel ready — starting engine loop
  G4  the Paper JNI surface resolved          [c-LoD] JNI bridge resolved: true
  G5  the streaming world resolved             [c-LoD] streaming world: <name>
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
        re.search(r"\[c-LoD\] booting: sky-grid disk \d+ blocks", log) is not None,
        "G1 inject (cplugin_init, config parsed)",
        "module never reached cplugin_init or config parsing failed",
    )
    # v6.4.2: the runtime's "pipeline ready: N module hook(s)" print and the
    # module's async log lines share stdout; the CI e2e job once read the
    # interleaved bytes ("pipeline ready: [c-LoD] pack server listening
    # ...1\n module hook(s)") and failed G2 on a healthy boot. Match the
    # count anywhere within a small window after the marker instead of
    # requiring one clean line.
    g2_ok = False
    m2 = re.search(r"pipeline ready:", log)
    if m2:
        hm = re.search(r"(\d+)\s+module hooks?", log[m2.end():m2.end() + 400], re.S)
        g2_ok = hm is not None and hm.group(1) == "1"
    need(
        g2_ok,
        "G2 runtime admitted the module",
        "runtime did not load exactly 1 module hook (scan error or double load)",
    )
    need(
        "kernel ready — starting engine loop" in log,
        "G3 kernel ready, engine loop started",
        "Bukkit never came up / engine thread never started",
    )
    need(
        re.search(r"JNI bridge resolved: true \(incl\. NMS particle surface\)", log) is not None,
        "G4 Paper JNI + NMS particle surface resolved",
        "Paper 26.3 API/NMS surface incomplete — module idles (see JNI FAILED above)",
    )
    m = re.search(r"streaming world: (\S+)", log)
    need(m is not None, "G5 streaming world resolved", "no Bukkit world available")
    pm = re.search(
        r"pack finalized \(ONE-TIME, static\): shader-only, (\d+) bytes, sha1 ([0-9a-f]{40})",
        log,
    )
    need(pm is not None, "G6 shader-only pack built once at boot", "packgen never produced clod-pack.zip")
    if pm:
        sha = pm.group(2)
        need(
            log.count(sha) >= 1 and "pack rebuilt" not in log,
            "G7 pack discipline: ONE build, ZERO revisions",
            "the pack must never rebuild in v5 (world data rides dust particles)",
        )
    # Sky-grid streaming discipline (only meaningful once the engine armed):
    armed = re.search(r"sky-grid stream armed: (\d+)x(\d+) cells at \((-?[\d.]+),(-?[\d.]+)\), (\d+) particles/tick, (\d+) ticks/lap", log)
    if armed:
        need(int(armed.group(1)) > 0, "G8 sky-grid stream armed (disk geometry)", "")
        spawned = re.search(r"spawned (\d+) / failed (\d+)", log)
        joined = "joined the game" in log
        if spawned and joined:
            need(
                int(spawned.group(1)) > 0 and int(spawned.group(2)) == 0,
                "G9 dust particles spawned through the NMS bridge",
                "sendParticles never succeeded (see STREAM_FAILED / exceptions)",
            )
        elif spawned:
            # v5.7 playerless leg (pre-join arm + SEEDGEN): no receiver
            # exists, so spawn success is undefined — require only that the
            # transport raised no failures and the native filler ran.
            need(
                int(spawned.group(2)) == 0,
                "G9 playerless arm: transport idle-clean + native fill",
                "STREAM_FAILED nonzero on a playerless boot",
            )
    # v6.5.1: IncompatibleClassChangeError joins the linkage family (run
    # 41293241318: the edits helper's mock-compiled bytecode called
    # invokevirtual on the INTERFACE org.bukkit.block.Block — every block
    # event died with "Found interface ... but class was expected" while
    # every other gate stayed green). A failed event dispatch means the
    # edit channel is dead even when registration looked fine.
    nse = (
        log.count("NoSuchMethodError")
        + log.count("NoClassDefFoundError")
        + log.count("IncompatibleClassChangeError")
    )
    evf = log.count("Could not pass event")
    need(
        nse == 0 and evf == 0,
        "G10 no linkage / event-dispatch errors",
        f"{nse} linkage errors, {evf} 'Could not pass event' in the boot log",
    )

    # v5.8 observability: the native filler + helper state (informational —
    # the fill numbers are re-measured, not asserted here).
    n = re.search(
        r"native (\d+)c/(\d+)ch fail (\d+) \((\d+)w q (\d+)\) helper (off|on(?: \(fails \d+\))?)",
        log,
    )
    if n:
        print(
            f"  INFO native fill: chunks {n.group(2)}, cells {n.group(1)}, "
            f"fails {n.group(3)}, workers {n.group(4)}, helper {n.group(6)}"
        )

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
