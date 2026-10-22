//! Biome colors — vanilla-exact, sampled like the 26.3 client (v4.4.0).
//!
//! The hand-written per-biome tables are gone. The module embeds the REAL
//! vanilla colormaps (grass / foliage / dry_foliage) and the REAL worldgen
//! biome parameters, generated from the reference client.jar by
//! `scripts/gen_biome_colors.py` (see src/biome_colors_data.rs).
//!
//! Sampling is byte-exact `net.minecraft.world.level.ColorMapColorUtil.get`:
//!     downfall *= temperature;
//!     x = (int)((1.0 - temperature) * 255.0);
//!     y = (int)((1.0 - downfall) * 255.0);   // downfall already scaled
//!     idx = (y << 8) | x;                    // row-major 256x256
//! with the decompiled vanilla fallbacks for the out-of-range indices and
//! the fixed per-block colors (birch / spruce leaves) applied by the scan.

use crate::biome_colors_data::{BiomeClimateRow, BIOME_CLIMATE};

const GRASS_MAP: &[u8] = include_bytes!("colormaps/grass.bin");
const FOLIAGE_MAP: &[u8] = include_bytes!("colormaps/foliage.bin");
const DRY_FOLIAGE_MAP: &[u8] = include_bytes!("colormaps/dry_foliage.bin");

// Decompiled vanilla default water color (every 26.3 biome JSON carries
// `water_color`, so this only guards unknown biome names).
const WATER_DEFAULT: [u8; 3] = [0x3F, 0x76, 0xE4];

/// Fixed per-block leaf colors (vanilla `BlockTintSources` constant sources):
/// birch and spruce leaves ignore the biome entirely.
pub const BIRCH_LEAVES_COLOR: [u8; 3] = [0x80, 0xA7, 0x55];
pub const SPRUCE_LEAVES_COLOR: [u8; 3] = [0x61, 0x99, 0x61];

fn climate(biome: &str) -> Option<BiomeClimateRow> {
    BIOME_CLIMATE
        .iter()
        .find(|(name, ..)| *name == biome)
        .copied()
}

/// `ColorMapColorUtil.get` — index arithmetic is exact. Vanilla indexes
/// `pixels[idx]` with the raw int (an out-of-range climate would crash the
/// client; no biome that renders grass/foliage ever goes out of range). For
/// the degenerate climates (nether temp=2.0) the sampler returns the
/// vanilla default-biome color instead of a garbage texel.
fn sample_map(map: &[u8], temp: f64, downfall: f64) -> [u8; 3] {
    let d2 = downfall * temp;
    let x = ((1.0 - temp) * 255.0) as i32;
    let y = ((1.0 - d2) * 255.0) as i32;
    if !(0..=255).contains(&x) || !(0..=255).contains(&y) {
        return default_sample(map);
    }
    let idx = ((y as usize) << 8) | x as usize;
    [map[idx * 3], map[idx * 3 + 1], map[idx * 3 + 2]]
}

/// The vanilla default biome: get(0.5, 1.0) — x = y = 127.
fn default_sample(map: &[u8]) -> [u8; 3] {
    let idx = (127 << 8) | 127;
    [map[idx * 3], map[idx * 3 + 1], map[idx * 3 + 2]]
}

fn channel(c: Option<u32>) -> Option<[u8; 3]> {
    c.map(|v| {
        [
            ((v >> 16) & 0xFF) as u8,
            ((v >> 8) & 0xFF) as u8,
            (v & 0xFF) as u8,
        ]
    })
}

/// Grass colormap color (tint kind 1): the biome's `grass_color` override
/// wins, else the grass colormap at the biome climate. The swamp /
/// dark_forest noise modifiers are pinned to their documented means in the
/// generated table (a server-side scan cannot run the position noise).
pub fn grass_tint(biome: &str) -> [u8; 3] {
    match climate(biome) {
        Some((_, t, d, g, _, _, _)) => {
            channel(g).unwrap_or_else(|| sample_map(GRASS_MAP, t as f64, d as f64))
        }
        // Vanilla default biome: GrassColor.getDefaultColor() = get(0.5, 1.0)
        None => sample_map(GRASS_MAP, 0.5, 1.0),
    }
}

