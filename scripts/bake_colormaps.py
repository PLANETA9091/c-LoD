#!/usr/bin/env python3
"""Bake the vanilla grass/foliage/dry_foliage colormaps into raw ARGB .bin
tables for the c-LoD module.

Minecraft 26.3 keeps the colormap PNGs in the CLIENT jar only
(assets/minecraft/textures/colormap/*.png) — the dedicated server's
GrassColor/FoliageColor pixel matrices stay zeroed, so Biome#getGrassColor
server-side would return black for every colormap biome. The module ships the
matrices and calls GrassColor.init(int[]) / FoliageColor.init(int[]) /
DryFoliageColor.init(int[]) once at world resolution, which makes the NMS
resolvers pixel-exact with the client (DH's live tint, v4.13.0).

Orientation (verified against the vanilla anchors): the client feeds the
NativeImage pixels ROW-MAJOR top-down — index = row*256 + col, row = the
downfall axis (0 = hot+wet product row), col = the temperature axis. Anchor
check: plains (temp 0.8, down 0.4) -> col 50, row 173 -> #91BD59 exactly.
"""
import struct
import sys
import zipfile
from io import BytesIO
from PIL import Image

CLIENT_JAR = sys.argv[1] if len(sys.argv) > 1 else "/home/z/my-project/analysis/mc263/client.jar"
OUT_DIR = sys.argv[2] if len(sys.argv) > 2 else "/home/z/my-project/c-LoD/src/colormaps"

MAPS = ["grass", "foliage", "dry_foliage"]

def main():
    import os
    os.makedirs(OUT_DIR, exist_ok=True)
    with zipfile.ZipFile(CLIENT_JAR) as z:
        for name in MAPS:
            raw = z.read(f"assets/minecraft/textures/colormap/{name}.png")
            im = Image.open(BytesIO(raw)).convert("RGBA")
            if im.size != (256, 256):
                raise SystemExit(f"{name}.png: expected 256x256, got {im.size}")
            px = im.load()
            # ARGB ints (alpha forced opaque), row-major top-down — exactly
            # what the client's NativeImage feed gives the init() call.
            ints = []
            for y in range(256):
                for x in range(256):
                    r, g, b, _ = px[x, y]
                    ints.append(0xFF000000 | (r << 16) | (g << 8) | b)
            out = os.path.join(OUT_DIR, f"{name}.bin")
            with open(out, "wb") as f:
                f.write(struct.pack(f"<{len(ints)}I", *ints))
            print(f"{name}: {len(ints)} px -> {out} ({os.path.getsize(out)} B)")
    # anchor verification against vanilla plains grass
    im = Image.open(BytesIO(zipfile.ZipFile(CLIENT_JAR).read("assets/minecraft/textures/colormap/grass.png"))).convert("RGBA")
    px = im.load()
    col = int((1.0 - 0.8) * 255)
    row = int((1.0 - 0.4 * 0.8) * 255)
    r, g, b, _ = px[col, row]
    assert (r, g, b) == (0x91, 0xBD, 0x59), f"anchor mismatch: #{r:02X}{g:02X}{b:02X}"
    print("anchor plains #91BD59 OK")

if __name__ == "__main__":
    main()
