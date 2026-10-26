//! The vanilla map-color palette (26.3 `net.minecraft.world.level.material.MapColor`)
//! as the streaming alphabet.
//!
//! The LOD atlas does not ship inside the resource pack anymore: every cell is
//! quantized to a MapColor packed id (`colorId << 2 | brightness`) and streamed
//! to the client through vanilla `ClientboundMapItemDataPacket`s. The client
//! stores the bytes in a 128x128 `DynamicTexture` (`minecraft:map/<id>`), the
//! held map renders the texture through the `core/text` shader (which the pack
//! overrides to park the quad in a fixed screen slot), and a post-chain pass
//! scrapes the slot back into a persistent render target — the streamed atlas.
//!
//! Byte -> RGB is EXACTLY `MapColor.getColorFromPackedId`:
//!   color = MATERIAL_COLORS[byte >> 2], shade = byte & 3,
//!   rgb = (channel * modifier) / 255 with integer floor division
//!   (ARGB.scaleRGB(int, int)), modifier = [180, 220, 255, 135][shade].
//!
//! Bytes 0..3 are `NONE` (transparent): the text shader discards alpha < 0.1,
//! so they can never carry data. Bytes whose RGB collides with another byte's
//! RGB (or sits within a 2/255 dead zone of a neighbor) are excluded from the
//! height alphabet so the screen-side decoder can invert the table safely.

/// Vanilla 26.3 MapColor table: (id, 0xRRGGBB). Ids 0..61, verbatim from the
/// decompiled client jar.
const COLORS: [(u8, u32); 62] = [
    (0, 0x000000),  // NONE (never used: transparent)
    (1, 8368696),   // GRASS
    (2, 16247203),  // SAND
    (3, 0xC7C7C7),  // WOOL
    (4, 0xFF0000),  // FIRE
    (5, 0xA0A0FF),  // ICE
    (6, 0xA7A7A7),  // METAL
    (7, 31744),     // PLANT
    (8, 0xFFFFFF),  // SNOW
    (9, 10791096),  // CLAY
    (10, 9923917),  // DIRT
    (11, 0x707070), // STONE
    (12, 0x4040FF), // WATER
    (13, 9402184),  // WOOD
    (14, 0xFFFCF5), // QUARTZ
    (15, 14188339), // COLOR_ORANGE
    (16, 11685080), // COLOR_MAGENTA
    (17, 6724056),  // COLOR_LIGHT_BLUE
    (18, 0xE5E533), // COLOR_YELLOW
    (19, 8375321),  // COLOR_LIGHT_GREEN
    (20, 15892389), // COLOR_PINK
    (21, 0x4C4C4C), // COLOR_GRAY
    (22, 0x999999), // COLOR_LIGHT_GRAY
    (23, 5013401),  // COLOR_CYAN
    (24, 8339378),  // COLOR_PURPLE
    (25, 3361970),  // COLOR_BLUE
    (26, 6704179),  // COLOR_BROWN
    (27, 6717235),  // COLOR_GREEN
    (28, 0x993333), // COLOR_RED
    (29, 0x191919), // COLOR_BLACK
    (30, 16445005), // GOLD
    (31, 6085589),  // DIAMOND
    (32, 4882687),  // LAPIS
    (33, 55610),    // EMERALD
    (34, 8476209),  // PODZOL
    (35, 0x700200), // NETHER
    (36, 13742497), // TERRACOTTA_WHITE
    (37, 10441252), // TERRACOTTA_ORANGE
    (38, 9787244),  // TERRACOTTA_MAGENTA
    (39, 7367818),  // TERRACOTTA_LIGHT_BLUE
    (40, 12223780), // TERRACOTTA_YELLOW
    (41, 6780213),  // TERRACOTTA_LIGHT_GREEN
    (42, 10505550), // TERRACOTTA_PINK
    (43, 0x392923), // TERRACOTTA_GRAY
    (44, 8874850),  // TERRACOTTA_LIGHT_GRAY
    (45, 0x575C5C), // TERRACOTTA_CYAN
    (46, 8014168),  // TERRACOTTA_PURPLE
    (47, 4996700),  // TERRACOTTA_BLUE
    (48, 4993571),  // TERRACOTTA_BROWN
    (49, 5001770),  // TERRACOTTA_GREEN
    (50, 9321518),  // TERRACOTTA_RED
    (51, 2430480),  // TERRACOTTA_BLACK
    (52, 12398641), // CRIMSON_NYLIUM
    (53, 9715553),  // CRIMSON_STEM
    (54, 6035741),  // CRIMSON_HYPHAE
    (55, 1474182),  // WARPED_NYLIUM
    (56, 3837580),  // WARPED_STEM
    (57, 5647422),  // WARPED_HYPHAE
    (58, 1356933),  // WARPED_WART_BLOCK
    (59, 0x646464), // DEEPSLATE
    (60, 14200723), // RAW_IRON
    (61, 8365974),  // GLOW_LICHEN
];

