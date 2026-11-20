#!/usr/bin/env python3
"""c-LoD v6.59.9 — the verdict on the constant-186 from the .mca heightmaps.

Paper NEVER writes the overworld regions while running; a graceful stop
(echo stop > /tmp/server.in) flushes dist/world/region/*.mca. This script
parses the MOTION_BLOCKING heightmaps straight from the region files
(hand-rolled: no anvil-parser dependency survives the recycles) and prints
per-window min/max/mean over the 16 cascade windows for ring 1 (64x64 cells
of 16 m = the window 11 square x 1886..2994, z 3100..4108).

Ring origin = (center-256, center-256) = (-1186, 540) with region_blocks=512.
The atlas says height 122 m everywhere (mx=mn=186 = 0xBA under the ±64 code)
at 71% marked. If the true world surface differs from 122 m in the decoded
windows — the ingest/decode path IS constant; if window 11 really is a
122 m plain — the cascade worked and only the frames were missing.
"""
import os
import struct
import sys

REGION_DIR = sys.argv[1] if len(sys.argv) > 1 else "/home/z/mc-test/dist/world/region"
CENTER_X = -930.0
CENTER_Z = 796.0
ORIGIN_X = CENTER_X - 256.0  # region_blocks=512
ORIGIN_Z = CENTER_Z - 256.0

def read_region(path):
    import zlib
    data = open(path, "rb").read()
    if len(data) < 8192:
        return
    for i in range(1024):
        b = data[i * 4:i * 4 + 4]
        if len(b) < 4:
            return
        off = (b[0] << 16) | (b[1] << 8) | b[2]  # 24-bit BE sector number
        sectors = b[3]  # sector count
        if off == 0 or sectors == 0:
            continue
        start = off * 4096
        if start + 5 > len(data):
            return
        plen = struct.unpack(">I", data[start:start + 4])[0]
        comp = data[start + 4]
        payload = data[start + 5:start + 4 + plen]
        try:
            if comp == 2:
                payload = zlib.decompress(payload)
            elif comp == 1:
                import gzip
                payload = gzip.decompress(payload)
            else:
                continue
        except Exception:
            continue
        # yield the UNCOMPRESSED chunk NBT
        yield i % 32, i // 32, payload

def read_longs(b):
    # NBT-lite: find the heightmap longs inside the uncompressed chunk NBT.
    raw = b
    # scan for the compressed NBT payload of the heightmap: walk top-level
    # tags is heavy; instead locate the byte pattern of 36 longs whose
    # 9-bit packing yields 256 in-range values — the "MOTION_BLOCKING"
    # name string appears in the NBT: find its offset and read the array
    # right after it.
    idx = raw.find(b"MOTION_BLOCKING")
    if idx < 0:
        return None
    # NBT layout: [type=12][name len][name][len i32 BE][36 x i64] — the
    # payload length header starts RIGHT after the name.
    p = idx + len(b"MOTION_BLOCKING")
    starts = [p]
    for q in range(p, min(len(raw), p + 256)):
        if raw[q] == 12:
            starts.append(q + 5)
    for st in starts:
        if st + 4 > len(raw):
            continue
        n = struct.unpack(">i", raw[st:st + 4])[0]
        if n not in (36, 37):
            continue
        vals = []
        for k in range(n):
            lo = struct.unpack(">q", raw[st + 4 + k * 8:st + 12 + k * 8])[0]
            for j in range(7, -1, -1):
                v = (lo >> (j * 9)) & 0x1FF
                if v:
                    vals.append(v)
        if len(vals) >= 200:
            return vals[:256]
    return None

def world_minmax(vals, bx0, bz0):
    # vals index = (z%16)*16 + (x%16) within the chunk
    raise SystemExit("unused")

def main():
    if not os.path.isdir(REGION_DIR):
        print(f"NO REGIONS: {REGION_DIR} missing — was the stop graceful?")
        return 1
    # world height cache: bx,bz -> h (surface = MOTION_BLOCKING - 1)
    hmap = {}
    for fn in sorted(os.listdir(REGION_DIR)):
        if not fn.endswith(".mca"):
            continue
        parts = fn.split(".")
        rx, rz = int(parts[1]), int(parts[2])
        for lx, lz, chunk in read_region(os.path.join(REGION_DIR, fn)):
            vals = read_longs(chunk)
            if not vals:
                continue
            for cz in range(16):
                for cx in range(16):
                    v = vals[cz * 16 + cx] if cz * 16 + cx < len(vals) else 0
                    hmap[(rx * 512 + lx * 16 + cx, rz * 512 + lz * 16 + cz)] = v - 1
    print(f"parsed {len(hmap)} columns")
    if not hmap:
        return 1
    # the 16 cascade windows of ring 1: window w = cells ((w%4)*64..+64,
    # (w/4)*64..+64) of 16 m from the ring origin
    print("window  world-x-range        world-z-range        min    max    mean   atlas-says")
    for w in range(16):
        wx0 = ORIGIN_X + (w % 4) * 64 * 16
        wz0 = ORIGIN_Z + (w // 4) * 64 * 16
        hs = [hmap.get((int(wx0 + cx * 16 + 8), int(wz0 + cz * 16 + 8)))
              for cz in range(64) for cx in range(64)]
        hs = [h for h in hs if h is not None]
        if not hs:
            print(f"{w:2}      ({wx0:6.0f}..{wx0 + 1023:6.0f}) ({wz0:6.0f}..{wz0 + 1023:6.0f})  no columns")
            continue
        print(f"{w:2}      ({wx0:6.0f}..{wx0 + 1023:6.0f}) ({wz0:6.0f}..{wz0 + 1023:6.0f})  {min(hs):4}   {max(hs):4}   {sum(hs) / len(hs):6.1f}  122")
    # the verdict line for window 11 and the sweep vantage area
    w11 = [hmap.get((int(ORIGIN_X + 192 * 16 + cx * 16 + 8), int(ORIGIN_Z + 160 * 16 + cz * 16 + 8)))
           for cz in range(64) for cx in range(64)]
    w11 = [h for h in w11 if h is not None]
    if w11:
        spread = max(w11) - min(w11)
        print(f"\nWINDOW 11 VERDICT: {len(w11)} cols, min {min(w11)}, max {max(w11)}, spread {spread}")
        if spread <= 4 and abs(sum(w11) / len(w11) - 122) < 8:
            print("  -> window 11 IS a ~122 m plain: the atlas constant 186 was CORRECT.")
        else:
            print("  -> window 11 is NOT flat 122 m: a constant ingest/decode bug is CONFIRMED.")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
