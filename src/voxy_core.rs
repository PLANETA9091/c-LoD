//! Voxy core (v4.3.0 "VOXY CORE"): the *substance* of Voxy's LOD pipeline,
//! ported 1:1 from the 26.3 source scan (analysis/voxy-e2e.md) to the
//! heightfield world of c-LoD:
//!
//! - `Mipper` (common/world/other/Mipper.java): a LOD cell at level L keeps
//!   the **max-opacity corner voxel's full id** — never an averaged color.
//!   Distant terrain is real blocks, thinned at 2^L.
//! - Opacity classes: solid = 15 (`getLightDampening`), water = 1, leaves
//!   forced solid (they ARE solid here — every cell is a surface).
//! - Corner priority (x | z<<1 for the heightfield's two free axes): ties
//!   resolve to the +x+z corner, matching Voxy's 111-first scan order.
//! - State table + baked face-tile atlas: Voxy renders LOD quads with REAL
//!   block textures from `blockModelAtlas` (3x2 face tiles per state,
//!   `frac(uv)` tiling = one texel-tile per cell, mip via derivatives). We
//!   bake the same structure server-side from the embedded vanilla 26.3
//!   textures, with an explicit mip strip (16/8/4/2/1) because
//!   `PostChainConfig$TextureInput` loads SimpleTextures WITHOUT mip chains.
//! - Greedy quads ≤16 do not need to be stored: `frac(worldXZ / 2^L)` tiling
//!   in the shader reproduces the merged-quad look exactly.

use crate::states_data::{MATERIAL_FACES, TEX_INDEX, TEX_RGBA, TEX_SIZE, TEX_TINT};

pub const TILE: usize = TEX_SIZE; // 16
/// Mip strip offsets inside one 31px face column: 16,8,4,2,1 px tiles.
pub const MIP_X: [usize; 5] = [0, 16, 24, 28, 30];
pub const MIP_W: [usize; 5] = [16, 8, 4, 2, 1];
/// Face column stride: 16+8+4+2+1 px.
pub const FACE_STRIDE: usize = 31;
/// States per atlas row (16 states * 3 faces * 31 px = 1488 px wide).
pub const STATES_PER_ROW: usize = 16;
pub const ATLAS_W: usize = STATES_PER_ROW * 3 * FACE_STRIDE; // 1488
pub const ATLAS_STATE_H: usize = TILE * 2; // two 16px rows per state
pub const MAX_STATES: u8 = 255; // 0 = air/void

#[derive(Clone, Copy, PartialEq)]
pub struct StateDef {
    /// TEX_RGBA tile index per face (0 DOWN, 1 UP, 2 N, 3 S, 4 W, 5 E).
    pub faces: [u16; 6],
    /// Tint kind per face: 0 none, 1 grass, 2 foliage, 3 water.
    pub tint: [u8; 6],
    /// Opacity class for the Mipper (0 air, 1 water, 15 solid).
    pub opacity: u8,
    /// The state's top face is water-like (translucent sea surface).
    pub water: bool,
    /// Any texel alpha <= 0.1 on the top/side tiles (cutout render).
    pub cutout: bool,
}

impl StateDef {
    const fn empty() -> Self {
        Self {
            faces: [0; 6],
            tint: [0; 6],
            opacity: 15,
            water: false,
            cutout: false,
        }
    }
}

/// Material -> state id. Deterministic: iteration over the compiled table.
pub struct StateRegistry {
    pub defs: Vec<StateDef>,
    ids: std::collections::HashMap<&'static str, u8>,
}

