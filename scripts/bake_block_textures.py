#!/usr/bin/env python3
"""Bake Distant Horizons 3.3.4-style LOD texture tiles from a vanilla client jar.

DH 3.3.4 renders near LODs with the block's actual texture when
`enableTexturedLods` is on (the default): every block state gets a set of
16x16 "ratio" tiles — the face texture divided by its own visible-pixel mean,
so the tile multiplies the flat LOD colour and preserves tint/shading
(analysis/dh-lod-pipeline.md §5).

Encoding mirrors BlockTextureRegistry.convertColorsToDifferenceRatios:
  avg  = simple per-channel mean over pixels with alpha != 0
  out  = round(c / max(avg, 1.0) * 127.5), clamped to 255 (128 == 1.0)
  a    = source alpha;  all-identical tiles are dropped (flat set 0)

Output: src/blocktextures.rs

Usage: bake_block_textures.py <client.jar>
"""
import io
import json
import sys
import zipfile

try:
    from PIL import Image
except ImportError:  # pragma: no cover
    Image = None

TILE = 16
TILES_PER_ROW = 256
MAX_TILES = 4096  # 256 tiles/row x 16 rows = 4096x256 px atlas

# DH Config blocksDontUseSideTextureCsv default: sides texture with the BOTTOM
# texture (repeating grass-side looks bad); blockTagsDontUseSideTextureCsv adds
# the grass_blocks/nylium tags, which in vanilla cover exactly these names.
BOTTOM_FOR_SIDES = {
    "grass_block", "mycelium", "nylium", "dirt_path", "grass",
    "crimson_nylium", "warped_nylium",
}


def ratio_encode(px, w, h):
    """16x16 RGBA ratio tile from a (w x h) pixel accessor; None if flat."""
    # nearest resample to 16x16, first animation frame only (DH bakeSpriteTexture)
    src_w = w
    src_h = min(w, h)
    samples = []
    for v in range(TILE):
        for u in range(TILE):
            x = (u * src_w) // TILE
            y = (v * src_h) // TILE
            samples.append(px[x, y])
    rs = gs = bs = 0
    visible = 0
    for r, g, b, a in samples:
        if a != 0:
            rs += r
            gs += g
            bs += b
            visible += 1
    if visible == 0:
        return None
    avg_r = max(rs / visible, 1.0)
    avg_g = max(gs / visible, 1.0)
    avg_b = max(bs / visible, 1.0)
    out = bytearray(TILE * TILE * 4)
    differs = False
    for i, (r, g, b, a) in enumerate(samples):
        er = min(int(round(r / avg_r * 127.5)), 255)
        eg = min(int(round(g / avg_g * 127.5)), 255)
        eb = min(int(round(b / avg_b * 127.5)), 255)
        o = i * 4
        out[o] = er
        out[o + 1] = eg
        out[o + 2] = eb
        out[o + 3] = a
        if a != 0 and (er != 128 or eg != 128 or eb != 128):
            differs = True
    return bytes(out) if differs else None