/// Brightness modifiers (MapColor.Brightness): LOW, NORMAL, HIGH, LOWEST.
const SHADES: [u32; 4] = [180, 220, 255, 135];

/// Alphabet layout (indices into the SAFE alphabet, both sides agree):
///   0..15  -> marker digits (slot, page, base-Y nibbles, spares),
///   16..31 -> reserved (never used as data),
///   32..   -> color layer + height layer codes.
/// Height codes are RELATIVE to a per-region base Y streamed in two marker
/// digits: y = base + (alphabet_index - HGT_BASE). The top two alphabet
/// indices are the WATER and VOID flags of the height layer.
pub const MARKER_DIGITS: usize = 16;
pub const HGT_BASE: usize = 32;

/// rgb (0..255 each) of one packed map byte, exactly like the client renders it.
/// Color ids >= 62 fall back to NONE = fully transparent on the client, so
/// they are reported as None and never enter the alphabet.
pub fn packed_rgb_checked(byte: u8) -> Option<[u8; 3]> {
    let cidx = (byte >> 2) as usize;
    if cidx >= COLORS.len() || cidx == 0 {
        return None; // NONE: transparent (text shader discards alpha < 0.1)
    }
    let color = COLORS[cidx].1;
    let m = SHADES[(byte & 3) as usize];
    Some([
        (((color >> 16) & 0xFF) * m / 255) as u8,
        (((color >> 8) & 0xFF) * m / 255) as u8,
        ((color & 0xFF) * m / 255) as u8,
    ])
}

pub fn packed_rgb(byte: u8) -> [u8; 3] {
    packed_rgb_checked(byte).unwrap_or([0, 0, 0])
}

/// The streaming alphabet: packed bytes usable for data (non-transparent,
/// pairwise RGB distance >= 3 on at least one channel). Sorted by byte value.
fn safe_alphabet() -> Vec<u8> {
    let mut out = Vec::new();
    for b in 4u16..=255u16 {
        let b = b as u8;
        let Some(rgb) = packed_rgb_checked(b) else {
            continue; // NONE fallback (color id >= 62): transparent, unusable
        };
        // Reject exact duplicates and near-neighbors: the screen decoder
        // inverts RGB -> byte with a nearest match; the quantization noise of
        // the post chain is bounded by 1/255, so a 3/255 margin is safe.
        let mut ok = true;
        for &o in out.iter() {
            let orgb = packed_rgb(o);
            let d = [
                rgb[0].abs_diff(orgb[0]),
                rgb[1].abs_diff(orgb[1]),
                rgb[2].abs_diff(orgb[2]),
            ];
            if d[0] < 3 && d[1] < 3 && d[2] < 3 {
                ok = false;
                break;
            }
        }
        if ok {
            out.push(b);
        }
    }
    out
}

