//! Assembles the c-LoD resource pack: pack.mcmeta, the two post-effect
//! chains (JSON with config values substituted), the GLSL shaders and the
//! heightmap atlas PNG. Output is a zip (STORE) + its SHA-1 for client
//! caching.

use crate::atlas::Atlas;
use crate::config::Config;
use crate::png;
use crate::sha1::Sha1;
use crate::zipw::ZipWriter;

pub struct Pack {
    pub zip: Vec<u8>,
    pub sha1: [u8; 20],
    pub sha1_hex: String,
    /// Atlas version at build time (logging only — the pack bytes themselves
    /// are deterministic and never depend on the internal version counter).
    #[allow(dead_code)]
    pub atlas_version: u64,
}

fn fmt_f(v: f32) -> String {
    if v.abs() < 1e-6 {
        return "0.0".to_string();
    }
    let s = format!("{v:.4}");
    let s = s.trim_end_matches('0').trim_end_matches('.').to_string();
    if s.is_empty() || s == "-" {
        "0.0".to_string()
    } else if s.ends_with('.') {
        format!("{s}0")
    } else {
        s
    }
}

fn vec4(v: [f32; 4]) -> String {
    format!(
        "[{}, {}, {}, {}]",
        fmt_f(v[0]),
        fmt_f(v[1]),
        fmt_f(v[2]),
        fmt_f(v[3])
    )
}

fn rgba(v: [f32; 3]) -> String {
    // colors are authored as RGB; the uniform is vec4 — append opaque alpha
    format!("[{}, {}, {}, 1.0]", fmt_f(v[0]), fmt_f(v[1]), fmt_f(v[2]))
}

fn substitute(template: &str, map: &[(&str, String)]) -> String {
    let mut out = template.to_string();
    for (k, v) in map {
        out = out.replace(k, v);
    }
    out
}