/// Foliage colormap color (tint kind 2).
pub fn foliage_tint(biome: &str) -> [u8; 3] {
    match climate(biome) {
        Some((_, t, d, _, f, _, _)) => {
            channel(f).unwrap_or_else(|| sample_map(FOLIAGE_MAP, t as f64, d as f64))
        }
        // Vanilla default biome: FoliageColor.getDefaultColor() = get(0.5, 1.0)
        None => sample_map(FOLIAGE_MAP, 0.5, 1.0),
    }
}

/// Dry foliage colormap color (tint kind 4 — pale oak leaves).
pub fn dry_foliage_tint(biome: &str) -> [u8; 3] {
    match climate(biome) {
        Some((_, t, d, _, _, df, _)) => match channel(df) {
            Some(c) => c,
            None => sample_map(DRY_FOLIAGE_MAP, t as f64, d as f64),
        },
        // Vanilla default biome: get(0.5, 1.0) on the dry map
        None => sample_map(DRY_FOLIAGE_MAP, 0.5, 1.0),
    }
}

/// Water color (tint kind 3): every 26.3 biome JSON carries `water_color`.
pub fn water_tint(biome: &str) -> [u8; 3] {
    match climate(biome) {
        Some((_, _, _, _, _, _, w)) => channel(Some(w)).unwrap(),
        None => WATER_DEFAULT,
    }
}
/// Final surface color for one atlas cell.
///
/// Legacy v4.2 flattened albedo — the real pipeline textures from the block
/// tiles and tints in-shader; kept for the fallback tests only.
#[allow(dead_code)] // v4.3: the shader textures from real block tiles now;
pub fn surface_color(material: &str, biome: &str, liquid: bool) -> [u8; 3] {
    let grass = grass_tint(biome);
    let water = water_tint(biome);

    // Exact names first — the bulk of any overworld surface.
    match material {
        "GRASS_BLOCK" | "GRASS" | "TALL_GRASS" | "FERN" | "LARGE_FERN" | "MOSS_CARPET" => {
            return grass;
        }
        _ => {}
    }

    if liquid {
        return water;
    }

    match material {
        // Snow world: the bench biome look — white tops everywhere.
        "SNOW" | "SNOW_BLOCK" | "POWDER_SNOW" => [0xF0, 0xFA, 0xFA],
        "ICE" | "PACKED_ICE" => [0x7D, 0xAD, 0xFF],
        "FROSTED_ICE" => [0x7D, 0xAD, 0xFF],
        "BLUE_ICE" => [0x74, 0xA8, 0xFD],

        // Soft ground.
        "DIRT" | "COARSE_DIRT" | "ROOTED_DIRT" | "FARMLAND" | "DIRT_PATH" => [0x86, 0x66, 0x4A],
        "PODZOL" => [0x5C, 0x3F, 0x24],
        "MYCELIUM" => [0x6F, 0x62, 0x6B],
        "MUD" => [0x3C, 0x3A, 0x3F],
        "CLAY" => [0xA4, 0xA8, 0xB8],
        "GRAVEL" => [0x7F, 0x7D, 0x7B],
        "SAND" | "SUSPICIOUS_SAND" => [0xDB, 0xD3, 0xA0],
        "RED_SAND" | "SUSPICIOUS_RED_SAND" => [0xC4, 0x67, 0x3B],
        "MOSS_BLOCK" => mul([0x5A, 0x73, 0x35], grass),

        // Stone family.
        "STONE" | "ANDESITE" | "COBBLESTONE" | "COBBLED_DEEPSLATE" | "INFESTED_STONE" => {
            [0x7A, 0x7A, 0x7A]
        }
        "DEEPSLATE" | "TUFF" => [0x4C, 0x4C, 0x50],
        "GRANITE" => [0x95, 0x79, 0x6B],
        "DIORITE" => [0xBC, 0xBC, 0xBE],
        "CALCITE" => [0xDF, 0xE2, 0xDF],
        "BASALT" | "BLACKSTONE" => [0x4A, 0x4A, 0x52],
        "DRIPSTONE_BLOCK" => [0x86, 0x67, 0x55],
        "OBSIDIAN" => [0x14, 0x0C, 0x1C],
        "BEDROCK" => [0x55, 0x55, 0x55],
        "TERRACOTTA" => [0x98, 0x5E, 0x43],
        "PACKED_MUD" => [0x8E, 0x67, 0x51],
        "NETHER_BRICKS" | "NETHERRACK" | "MAGMA_BLOCK" => [0x6E, 0x36, 0x3A],
        "END_STONE" => [0xDB, 0xDF, 0xA7],

        // Vegetation blocks that carry the biome tint.
        "OAK_LEAVES" => mul([0x48, 0xB5, 0x18], grass),
        "SPRUCE_LEAVES" => SPRUCE_LEAVES_COLOR,
        "BIRCH_LEAVES" => BIRCH_LEAVES_COLOR,
        "JUNGLE_LEAVES" => mul([0x30, 0xA0, 0x24], grass),
        "ACACIA_LEAVES" => mul([0x30, 0xA0, 0x24], grass),
        "DARK_OAK_LEAVES" => mul([0x30, 0x70, 0x24], grass),
        "MANGROVE_LEAVES" => mul([0x30, 0xA0, 0x24], grass),
        "CHERRY_LEAVES" => [0xF2, 0xB8, 0xC9],
        "AZALEA_LEAVES" => mul([0x48, 0x9B, 0x30], grass),
        "AZALEA" | "FLOWERING_AZALEA" => mul([0x48, 0x9B, 0x30], grass),
        "PUMPKIN" => [0xC0, 0x76, 0x18],
        "MELON" => [0x76, 0xB0, 0x2E],
        "HAY_BLOCK" => [0xC2, 0xA4, 0x34],
        "CACTUS" => mul([0x0F, 0x8B, 0x0F], grass),

        // Wood.
        "OAK_LOG" | "SPRUCE_LOG" | "OAK_WOOD" | "SPRUCE_WOOD" => [0x67, 0x4C, 0x2B],
        "BIRCH_LOG" | "BIRCH_WOOD" => [0xD7, 0xC9, 0x8D],
        "JUNGLE_LOG" | "ACACIA_LOG" | "JUNGLE_WOOD" => [0x53, 0x43, 0x28],
        "DARK_OAK_LOG" | "DARK_OAK_WOOD" => [0x3B, 0x2A, 0x15],
        "MANGROVE_LOG" | "MANGROVE_WOOD" => [0x58, 0x38, 0x31],
        "CHERRY_LOG" | "CHERRY_WOOD" => [0x2E, 0x22, 0x28],
        "CRIMSON_STEM" | "WARPED_STEM" => [0x5C, 0x30, 0x44],

        // Nether end-cases.
        "SOUL_SAND" => [0x51, 0x41, 0x3C],
        "SOUL_SOIL" => [0x4A, 0x38, 0x33],
        "GLOWSTONE" => [0xC7, 0xA0, 0x68],
        "SHROOMLIGHT" => [0xE8, 0x8C, 0x3C],

        _ => keyword_fallback(material, biome),
    }
}