class TileRegistry:
    def __init__(self):
        flat = bytes([128, 128, 128, 255]) * (TILE * TILE)
        self.pixels = [flat]  # tile 0 = the reserved flat tile
        self.by_content = {}

    def get_or_create(self, tex_path, baker):
        tile = baker.tile_pixels(tex_path)
        if tile is None:
            return 0
        got = self.by_content.get(tile)
        if got is not None:
            return got
        if len(self.pixels) >= MAX_TILES:
            return 0
        tid = len(self.pixels)
        self.pixels.append(tile)
        self.by_content[tile] = tid
        return tid


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 1
    sys.path.insert(0, "scripts")
    from bake_block_colors import Baker

    baker = Baker(sys.argv[1])
    reg = TileRegistry()

    blocks = sorted(
        n[len("assets/minecraft/blockstates/"):-len(".json")]
        for n in baker.names
        if n.startswith("assets/minecraft/blockstates/") and n.endswith(".json")
    )
    sets = []
    for name in blocks:
        try:
            model = baker.model_for_block(name)
        except Exception:
            continue
        if model is None:
            continue
        top_tex = baker.face_texture_ref(model, name, "up")
        if top_tex is None:
            top_tex = baker.face_texture_ref(model, name, None)
        if top_tex is None:
            # textureless (liquids): DH bakes the particle sprite
            top_tex = baker.resolve_tex(model, model["textures"].get("particle"))
        if top_tex is None:
            continue
        if name in BOTTOM_FOR_SIDES:
            side_tex = baker.face_texture_ref(model, name, "down") or top_tex
        else:
            side_tex = baker.face_texture_ref(model, name, "north")
            if side_tex is None:
                side_tex = baker.face_texture_ref(model, name, None)
            if side_tex is None:
                side_tex = top_tex  # DH: missing side faces fall back to UP
        top_id = reg.get_or_create(top_tex, baker)
        side_id = reg.get_or_create(side_tex, baker)
        if top_id == 0 and side_id == 0:
            continue  # uniformly-coloured block: flat set, no entry needed
        sets.append((name, top_id, side_id))

    n_tiles = len(reg.pixels)
    rows = (n_tiles + TILES_PER_ROW - 1) // TILES_PER_ROW
    print(
        "baked %d tile sets, %d tiles (%dx%d px atlas)"
        % (len(sets), n_tiles, TILES_PER_ROW * TILE, max(rows, 1) * TILE),
        file=sys.stderr,
    )

    out = [
        "// @generated by scripts/bake_block_textures.py — do not edit by hand.",
        "// Distant Horizons 3.3.4 ratio-encoded LOD texture tiles",
        "// (analysis/dh-lod-pipeline.md §5): tile = face texture / own visible",
        "// mean, 128 == 1.0, alpha passthrough. Tile 0 is the reserved flat",
        "// tile; a block with only flat tiles has no entry.",
        "",
        "/// Texture tile edge in pixels (DH TextureUtil.TEXTURE_WIDTH_AND_HEIGHT).",
        "pub const TILE_PX: u32 = %d;" % TILE,
        "/// Tiles per atlas row (DH AbstractBlockTextureAtlas.TILES_PER_ROW).",
        "pub const TILES_PER_ROW: u32 = %d;" % TILES_PER_ROW,
        "/// Number of registered tiles, including the reserved flat tile 0.",
        "pub const TILE_COUNT: u32 = %d;" % n_tiles,
        "",
        "/// Ratio-encoded RGBA pixels, `TILE_COUNT` tiles of `TILE_PX * TILE_PX`,",
        "/// row-major per tile (v outer, u inner) — the DH upload order.",
        "pub static TILE_PIXELS: &[u8] = &[",
    ]
    px = bytearray()
    for tile in reg.pixels:
        px += tile
    for i in range(0, len(px), 12):
        out.append("    " + ",".join(str(b) for b in px[i:i + 12]) + ",")
    out.append("];")
    out.append("")
    out.append("/// (block path, top-face tile id, side-face tile id), sorted by name.")
    out.append("pub static BLOCK_SETS: &[(&str, u16, u16)] = &[")
    for name, top_id, side_id in sorted(sets):
        out.append('    ("%s", %d, %d),' % (name, top_id, side_id))
    out.append("];")
    out.append("")
    out.append("/// Tile set lookup by block path (None = render flat).")
    out.append("pub fn lookup(path: &str) -> Option<(u16, u16)> {")
    out.append("    BLOCK_SETS")
    out.append("        .binary_search_by(|e| e.0.cmp(path))")
    out.append("        .ok()")
    out.append("        .map(|i| (BLOCK_SETS[i].1, BLOCK_SETS[i].2))")
    out.append("}")
    out.append("")
    with open("src/blocktextures.rs", "w") as fh:
        fh.write("\n".join(out))
    print("wrote src/blocktextures.rs (%d sets)" % len(sets), file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
