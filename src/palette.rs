//! Surface color resolution: block material + biome -> final albedo RGB.
//!
//! This is the server-side counterpart of what Voxy computes from live voxel
//! data: LOD cells are painted with the REAL surface colors of the world
//! (vanilla map-style block colors, biome-tinted grass/foliage/water), so the
//! distant terrain reads as Minecraft terrain instead of a flat two-tone
//! gradient. Runs once per scanned atlas cell on the main thread; the result
//! is baked into the static pack (`atlas_col.png`) — zero runtime cost.

/// Vanilla-ish grass tint per biome (grass top / foliage base multiply).
pub fn grass_tint(biome: &str) -> [u8; 3] {
    match biome {
        "PLAINS" | "SUNFLOWER_PLAINS" | "ICE_SPIKES" => [0x91, 0xBD, 0x59],
        "FOREST" | "FLOWER_FOREST" | "WOODED_HILLS" => [0x79, 0xC0, 0x5A],
        "BIRCH_FOREST" | "OLD_GROWTH_BIRCH_FOREST" => [0x88, 0xBB, 0x67],
        "DARK_FOREST" => [0x50, 0x7A, 0x32],
        "OLD_GROWTH_PINE_TAIGA" | "OLD_GROWTH_SPRUCE_TAIGA" => [0x68, 0xA4, 0x64],
        "TAIGA" | "WINDSWEPT_FOREST" => [0x86, 0xB7, 0x83],
        "SNOWY_PLAINS" | "SNOWY_SLOPES" | "FROZEN_PEAKS" | "JAGGED_PEAKS" | "SNOWY_BEACH"
        | "SNOWY_TAIGA" | "GROVE" | "FROZEN_RIVER" | "FROZEN_OCEAN" | "DEEP_FROZEN_OCEAN" => {
            [0x80, 0xB4, 0x97]
        }
        "SAVANNA" | "SAVANNA_PLATEAU" | "WINDSWEPT_SAVANNA" => [0xBF, 0xB7, 0x55],
        "JUNGLE" | "BAMBOO_JUNGLE" => [0x59, 0xC9, 0x3C],
        "SPARSE_JUNGLE" => [0x64, 0xC7, 0x3F],
        "BADLANDS" | "WOODED_BADLANDS" | "ERODED_BADLANDS" => [0x90, 0x81, 0x4D],
        "DESERT" => [0xBF, 0xB7, 0x55],
        "SWAMP" | "MANGROVE_SWAMP" => [0x6A, 0x70, 0x39],
        "MEADOW" => [0x83, 0xBB, 0x6D],
        "CHERRY_GROVE" => [0xB6, 0xDB, 0x61],
        "WINDSWEPT_HILLS" | "WINDSWEPT_GRAVELLY_HILLS" => [0x8A, 0xB6, 0x89],
        "STONY_PEAKS" | "STONY_SHORE" => [0x8A, 0xB6, 0x89],
        "RIVER" | "BEACH" => [0x71, 0xA7, 0x4D],
        "OCEAN"
        | "DEEP_OCEAN"
        | "WARM_OCEAN"
        | "LUKEWARM_OCEAN"
        | "COLD_OCEAN"
        | "DEEP_LUKEWARM_OCEAN"
        | "DEEP_COLD_OCEAN" => [0x8E, 0xB9, 0x71],
        "MUSHROOM_FIELDS" => [0x55, 0xC9, 0x3F],
        "THE_VOID" | "END_HIGHLANDS" | "NETHER_WASTES" | "CRIMSON_FOREST" | "WARPED_FOREST"
        | "SOUL_SAND_VALLEY" | "BASALT_DELTAS" | "THE_END" | "SMALL_END_ISLANDS"
        | "END_MIDLANDS" | "END_BARRENS" => [0x8E, 0xB9, 0x71],
        _ => [0x91, 0xBD, 0x59],
    }
}

/// Water color per biome (vanilla water tint base).
pub fn water_tint(biome: &str) -> [u8; 3] {
    match biome {
        "SWAMP" | "MANGROVE_SWAMP" => [0x61, 0x7B, 0x64],
        "WARM_OCEAN" | "LUKEWARM_OCEAN" | "DEEP_LUKEWARM_OCEAN" => [0x43, 0xD5, 0xEE],
        "COLD_OCEAN" | "DEEP_COLD_OCEAN" => [0x3D, 0x57, 0xD6],
        "FROZEN_OCEAN" | "DEEP_FROZEN_OCEAN" | "FROZEN_RIVER" => [0x39, 0x38, 0xC9],
        "RIVER" => [0x3F, 0x76, 0xE4],
        _ => [0x3F, 0x76, 0xE4],
    }
}

#[allow(dead_code)] // used by surface_color (tests keep it alive)
fn mul(base: [u8; 3], tint: [u8; 3]) -> [u8; 3] {
    [
        ((base[0] as u16 * tint[0] as u16) / 255) as u8,
        ((base[1] as u16 * tint[1] as u16) / 255) as u8,
        ((base[2] as u16 * tint[2] as u16) / 255) as u8,
    ]
}

/// Final surface color for one atlas cell.
///
/// `material` is the Bukkit `Material.name()` of the top block, `biome` the
/// Bukkit `Biome.name()` of the column. Unknown names fall through keyword
/// rules; the last resort is a neutral stone gray (never bright, never void).
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
        "SPRUCE_LEAVES" => [0x61, 0x77, 0x61],
        "BIRCH_LEAVES" => [0x80, 0xA7, 0x55],
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
        return mul([0x48, 0xB5, 0x18], grass_tint(biome));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grass_follows_biome() {
        let plains = surface_color("GRASS_BLOCK", "PLAINS", false);
        let swamp = surface_color("GRASS_BLOCK", "SWAMP", false);
        let snowy = surface_color("GRASS_BLOCK", "SNOWY_PLAINS", false);
        assert_ne!(plains, swamp);
        assert_ne!(plains, snowy);
        // Snowy grass is desaturated vs plains.
        assert!(snowy[0] < plains[0]);
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
        assert_eq!(a, [0x61, 0x77, 0x61]);
    }
}