/// Keyword rules for the long tail of surface-ish materials.
#[allow(dead_code)] // used by surface_color (tests keep it alive)
fn keyword_fallback(material: &str, biome: &str) -> [u8; 3] {
    let m = material;
    if m.contains("LEAVES") {
        return foliage_tint(biome);
    }
    if m.contains("GRASS") || m.contains("FERN") || m.contains("MOSS") || m.contains("AZALEA") {
        return mul([0x7F, 0xA8, 0x54], grass_tint(biome));
    }
    if m.contains("WATER") || m.contains("KELP") || m.contains("SEAGRASS") {
        return water_tint(biome);
    }
    if m.contains("SNOW") {
        return [0xF0, 0xFA, 0xFA];
    }
    if m.contains("ICE") {
        return [0x7D, 0xAD, 0xFF];
    }
    if m.contains("SAND") {
        return [0xDB, 0xD3, 0xA0];
    }
    if m.contains("LOG") || m.contains("WOOD") || m.contains("PLANKS") || m.contains("STEM") {
        return [0x67, 0x4C, 0x2B];
    }
    if m.contains("ORE") || m.contains("DEEPSLATE") {
        return [0x5B, 0x5B, 0x5F];
    }
    if m.contains("TERRACOTTA") {
        return [0x98, 0x5E, 0x43];
    }
    if m.contains("GLASS") || m.contains("AIR") {
        return [0x9A, 0xA7, 0xB0];
    }
    if m.contains("WOOL") || m.contains("CARPET") {
        return [0xE0, 0xDD, 0xD5];
    }
    // Neutral stone gray: plausible for the remaining stone-ish tops.
    [0x7A, 0x7A, 0x7A]
}

