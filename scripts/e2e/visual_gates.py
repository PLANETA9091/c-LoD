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
  G-V3c the LOD band tracks the celestial light: the band box is ANCHORED
        to the horizon (topmost terrain row) of the DAY frame, so clouds
        can never displace it (the old fixed box sat on a moonlit cloud in
        run 1438 and would have false-failed)
  G-V4  client log: pack applied, no missing clod post effect AFTER the
        pack accept marker
  G-V5  server log: static pack finalized exactly once, zero rebuilds
  G-V6  the LOD band exists in the yaw shots 02/03/04: a strip right below
        the detected horizon has terrain structure (std) and is not a flat
        white fog wall (mean). Catches "band missing entirely", which the
        day/night pair cannot see (both frames would agree on no band)
  G-V7  the reference view 01 shows the SAME band as 08_day_return: the box
        is anchored to the horizon of 08 (the chain is fully warm by then —
        live-observed: shot 01 right after the tp had no band while 08, the
        same camera minutes later, showed the full band). The same box must
        contain structure in 01. Catches "the reference frame lost its LOD"
"""
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from PIL import Image  # noqa: E402

# The reference camera (-929.67 173.38 796.37, pitch slightly down) puts the
# horizon in the upper middle of the frame; clouds (world y=192) always sit
# ABOVE the horizon line, so a box anchored a few rows BELOW it can never
# land on a cloud. Horizon detection uses the right 55..95% of the viewport:
# the reference view has the far LOD band (snowy terraces) there, while the
# near vanilla jungle is lower and cannot fake an early horizon.
HORIZON_X0, HORIZON_X1 = 0.55, 0.95
BAND_X0, BAND_X1 = 0.30, 0.90
BAND_TOP, BAND_BOTTOM = 6, 56  # rows below the horizon

failures = []


def gate(name: str, ok: bool, detail: str = "") -> None:
    print(f"  {'OK  ' if ok else 'FAIL'} {name}" + (f" — {detail}" if detail else ""))
    if not ok:
        failures.append(name)


def stats(path: Path, box=None):
    im = Image.open(path).convert("RGB")
    return img_stats(im, box)


def img_stats(im, box=None):
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


def viewport(im):
    """Bbox of the non-black game render inside the black screenshot frame.

    Screenshots are full-Xvfb grabs; the game window is centered with large
    black margins. Detecting the render bbox keeps every downstream crop
    resolution-independent (the old fixed (820,320) box silently assumed
    1280x720 + centered window).
    """
    g = im.convert("L")
    w, h = g.size
    px = g.load()

    def row_frac_black(y):
        n = sum(1 for x in range(0, w, 4) if px[x, y] < 10)
        return n / (w // 4 + 1)

    def col_frac_black(x):
        n = sum(1 for y in range(0, h, 4) if px[x, y] < 10)
        return n / (h // 4 + 1)

    top = next((y for y in range(h) if row_frac_black(y) < 0.9), 0)
    bot = next((y for y in range(h - 1, -1, -1) if row_frac_black(y) < 0.9), h - 1)
    left = next((x for x in range(w) if col_frac_black(x) < 0.9), 0)
    right = next((x for x in range(w - 1, -1, -1) if col_frac_black(x) < 0.9), w - 1)
    return left, top, right, bot


def horizon_row(im, vp):
    """Topmost terrain row: first viewport row (scanning top-down) whose
    pixels are mostly NOT sky-like within the detection window.

    Sky-like = blue sky (B > R+8, B > 90), cloud (near-grey, bright) or
    night blackness (lum < 16). Terrain — textured, green/brown/dark-blue —
    is everything else. Clouds always sit ABOVE the horizon (world y=192 vs
    camera y=173), so the first mostly-non-sky row is real geometry.
    """
    r, g, b = im.convert("RGB").split()
    rp, gp, bp = r.load(), g.load(), b.load()
    x0 = vp[0] + int((vp[2] - vp[0]) * HORIZON_X0)
    x1 = vp[0] + int((vp[2] - vp[0]) * HORIZON_X1)
    xs = range(x0, x1, max(1, (x1 - x0) // 60))
    for y in range(vp[1], vp[3]):
        nonsky = tot = 0
        for x in xs:
            R, G, B = rp[x, y], gp[x, y], bp[x, y]
            tot += 1
            lum = 0.299 * R + 0.587 * G + 0.114 * B
            blue_sky = B > R + 8 and B > 90
            cloud = abs(R - G) < 14 and abs(G - B) < 14 and lum > 140
            if not (blue_sky or cloud or lum < 16):
                nonsky += 1
        if nonsky / max(tot, 1) > 0.5:
            return y
    return None


def band_box(vp, hr):
    """LOD-band crop: full-ish width strip right below the horizon."""
    vw = vp[2] - vp[0]
    return (
        vp[0] + int(vw * BAND_X0),
        hr + BAND_TOP,
        vp[0] + int(vw * BAND_X1),
        hr + BAND_BOTTOM,
    )


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
        # while the sky goes black.
        #
        # The band box is ANCHORED: the horizon (topmost terrain row) is
        # detected in the DAY frame and the strip right below it is sampled
        # in BOTH frames with the same coordinates (the camera is identical).
        # A fixed box cannot work: run 1438 drifted a moonlit cloud into it
        # and the gate went blind. Anchoring below the horizon puts the box
        # on far LOD geometry in every frame — clouds are always above the
        # horizon.
        if day.exists():
            dim = Image.open(day).convert("RGB")
            nim = Image.open(night).convert("RGB")
            vp = viewport(dim)
            hr = horizon_row(dim, vp)
            if hr is None:
                gate("V3c night LOD dims with the sky", False, "day horizon not found")
            else:
                box = band_box(vp, hr)
                dmean, dstd = img_stats(dim, box)
                nmean, nstd = img_stats(nim, box)
                # anchor sanity: the day strip must contain LOD structure
                # (a flat sky/fog strip would make the ratio meaningless)
                gate("V3c-a day band shows LOD structure", dstd > 12.0, f"day band std {dstd:.1f} box {box}")
                gate("V3c-b night band alive", nstd > 1.5, f"night band std {nstd:.1f}")
                gate(
                    "V3c night LOD dims with the sky",
                    nmean < dmean * 0.6,
                    f"lod band day {dmean:.1f} -> night {nmean:.1f}",
                )

    # G-V6: the LOD band must EXIST in the yaw shots. The day/night pair
    # alone cannot see a missing band (both frames would agree on whatever
    # the band area shows). A strip right below the horizon must have
    # terrain structure (std) and must not be a flat white fog wall (mean).
    # Calibrated on runs 1402/1438/1450: structure std 15.0..21.6, mean
    # 59..74 (a fog wall would be >200 with std < 5).
    for yaw_shot in ("02_yaw_south.png", "03_yaw_east.png", "04_yaw_north.png"):
        p = shots / yaw_shot
        if not p.exists():
            continue
        im = Image.open(p).convert("RGB")
        vp = viewport(im)
        hr = horizon_row(im, vp)
        if hr is None:
            gate(f"V6 {yaw_shot} LOD band present", False, "horizon not found (no terrain?)")
            continue
        box = band_box(vp, hr)
        mean, std = img_stats(im, box)
        gate(
            f"V6 {yaw_shot} LOD band present",
            std > 8.0 and mean < 190.0,
            f"band std {std:.1f} mean {mean:.1f}",
        )

    # G-V7: the reference view must show the band that 08_day_return shows.
    # Anchor the band box to the horizon of 08 (the post-effect chain is
    # fully warm there) and demand structure in 01 at the SAME coordinates
    # (identical camera). Live-observed hole: shot 01 taken seconds after
    # the tp had NO band (chain re-activation) while every other gate
    # passed — the day/night pair and the near-terrain anchor could not see
    # it. With the band missing, 01's box is flat sky (std ~ 2); with the
    # band present it matches 08's structure (std >= 8).
    day_ref = shots / "01_user_day.png"
    ret = shots / "08_day_return.png"
    if day_ref.exists() and ret.exists():
        rim = Image.open(ret).convert("RGB")
        dim = Image.open(day_ref).convert("RGB")
        rvp = viewport(rim)
        rhr = horizon_row(rim, rvp)
        if rhr is None:
            gate("V7 01 shows the 08 band", False, "08 horizon not found")
        else:
            box = band_box(rvp, rhr)
            _, rstd = img_stats(rim, box)
            _, dstd = img_stats(dim, box)
            gate(
                "V7 01 shows the 08 band",
                dstd > 8.0,
                f"band std 08 {rstd:.1f} -> 01 {dstd:.1f} box {box}",
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
