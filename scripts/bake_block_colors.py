#!/usr/bin/env python3
"""Bake per-block average top/side colours from a vanilla client jar.

This is the offline half of what Voxy does at runtime with its
`SoftwareModelTextureBakery`: resolve blockstate -> model -> texture, rasterise
(or in our case: average) the texture, so the LOD is painted with the *real*
block colours instead of a hand-picked palette. An average is exactly what
Voxy's mip chain collapses to at high LOD levels, so far terrain ends up the
same colour.

Output: src/blockcolors.rs

Usage: bake_block_colors.py <client.jar> [--json out.json]
"""
import io
import json
import sys
import zipfile

try:
    from PIL import Image
except ImportError:  # pragma: no cover
    Image = None

# Tint kinds (must match src/blockcolors.rs / lod_terrain.fsh)
TINT_NONE, TINT_GRASS, TINT_FOLIAGE, TINT_WATER = 0, 1, 2, 3

GRASS_TINT = {
    "grass_block", "short_grass", "grass", "tall_grass", "fern", "large_fern",
    "potted_fern", "sugar_cane", "attached_melon_stem", "attached_pumpkin_stem",
    "melon_stem", "pumpkin_stem", "vine", "lily_pad", "pink_petals", "wildflowers",
}
FOLIAGE_TINT_SUFFIX = ("_leaves",)
FOLIAGE_TINT = {"vine", "mangrove_roots", "azalea", "flowering_azalea"}
WATER_TINT = {
    "water", "flowing_water", "bubble_column", "seagrass", "tall_seagrass",
    "kelp", "kelp_plant", "water_cauldron",
}


def tint_kind(name):
    if name in WATER_TINT:
        return TINT_WATER
    if name in GRASS_TINT:
        return TINT_GRASS
    if name in FOLIAGE_TINT or name.endswith(FOLIAGE_TINT_SUFFIX):
        return TINT_FOLIAGE
    return TINT_NONE