impl StateRegistry {
    pub fn build() -> Self {
        let tex_id = |name: &str| -> u16 {
            TEX_INDEX
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, i)| *i as u16)
                .unwrap_or_else(|| panic!("embedded texture missing: {name}"))
        };
        let tint_of = |name: &str| -> u8 {
            TEX_TINT
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, k)| *k)
                .unwrap_or(0)
        };
        let mut defs = Vec::new();
        let mut ids = std::collections::HashMap::new();
        // State 0 = air/void sentinel (never rendered).
        defs.push(StateDef::empty());
        for (mat, faces) in MATERIAL_FACES {
            let mut def = StateDef::empty();
            let water = *mat == "WATER" || *mat == "FLOWING_WATER";
            def.water = water;
            def.opacity = if water { 1 } else { 15 };
            for f in 0..6 {
                let tex = faces[f.min(faces.len() - 1)];
                def.faces[f] = tex_id(tex);
                def.tint[f] = tint_of(tex);
            }
            // Cutout: any alpha <= 0.1 among the face tiles.
            def.cutout = def.faces.iter().any(|&t| {
                let base = t as usize * TILE * TILE * 4;
                (0..TILE * TILE).any(|i| TEX_RGBA[base + i * 4 + 3] <= 25)
            });
            if water {
                def.cutout = false;
            }
            let id = defs.len() as u8;
            if usize::from(id) >= MAX_STATES as usize {
                break; // table full: the rest falls back to unknown tiles
            }
            ids.insert(*mat, id);
            defs.push(def);
        }
        Self { defs, ids }
    }

    /// Id for a scanned material name. The JNI bridge returns Bukkit
    /// `Material.name()` (UPPERCASE). Unknown materials map to 0 (the
    /// shader then falls back to the baked per-cell albedo gradient).
    pub fn id_of(&self, material: &str) -> u8 {
        let key = material
            .trim_start_matches("minecraft:")
            .trim_start_matches("block.minecraft.")
            .to_ascii_uppercase();
        *self.ids.get(key.as_str()).unwrap_or(&0)
    }

    pub fn def(&self, id: u8) -> StateDef {
        self.defs
            .get(id as usize)
            .copied()
            .unwrap_or_else(StateDef::empty)
    }

    pub fn count(&self) -> u8 {
        self.defs.len().min(MAX_STATES as usize) as u8
    }
}

/// One level cell of the mip pyramid (v4.3 snapshot cell = heightfield
/// column + the FULL Voxy voxel id pieces: state, biome tint, light).
#[derive(Clone, Copy, Default, PartialEq)]
pub struct VCell {
    pub height: i16,
    pub water: bool,
    pub valid: bool,
    pub state: u8,
    pub light: u8, // (block << 4) | sky, Voxy lightId layout
}

/// Mipper (heightfield port of other/Mipper.java): among the four columns
/// of a 2x2 group, the winner is the max (opacity, corner) rank — land
/// (op 15) beats water (op 1), ties resolve to the +x+z corner, exactly
/// like Voxy's (opacity<<4 | cornerIdx) maximum. Void loses to everything.
pub fn mip4(a: &VCell, b: &VCell, c: &VCell, d: &VCell) -> VCell {
    // Corner rank: bit0 = x (a=0,b=1), bit1 = z (a=0,c=1) — d wins ties.
    let rank = |cell: &VCell, corner: u8| -> (u8, u8) {
        if cell.valid {
            let op = if cell.water { 1 } else { 15 };
            (op, corner)
        } else {
            (0, corner)
        }
    };
    let cands = [
        (rank(a, 0b00), *a),
        (rank(b, 0b01), *b),
        (rank(c, 0b10), *c),
        (rank(d, 0b11), *d),
    ];
    let (_, best) = cands.into_iter().max_by_key(|(r, _)| (*r, 0u8)).unwrap();
    if best.valid {
        best
    } else {
        VCell::default()
    }
}

/// Downsample an n×n level into (n/2)×(n/2). Row-major, z outer.
pub fn mip_level(cells: &[VCell], n: usize) -> Vec<VCell> {
    let m = n / 2;
    let mut out = vec![VCell::default(); m * m];
    for z in 0..m {
        for x in 0..m {
            let g = |xx: usize, zz: usize| cells[(zz * n + xx).min(cells.len() - 1)];
            out[z * m + x] = mip4(
                &g(x * 2, z * 2),
                &g(x * 2 + 1, z * 2),
                &g(x * 2, z * 2 + 1),
                &g(x * 2 + 1, z * 2 + 1),
            );
        }
    }
    out
}

