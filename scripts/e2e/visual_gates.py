#!/usr/bin/env python3
"""Visual + client-log gates for the DH-exact e2e screenshot matrix.

The bench world is FIXED (seed 4819696040575474753, reference camera
-929.67 173.38 796.37 yaw -856.27 pitch 15.40), so the gates are structural,
not scene-calibrated:
  G-V1  all expected shots exist and are non-trivial PNGs
  G-V2  the day reference shot actually shows terrain (pixel variance far
        above a flat-sky frame) — the LOD pass did something
  G-V3  the night shot is dark overall (water must not glow; veil tracks
        the sky) but not pure black (stars/silhouette remain)
  G-V4  client log: pack applied, no missing clod post effect AFTER the
        pack accept marker
  G-V5  server log: static pack finalized exactly once, zero rebuilds
"""
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from PIL import Image  # noqa: E402

failures = []


def gate(name: str, ok: bool, detail: str = "") -> None:
    print(f"  {'OK  ' if ok else 'FAIL'} {name}" + (f" — {detail}" if detail else ""))
    if not ok:
        failures.append(name)


def stats(path: Path, box=None):
    im = Image.open(path).convert("RGB")
    # game window is centered on the Xvfb screen; sample the middle half
    w, h = im.size
    if box is None:
        box = (w // 4, h // 4, 3 * w // 4, 3 * h // 4)
    c = im.crop(box)
    px = list(c.getdata())
    n = len(px)
    lum = [0.299 * r + 0.587 * g + 0.114 * b for r, g, b in px]
    mean = sum(lum) / n
    var = sum((v - mean) ** 2 for v in lum) / n
    return mean, var ** 0.5


def main() -> int:
    shots = Path(sys.argv[1])
    client_log = Path(sys.argv[2])
    server_log = Path(sys.argv[3])

    expected = [
        "00_join_state.png",
        "00_pack_applied.png",
        "01_user_day.png",
        "02_yaw_south.png",
        "03_yaw_east.png",
        "04_yaw_north.png",
        "05_aerial.png",
        "06_sunset.png",
        "07_night.png",
        "08_day_return.png",
    ]
    # G-V1
    missing = [e for e in expected if not (shots / e).exists()]
    gate("V1 all shots exist", not missing, f"missing: {missing}" if missing else f"{len(expected)} files")

    day = shots / "01_user_day.png"
    if day.exists():
        # G-V2: terrain visible -> real pixel variance (a flat sky frame has
        # stddev < ~6; any LOD geometry pushes it way above)
        mean, std = stats(day)
        gate("V2 day frame shows terrain", std > 12.0, f"luma std {std:.1f}, mean {mean:.1f}")

        # the LOD band must not be a flat colour wall either
        mean8, std8 = stats(shots / "08_day_return.png")
        gate("V2b day return shows terrain", std8 > 12.0, f"luma std {std8:.1f}")

    night = shots / "07_night.png"
    if night.exists():
        # G-V3: night = dark, but alive (stars/veil), water must not glow
        mean, std = stats(night)
        gate("V3 night is dark", mean < 60.0, f"luma mean {mean:.1f}")
        gate("V3b night not blank", std > 2.0, f"luma std {std:.1f}")

        # G-V3c: the LOD band must track the celestial light (rot pixel (1,0),
        # the day channel). A broken channel pins dayF to the camera azimuth:
        # the distant snow stays at its daylight luminance through the night
        # while the sky goes black. Sample the far LOD slope right of the
        # reference view (beyond the vanilla render distance in both shots).
        if day.exists():
            box = (820, 320, 820 + 140, 320 + 90)
            dmean, _ = stats(day, box)
            nmean, _ = stats(night, box)
            gate(
                "V3c night LOD dims with the sky",
                nmean < dmean * 0.6,
                f"lod slope day {dmean:.1f} -> night {nmean:.1f}",
            )

    # G-V4 client log
    try:
        clog = client_log.read_text(errors="replace")
    except OSError:
        clog = ""
    gate("V4 client joined", "Stepan joined" in clog or "Connected to" in clog or "logged in with entity" in clog)
    # Missing-effect warnings are EXPECTED between join and pack acceptance
    # (the module kicks /posteffect immediately). They are only fatal when
    # they recur after the pack actually applied — the apply triggers a
    # resource reload whose pack list contains "clod".
    apply_idx = 0
    for m in re.finditer(r"Reloading ResourceManager[^\n]*server/", clog):
        apply_idx = m.end()
    missing_effects = re.findall(r"Requested post effect does not exist: clod:(\w+)", clog[apply_idx:])
    gate("V4b clod post effects loaded", not missing_effects, f"missing: {sorted(set(missing_effects))}" if missing_effects else "lod_terrain+atmosphere resolved")

    # G-V5 server log
    try:
        slog = server_log.read_text(errors="replace")
    except OSError:
        slog = ""
    # the tee'd log duplicates every line with different prefixes — identify
    # finalizations by the pack sha1, which is their actual identity
    sha1s = re.findall(r"pack finalized \(ONE-TIME, static\).*?sha1 ([0-9a-f]{40})", slog)
    gate("V5 pack finalized exactly once", len(sha1s) >= 1 and len(set(sha1s)) == 1, f"finalizations {len(sha1s)}, unique sha1 {len(set(sha1s))}")
    gate("V5b zero runtime rebuilds", "runtime pack rebuild" not in slog)

    if failures:
        print(f"visual gates: {len(failures)} FAILED: {failures}")
        return 1
    print("visual gates: ALL PASSED")
    return 0


if __name__ == "__main__":
    sys.exit(main())