class Baker:
    def __init__(self, jar):
        self.z = zipfile.ZipFile(jar)
        self.names = set(self.z.namelist())
        self.models = {}
        self.tex_cache = {}

    # -- models ---------------------------------------------------------
    def model(self, ref):
        ref = ref.replace("minecraft:", "")
        if not ref.startswith("block/"):
            ref = "block/" + ref
        if ref in self.models:
            return self.models[ref]
        path = "assets/minecraft/models/%s.json" % ref
        if path not in self.names:
            self.models[ref] = None
            return None
        data = json.loads(self.z.read(path))
        entry = {"textures": {}, "elements": None, "parent": data.get("parent")}
        parent = data.get("parent")
        if parent:
            pm = self.model(parent)
            if pm:
                entry["textures"].update(pm["textures"])
                entry["elements"] = pm["elements"]
        entry["textures"].update(data.get("textures") or {})
        if data.get("elements") is not None:
            entry["elements"] = data["elements"]
        self.models[ref] = entry
        return entry

    def resolve_tex(self, model, ref, depth=0):
        """Follow '#var' indirections down to a texture path."""
        if ref is None or depth > 8:
            return None
        if ref.startswith("#"):
            var = ref[1:]
            nxt = model["textures"].get(var)
            if nxt is None:
                # Voxy/vanilla resolve a missing variable to the particle one
                nxt = model["textures"].get("particle")
            return self.resolve_tex(model, nxt, depth + 1)
        return ref if ref.startswith("block/") else ref

    @staticmethod
    def _face(model, face):
        """(texture ref, tintindex or None) of the largest element face."""
        els = model.get("elements") or []
        best = None
        for el in els:
            faces = el.get("faces") or {}
            f = faces.get(face)
            if not f:
                continue
            # Prefer the biggest element: for grass_block the "up" face lives on
            # the full cube; for cross models there is no up face at all.
            lo, hi = el.get("from", [0, 0, 0]), el.get("to", [16, 16, 16])
            dx = max(1.0, hi[0] - lo[0])
            dy = max(1.0, hi[1] - lo[1])
            dz = max(1.0, hi[2] - lo[2])
            score = dy if face in ("up", "down") else (dx * dy if face in ("north", "south") else dz * dy)
            if best is None or score > best[0]:
                best = (score, f.get("texture"), f.get("tintindex"))
        return (best[1], best[2]) if best else (None, None)

    # -- textures -------------------------------------------------------
    def avg_color(self, tex):
        if not tex:
            return None
        if tex in self.tex_cache:
            return self.tex_cache[tex]
        path = "assets/minecraft/textures/%s.png" % tex.replace("minecraft:", "")
        if path not in self.names:
            self.tex_cache[tex] = None
            return None
        raw = self.z.read(path)
        col = None
        if Image is not None:
            im = Image.open(io.BytesIO(raw)).convert("RGBA")
            w, h = im.size
            # Animated textures are vertical strips; only the first frame counts.
            frame = min(w, h)
            px = im.crop((0, 0, w, frame)).load()
            tr = tg = tb = ta = 0.0
            for y in range(frame):
                for x in range(w):
                    r, g, b, a = px[x, y]
                    if a < 26:  # alpha cutout (leaves, plants, glass) is air
                        continue
                    wgt = a / 255.0
                    tr += r * wgt
                    tg += g * wgt
                    tb += b * wgt
                    ta += wgt
            if ta > 0:
                col = (
                    int(round(tr / ta)),
                    int(round(tg / ta)),
                    int(round(tb / ta)),
                )
        self.tex_cache[tex] = col
        return col

    # -- blocks ---------------------------------------------------------
    def block_entry(self, name):
        path = "assets/minecraft/blockstates/%s.json" % name
        if path not in self.names:
            return None
        data = json.loads(self.z.read(path))
        model_ref = None
        if "variants" in data:
            variants = data["variants"]
            # Prefer the plain variant, then the one with the fewest properties.
            keys = sorted(variants.keys(), key=lambda k: (k != "", k.count(","), k))
            for k in keys:
                v = variants[k]
                if isinstance(v, list) and v:
                    v = v[0]
                if isinstance(v, dict) and "model" in v:
                    model_ref = v["model"]
                    break
        elif "multipart" in data:
            for part in data["multipart"]:
                ap = part.get("apply")
                if isinstance(ap, list) and ap:
                    ap = ap[0]
                if isinstance(ap, dict) and "model" in ap:
                    model_ref = ap["model"]
                    break
        if not model_ref:
            return None
        model = self.model(model_ref.replace("minecraft:", ""))
        if not model:
            return None

        up_ref, up_tint = self._face(model, "up")
        north_ref, north_tint = self._face(model, "north")
        up = self.resolve_tex(model, up_ref)
        north = self.resolve_tex(model, north_ref)
        tex = model["textures"]
        if up is None:
            up = self.resolve_tex(model, tex.get("top") or tex.get("all") or tex.get("particle"))
        if north is None:
            north = self.resolve_tex(model, tex.get("side") or tex.get("north") or tex.get("all") or tex.get("particle"))
        top = self.avg_color(up) or self.avg_color(north)
        side = self.avg_color(north) or top
        if top is None:
            return None
        # Tint is per face: grass_block's top is tinted, its side (dirt with a
        # green rim) is not. Models without elements (fluids) fall back to the
        # block-name heuristic.
        base = tint_kind(name)
        if up_ref is None and north_ref is None:
            t_top = t_side = base
        else:
            t_top = base if (up_ref is None or up_tint is not None) else TINT_NONE
            t_side = base if (north_ref is None or north_tint is not None) else TINT_NONE
        return top, side, t_top, t_side


def _colormap_color(pixels, size, temperature, downfall, default):
    """net.minecraft.world.level.ColorMapColorUtil.get (verified bytecode)."""
    downfall = downfall * temperature
    i = int((1.0 - temperature) * 255.0)
    j = int((1.0 - downfall) * 255.0)
    idx = (j << 8) | i
    if 0 <= idx < len(pixels):
        return pixels[idx]
    return default


def _load_colormap(z, path, size):
    if Image is None:
        return None
    im = Image.open(io.BytesIO(z.read(path))).convert("RGBA")
    im = im.resize((size, size), Image.BILINEAR)
    px = im.load()
    out = []
    for j in range(size):
        for i in range(size):
            r, g, b, _ = px[i, j]
            out.append((r << 16) | (g << 8) | b)
    return out


def _as_int(v, default=None):
    if isinstance(v, int):
        return v
    if isinstance(v, str):
        v = v.strip()
        if v.startswith("#"):
            return int(v[1:], 16)
        return int(v)
    return default