/// Build the full pyramid L0..=L4 from the level-0 grid. Odd sizes are
/// truncated by the mip (the atlas side is a power of two).
pub fn build_pyramid(level0: &[VCell], n: usize) -> Vec<Vec<VCell>> {
    let mut levels = vec![level0.to_vec()];
    let mut cur = n;
    for _ in 0..4 {
        cur /= 2;
        if cur == 0 {
            break;
        }
        levels.push(mip_level(levels.last().unwrap(), cur * 2));
    }
    levels
}

/// Alpha-aware 2x2 box-filter mip of a 16px RGBA tile.
fn mip_tile(src: &[u8], w: usize) -> Vec<u8> {
    let hw = w / 2;
    let mut out = vec![0u8; hw * hw * 4];
    for y in 0..hw {
        for x in 0..hw {
            for c in 0..4 {
                let s00 = src[((y * 2) * w + x * 2) * 4 + c] as u32;
                let s10 = src[((y * 2) * w + x * 2 + 1) * 4 + c] as u32;
                let s01 = src[((y * 2 + 1) * w + x * 2) * 4 + c] as u32;
                let s11 = src[((y * 2 + 1) * w + x * 2 + 1) * 4 + c] as u32;
                out[(y * hw + x) * 4 + c] = ((s00 + s10 + s01 + s11) / 4) as u8;
            }
        }
    }
    out
}

