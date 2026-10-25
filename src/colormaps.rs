//! The vanilla grass/foliage/dry_foliage colormap pixel matrices (baked by
//! `scripts/bake_colormaps.py` from the 26.3 client jar,
//! `assets/minecraft/textures/colormap/*.png`, 256×256 row-major ARGB).
//!
//! The dedicated server never loads these textures (the reload listeners are
//! client-only), so the NMS `GrassColor`/`FoliageColor` matrices stay zeroed
//! and `Biome#getGrassColor(x, z)` returns black for every colormap biome.
//! The module pushes these arrays into the JVM once at world resolution
//! (`GrassColor.init(int[])` — a public static, Mojang-mapped on Paper 26.3),
//! which makes the server-side resolvers pixel-identical to the client's —
//! exactly the live tint Distant Horizons reads when it colours a column.

/// `GrassColor.init` payload — row-major ARGB, top-down.
pub const GRASS: &[u8] = include_bytes!("colormaps/grass.bin");
/// `FoliageColor.init` payload.
pub const FOLIAGE: &[u8] = include_bytes!("colormaps/foliage.bin");
/// `DryFoliageColor.init` payload (pale garden).
pub const DRY_FOLIAGE: &[u8] = include_bytes!("colormaps/dry_foliage.bin");

/// The map as JVM jint values (ARGB), one per pixel.
pub fn as_jints(bytes: &[u8]) -> Vec<i32> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|c| i32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_are_full_size_and_opaque() {
        for m in [GRASS, FOLIAGE, DRY_FOLIAGE] {
            assert_eq!(m.len(), 65536 * 4);
            for c in m.as_chunks::<4>().0 {
                assert_eq!(c[3], 0xFF, "colormap alpha must be opaque");
            }
        }
    }

    #[test]
    fn plains_anchor_is_the_vanilla_tint() {
        // plains: temp 0.8, downfall 0.4 -> col 50, row 173 -> 0xFF91BD59
        let col = ((1.0 - 0.8) * 255.0) as usize;
        let row = ((1.0 - 0.4 * 0.8) * 255.0) as usize;
        let i = (row * 256 + col) * 4;
        assert_eq!(&GRASS[i..i + 4], &[0x59, 0xBD, 0x91, 0xFF]);
    }
}