#[allow(dead_code)] // used by surface_color (tests keep it alive)
fn mul(base: [u8; 3], tint: [u8; 3]) -> [u8; 3] {
    [
        ((base[0] as u16 * tint[0] as u16) / 255) as u8,
        ((base[1] as u16 * tint[1] as u16) / 255) as u8,
        ((base[2] as u16 * tint[2] as u16) / 255) as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grass_follows_biome() {
        let plains = grass_tint("PLAINS");
        let jungle = grass_tint("JUNGLE");
        let snowy = grass_tint("SNOWY_PLAINS");
        // Vanilla colormap values (well-known anchors).
        assert_eq!(plains, [0x91, 0xBD, 0x59], "plains grass");
        assert_eq!(jungle, [0x59, 0xC9, 0x3C], "jungle grass");
        assert_eq!(snowy, [0x80, 0xB4, 0x97], "snowy grass");
        assert_ne!(plains, jungle);
    }

    #[test]
    fn foliage_differs_from_grass() {
        // Vanilla: forest grass 0x79C05A, forest foliage 0x59AE30.
        assert_eq!(grass_tint("FOREST"), [0x79, 0xC0, 0x5A]);
        assert_eq!(foliage_tint("FOREST"), [0x59, 0xAE, 0x30]);
        // Desert (temp 2.0) never renders vanilla foliage — the vanilla
        // sampler would index out of bounds; our sampler returns the
        // default-biome texel instead. Both degenerate climates agree.
        assert_eq!(foliage_tint("DESERT"), foliage_tint("BASALT_DELTAS"));
    }

    #[test]
    fn overrides_win_over_colormaps() {
        // Cherry grove pins both grass and foliage.
        assert_eq!(grass_tint("CHERRY_GROVE"), [0xB6, 0xDB, 0x61]);
        assert_eq!(foliage_tint("CHERRY_GROVE"), [0xB6, 0xDB, 0x61]);
        // Swamp grass uses the pinned modifier mean.
        assert_eq!(grass_tint("SWAMP"), [0x6A, 0x70, 0x39]);
        assert_eq!(water_tint("SWAMP"), [0x61, 0x7B, 0x64]);
        // Warm ocean water.
        assert_eq!(water_tint("WARM_OCEAN"), [0x43, 0xD5, 0xEE]);
    }

    #[test]
    fn unknown_biome_falls_back_to_vanilla_defaults() {
        // GrassColor.getDefaultColor() = get(0.5, 1.0) = the map texel at
        // (127, 127). OCEAN has its own climate (downfall 0.4) and its own
        // color — it must NOT equal the default.
        assert_eq!(grass_tint("SOME_FUTURE_BIOME"), [0x7C, 0xBD, 0x6B]);
        assert_ne!(grass_tint("OCEAN"), grass_tint("SOME_FUTURE_BIOME"));
        assert_eq!(foliage_tint("NOT_A_BIOME"), foliage_tint("DESERT"));
        assert_eq!(dry_foliage_tint("NOT_A_BIOME"), dry_foliage_tint("DESERT"));
        assert_eq!(water_tint("NOT_A_BIOME"), WATER_DEFAULT);
    }

    #[test]
    fn snow_is_white_water_is_blue() {
        let snow = surface_color("SNOW_BLOCK", "SNOWY_PLAINS", false);
        assert!(snow[0] > 220 && snow[1] > 230 && snow[2] > 230);
        let w = surface_color("WATER", "OCEAN", true);
        assert!(w[2] > w[0] && w[2] > 150);
    }

    #[test]
    fn fallbacks_are_never_bright_or_void() {
        for m in [
            "SOME_FUTURE_BLOCK",
            "MAGIC_STONE",
            "UNKNOWN_LEAVES_BLOCK",
            "RED_WOOL_THING",
        ] {
            let c = surface_color(m, "PLAINS", false);
            assert!(c.iter().all(|&v| v <= 0xE6));
            // Never void: every channel stays visibly above zero.
            assert!(c.iter().all(|&v| v >= 0x04));
        }
    }

    #[test]
    fn deterministic_and_in_range() {
        let a = surface_color("SPRUCE_LEAVES", "TAIGA", false);
        let b = surface_color("SPRUCE_LEAVES", "TAIGA", false);
        assert_eq!(a, b);
        assert_eq!(a, SPRUCE_LEAVES_COLOR);
    }
}