/// Bake the block-state atlas (the c-LoD blockModelAtlas):
/// per state — 3x2 face tiles of 16 px plus the mip strip to the right
/// (16/8/4/2/1). States that exceed the table map to the air sentinel and
/// are never read by the shader (their ids never ship in cell data).
pub fn bake_states_atlas(registry: &StateRegistry) -> (u32, u32, Vec<u8>) {
    let count = registry.count() as usize;
    let rows = count.div_ceil(STATES_PER_ROW).max(1);
    let h = rows * ATLAS_STATE_H;
    let mut px = vec![0u8; ATLAS_W * h * 4];
    for (sid, def) in registry.defs.iter().enumerate().take(count) {
        if sid == 0 {
            continue; // air sentinel stays transparent
        }
        let col = sid % STATES_PER_ROW;
        let row = sid / STATES_PER_ROW;
        for f in 0..6 {
            let face_col = f / 2;
            let face_row = f % 2;
            // Build the mip chain for this face's tile.
            let mut chain = Vec::with_capacity(5);
            let base = def.faces[f] as usize * TILE * TILE * 4;
            chain.push(TEX_RGBA[base..base + TILE * TILE * 4].to_vec());
            for m in 1..5 {
                let prev = &chain[m - 1];
                chain.push(mip_tile(prev, MIP_W[m - 1]));
            }
            for (m, tile) in chain.iter().enumerate() {
                let tw = MIP_W[m];
                let ox = col * 3 * FACE_STRIDE + face_col * FACE_STRIDE + MIP_X[m];
                let oy = row * ATLAS_STATE_H + face_row * TILE;
                for y in 0..tw {
                    for x in 0..tw {
                        let s = (y * tw + x) * 4;
                        let d = ((oy + y) * ATLAS_W + ox + x) * 4;
                        px[d..d + 4].copy_from_slice(&tile[s..s + 4]);
                    }
                }
            }
        }
    }
    (ATLAS_W as u32, h as u32, px)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(h: i16, water: bool, state: u8) -> VCell {
        VCell {
            height: h,
            water,
            valid: h > -64,
            state,
            light: 0xF0,
        }
    }

    #[test]
    fn mipper_land_beats_water_low_terrain() {
        // Island in the ocean: land wins even at height 1 vs water at 63.
        let out = mip4(
            &cell(1, false, 3),  // land, low
            &cell(63, true, 99), // water, high
            &cell(63, true, 99),
            &cell(63, true, 99),
        );
        assert_eq!(out.state, 3);
        assert!(!out.water);
        assert_eq!(out.height, 1);
    }

    #[test]
    fn mipper_corner_priority_plus_x_plus_z() {
        // All land: the +x+z corner column wins (Voxy 111-first order).
        let out = mip4(
            &cell(70, false, 1),
            &cell(80, false, 2),
            &cell(90, false, 3),
            &cell(10, false, 4),
        );
        assert_eq!(out.state, 4);
        assert_eq!(out.height, 10);
    }

    #[test]
    fn mipper_all_void_stays_void() {
        let out = mip4(
            &VCell::default(),
            &VCell::default(),
            &VCell::default(),
            &VCell::default(),
        );
        assert!(!out.valid);
    }

    #[test]
    fn pyramid_sizes_shrink_by_two() {
        let n = 16; // 5 levels: 16, 8, 4, 2, 1
        let l0: Vec<VCell> = (0..n * n).map(|i| cell(64 + i as i16, false, 1)).collect();
        let levels = build_pyramid(&l0, n);
        assert_eq!(levels.len(), 5);
        assert_eq!(levels[1].len(), 64);
        assert_eq!(levels[4].len(), 1);
        // The single L4 cell must be the +x+z corner of the whole grid.
        assert_eq!(levels[4][0].state, l0[15 * n + 15].state);
    }

    #[test]
    fn registry_binds_materials_deterministically() {
        let reg = StateRegistry::build();
        let grass = reg.id_of("grass_block");
        let stone = reg.id_of("stone");
        assert_ne!(grass, 0, "grass_block must be a known state");
        assert_ne!(stone, 0);
        assert_ne!(grass, stone);
        assert_eq!(reg.id_of("grass_block"), grass, "deterministic");
        assert_eq!(reg.id_of("not_a_real_block"), 0, "unknown -> air sentinel");
        // Grass top face is grass_block_top with grass tint.
        let gd = reg.def(grass);
        assert_eq!(gd.tint[1], 1, "grass top face uses the grass colormap");
        assert_ne!(gd.faces[0], gd.faces[1], "dirt bottom vs grass top");
        // Water is water-classed with a water-tinted top face.
        let wd = reg.def(reg.id_of("water"));
        assert!(wd.water);
        assert_eq!(wd.opacity, 1);
        assert_eq!(wd.tint[1], 3);
    }

    #[test]
    fn states_atlas_bakes_with_mip_strip() {
        let reg = StateRegistry::build();
        let (w, h, px) = bake_states_atlas(&reg);
        assert_eq!(w as usize, ATLAS_W);
        assert!(h as usize >= ATLAS_STATE_H);
        assert_eq!(px.len(), ATLAS_W * h as usize * 4);
        // The stone state's mip-4 (1x1) tile must be a solid opaque pixel.
        let stone = reg.id_of("stone") as usize;
        let ox = stone % STATES_PER_ROW * 3 * FACE_STRIDE + MIP_X[4];
        let oy = stone / STATES_PER_ROW * ATLAS_STATE_H;
        let d = (oy * ATLAS_W + ox) * 4;
        assert_eq!(px[d + 3], 255, "stone 1x1 mip alpha");
        assert!(
            px[d] > 60 && px[d] < 200,
            "stone 1x1 mip is gray, got {}",
            px[d]
        );
        // Air sentinel (state 0) tiles stay fully transparent: its region is
        // 3 face columns wide and 2 tile rows tall at the atlas origin.
        for y in 0..ATLAS_STATE_H {
            for x in 0..3 * FACE_STRIDE {
                assert_eq!(px[(y * ATLAS_W + x) * 4 + 3], 0, "air tile at {x},{y}");
            }
        }
    }

    #[test]
    fn leaves_are_cutout_water_is_not() {
        let reg = StateRegistry::build();
        let leaves = reg.id_of("oak_leaves");
        assert!(reg.def(leaves).cutout, "oak leaves carry alpha holes");
        let water = reg.id_of("water");
        assert!(!reg.def(water).cutout);
    }

    // ---- shader-contract mirrors (assets/glsl/lod_terrain.fsh state_tile) --

    /// Mirror of the shader's `state_tile` texel mapping (v4.3.1): the uv
    /// clamp already insets half a texel, so the sample spans EXACT texel
    /// centers. v4.3.0 added another +0.5 on top, shifting every tile one
    /// texel over — its right/bottom edge then bilinear-blended with the
    /// NEIGHBOURING tile (another face or another block state): the
    /// "block textures overlap" artifact the shader-contract tests pin out.
    #[test]
    fn state_tile_uv_stays_inside_the_requested_tile() {
        for sid in 1..48usize {
            for face in 0..6usize {
                for mip in 0..5usize {
                    let tw = 16usize >> mip;
                    let off = 32 - (1usize << (5 - mip));
                    assert_eq!(off, MIP_X[mip], "shader offset must match storage");
                    assert_eq!(tw, MIP_W[mip]);
                    let ox =
                        (sid % STATES_PER_ROW) * 3 * FACE_STRIDE + (face / 2) * FACE_STRIDE + off;
                    let oy = (sid / STATES_PER_ROW) * ATLAS_STATE_H + (face % 2) * TILE;
                    for &uv in &[0.0f64, 0.31, 0.5, 0.77, 0.999_999] {
                        let ftw = tw as f64;
                        let cl = uv.clamp(0.5 / ftw, 1.0 - 0.5 / ftw) * ftw;
                        let sx = ox as f64 + cl;
                        let sy = oy as f64 + cl;
                        assert!(
                            sx >= ox as f64 && sx < (ox + tw) as f64,
                            "sid {sid} face {face} mip {mip} uv {uv}: x sample {sx} leaves the tile"
                        );
                        assert!(
                            sy >= oy as f64 && sy < (oy + TILE) as f64,
                            "sid {sid} face {face} mip {mip} uv {uv}: y sample {sy} leaves the tile"
                        );
                    }
                }
            }
        }
    }

    /// Texel-center uv points must read back exactly the source vanilla
    /// texel that `bake_states_atlas` stored there (bilinear at a texel
    /// center returns the pure texel — no bleed from neighbours).
    #[test]
    fn state_tile_addressing_reads_the_right_texels() {
        let reg = StateRegistry::build();
        let (w, h, px) = bake_states_atlas(&reg);
        for &mat in &["stone", "grass_block", "oak_leaves"] {
            let sid = reg.id_of(mat) as usize;
            assert_ne!(sid, 0, "{mat} must be a known state");
            for face in 0..6usize {
                let def = reg.def(sid as u8);
                let base = def.faces[face] as usize * TILE * TILE * 4;
                let off = MIP_X[0];
                let ox = (sid % STATES_PER_ROW) * 3 * FACE_STRIDE + (face / 2) * FACE_STRIDE + off;
                let oy = (sid / STATES_PER_ROW) * ATLAS_STATE_H + (face % 2) * TILE;
                // (u, v, texel x, texel y): uv = (texel + 0.5) / 16 centers.
                for &(u, v, tx, ty) in &[
                    (1.0 / 32.0, 1.0 / 32.0, 0usize, 0usize),
                    (17.0 / 32.0, 9.0 / 32.0, 8, 4),
                    (31.0 / 32.0, 23.0 / 32.0, 15, 11),
                ] {
                    let sx = ox as f64 + u * 16.0;
                    let sy = oy as f64 + v * 16.0;
                    let ax = (sx.floor() as usize).min(w as usize - 1);
                    let ay = (sy.floor() as usize).min(h as usize - 1);
                    let d = ((ay * w as usize) + ax) * 4;
                    let s = base + (ty * TILE + tx) * 4;
                    assert_eq!(
                        &px[d..d + 4],
                        &TEX_RGBA[s..s + 4],
                        "{mat} face {face} texel ({tx},{ty})"
                    );
                }
            }
        }
    }
}