/// Static alphabet (computed once, deterministic).
fn alphabet() -> &'static Vec<u8> {
    use std::sync::OnceLock;
    static ALPHA: OnceLock<Vec<u8>> = OnceLock::new();
    ALPHA.get_or_init(safe_alphabet)
}

/// Encode a value 0..=N into a data byte (indices into the safe alphabet).
/// Values beyond the alphabet saturate.
fn code_byte(index: usize) -> u8 {
    let a = alphabet();
    *a.get(index).unwrap_or(a.last().unwrap_or(&4))
}

/// Decode a data byte back to its alphabet index (for tests / server debug).
#[allow(dead_code)]
fn code_index(byte: u8) -> Option<usize> {
    alphabet().iter().position(|&b| b == byte)
}

/// Marker digit byte for v in 0..16 (alphabet index = v).
pub fn marker_byte(v: u8) -> u8 {
    code_byte(v as usize)
}

/// Decode a marker byte to its digit value (None when the byte is not a
/// marker digit).
#[allow(dead_code)]
pub fn marker_value(byte: u8) -> Option<u8> {
    code_index(byte)
        .filter(|i| *i < MARKER_DIGITS)
        .map(|i| i as u8)
}

/// Height-layer code for a surface Y relative to the region base.
pub fn height_code(y: i32, base_y: i32) -> u8 {
    let a = alphabet();
    let n = a.len();
    // Top two indices are WATER / VOID; usable height span = base..base+span.
    let span = (n - HGT_BASE - 2).max(1);
    let h = (y - base_y).clamp(0, span as i32 - 1) as usize;
    code_byte(HGT_BASE + h)
}

pub fn water_code() -> u8 {
    let n = alphabet().len();
    code_byte(n - 1)
}

pub fn void_code() -> u8 {
    let n = alphabet().len();
    code_byte(n - 2)
}

/// Quantize a surface color to the nearest palette byte (color layer).
/// Linear-RGB nearest by squared distance — the LOD is viewed at distance,
/// perceptual precision is unnecessary and nearest keeps the decoder simple.
pub fn color_byte(rgb: [u8; 3]) -> u8 {
    let a = alphabet();
    let mut best = a[0];
    let mut best_d = u32::MAX;
    for (i, &b) in a.iter().enumerate() {
        // Marker digits (alphabet indices < 16) stay reserved for both
        // layers so a stray marker never leaks into terrain colors.
        if i < MARKER_DIGITS {
            continue;
        }
        let c = packed_rgb(b);
        let dr = rgb[0] as i32 - c[0] as i32;
        let dg = rgb[1] as i32 - c[1] as i32;
        let db = rgb[2] as i32 - c[2] as i32;
        let d = (dr * dr + dg * dg + db * db) as u32;
        if d < best_d {
            best_d = d;
            best = b;
        }
    }
    best
}

