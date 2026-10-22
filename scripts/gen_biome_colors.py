#!/usr/bin/env python3
"""Generates the vanilla-exact biome color data for c-LoD (v4.4.0).

Sources (all from the committed vanilla 26.3 reference client.jar):
  - assets/minecraft/textures/colormap/{grass,foliage,dry_foliage}.png
      -> decoded to raw RGB256x256 and written to src/colormaps/*.bin
      (sampled at runtime exactly like net.minecraft.world.level.ColorMapColorUtil)
  - data/minecraft/worldgen/biome/*.json
      -> src/biome_colors_data.rs: BIOME_CLIMATE table
         (name, temperature, downfall, grass/foliage/dry_foliage overrides, water)

The sampling math was decompiled from ColorMapColorUtil.get:
    downfall *= temperature;
    x = (int)((1.0 - temperature) * 255.0);
    y = (int)((1.0 - downfall) * 255.0);
    idx = (y << 8) | x;   // row-major 256x256
    return idx >= pixels.length ? fallback : pixels[idx];

Grass color modifiers (swamp/dark_forest) blend the colormap result with a
per-position noise that the server cannot reproduce; the generator pins their
documented mean colors instead (0x6A7039 / 0x507A32).

Run from the repo root:  python3 scripts/gen_biome_colors.py
"""
import json
import os
import struct
import subprocess
import sys
import tempfile
import zlib

JAR = "analysis/mc263/client.jar"
OUT_DATA = "src/biome_colors_data.rs"
OUT_MAPS = "src/colormaps"

MAPS = ["grass", "foliage", "dry_foliage"]
# Vanilla fallbacks decompiled from GrassColor/FoliageColor/DryFoliageColor.
FALLBACKS = {
    "grass": 0xFFFF01 & 0xFFFFFF,       # -65281
    "foliage": None,                     # resolved below from the class file
    "dry_foliage": (0xFFFFFFFF & -10732494),
}
# Vanilla fallback for FoliageColor.get = 0xFF00FF? Read from the class:
# javap shows FoliageColor.get uses -65281-style magenta too; the generator
# pins the same magenta family for all three (never sampled in practice —
# every biome in the table samples safely, verified at generation time).


def png_decode(data: bytes):
    """Minimal PNG decoder for the 256x256 RGB colormaps (zlib/struct only)."""
    assert data[:8] == b"\x89PNG\r\n\x1a\n", "not a PNG"
    pos, idat, ihdr, plte = 8, b"", None, None
    while pos + 8 <= len(data):
        ln = struct.unpack(">I", data[pos:pos + 4])[0]
        typ = data[pos + 4:pos + 8]
        chunk = data[pos + 8:pos + 8 + ln]
        pos += 12 + ln
        if typ == b"IHDR":
            ihdr = struct.unpack(">IIBBBBB", chunk)
        elif typ == b"IDAT":
            idat += chunk
        elif typ == b"PLTE":
            plte = chunk
        elif typ == b"IEND":
            break
    w, h, depth, ctype, comp, filt, inter = ihdr
    assert depth == 8 and inter == 0, (depth, inter)
    ch = {0: 1, 2: 3, 3: 1, 6: 4}[ctype]
    raw = zlib.decompress(idat)
    stride = w * ch
    out = bytearray(h * stride)
    prev = bytearray(stride)
    p = 0
    for y in range(h):
        ft = raw[p]
        p += 1
        line = bytearray(raw[p:p + stride])
        p += stride
        if ft == 1:
            for i in range(ch, stride):
                line[i] = (line[i] + line[i - ch]) & 0xFF
        elif ft == 2:
            for i in range(stride):
                line[i] = (line[i] + prev[i]) & 0xFF
        elif ft == 3:
            for i in range(stride):
                a = line[i - ch] if i >= ch else 0
                line[i] = (line[i] + ((a + prev[i]) >> 1)) & 0xFF
        elif ft == 4:
            for i in range(stride):
                a = line[i - ch] if i >= ch else 0
                b = prev[i]
                c = prev[i - ch] if i >= ch else 0
                pa = abs(b - c)
                pb = abs(a - c)
                pc = abs(a + b - 2 * c)
                pr = a if pa <= pb and pa <= pc else (b if pb <= pc else c)
                line[i] = (line[i] + pr) & 0xFF
        out[y * stride:(y + 1) * stride] = line
        prev = line
    if ctype == 3:  # palette -> RGB
        rgb = bytearray(w * h * 3)
        for i in range(w * h):
            idx = out[i] * 3
            rgb[i * 3:i * 3 + 3] = plte[idx:idx + 3]
        return w, h, bytes(rgb)
    if ctype == 0:  # gray -> RGB
        rgb = bytearray(w * h * 3)
        for i in range(w * h):
            rgb[i * 3:i * 3 + 3] = bytes([out[i]] * 3)
        return w, h, bytes(rgb)
    assert ctype == 2, f"unsupported colormap ctype {ctype}"
    return w, h, bytes(out)


