//! Surface color resolution: block material + biome -> final albedo RGB.
//!
//! This is the server-side counterpart of what Voxy computes from live voxel
//! data: LOD cells are painted with the REAL surface colors of the world
//! (vanilla map-style block colors, biome-tinted grass/foliage/water), so the
//! distant terrain reads as Minecraft terrain instead of a flat two-tone
//! gradient. Runs once per sampled cell at scan time; the result rides the
//! bit-stream as a 6x6x6 RGB lattice index on the LO dust plane — the pack
//! itself carries zero colors, zero textures.

/// Vanilla-ish grass tint per biome (grass top / foliage base multiply).
fn grass_tint(biome: &str) -> [u8; 3] {
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
fn water_tint(biome: &str) -> [u8; 3] {
    match biome {
        "SWAMP" | "MANGROVE_SWAMP" => [0x61, 0x7B, 0x64],
        "WARM_OCEAN" | "LUKEWARM_OCEAN" | "DEEP_LUKEWARM_OCEAN" => [0x43, 0xD5, 0xEE],
        "COLD_OCEAN" | "DEEP_COLD_OCEAN" => [0x3D, 0x57, 0xD6],
        "FROZEN_OCEAN" | "DEEP_FROZEN_OCEAN" | "FROZEN_RIVER" => [0x39, 0x38, 0xC9],
        "RIVER" => [0x3F, 0x76, 0xE4],
        _ => [0x3F, 0x76, 0xE4],
    }
}

fn mul(base: [u8; 3], tint: [u8; 3]) -> [u8; 3] {
    [
        ((base[0] as u16 * tint[0] as u16) / 255) as u8,
        ((base[1] as u16 * tint[1] as u16) / 255) as u8,
        ((base[2] as u16 * tint[2] as u16) / 255) as u8,
    ]
}

/// Final surface color for one scanned LOD cell.
///
/// `material` is the Bukkit `Material.name()` of the top block, `biome` the
/// Bukkit `Biome.name()` of the column. Unknown names fall through keyword
/// rules; the last resort is a neutral stone gray (never bright, never void).
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
        // v6.27.0: voxy snow bake (249,254,254) -> bin (5,5,5); the old
        // (240,250,250) landed in bin 4 = march decode (204,204,204), a
        // -50/channel miss on every snowfield.
        "SNOW" | "SNOW_BLOCK" | "POWDER_SNOW" => [0xFF, 0xFF, 0xFF],
        "ICE" | "PACKED_ICE" => [0x99, 0xCC, 0xFF],
        "FROSTED_ICE" => [0x99, 0xCC, 0xFF],
        "BLUE_ICE" => [0x74, 0xA8, 0xFD],

        // Soft ground. v6.27.0: voxy bakes dirt (138,99,70) -> bin (3,2,1);
        // coarse_dirt (125,90,63) and rooted_dirt (148,107,82) differ.
        "DIRT" | "FARMLAND" | "DIRT_PATH" => [0x99, 0x66, 0x33],
        "COARSE_DIRT" => [0x86, 0x66, 0x4A],
        "ROOTED_DIRT" => [0x99, 0x66, 0x66],
        // v6.27.0 PALETTE BAKE PARITY: voxy bakes block colors by 2x2
        // mip-averaging the REAL textures in LINEAR space (MipGen/TextureUtils
        // .mipColours — srgbToLinear -> mean -> linearToSrgb, vendored source
        // verified); our table carried vanilla MapColors, which differ by up
        // to 70 per channel. The march paints cell colors through the 6x6x6
        // lattice (decode = bin*51), so ONLY THE BIN of each value reaches
        // march-owned pixels (>64 m — server view-distance 4): every value
        // below is chosen so floor(v/51) lands in the bin nearest the voxy
        // baked color (values computed by scripts/voxy_bake_table.py from
        // the 26.3 client.jar textures). Bin-neutral rows are left untouched.
        "PODZOL" => [0x66, 0x33, 0x00],
        "MYCELIUM" => [0x66, 0x66, 0x66],
        "MUD" => [0x3C, 0x3A, 0x3F],
        "CLAY" => [0xA4, 0xA8, 0xB8],
        "GRAVEL" => [0x99, 0x99, 0x99],
        "SAND" | "SUSPICIOUS_SAND" => [0xDB, 0xD3, 0xA0],
        "RED_SAND" | "SUSPICIOUS_RED_SAND" => [0xCC, 0x66, 0x33],
        // moss is NOT biome-tinted in vanilla (the mul was double-tinting);
        // voxy bake (91,111,46) -> bin (2,2,1).
        "MOSS_BLOCK" => [0x66, 0x66, 0x33],

        // Stone family (voxy bakes: stone 126, andesite/cobble 132-137,
        // cobbled_deepslate 81, deepslate 83, tuff 111, granite (152,106,89),
        // diorite 193).
        "STONE" | "INFESTED_STONE" => [0x7A, 0x7A, 0x7A],
        "ANDESITE" | "COBBLESTONE" => [0x99, 0x99, 0x99],
        "COBBLED_DEEPSLATE" => [0x66, 0x66, 0x66],
        "DEEPSLATE" | "TUFF" => [0x66, 0x66, 0x66],
        "GRANITE" => [0x99, 0x66, 0x66],
        "DIORITE" => [0xCC, 0xCC, 0xCC],
        "CALCITE" => [0xDF, 0xE2, 0xDF],
        "BASALT" | "BLACKSTONE" => [0x4A, 0x4A, 0x52],
        "DRIPSTONE_BLOCK" => [0x99, 0x66, 0x66],
        "OBSIDIAN" => [0x00, 0x00, 0x33],
        "BEDROCK" => [0x66, 0x66, 0x66],
        "TERRACOTTA" => [0x99, 0x66, 0x33],
        "PACKED_MUD" => [0x8E, 0x67, 0x51],
        "NETHER_BRICKS" | "NETHERRACK" | "MAGMA_BLOCK" => [0x6E, 0x36, 0x3A],
        "END_STONE" => [0xDB, 0xDF, 0xA7],

        // Concrete family — v6.27.0: values whose LATTICE BIN differed from
        // the voxy texture-bake are re-snapped (the bin is what march-owned
        // pixels decode to): light_blue R bin 2->1 (voxy (36,137,199)!),
        // orange G bin 1->2, purple G 0->1, green B 0->1. The MapColors that
        // already land in the right bin stay (red/white/cyan/black/orange
        // map colors coincided with the texture means for those colors).
        // RED_CONCRETE keeps its exact transport-pinned value (color index 2
        // = cword 0x0200, the layer-8 marker; its bin is already optimal).
        "WHITE_CONCRETE" => [0xCF, 0xD5, 0xD6],
        "ORANGE_CONCRETE" => [0xCC, 0x66, 0x00],
        "MAGENTA_CONCRETE" => [0xBE, 0x45, 0xB4],
        "LIGHT_BLUE_CONCRETE" => [0x33, 0x99, 0xCC],
        "YELLOW_CONCRETE" => [0xF0, 0xC4, 0x13],
        "LIME_CONCRETE" => [0x5D, 0xB7, 0x28],
        "PINK_CONCRETE" => [0xED, 0x8D, 0xAD],
        "GRAY_CONCRETE" => [0x3D, 0x43, 0x4A],
        "LIGHT_GRAY_CONCRETE" => [0x8E, 0x8E, 0x87],
        "CYAN_CONCRETE" => [0x15, 0x77, 0x88],
        "PURPLE_CONCRETE" => [0x66, 0x33, 0x99],
        "BLUE_CONCRETE" => [0x35, 0x39, 0x9D],
        "BROWN_CONCRETE" => [0x72, 0x47, 0x28],
        "GREEN_CONCRETE" => [0x33, 0x66, 0x33],
        "RED_CONCRETE" => [0x8E, 0x21, 0x21],
        "BLACK_CONCRETE" => [0x08, 0x0A, 0x0F],

        // Vegetation blocks that carry the biome tint.
        // v6.27.0: OAK_LEAVES was mul([0x48,0xB5,0x18], tint) = (40,134,8)
        // on plains -> lattice bins (0,2,0) -> march decodes (0,102,0) — R
        // AND B zeroed (the forest canopy = the biggest march-owned surface;
        // voxy bakes the tinted leaves texture to ~(70,92,43) at the relevant
        // mips). Direct voxy-snapped value, bin (1,2,1). The other mul'd
        // leaves keep their tints pending per-biome bake data.
        "OAK_LEAVES" => [0x33, 0x66, 0x33],
        "SPRUCE_LEAVES" => [0x61, 0x77, 0x61],
        "BIRCH_LEAVES" => [0x80, 0xA7, 0x55],
        "JUNGLE_LEAVES" => mul([0x30, 0xA0, 0x24], grass),
        "ACACIA_LEAVES" => mul([0x30, 0xA0, 0x24], grass),
        "DARK_OAK_LEAVES" => mul([0x30, 0x70, 0x24], grass),
        "MANGROVE_LEAVES" => mul([0x30, 0xA0, 0x24], grass),
        "CHERRY_LEAVES" => [0xF2, 0xB8, 0xC9],
        "AZALEA_LEAVES" => mul([0x48, 0x9B, 0x30], grass),
        "AZALEA" | "FLOWERING_AZALEA" => mul([0x48, 0x9B, 0x30], grass),
        "PUMPKIN" => [0xCC, 0x66, 0x33],
        "MELON" => [0x66, 0x99, 0x33],
        "HAY_BLOCK" => [0xC2, 0xA4, 0x34],
        "CACTUS" => mul([0x0F, 0x8B, 0x0F], grass),

        // Wood. v6.27.0 voxy bakes: oak_log (113,88,52) -> bin (2,2,1),
        // spruce_log (60,39,18) -> (1,1,0), birch_log (224,223,219) -> (4,4,4).
        "OAK_LOG" | "SPRUCE_WOOD" | "OAK_WOOD" => [0x66, 0x66, 0x33],
        "SPRUCE_LOG" => [0x33, 0x33, 0x00],
        "BIRCH_LOG" | "BIRCH_WOOD" => [0xCC, 0xCC, 0xCC],
        "JUNGLE_LOG" | "ACACIA_LOG" | "JUNGLE_WOOD" => [0x53, 0x43, 0x28],
        "DARK_OAK_LOG" | "DARK_OAK_WOOD" => [0x3B, 0x2A, 0x15],
        "MANGROVE_LOG" | "MANGROVE_WOOD" => [0x58, 0x38, 0x31],
        "CHERRY_LOG" | "CHERRY_WOOD" => [0x2E, 0x22, 0x28],
        "CRIMSON_STEM" | "WARPED_STEM" => [0x5C, 0x30, 0x44],

        // Nether end-cases.
        "SOUL_SAND" => [0x51, 0x41, 0x3C],
        "SOUL_SOIL" => [0x4A, 0x38, 0x33],
        "GLOWSTONE" => [0xCC, 0x99, 0x66],
        "SHROOMLIGHT" => [0xE8, 0x8C, 0x3C],

        _ => keyword_fallback(material, biome),
    }
}

