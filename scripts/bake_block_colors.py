#!/usr/bin/env python3
"""Bake per-block average top/side colours from a vanilla client jar.

Colour math mirrors Distant Horizons 3.3.4 ClientBlockStateColorCache
(analysis/dh-lod-pipeline.md §4): the average is computed in LINEAR light
space, alpha-weighted, then converted back to sRGB — the sRGB-space average
this script used before is measurably brighter/desaturated compared to DH.
Texture pick order also mirrors DH: UP -> unculled -> N/E/W/S -> DOWN, rotated
pillars (logs) use their bark side, liquids use the particle sprite.

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

# ClientBlockStateColorCache.FLOWER_COLOR_SCALE
FLOWER_COLOR_SCALE = 5

# Blocks whose models carry no elements (DH treats every FlowerBlock via
# instanceof); the ×5 weighting only matters on these.
FLOWERS = {
    "dandelion", "poppy", "blue_orchid", "allium", "azure_bluet",
    "red_tulip", "orange_tulip", "white_tulip", "pink_tulip", "oxeye_daisy",
    "cornflower", "lily_of_the_valley", "wither_rose", "torchflower",
    "open_eyeblossom", "closed_eyeblossom", "sunflower", "lilac",
    "rose_bush", "peony", "pink_petals", "wildflowers",
}

# DH: HugeMushroomBlock resolves the DEFAULT state (all sides primary colour).
MUSHROOM_BLOCKS = {"brown_mushroom_block", "red_mushroom_block", "mushroom_stem"}


def _srgb_to_linear(c):
    """Standard sRGB EOTF — the exact curve DH's lookup table quantises."""
    v = c / 255.0
    return v / 12.92 if v <= 0.04045 else ((v + 0.055) / 1.055) ** 2.4


SRGB_TO_LINEAR = [_srgb_to_linear(i) for i in range(256)]