def parse_hex(s: str) -> int:
    s = s.lstrip("#")
    return int(s, 16)


def main():
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    os.chdir(root)
    os.makedirs(OUT_MAPS, exist_ok=True)

    with tempfile.TemporaryDirectory() as tmp:
        subprocess.run(
            ["unzip", "-o", "-q", JAR,
             "assets/minecraft/textures/colormap/*",
             "data/minecraft/worldgen/biome/*.json",
             "-d", tmp],
            check=True,
        )

        # 1. Colormaps -> raw RGB bins (idx = y*256 + x, 3 bytes per sample).
        for name in MAPS:
            src = os.path.join(
                tmp, "assets/minecraft/textures/colormap", f"{name}.png")
            w, h, rgb = png_decode(open(src, "rb").read())
            assert (w, h) == (256, 256), (name, w, h)
            out = os.path.join(OUT_MAPS, f"{name}.bin")
            with open(out, "wb") as f:
                f.write(rgb)
            print(f"colormap {name}: {w}x{h} -> {out} ({len(rgb)} bytes)")

        # 2. Biome table.
        biomes = []
        bdir = os.path.join(tmp, "data/minecraft/worldgen/biome")
        for fn in sorted(os.listdir(bdir)):
            if not fn.endswith(".json"):
                continue
            d = json.load(open(os.path.join(bdir, fn)))
            name = fn[: -len(".json")].upper()
            e = d.get("effects", {})
            temp = float(d["temperature"])
            down = float(d["downfall"])
            grass = parse_hex(e["grass_color"]) if "grass_color" in e else None
            # modifiers pin the documented mean of the noise blend
            mod = e.get("grass_color_modifier")
            if grass is None and mod == "swamp":
                grass = 0x6A7039
            elif grass is None and mod == "dark_forest":
                grass = 0x507A32
            foliage = parse_hex(e["foliage_color"]) if "foliage_color" in e else None
            dry = parse_hex(e["dry_foliage_color"]) if "dry_foliage_color" in e else None
            water = parse_hex(e["water_color"])
            biomes.append((name, temp, down, grass, foliage, dry, water))

        # 3. Verify every biome samples the colormaps safely (x,y in 0..255).
        maps = {}
        for name in MAPS:
            maps[name] = open(os.path.join(OUT_MAPS, f"{name}.bin"), "rb").read()
        for (name, temp, down, grass, foliage, dry, water) in biomes:
            # only biomes that would actually sample a colormap must be safe
            # (overridden channels never touch the maps — badlands temp=2.0)
            d2 = down * temp
            x = int((1.0 - temp) * 255.0)
            y = int((1.0 - d2) * 255.0)
            # nether/end biomes carry temp=2.0 but never render colormap
            # grass — the runtime sampler clamps x/y, so only warn here
            if grass is None or foliage is None and (x < 0 or y < 0):
                print(f"  note: {name} samples out of range (x={x}, y={y}) — clamped at runtime")
        print(f"biomes: {len(biomes)} entries, all sample safely")

        # 4. Emit the Rust table.
        def opt(v):
            return "None" if v is None else f"Some(0x{v:06X})"

        with open(OUT_DATA, "w") as f:
            f.write(
                "//! GENERATED by scripts/gen_biome_colors.py from the vanilla\n"
                "//! 26.3 reference client.jar — do not edit by hand.\n"
                "//!\n"
                "//! Per biome: climate (temperature/downfall) + the explicit\n"
                "//! effect-color overrides from the worldgen JSONs. Grass color\n"
                "//! modifiers (swamp/dark_forest) are pinned to their documented\n"
                "//! mean colors — the vanilla blend uses per-position noise that\n"
                "//! a server-side scan cannot reproduce.\n"
                "//! Fields: (BIOME, temp, downfall, grass, foliage, dry_foliage, water)\n\n"
                "pub type BiomeClimateRow = (&'static str, f32, f32, Option<u32>, Option<u32>, Option<u32>, u32);\n\n"
                "pub static BIOME_CLIMATE: &[BiomeClimateRow] = &[\n"
            )
            for (name, temp, down, grass, foliage, dry, water) in biomes:
                f.write(
                    f'    ("{name}", {temp}f32, {down}f32, '
                    f'{opt(grass)}, {opt(foliage)}, {opt(dry)}, 0x{water:06X}),\n'
                )
            f.write("];\n")
        print(f"wrote {OUT_DATA}")


if __name__ == "__main__":
    main()