/// Emit the GLSL side of the alphabet: a const array of vec3 (RGB / 255, or
/// vec3(-1) for unused slots) + the byte value of each slot. Indices here are
/// ALPHABET indices (0..N); the GLSL decoder receives the alphabet as data
/// because the Rust side owns the exact filtering decisions.
pub fn glsl_alphabet() -> String {
    let a = alphabet();
    let mut out = String::with_capacity(a.len() * 48);
    out.push_str(&format!("const int CLOD_ALPHA_N = {};\n", a.len()));
    out.push_str("const vec3 CLOD_ALPHA_RGB[256] = vec3[256](\n");
    for i in 0..256 {
        if let Some(&b) = a.get(i) {
            let c = packed_rgb(b);
            out.push_str(&format!(
                "vec3({}, {}, {})",
                c[0] as f32 / 255.0,
                c[1] as f32 / 255.0,
                c[2] as f32 / 255.0
            ));
        } else {
            out.push_str("vec3(-1.0)");
        }
        out.push_str(if i == 255 { "\n" } else { ",\n" });
    }
    out.push_str(");\n");
    out.push_str("const int CLOD_ALPHA_BYTE[256] = int[256](\n");
    for i in 0..256 {
        match a.get(i) {
            Some(&b) => out.push_str(&format!("{b}")),
            None => out.push_str("-1"),
        }
        out.push_str(if i == 255 { "\n" } else { ",\n" });
    }
    out.push_str(");\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lut_matches_client_formula() {
        // Spot-check against the decompiled ARGB.scaleRGB(int, int):
        // channel * modifier / 255 (integer floor), colors verbatim.
        // SNOW (id 8) HIGH (shade 2, modifier 255) -> 255,255,255.
        assert_eq!(packed_rgb((8 << 2) | 2), [255, 255, 255]);
        // GRASS 8368696 = 0x7FB238 -> (127,178,56); LOW (180): 127*180/255=89,
        // 178*180/255=125, 56*180/255=39.
        assert_eq!(packed_rgb(1 << 2), [89, 125, 39]);
        // LOWEST (135): 127*135/255=67, 178*135/255=94, 56*135/255=29.
        assert_eq!(packed_rgb((1 << 2) | 3), [67, 94, 29]);
        // WATER 0x4040FF NORMAL (220): 64*220/255=55, 55, 255*220/255=220.
        assert_eq!(packed_rgb((12 << 2) | 1), [55, 55, 220]);
    }

    #[test]
    fn alphabet_excludes_transparent_and_is_big() {
        let a = alphabet();
        assert!(!a.iter().any(|&b| b < 4), "NONE bytes must be excluded");
        // 61 usable colors x 4 shades = 244 candidates; the dead-zone filter
        // must keep the overwhelming majority (the palette is well spread).
        assert!(a.len() > 200, "alphabet too small: {}", a.len());
        // Deterministic + strictly the same across calls (OnceLock).
        assert_eq!(a, &safe_alphabet());
    }

    #[test]
    fn markers_are_unique_and_decodable() {
        let mut seen = Vec::new();
        for v in 0..MARKER_DIGITS as u8 {
            let b = marker_byte(v);
            assert!(!seen.contains(&b), "marker {v} collides");
            seen.push(b);
            assert_eq!(marker_value(b), Some(v));
        }
        // Water/void flags never collide with marker digits.
        assert_ne!(marker_byte(0), water_code());
        assert_ne!(marker_byte(0), void_code());
        assert_ne!(water_code(), void_code());
    }

    #[test]
    fn height_roundtrip_relative_to_base() {
        for base in [-128i32, -64, 0, 64] {
            let b = height_code(base + 10, base);
            let idx = code_index(b).unwrap();
            assert_eq!(idx - HGT_BASE, 10);
        }
        // Above the region span saturates; below the base clamps to 0.
        let base = 0;
        let top = height_code(100_000, base);
        assert_eq!(code_index(top).unwrap(), alphabet().len() - 3);
        assert_eq!(height_code(-1, base), height_code(0, base));
    }

    #[test]
    fn color_quantizer_prefers_exact_hits() {
        // A palette color quantizes to itself.
        let grass = packed_rgb(marker_byte(0)); // deterministic probe color
        let _ = grass;
        let snow = packed_rgb((8 << 2) | 2);
        let b = color_byte(snow);
        assert_eq!(packed_rgb(b), snow, "SNOW must roundtrip");
        // Black-ish input stays dark (never bright).
        let dark = color_byte([8, 8, 10]);
        let c = packed_rgb(dark);
        assert!(c.iter().all(|&v| v < 60), "dark must stay dark: {c:?}");
    }

    #[test]
    fn glsl_alphabet_is_wellformed() {
        let glsl = glsl_alphabet();
        assert!(glsl.contains("CLOD_ALPHA_N = "));
        assert!(glsl.matches("vec3(").count() >= 200);
        assert!(glsl.contains("const int CLOD_ALPHA_BYTE[256]"));
        assert_eq!(glsl.matches(",\n").count(), 510, "two 256-entry arrays");
    }
}