/// Keyword rules for the long tail of surface-ish materials.
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
        return [0xFF, 0xFF, 0xFF];
    }
    if m.contains("ICE") {
        return [0x99, 0xCC, 0xFF];
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
        return [0x99, 0x66, 0x33];
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

    #[test]
    fn concrete_family_carries_the_dye() {
        // The exact vanilla MapColors — pinned because the CI visual test
        // (layer 8) looks for RED_CONCRETE pixels on screen.
        assert_eq!(
            surface_color("RED_CONCRETE", "PLAINS", false),
            [0x8E, 0x21, 0x21]
        );
        assert_eq!(
            surface_color("WHITE_CONCRETE", "PLAINS", false),
            [0xCF, 0xD5, 0xD6]
        );
        assert_eq!(
            surface_color("YELLOW_CONCRETE", "PLAINS", false),
            [0xF0, 0xC4, 0x13]
        );
        // No concrete may fall through to the neutral-stone fallback that the
        // whole family hit before v6.4 (a placed build would vanish against
        // stone terrain).
        for m in [
            "ORANGE_CONCRETE",
            "MAGENTA_CONCRETE",
            "LIGHT_BLUE_CONCRETE",
            "LIME_CONCRETE",
            "PINK_CONCRETE",
            "GRAY_CONCRETE",
            "LIGHT_GRAY_CONCRETE",
            "CYAN_CONCRETE",
            "PURPLE_CONCRETE",
            "BLUE_CONCRETE",
            "BROWN_CONCRETE",
            "GREEN_CONCRETE",
            "BLACK_CONCRETE",
        ] {
            let c = surface_color(m, "PLAINS", false);
            assert_ne!(c, [0x7A, 0x7A, 0x7A], "{m} fell to the stone fallback");
        }
        // The marker red must stay visually separated from terrain reds
        // (dirt/terracotta are brown-shifted) — the on-screen probe
        // thresholds fogged pixels on r-g >= 30, so the source color needs a
        // wide margin above every overworld terrain red.
        let red = surface_color("RED_CONCRETE", "PLAINS", false);
        let dirt = surface_color("DIRT", "PLAINS", false);
        let terr = surface_color("TERRACOTTA", "PLAINS", false);
        let sep = |c: [u8; 3]| c[0] as i32 - c[1] as i32;
        assert!(sep(red) > 80, "marker red must be strongly r-dominant");
        assert!(
            sep(red) - sep(dirt) > 40,
            "marker red must separate from dirt"
        );
        assert!(
            sep(red) - sep(terr) > 40,
            "marker red must separate from terracotta"
        );
    }

    #[test]
    fn march_lattice_bins_match_voxy_bakes() {
        // v6.27.0: the march decodes cell colors through the 6x6x6 lattice
        // (bin*51), so only the BIN of each palette value reaches march-owned
        // pixels. Each entry here pins the bin derived from the voxy texture
        // bake (scripts/voxy_bake_table.py, 26.3 client.jar textures,
        // linear-space mip average) — a regression of any of these bins
        // re-opens the hot delta class on the >64 m terrain.
        let bin = |v: u8| v / 51;
        let expect = [
            ("DIRT", (3, 2, 1)),
            ("PODZOL", (2, 1, 0)),
            ("MYCELIUM", (2, 2, 2)),
            ("GRAVEL", (3, 3, 3)),
            ("RED_SAND", (4, 2, 1)),
            ("ANDESITE", (3, 3, 3)),
            ("COBBLESTONE", (3, 3, 3)),
            ("COBBLED_DEEPSLATE", (2, 2, 2)),
            ("DEEPSLATE", (2, 2, 2)),
            ("TUFF", (2, 2, 2)),
            ("GRANITE", (3, 2, 2)),
            ("DIORITE", (4, 4, 4)),
            ("DRIPSTONE_BLOCK", (3, 2, 2)),
            ("OBSIDIAN", (0, 0, 1)),
            ("BEDROCK", (2, 2, 2)),
            ("TERRACOTTA", (3, 2, 1)),
            ("SNOW_BLOCK", (5, 5, 5)),
            ("ICE", (3, 4, 5)),
            ("OAK_LEAVES", (1, 2, 1)),
            ("MOSS_BLOCK", (2, 2, 1)),
            ("OAK_LOG", (2, 2, 1)),
            ("SPRUCE_LOG", (1, 1, 0)),
            ("BIRCH_LOG", (4, 4, 4)),
            ("PUMPKIN", (4, 2, 1)),
            ("MELON", (2, 3, 1)),
            ("GLOWSTONE", (4, 3, 2)),
            ("ORANGE_CONCRETE", (4, 2, 0)),
            ("LIGHT_BLUE_CONCRETE", (1, 3, 4)),
            ("PURPLE_CONCRETE", (2, 1, 3)),
            ("GREEN_CONCRETE", (1, 2, 1)),
        ];
        for (m, want) in expect {
            let c = surface_color(m, "PLAINS", false);
            let got = (bin(c[0]), bin(c[1]), bin(c[2]));
            assert_eq!(got, want, "{m} lattice bin drifted off the voxy bake");
        }
        // The transport-pinned marker index stays 2 (cword 0x0200).
        let red = surface_color("RED_CONCRETE", "PLAINS", false);
        assert_eq!(
            bin(red[0]) + bin(red[1]) * 6 + bin(red[2]) * 36,
            2,
            "RED_CONCRETE color word index is transport-pinned"
        );
    }
}