def bake_biomes(z, names):
    grass_map = _load_colormap(z, "assets/minecraft/textures/colormap/grass.png", 256)
    foliage_map = _load_colormap(z, "assets/minecraft/textures/colormap/foliage.png", 256)
    rows = []
    for n in sorted(names):
        if not n.startswith("data/minecraft/worldgen/biome/") or not n.endswith(".json"):
            continue
        name = n[len("data/minecraft/worldgen/biome/"):-len(".json")]
        data = json.loads(z.read(n))
        temp = float(data.get("temperature", 0.8))
        downfall = float(data.get("downfall", 0.4))
        effects = data.get("effects") or {}
        modifier = effects.get("grass_color_modifier", "none")
        grass_override = _as_int(effects.get("grass_color"))
        foliage_override = _as_int(effects.get("foliage_color"))
        water = _as_int(effects.get("water_color"), 0x3F76E4)
        if grass_override is not None:
            grass = grass_override
        else:
            grass = _colormap_color(grass_map, 256, temp, downfall, 0x91BD59)
            if modifier == "swamp":
                grass = (((grass & 0xFEFEFE) + 0x283C0A) >> 1) & 0xFFFFFF
            elif modifier == "dark_forest":
                grass = 0x507A32
        if foliage_override is not None:
            foliage = foliage_override
        else:
            foliage = _colormap_color(foliage_map, 256, temp, downfall, 0x48B518)
        rows.append((name, grass & 0xFFFFFF, foliage & 0xFFFFFF, water & 0xFFFFFF))
    return rows


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 1
    jar = sys.argv[1]
    json_out = None
    if "--json" in sys.argv:
        json_out = sys.argv[sys.argv.index("--json") + 1]
    b = Baker(jar)
    blocks = sorted(
        n[len("assets/minecraft/blockstates/"):-len(".json")]
        for n in b.names
        if n.startswith("assets/minecraft/blockstates/") and n.endswith(".json")
    )
    rows = []
    for name in blocks:
        try:
            e = b.block_entry(name)
        except Exception as exc:  # noqa: BLE001 - skip broken models
            print("skip %s: %s" % (name, exc), file=sys.stderr)
            continue
        if e is None:
            continue
        rows.append((name, e[0], e[1], e[2], e[3]))
    print("baked %d/%d blocks" % (len(rows), len(blocks)), file=sys.stderr)
    missing = [n for n in blocks if n not in {r[0] for r in rows}]
    if missing:
        print("missing: %s" % ", ".join(missing[:12]), file=sys.stderr)

    biomes = bake_biomes(b.z, b.names)

    if json_out:
        with open(json_out, "w") as fh:
            json.dump(
                [
                    {"name": n, "top": t, "side": s, "tint_top": kt, "tint_side": ks}
                    for n, t, s, kt, ks in rows
                ],
                fh,
                indent=0,
            )

    out = [
        "// @generated by scripts/bake_block_colors.py — do not edit by hand.",
        "// Per-block average top/side texture colours (mip average, alpha weighted),",
        "// resolved blockstate -> model -> texture out of the vanilla client jar.",
        "",
        "pub const TINT_NONE: u8 = 0;",
        "pub const TINT_GRASS: u8 = 1;",
        "pub const TINT_FOLIAGE: u8 = 2;",
        "pub const TINT_WATER: u8 = 3;",
        "",
        "/// (block path, top rgb, side rgb, top tint, side tint), sorted by name.",
        "pub static BLOCK_COLORS: &[(&str, u32, u32, u8, u8)] = &[",
    ]
    for name, top, side, tint_top, tint_side in sorted(rows):
        out.append(
            '    ("%s", 0x%02X%02X%02X, 0x%02X%02X%02X, %d, %d),'
            % (name, top[0], top[1], top[2], side[0], side[1], side[2], tint_top, tint_side)
        )
    out.append("];")
    out.append("")
    out.append("/// Binary search over [`BLOCK_COLORS`] by block path.")
    out.append("pub fn lookup(path: &str) -> Option<(u32, u32, u8, u8)> {")
    out.append("    BLOCK_COLORS")
    out.append("        .binary_search_by(|e| e.0.cmp(path))")
    out.append("        .ok()")
    out.append("        .map(|i| {")
    out.append("            (BLOCK_COLORS[i].1, BLOCK_COLORS[i].2, BLOCK_COLORS[i].3, BLOCK_COLORS[i].4)")
    out.append("        })")
    out.append("}")
    out.append("")
    out.append("/// (biome path, grass rgb, foliage rgb, water rgb), sorted by name.")
    out.append("/// Vanilla tint values (colormap + grass colour modifier), baked so the")
    out.append("/// scan can apply them without touching client-only classes.")
    out.append("pub static BIOMES: &[(&str, u32, u32, u32)] = &[")
    for name, grass, foliage, water in biomes:
        out.append('    ("%s", 0x%06X, 0x%06X, 0x%06X),' % (name, grass, foliage, water))
    out.append("];")
    out.append("")
    out.append("/// Biome tint lookup with the vanilla plains fallback.")
    out.append("pub fn biome(name: &str) -> (u32, u32, u32) {")
    out.append("    match BIOMES.binary_search_by(|e| e.0.cmp(name)) {")
    out.append("        Ok(i) => (BIOMES[i].1, BIOMES[i].2, BIOMES[i].3),")
    out.append("        Err(_) => (0x91BD59, 0x48B518, 0x3F76E4),")
    out.append("    }")
    out.append("}")
    out.append("")
    with open("src/blockcolors.rs", "w") as fh:
        fh.write("\n".join(out))
    print("wrote src/blockcolors.rs (%d entries)" % len(rows), file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