pub fn build(cfg: &Config, atlas: &Atlas) -> Pack {
    let atlas_version = atlas.version();

    let lod_terrain_tpl = include_str!("../assets/post_effect/lod_terrain.json");
    let atmosphere_tpl = include_str!("../assets/post_effect/atmosphere.json");

    let origin_x = atlas.center_x - atlas.half();
    let origin_z = atlas.center_z - atlas.half();
    // Upstream v4.0.0: the veil window is expressed in EYE-SPACE BLOCKS (the
    // shader converts device depth via eyeZ = depth_near / d), so it starts
    // just before the LOD band and reaches full strength at `veil_far`.
    let atmo_start = fmt_f(cfg.lod_near * 0.85);
    let atmo_end = fmt_f(cfg.veil_far);
    let fog_start = cfg.lod_near * 1.05;
    // DH-exact far-fog plane: the LOD draw distance = the configured data
    // radius (GlDhFogShader: uFogScale = 1 / lodChunkRenderDistanceRadius*16;
    // farFogStart 0.4, farFogEnd 1.0 are fractions of it). The old local tweak
    // (size*0.72 / half*0.92) existed to hide the walls that scan holes used
    // to grow — the renderer now treats void cells as "no geometry" exactly
    // like DH's drawnTo check, so the fog can sit at the true data edge.
    let fog_end = atlas.half() as f32;

    // Highest scanned height in the atlas: anything above it in the scene can
    // only be the cloud layer (or weather), never LOD terrain.
    let atlas_max_h = atlas
        .snapshot()
        .iter()
        .filter(|c| c.valid)
        .map(|c| c.height)
        .max()
        .unwrap_or(0) as f32;

    // DH 3.3.4 textured LODs: ratio-tile atlas geometry (256 tiles/row).
    let tiles_w = crate::blocktextures::TILES_PER_ROW * crate::blocktextures::TILE_PX;
    let tile_rows = crate::blocktextures::TILE_COUNT.div_ceil(crate::blocktextures::TILES_PER_ROW);
    let tiles_h = (tile_rows * crate::blocktextures::TILE_PX).max(16);

    let veil_rgba = [
        cfg.color_veil[0],
        cfg.color_veil[1],
        cfg.color_veil[2],
        cfg.snow_line,
    ];

    // Top-face colours as the DH detail pyramid strip (v4.7): level 0 is the
    // per-cell image, every next level averages 2x2 — the shader picks the
    // octave by the pixel's world footprint, which is what makes DH's far
    // LODs smooth instead of per-cell speckle.
    let (cw, ch, color_rgba) = atlas.encode_colors_pyramid();
    let color_png = png::encode_png(cw, ch, &color_rgba);
    // Side-index pyramid (v4.7.1): the mode-aggregated wall material per
    // octave — the FALLBACK wall path for octaves the DH slice strip cannot
    // cover (big-atlas configs only; see encode_slices).
    let (pw_, ph_, sidepyr_rgba) = atlas.encode_side_pyramid();
    let sidepyr_png = png::encode_png(pw_, ph_, &sidepyr_rgba);
    // DH vertical slices (v4.9/v4.10): the banded cliff walls. Width 3n
    // (texel triples A|B|C per cell — C carries the run's real solid
    // bottom), height = sum of K(detail)*(n>>L) per octave.
    let (sw, sh, slice_rgba, slice_max_oct) = atlas.encode_slices();
    let slice_png = png::encode_png(sw, sh, &slice_rgba);

    // DH LodQuadBuilder FADE_TO_DIRT target: getDirtBlockColor() — the baked
    // dirt block colour. ColLow was a v1 palette remnant (unused since the
    // scan baked per-cell colours); it now carries the dirt colour.
    let dirt_rgb = crate::blockcolors::lookup("dirt").map_or(0x86_64_46u32, |(t, _, _, _)| t);
    let dirt_rgba = [
        ((dirt_rgb >> 16) & 0xFF) as f32 / 255.0,
        ((dirt_rgb >> 8) & 0xFF) as f32 / 255.0,
        (dirt_rgb & 0xFF) as f32 / 255.0,
    ];

    // DH earth curvature (GlDhTerrainShaderProgram): uEarthRadius =
    // 6371000.0 / curveRatio when |ratio| > 1, else 0 (disabled).
    let earth_radius = if cfg.earth_curve_ratio.abs() > 1 {
        6_371_000.0_f32 / cfg.earth_curve_ratio as f32
    } else {
        0.0
    };

    let map: Vec<(&str, String)> = vec![
        ("__FOV__", fmt_f(cfg.fov)),
        ("__ORIGIN_X__", fmt_f(origin_x as f32)),
        ("__ORIGIN_Z__", fmt_f(origin_z as f32)),
        ("__ATLAS_SIZE__", fmt_f(atlas.size as f32)),
        ("__ATLAS_CELLS__", atlas.cells.to_string()),
        ("__COLOR_STRIP_H__", ch.to_string()),
        ("__SIDE_STRIP_H__", ph_.to_string()),
        ("__SLICE_STRIP_H__", sh.to_string()),
        ("__SLICE_W__", (sw as usize).to_string()),
        ("__SLICE_MAX_LVL__", format!("{slice_max_oct}.0")),
        ("__EARTH_RADIUS__", fmt_f(earth_radius)),
        ("__CELL__", fmt_f(atlas.cell as f32)),
        ("__TILES_W__", tiles_w.to_string()),
        ("__TILES_H__", tiles_h.to_string()),
        ("__LOD_NEAR__", fmt_f(cfg.lod_near)),
        ("__MAX_HEIGHT__", fmt_f(cfg.max_height as f32)),
        ("__SEA_LEVEL__", fmt_f(cfg.sea_level as f32)),
        ("__MIN_CONF__", fmt_f(0.25)),
        ("__FOG_START__", fmt_f(fog_start)),
        ("__FOG_END__", fmt_f(fog_end)),
        // 26.3 post-chain codec: a vec4 uniform value MUST be a 4-element
        // list (client error: "Input is not a list of 4 elements") — pad the
        // RGB palette colors with alpha 1.0. ColLow now carries the DH
        // FADE_TO_DIRT dirt colour (the v1 low-palette is long gone).
        ("__COL_LOW__", rgba(dirt_rgba)),
        ("__COL_HIGH__", rgba(cfg.color_high)),
        ("__COL_WATER__", rgba(cfg.color_water)),
        ("__VEIL_RGBA__", vec4(veil_rgba)),
        ("__DEBUG__", format!("{}.0", cfg.debug_lod)),
        ("__ATLAS_MAX_H__", fmt_f(atlas_max_h + 3.0)),
        ("__TILE_ROWS__", tile_rows.to_string()),
        ("__ATMO_START__", atmo_start),
        ("__ATMO_END__", atmo_end),
        // The distance veil is disabled (strength 0): it sits on real geometry
        // only, but a strength > 0 also tints distant clouds and the chunk
        // wall; LOD fog already handles the horizon, and the veil must never
        // starve the sun tracker.
        ("__ATMO_STRENGTH__", fmt_f(0.0)),
        // 26.3 device depth: d = depth_near / eyeZ (infinite far, sky == 0.0).
        ("__DEPTH_NEAR__", fmt_f(cfg.depth_near)),
    ];

    let lod_terrain_json = substitute(lod_terrain_tpl, &map);
    let atmosphere_json = substitute(atmosphere_tpl, &map);

    let col_scan = include_str!("../assets/glsl/col_scan.fsh");
    let rot_solve = include_str!("../assets/glsl/rot_solve.fsh");
    let rot_copy = include_str!("../assets/glsl/rot_copy.fsh");
    let lod_terrain = include_str!("../assets/glsl/lod_terrain.fsh");
    let clod_blit = include_str!("../assets/glsl/clod_blit.fsh");
    let atmosphere = include_str!("../assets/glsl/atmosphere.fsh");
    // v4.1 camera channel: the vanilla sky core-shader override. The client
    // resolves core shader sources through the resource system (verified:
    // RenderPipelines binds pipeline/sky to shader ids "core/sky"), so the
    // server pack layers the payload-instrumented sky on top of vanilla.
    let sky_vsh = include_str!("../assets/glsl/sky.vsh");
    let sky_fsh = include_str!("../assets/glsl/sky.fsh");
    // Vanilla terrain fog is disabled through a core-shader override: vanilla
    // fades the real chunks to the fog colour at the server view distance,
    // which drew a bright ring exactly where the LOD band starts.
    let terrain_vsh = include_str!("../assets/mc/terrain.vsh");

    // 26.3 moved resource packs to the major.minor scheme (26.3 = 97.1) and
    // REJECTS a bare `pack_format` when it is newer than 64:
    //   "Pack declares support for version newer than 64, but is missing
    //    mandatory fields min_format and max_format"
    // `PackFormat` is decoded with `compactListCodec(INT, INT.listOf())`, so
    // `[major, minor]` is the canonical form. Without this the client refuses
    // the pack and every `clod:*` post effect stays "does not exist".
    let mcmeta = concat!(
        "{\"pack\":{",
        "\"description\":\"c-LoD v3 static LOD terrain (posteffect + heightfield raymarch)\",",
        "\"min_format\":[97,0],",
        "\"max_format\":[97,1]",
        "}}"
    )
    .to_string();

    let (aw, ah, atlas_rgba) = atlas.encode_rgba();
    let atlas_png = png::encode_png(aw, ah, &atlas_rgba);
    // Side/cliff colours: one 256x1 strip, indexed by the atlas alpha channel.
    let side_lut = atlas.side_lut();
    let mut side_rgba = vec![0u8; 256 * 4];
    for (i, c) in side_lut.iter().enumerate() {
        side_rgba[i * 4] = ((c >> 16) & 0xFF) as u8;
        side_rgba[i * 4 + 1] = ((c >> 8) & 0xFF) as u8;
        side_rgba[i * 4 + 2] = (c & 0xFF) as u8;
        side_rgba[i * 4 + 3] = 255;
    }
    let side_png = png::encode_png(256, 1, &side_rgba);
    // DH 3.3.4 textured LODs: the ratio-tile atlas (16px tiles, 256/row) and
    // the per-cell tile ids (nearest-filtered, ids never blend). The last
    // atlas row is zero-padded to a full rectangle (unreferenced ids).
    let tiles_len = (tiles_w * tiles_h * 4) as usize;
    let mut tiles_rgba = vec![0u8; tiles_len];
    let src = crate::blocktextures::TILE_PIXELS;
    tiles_rgba[..src.len()].copy_from_slice(src);
    let tiles_png = png::encode_png(tiles_w, tiles_h, &tiles_rgba);
    let (txw, txh, tex_rgba) = atlas.encode_tiles();
    let tex_png = png::encode_png(txw, txh, &tex_rgba);

    let mut zip = ZipWriter::new();
    zip.add("pack.mcmeta", mcmeta.as_bytes());
    zip.add(
        "assets/clod/post_effect/lod_terrain.json",
        lod_terrain_json.as_bytes(),
    );
    zip.add(
        "assets/clod/post_effect/atmosphere.json",
        atmosphere_json.as_bytes(),
    );
    zip.add("assets/clod/shaders/post/col_scan.fsh", col_scan.as_bytes());
    zip.add(
        "assets/clod/shaders/post/rot_solve.fsh",
        rot_solve.as_bytes(),
    );
    zip.add("assets/clod/shaders/post/rot_copy.fsh", rot_copy.as_bytes());
    zip.add(
        "assets/clod/shaders/post/lod_terrain.fsh",
        lod_terrain.as_bytes(),
    );
    zip.add(
        "assets/clod/shaders/post/clod_blit.fsh",
        clod_blit.as_bytes(),
    );
    zip.add(
        "assets/clod/shaders/post/atmosphere.fsh",
        atmosphere.as_bytes(),
    );
    // Camera channel: vanilla sky core-shader override (exact 26.3 vsh +
    // payload fsh). Resolves through the resource stack above vanilla.
    zip.add("assets/minecraft/shaders/core/sky.vsh", sky_vsh.as_bytes());
    zip.add("assets/minecraft/shaders/core/sky.fsh", sky_fsh.as_bytes());
    zip.add(
        "assets/minecraft/shaders/core/terrain.vsh",
        terrain_vsh.as_bytes(),
    );
    zip.add("assets/clod/textures/effect/atlas.png", &atlas_png);
    zip.add("assets/clod/textures/effect/color.png", &color_png);
    zip.add("assets/clod/textures/effect/sidepyr.png", &sidepyr_png);
    zip.add("assets/clod/textures/effect/slices.png", &slice_png);
    zip.add("assets/clod/textures/effect/side.png", &side_png);
    zip.add("assets/clod/textures/effect/tiles.png", &tiles_png);
    zip.add("assets/clod/textures/effect/tex.png", &tex_png);
    let zip_bytes = zip.finish();

    let mut h = Sha1::new();
    h.update(&zip_bytes);
    let sha1 = h.finish();
    let sha1_hex = crate::sha1::sha1_hex(&zip_bytes);

    Pack {
        zip: zip_bytes,
        sha1,
        sha1_hex,
        atlas_version,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zipw;

    #[test]
    fn pack_builds_and_is_valid_zip() {
        let cfg = Config::parse("");
        let atlas = Atlas::new(
            cfg.center_x,
            cfg.center_z,
            cfg.atlas_size,
            cfg.cell_size,
            cfg.sea_level,
        );
        atlas.set(0, 0, 70, false, 0x808080, 0x707070, 0, 0);
        let pack = build(&cfg, &atlas);

        assert!(!pack.zip.is_empty());
        assert_eq!(pack.sha1_hex.len(), 40);
        assert_eq!(pack.atlas_version, atlas.version());

        // EOCD present
        let n = pack.zip.len();
        assert_eq!(&pack.zip[n - 22..n - 18], &[0x50, 0x4b, 0x05, 0x06]);

        // Must contain the essential entries: scan by walking central dir.
        let count = u16::from_le_bytes([pack.zip[n - 12], pack.zip[n - 11]]) as usize;
        assert_eq!(
            count, 19,
            "pack.mcmeta + 2 json + 6 fsh + 7 textures + sky.vsh/fsh + terrain.vsh"
        );

        // JSON chains are valid JSON (parenthesis balance + token-free).
        let s = String::from_utf8_lossy(pack.zip.as_slice());
        assert!(!s.contains("__FOV__"), "tokens must be substituted");
        let _ = zipw::crc32(pack.zip.as_slice());
    }

    #[test]
    fn mcmeta_declares_the_26_3_minor_range() {
        // 26.3 rejects a bare `pack_format` > 64 ("missing mandatory fields
        // min_format and max_format"): the whole pack — post-effect chains
        // included — is then never registered by the client.
        let cfg = Config::parse("");
        let atlas = Atlas::new(
            cfg.center_x,
            cfg.center_z,
            cfg.atlas_size,
            cfg.cell_size,
            cfg.sea_level,
        );
        let pack = build(&cfg, &atlas);
        let s = String::from_utf8_lossy(&pack.zip);
        assert!(s.contains("\"min_format\":[97,0]"), "min_format missing");
        assert!(s.contains("\"max_format\":[97,1]"), "max_format missing");
        assert!(!s.contains("\"pack_format\""), "legacy pack_format kept");
    }

    #[test]
    fn pack_bytes_are_deterministic_across_atlas_instances() {
        // Static-pack SHA-1 stability: two independent atlases with the same
        // cell data (e.g. live vs restored from disk) must produce the exact
        // same pack bytes, regardless of internal version counters.
        let cfg = Config::parse("");
        let a = Atlas::new(0, 0, 256, 4, 63);
        a.set(1, 1, 90, false, 0x808080, 0x707070, 0, 0);
        a.set(2, 2, 40, true, 0x808080, 0x707070, 0, 0);
        let pack_a = build(&cfg, &a);

        let b = Atlas::new(0, 0, 256, 4, 63);
        b.set(1, 1, 90, false, 0x808080, 0x707070, 0, 0);
        b.set(2, 2, 40, true, 0x808080, 0x707070, 0, 0);
        let pack_b = build(&cfg, &b);

        assert_eq!(pack_a.zip, pack_b.zip, "pack bytes must be deterministic");
        assert_eq!(pack_a.sha1_hex, pack_b.sha1_hex);
    }

    #[test]
    fn fmt_f_stable() {
        assert_eq!(fmt_f(70.0), "70");
        assert_eq!(fmt_f(0.0), "0.0");
        assert_eq!(fmt_f(-1.5), "-1.5");
        assert_eq!(fmt_f(0.25), "0.25");
    }

    /// Walks the local file headers of a STORE-only zip produced by zipw.
    fn zip_entries(zip: &[u8]) -> Vec<(String, Vec<u8>)> {
        let mut out = Vec::new();
        let mut p = 0usize;
        while p + 4 <= zip.len() && zip[p..p + 4] == [0x50, 0x4b, 0x03, 0x04] {
            let name_len = u16::from_le_bytes([zip[p + 26], zip[p + 27]]) as usize;
            let extra_len = u16::from_le_bytes([zip[p + 28], zip[p + 29]]) as usize;
            let size =
                u32::from_le_bytes([zip[p + 18], zip[p + 19], zip[p + 20], zip[p + 21]]) as usize;
            let name = String::from_utf8_lossy(&zip[p + 30..p + 30 + name_len]).into_owned();
            let data_start = p + 30 + name_len + extra_len;
            out.push((name, zip[data_start..data_start + size].to_vec()));
            p = data_start + size;
        }
        out
    }

    #[test]
    fn camera_channel_sky_override_is_shipped() {
        // v4.1: the pack must layer the payload-instrumented vanilla sky core
        // shader over the vanilla one (resolved through the resource system),
        // and the sun tracker shaders must be gone.
        let cfg = Config::parse("");
        let atlas = Atlas::new(0, 0, 256, 4, 63);
        let pack = build(&cfg, &atlas);
        let entries = zip_entries(&pack.zip);
        let names: Vec<&str> = entries.iter().map(|(n, _)| n.as_str()).collect();

        assert!(
            names.contains(&"assets/minecraft/shaders/core/sky.fsh"),
            "sky.fsh override missing: {names:?}"
        );
        assert!(names.contains(&"assets/minecraft/shaders/core/sky.vsh"));
        assert!(
            !names.iter().any(|n| n.contains("sun_")),
            "sun tracker must be gone"
        );

        let sky = &entries
            .iter()
            .find(|(n, _)| n == "assets/minecraft/shaders/core/sky.fsh")
            .unwrap()
            .1;
        let sky = String::from_utf8_lossy(sky);
        assert!(sky.contains("camera channel"), "payload marker missing");
        assert!(
            sky.contains("transpose(mat3(ModelViewMat))"),
            "basis source missing"
        );
        assert!(
            sky.contains("#include <minecraft:projection.glsl>"),
            "ProjMat include missing"
        );

        let chain = &entries
            .iter()
            .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
            .unwrap()
            .1;
        let chain = String::from_utf8_lossy(chain);
        assert!(
            chain.contains("clod:post/col_scan"),
            "col_scan pass missing"
        );
        assert!(
            chain.contains("clod:post/rot_solve"),
            "rot_solve pass missing"
        );
        assert!(
            !chain.contains("sun_"),
            "sun passes must be gone from the chain"
        );
    }
}
