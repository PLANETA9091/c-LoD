//! Assembles the c-LoD resource pack: pack.mcmeta, the two post-effect
//! chains (JSON with config values substituted), the GLSL shaders and the
//! core-shader overrides (sky camera channel + text map carrier). Output is
//! a zip (STORE) + its SHA-1 for client caching.
//!
//! v5.0: THE PACK CONTAINS NO WORLD DATA. No atlas PNGs, nothing
//! world-derived — every LOD byte streams at runtime through vanilla map
//! packets. The pack is therefore byte-identical for the module's whole
//! lifetime, is pushed exactly once per client join, and never rebuilds.

use crate::config::Config;
use crate::mapcolor;
use crate::sha1::Sha1;
use crate::zipw::ZipWriter;

pub struct Pack {
    pub zip: Vec<u8>,
    pub sha1: [u8; 20],
    pub sha1_hex: String,
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

pub fn build(cfg: &Config) -> Pack {
    let lod_terrain_tpl = include_str!("../assets/post_effect/lod_terrain.json");
    let atmosphere_tpl = include_str!("../assets/post_effect/atmosphere.json");

    // 26.3 veil window in EYE-SPACE BLOCKS: the veil starts taking over just
    // before the LOD band begins and reaches full strength at `veil_far`.
    let atmo_start = fmt_f(cfg.lod_near * 0.85);
    let atmo_end = fmt_f(cfg.veil_far);
    // Static fallback fog (pre-first-frame only): spans the march horizon.
    let fog_start = cfg.lod_near * 1.05;
    let fog_end = cfg.region_blocks as f32 * 1.5;

    let veil_rgba = [
        cfg.color_veil[0],
        cfg.color_veil[1],
        cfg.color_veil[2],
        cfg.snow_line,
    ];

    let map: Vec<(&str, String)> = vec![
        ("__FOV__", fmt_f(cfg.fov)),
        ("__REGION_BLOCKS__", fmt_f(cfg.region_blocks as f32)),
        ("__CELL__", fmt_f(cfg.cell_size as f32)),
        ("__LOD_NEAR__", fmt_f(cfg.lod_near)),
        ("__MAX_HEIGHT__", fmt_f(cfg.max_height as f32)),
        ("__SEA_LEVEL__", fmt_f(cfg.sea_level as f32)),
        ("__MIN_CONF__", fmt_f(0.25)),
        ("__FOG_START__", fmt_f(fog_start)),
        ("__FOG_END__", fmt_f(fog_end)),
        // 26.3 post-chain codec: a vec4 uniform value MUST be a 4-element
        // list (client error: "Input is not a list of 4 elements") — pad the
        // RGB palette colors with alpha 1.0.
        ("__COL_LOW__", rgba(cfg.color_low)),
        ("__COL_HIGH__", rgba(cfg.color_high)),
        ("__COL_WATER__", rgba(cfg.color_water)),
        ("__VEIL_RGBA__", vec4(veil_rgba)),
        ("__ATMO_START__", atmo_start),
        ("__ATMO_END__", atmo_end),
        ("__ATMO_STRENGTH__", fmt_f(0.92)),
        // 26.3 device depth: d = depth_near / eyeZ (infinite far, sky == 0.0).
        ("__DEPTH_NEAR__", fmt_f(cfg.depth_near)),
        // The streaming alphabet (identical to src/mapcolor.rs on the server).
        ("__CLOD_ALPHA__", mapcolor::glsl_alphabet()),
    ];

    let lod_terrain_json = substitute(lod_terrain_tpl, &map);
    let atmosphere_json = substitute(atmosphere_tpl, &map);

    let col_scan = include_str!("../assets/glsl/col_scan.fsh");
    let rot_solve = include_str!("../assets/glsl/rot_solve.fsh");
    let rot_copy = include_str!("../assets/glsl/rot_copy.fsh");
    let scrape = substitute(include_str!("../assets/glsl/scrape.fsh"), &map);
    let merge = include_str!("../assets/glsl/merge.fsh");
    let lod_terrain = include_str!("../assets/glsl/lod_terrain.fsh");
    let clod_blit = include_str!("../assets/glsl/clod_blit.fsh");
    let atmosphere = include_str!("../assets/glsl/atmosphere.fsh");
    // Camera channel: the vanilla sky core-shader override.
    let sky_vsh = include_str!("../assets/glsl/sky.vsh");
    let sky_fsh = include_str!("../assets/glsl/sky.fsh");
    // Map carrier: the vanilla text core-shader override (parks the streamed
    // map quad into the scrape slot; everything else is vanilla).
    let text_vsh = include_str!("../assets/glsl/text.vsh");
    let text_fsh = include_str!("../assets/glsl/text.fsh");

    // 26.3 pack.mcmeta schema (see v3 notes): min_format/max_format mandatory
    // for formats > 64; 97 = resource_major of 26.3.
    let mcmeta = "{\"pack\":{\"description\":\"c-LoD v5 streamed LOD (shader-only pack, world data rides vanilla map packets)\",\"min_format\":97,\"max_format\":97}}".to_string();

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
    zip.add("assets/clod/shaders/post/scrape.fsh", scrape.as_bytes());
    zip.add("assets/clod/shaders/post/merge.fsh", merge.as_bytes());
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
    zip.add("assets/minecraft/shaders/core/sky.vsh", sky_vsh.as_bytes());
    zip.add("assets/minecraft/shaders/core/sky.fsh", sky_fsh.as_bytes());
    zip.add(
        "assets/minecraft/shaders/core/text.vsh",
        text_vsh.as_bytes(),
    );
    zip.add(
        "assets/minecraft/shaders/core/text.fsh",
        text_fsh.as_bytes(),
    );
    let zip_bytes = zip.finish();

    let mut h = Sha1::new();
    h.update(&zip_bytes);
    let sha1 = h.finish();
    let sha1_hex = crate::sha1::sha1_hex(&zip_bytes);

    Pack {
        zip: zip_bytes,
        sha1,
        sha1_hex,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_builds_and_is_valid_zip() {
        let cfg = Config::parse("");
        let pack = build(&cfg);

        assert!(!pack.zip.is_empty());
        assert_eq!(pack.sha1_hex.len(), 40);

        // EOCD present
        let n = pack.zip.len();
        assert_eq!(&pack.zip[n - 22..n - 18], &[0x50, 0x4b, 0x05, 0x06]);

        let count = u16::from_le_bytes([pack.zip[n - 12], pack.zip[n - 11]]) as usize;
        assert_eq!(
            count, 15,
            "pack.mcmeta + 2 json + 8 fsh + sky.vsh + sky.fsh + text.vsh + text.fsh"
        );
    }

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
    fn pack_contains_zero_textures_and_no_tokens() {
        let cfg = Config::parse("");
        let pack = build(&cfg);
        let entries = zip_entries(&pack.zip);
        for (name, _) in &entries {
            assert!(!name.ends_with(".png"), "no textures may ship: {name}");
            assert!(!name.contains("textures/"), "no textures may ship: {name}");
        }
        let blob = String::from_utf8_lossy(&pack.zip);
        assert!(!blob.contains("__FOV__"), "tokens must be substituted");
        assert!(!blob.contains("__CLOD_ALPHA__"), "LUT must be substituted");
        assert!(blob.contains("CLOD_ALPHA_N"), "LUT must be present");
    }

    #[test]
    fn pack_bytes_are_deterministic() {
        let a = build(&Config::parse(""));
        let b = build(&Config::parse(""));
        assert_eq!(a.zip, b.zip, "pack bytes must be deterministic");
        assert_eq!(a.sha1_hex, b.sha1_hex);
    }

    #[test]
    fn chain_has_stream_passes_and_no_atlas_inputs() {
        let cfg = Config::parse("");
        let pack = build(&cfg);
        let entries = zip_entries(&pack.zip);
        let chain = &entries
            .iter()
            .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
            .unwrap()
            .1;
        let chain = String::from_utf8_lossy(chain);
        assert!(chain.contains("clod:post/scrape"), "scrape pass missing");
        assert!(chain.contains("clod:post/merge"), "merge pass missing");
        assert!(
            !chain.contains("clod:atlas"),
            "no atlas texture inputs may remain"
        );
        assert!(chain.contains("\"clod_scol_a\""), "stream target missing");
        assert!(
            chain.contains("\"persistent\": true"),
            "persistence missing"
        );
        assert!(
            !chain.contains("ScanBox"),
            "the static scan box is gone (region grid)"
        );
    }

    #[test]
    fn camera_and_carrier_overrides_ship() {
        let cfg = Config::parse("");
        let pack = build(&cfg);
        let entries = zip_entries(&pack.zip);
        let names: Vec<&str> = entries.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"assets/minecraft/shaders/core/sky.fsh"));
        assert!(names.contains(&"assets/minecraft/shaders/core/text.vsh"));
        assert!(names.contains(&"assets/minecraft/shaders/core/text.fsh"));

        let text = &entries
            .iter()
            .find(|(n, _)| n == "assets/minecraft/shaders/core/text.vsh")
            .unwrap()
            .1;
        let text = String::from_utf8_lossy(text);
        assert!(text.contains("MAP CARRIER"), "carrier marker missing");
        assert!(text.contains("vClodMap"), "map varying missing");
        assert!(
            text.contains("#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)"),
            "variant guards must be preserved"
        );
    }
}