def LINEAR_TO_SRGB(v):
    """Standard inverse EOTF (DH's fast table approximates this to <=1 LSB)."""
    if v <= 0.0031308:
        v *= 12.92
    else:
        v = 1.055 * (v ** (1.0 / 2.4)) - 0.055
    return max(0, min(255, int(round(v * 255.0))))

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

    # -- shared helpers (also used by bake_block_textures.py) ------------
    def blockstate_model_ref(self, name):
        """The model ref DH's blockstate would resolve to (single-variant pick)."""
        path = "assets/minecraft/blockstates/%s.json" % name
        if path not in self.names:
            return None
        data = json.loads(self.z.read(path))
        model_ref = None
        if "variants" in data:
            variants = data["variants"]
            if name in MUSHROOM_BLOCKS:
                keys = sorted(variants.keys(), key=lambda k: (-k.count("=true"), k))
            else:
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
        return model_ref

    def model_for_block(self, name):
        ref = self.blockstate_model_ref(name)
        if not ref:
            return None
        return self.model(ref.replace("minecraft:", ""))

    def face_texture_ref(self, model, name, face):
        """Resolved texture path of the largest element face, DH quad rules.

        face=None probes the unculled quads (cross models). The untinted quad
        wins when a face mixes tinted and untinted elements (pickFaceQuad).
        """
        if face is None:
            ref = self.resolve_tex(model, model["textures"].get("cross"))
            if ref is not None:
                return ref
            for d in ("north", "up", "west", "east", "south", "down"):
                r = self.face_texture_ref(model, name, d)
                if r is not None:
                    return r
            return None
        els = model.get("elements") or []
        untinted = tinted = None
        for el in els:
            f = (el.get("faces") or {}).get(face)
            if not f:
                continue
            lo, hi = el.get("from", [0, 0, 0]), el.get("to", [16, 16, 16])
            dx = max(1.0, hi[0] - lo[0])
            dy = max(1.0, hi[1] - lo[1])
            dz = max(1.0, hi[2] - lo[2])
            score = dy if face in ("up", "down") else (
                dx * dy if face in ("north", "south") else dz * dy
            )
            cand = (score, f.get("texture"), f.get("tintindex"))
            if cand[2] is None:
                if untinted is None or score > untinted[0]:
                    untinted = cand
            elif tinted is None or score > tinted[0]:
                tinted = cand
        best = untinted or tinted
        return self.resolve_tex(model, best[1]) if best else None

    def tile_pixels(self, tex):
        """16x16 RGBA ratio tile of the texture's first frame (DH encoding)."""
        from bake_block_textures import ratio_encode

        if not tex:
            return None
        if not hasattr(self, "_tile_cache"):
            self._tile_cache = {}
        if tex in self._tile_cache:
            return self._tile_cache[tex]
        path = "assets/minecraft/textures/%s.png" % tex.replace("minecraft:", "")
        tile = None
        if Image is not None and path in self.names:
            im = Image.open(io.BytesIO(self.z.read(path))).convert("RGBA")
            tile = ratio_encode(im.load(), *im.size)
        self._tile_cache[tex] = tile
        return tile

    def resolve_tex(self, model, ref, depth=0):
        """Follow '#var' indirections down to a texture path.

        MC 26.3 texture values may be sprite objects:
        {"force_translucent": true, "sprite": "minecraft:block/glass"}.
        """
        if isinstance(ref, dict):
            ref = ref.get("sprite")
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

    def _face_tinted(self, model, face):
        """True when the largest face of `face` carries a tintindex."""
        _ref, tint = self._face(model, face)
        return tint is not None

    # -- textures -------------------------------------------------------
    @staticmethod
    def _color_mode(name):
        """ClientBlockStateColorCache.EColorMode.getColorMode (instanceof chain)."""
        if name.endswith("_leaves") or name in ("azalea", "flowering_azalea",
                                                 "mangrove_roots"):
            return "Leaves"
        if name in FLOWERS:
            return "Flower"
        if "glass" in name:
            return "Glass"
        return "Default"

    def avg_color(self, tex, mode="Default"):
        """DH calculateColorFromTexture: LINEAR-space alpha-weighted average."""
        if not tex:
            return None
        key = (tex, mode)
        if key in self.tex_cache:
            return self.tex_cache[key]
        path = "assets/minecraft/textures/%s.png" % tex.replace("minecraft:", "")
        if path not in self.names:
            self.tex_cache[key] = None
            return None
        col = None
        if Image is not None:
            im = Image.open(io.BytesIO(self.z.read(path))).convert("RGBA")
            w, h = im.size
            # Animated textures are vertical strips; only the first frame counts.
            frame = min(w, h)
            px = im.crop((0, 0, w, frame)).load()
            count = 0
            alpha = 0
            red = green = blue = 0.0
            for y in range(frame):
                for x in range(w):
                    r, g, b, a = px[x, y]
                    scale = 1
                    if mode == "Leaves":
                        if a == 0:
                            continue
                        a = 255  # semi-transparent pixels count as solid
                    elif a == 0 and mode != "Glass":
                        continue
                    elif mode == "Flower" and (g + 25 < b or g + 25 < r):
                        scale = FLOWER_COLOR_SCALE
                    count += scale
                    alpha += a * scale
                    # "gamma correction is complicated" — sums are linear-light,
                    # weighted by the (linear) alpha.
                    red += SRGB_TO_LINEAR[r] * a * scale
                    green += SRGB_TO_LINEAR[g] * a * scale
                    blue += SRGB_TO_LINEAR[b] * a * scale
            if count > 0 and alpha > 0:
                col = (
                    LINEAR_TO_SRGB(red / alpha),
                    LINEAR_TO_SRGB(green / alpha),
                    LINEAR_TO_SRGB(blue / alpha),
                )
            # count == 0 (fully transparent / chisel) -> invisible
        self.tex_cache[key] = col
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
            if name in MUSHROOM_BLOCKS:
                # DH: HugeMushroomBlock uses the DEFAULT state, i.e. the
                # all-props-true variant (the primary colour on every side),
                # not the fewest-props one.
                keys = sorted(variants.keys(), key=lambda k: (-k.count("=true"), k))
            else:
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

        # DH texture pick order (COLOR_RESOLUTION_DIRECTION_ORDER): UP, then the
        # unculled faces (cross models), then N/E/W/S, then DOWN. Rotated
        # pillars (logs) skip UP so the bark side wins over the ring top.
        order = ["up", None, "north", "east", "west", "south", "down"]
        if name.endswith(("_log", "_stem")) and not name.endswith("_hyphae"):
            order = [None, "north", "east", "west", "south", "down"]
        mode = self._color_mode(name)
        top = side = None
        top_tinted = side_tinted = False
        top_concrete = side_concrete = False
        for d in order:
            ref, tint = self._face(model, d) if d else (None, None)
            resolved = self.resolve_tex(model, ref)
            concrete = resolved is not None
            if resolved is None and d is None:
                # unculled probe: cross models keep their texture under "cross";
                # the cross planes are the model's north/south faces
                resolved = self.resolve_tex(model, model["textures"].get("cross"))
                _r, tint = self._face(model, "north")
            if resolved is not None and self.avg_color(resolved, mode) is not None:
                top, top_tinted, top_concrete = resolved, tint is not None, concrete
                break
        # side: independent probe (the wall colour), no pillar rule
        for d in ["north", None, "west", "east", "south", "down"]:
            ref, tint = self._face(model, d) if d else (None, None)
            resolved = self.resolve_tex(model, ref)
            concrete = resolved is not None
            if resolved is None and d is None:
                resolved = self.resolve_tex(model, model["textures"].get("cross"))
                _r, tint = self._face(model, "north")
            if resolved is not None and self.avg_color(resolved, mode) is not None:
                side, side_tinted, side_concrete = resolved, tint is not None, concrete
                break
        tex = model["textures"]
        if top is None:
            # liquids (no elements) and textureless models: DH falls back to
            # the PARTICLE sprite; liquids keep needing tint, other
            # textureless blocks render with the raw base colour
            top = self.resolve_tex(model, tex.get("particle"))
            top_tinted = name in WATER_TINT
            if top is not None and self.avg_color(top, mode) is None:
                top = None
        if side is None:
            side = top
            side_tinted = top_tinted
        top_col = self.avg_color(top, mode) if top else None
        side_col = self.avg_color(side, mode) if side else None
        top_col = top_col or side_col
        side_col = side_col or top_col
        if top_col is None:
            return None
        if name.endswith(("_log", "_wood", "_stem")) or name.endswith("_hyphae"):
            # bark both ways (the ring top is never what LOD tops should show)
            top_col = side_col
        # DH: tint applies when the chosen quad is tinted (needPostTinting);
        # a textureless fallback keeps the tint only for liquids.
        base = tint_kind(name)
        t_top = base if top_tinted else TINT_NONE if top_concrete else (TINT_WATER if name in WATER_TINT else TINT_NONE)
        t_side = base if side_tinted else TINT_NONE if side_concrete else (TINT_WATER if name in WATER_TINT else TINT_NONE)
        return top_col, side_col, t_top, t_side


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
        "// Per-block average top/side texture colours, Distant Horizons 3.3.4",
        "// colour math (linear-light alpha-weighted average, see",
        "// analysis/dh-lod-pipeline.md §4), resolved blockstate -> model ->",
        "// texture out of the vanilla client jar.",
        "",
        "#[allow(dead_code)] // referenced by ordinal in jni.rs, not by name",
        "pub const TINT_NONE: u8 = 0;",
        "#[allow(dead_code)] // referenced by ordinal in jni.rs, not by name",
        "pub const TINT_GRASS: u8 = 1;",
        "#[allow(dead_code)]",
        "pub const TINT_FOLIAGE: u8 = 2;",
        "#[allow(dead_code)]",
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
    # The sea/lake surface colour: DH paints water LODs with the water data
    # point's colour = particle sprite (water_still) × biome water tint. Emit
    # the baked base so jni.rs no longer hardcodes a guess.
    water_row = next((r for r in rows if r[0] == "water"), None)
    if water_row is not None:
        out.append("/// Baked water_still average (DH liquid particle colour).")
        out.append("pub const WATER_BASE: u32 = 0x%02X%02X%02X;" % tuple(water_row[1]))
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
