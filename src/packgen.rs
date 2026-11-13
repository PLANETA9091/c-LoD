//! Assembles the c-LoD resource pack: pack.mcmeta, the two post-effect
//! chains (JSON with config values substituted) and the GLSL shaders plus
//! the core sky override (camera channel). Output is a zip (STORE) + its
//! SHA-1 for client caching.
//!
//! v5.0: THE PACK CONTAINS NO WORLD DATA AND NO TEXTURES AT ALL. Every LOD
//! byte streams at runtime as dust particles over the sky-grid; the pack is
//! byte-identical for the module's whole lifetime, is pushed exactly once
//! per client join, and never rebuilds.

use crate::config::Config;
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

/// v6.22: the rank ladder as the INSIDE of a JSON vec4 value — one
/// component per rank ("80.0, 140.0, 200.0, 260.0"). NOT fmt_f: fmt_f
/// trims to "80" (the v6.19.3 lesson), and the pack assert greps the
/// one-decimal ladder verbatim; both forms are valid JSON numbers, the
/// .1 format just keeps the ladder self-describing.
fn tiers_vec(v: [f64; 4]) -> String {
    v.iter()
        .map(|&y| format!("{y:.1}"))
        .collect::<Vec<_>>()
        .join(", ")
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
    // v6.7 CAPTURE LENS: `debug_lens=true` swaps in the debug chain (the
    // capture-lens pass appended after the blit) and ships its shader. The
    // production pack is byte-identical to the pre-v6.7 artifact; the debug
    // pack's SHA differs (one-time re-push on the CI runner — expected).
    // v6.18 UN-FOG: `unfog=true` swaps in the un-fog chain (the inversion
    // pass between dust_mask and the march — assets/glsl/unfog.fsh) and
    // ships its shader; same one-time-re-push semantics. The combo with
    // debug_lens ships BOTH (the §5b evidence lane: lens + un-fog); the
    // parity world session runs the plain production unfog chain.
    let debug_lens = cfg.debug_lens;
    let unfog = cfg.unfog;
    let lod_terrain_tpl = if debug_lens && unfog {
        // v6.18.2: the debug chain + the un-fog pass — the §5b evidence lane
        // (lens panels + the un-fog engagement map in one client).
        include_str!("../assets/post_effect/lod_terrain_debug_unfog.json")
    } else if debug_lens {
        include_str!("../assets/post_effect/lod_terrain_debug.json")
    } else if unfog {
        include_str!("../assets/post_effect/lod_terrain_unfog.json")
    } else {
        include_str!("../assets/post_effect/lod_terrain.json")
    };
    let atmosphere_tpl = include_str!("../assets/post_effect/atmosphere.json");

    // 26.3 veil window in EYE-SPACE BLOCKS: the veil starts taking over just
    // before the LOD band begins and reaches full strength at `veil_far`.
    let atmo_start = fmt_f(cfg.lod_near * 0.85);
    let atmo_end = fmt_f(cfg.veil_far);
    // Static fallback fog (pre-first-frame only): spans the march horizon.
    // v6.18.2: the static fallback fog = the REAL 26.3 overworld defaults,
    // read from the client.jar itself: dimension attributes
    // visual/fog_start_distance = 0.0 and visual/fog_end_distance = 1024.0
    // (EnvironmentAttributes.<clinit> defaults; the overworld dimension_type
    // JSON overrides neither). With this pair the march's no-channel fog
    // EXACTLY equals the voxy finish() formula the 07_far_north pair
    // verified at 99.5% (#287) — the pre-channel frames stop carrying a
    // foreign fog class. The previous values (lod_near*1.05, region*1.5)
    // were pre-v4.2 guesses from before the fog semantics were extracted.
    let fog_start = 0.0f32;
    let fog_end = 1024.0f32;

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
        // v6.19 G8d MESH PAINT: 1.0 arms the march's vanilla path (the
        // depth-tested paint-over in lod_terrain.fsh).
        (
            "__MESH_PAINT__",
            fmt_f(if cfg.mesh_paint { 1.0 } else { 0.0 }),
        ),
        // SKY-GRID geometry (must match src/stream.rs exactly).
        // v6.4.2: the config names the disk CENTER (see engine::arm_from_config);
        // GridInfo.xy is the disk ORIGIN (the shader maps world->cell via
        // (xz - GridInfo.xy)/cell), so the same center->origin conversion
        // applies here. Passing the center verbatim shifted every painted
        // dome 256 blocks off the streamed disk.
        (
            "__ORIGIN_X__",
            fmt_f((cfg.center_x.unwrap_or(0.0) - f64::from(cfg.region_blocks) * 0.5) as f32),
        ),
        (
            "__ORIGIN_Z__",
            fmt_f((cfg.center_z.unwrap_or(0.0) - f64::from(cfg.region_blocks) * 0.5) as f32),
        ),
        ("__PLANE_HI_Y__", fmt_f(crate::stream::PLANE_HI_Y as f32)),
        ("__PLANE_LO_Y__", fmt_f(crate::stream::PLANE_LO_Y as f32)),
        // v6.22 TIERED TRANSPORT: the rank ladders ride the ClodTiers UBO in
        // every capture/dust_mask pass of the chain JSONs — the same single
        // source of truth (src/stream.rs) as the v5.1 plane tokens above.
        ("__TIERS_LO__", tiers_vec(crate::stream::LO_TIERS)),
        ("__TIERS_HI__", tiers_vec(crate::stream::HI_TIERS)),
        ("__TIERS_RS__", tiers_vec(crate::stream::RS_TIERS)),
        (
            "__SUB__",
            fmt_f(cfg.cell_size as f32 / crate::stream::SUB_PER_CELL as f32),
        ),
        // v6.2 edit-reset plane (must match src/stream.rs exactly); v6.22:
        // the RANK-0 reset plane — the JSON still carries a single scalar
        // (StreamInfo.z, legacy mode-2 lane), per-cell resets ride the
        // ClodTiers ladder the capture walks instead.
        ("__RESET_Y__", fmt_f(crate::stream::RS_TIERS[0] as f32)),
    ];

    // v6.7 lens window: 64x64 cells around the debug window center (the
    // /clod-cells center when set, else the disk center), clamped into the
    // fixed 256x256 capture target.
    if debug_lens {
        let side = (cfg.region_blocks / cfg.cell_size.max(1)).clamp(1, 256);
        let win = 64i32.min(side);
        let (ccx, ccz) = cfg.debug_cell.unwrap_or((side / 2, side / 2));
        let lx = (ccx - win / 2).clamp(0, side - win).max(0);
        let lz = (ccz - win / 2).clamp(0, side - win).max(0);
        let mut m2 = map;
        m2.push(("__LENS_X__", fmt_f(lx as f32)));
        m2.push(("__LENS_Z__", fmt_f(lz as f32)));
        m2.push(("__LENS_WIN__", fmt_f(win as f32)));
        return build_zip(
            lod_terrain_tpl,
            atmosphere_tpl,
            m2,
            debug_lens,
            unfog,
            cfg.unfog_debug,
            cfg.mesh_debug,
            cfg.quant_ssao,
            cfg.fog_voxy,
            cfg.rim_mask,
        );
    }
    build_zip(
        lod_terrain_tpl,
        atmosphere_tpl,
        map,
        debug_lens,
        unfog,
        cfg.unfog_debug,
        cfg.mesh_debug,
        cfg.quant_ssao,
        cfg.fog_voxy,
        cfg.rim_mask,
    )
}

#[allow(clippy::too_many_arguments)] // the knob set grows with every CI-lever pack; v6.30.0 added rim_mask
fn build_zip(
    lod_terrain_tpl: &str,
    atmosphere_tpl: &str,
    map: Vec<(&str, String)>,
    debug_lens: bool,
    unfog: bool,
    unfog_debug: bool,
    mesh_debug: bool,
    quant_ssao: bool,
    fog_voxy: bool,
    rim_mask: bool,
) -> Pack {
    let lod_terrain_json = substitute(lod_terrain_tpl, &map);
    let atmosphere_json = substitute(atmosphere_tpl, &map);
    // v6.18.2: the evidence lane points the chain's un-fog pass at the mask
    // shader (the string occurs exactly once — the pass fragment_shader).
    let lod_terrain_json = if unfog && unfog_debug {
        lod_terrain_json.replace("clod:post/unfog", "clod:post/unfog_debug")
    } else {
        lod_terrain_json
    };

    let col_scan = include_str!("../assets/glsl/col_scan.fsh");
    let rot_solve = include_str!("../assets/glsl/rot_solve.fsh");
    let rot_copy = include_str!("../assets/glsl/rot_copy.fsh");
    let capture = include_str!("../assets/glsl/capture.fsh");
    let dust_mask = include_str!("../assets/glsl/dust_mask.fsh");
    // v6.19.2 MESH DEBUG: the probe rides a COMPILE-TIME #define — it cannot
    // fail to bind (the PaintParams UBO is the suspect the probe tests).
    // The marker line stays a comment in the default build (glslang sees the
    // raw file; the game sees the substituted one).
    let lod_terrain = if mesh_debug {
        include_str!("../assets/glsl/lod_terrain.fsh")
            .replace("// __MESH_DEBUG__", "#define MESH_DEBUG 1")
    } else {
        include_str!("../assets/glsl/lod_terrain.fsh").to_string()
    };
    // v6.26.3 QUANT SSAO (candidate (b)): floor()-quantize the march_ssao
    // occluder probe height — the AO field becomes block-stepped like the
    // voxy depth buffer. The swap target is the exact probe statement
    // carrying the __QUANT_SSAO__ marker comment; the default build keeps
    // the line byte-identical to the template (marker stays a comment).
    let lod_terrain = if quant_ssao {
        lod_terrain.replace(
            "float hq = ground_at(q, 0.0, 0.0, sw, sok); // __QUANT_SSAO__",
            "float hq = floor(ground_at(q, 0.0, 0.0, sw, sok)); // __QUANT_SSAO__",
        )
    } else {
        lod_terrain
    };
    // v6.26.7 FOG VOXY EXACT (rim-class candidate: 07/12 flat_signed +61 = ours
    // lighter — our far fog overshoots): the march fog mix ports voxy's blit
    // fog — post/blit_texture_depth_cutout.frag (HAS_FOG block) +
    // NormalRenderPipeline.java:99-107; FogMode default FOG_AND_FADE
    // (VoxyConfig.getFogMode(): null -> FOG_AND_FADE; the CI voxy-config.json
    // pins no fogMode) and getVanillaRenderDistance() = options RD * 16 (blocks,
    // VoxyRenderSystem.java:441-443). The voxy lerp CLAMPS at a cap computed
    // from endDistance = max(vanillaRD_blocks, 20*16) * sqrt(3) — at RD 8 the
    // cap is 0.541 of the 0..1024 pair, our current line runs to 1.0. The env
    // pair rides FogParams (the 26.3 overworld defaults 0..1024 — the static
    // fallback, live-identical at noon parity); vanRD (fogRng.b) = RD blocks.
    // d = length(point.xyz) (full 3D, rev3d) == our tHit basis. The default
    // build keeps the fog line byte-identical to the template (the marker
    // stays a comment; glslang sees the raw file).
    let lod_terrain = if fog_voxy {
        lod_terrain.replace(
            "float fog = fogLive ? clamp(tHit * (1.0 / 1024.0), 0.0, 1.0) * fogOpacity : 0.0; // __FOG_VOXY__",
            "// v6.26.7 FOG VOXY EXACT: voxy post/blit_texture_depth_cutout.frag (HAS_FOG)\n    // + NormalRenderPipeline.java:99-107, FogMode default FOG_AND_FADE.\n    float invEndFogDelta = 1.0 / (FogParams.y - FogParams.x);\n    float startDelta = -FogParams.x * invEndFogDelta;\n    float endDistance = max(vanRD, 320.0) * 1.7320508; // sqrt(3)\n    float fogCap = clamp(endDistance * invEndFogDelta + startDelta, 0.0, 1.0);\n    float fog = fogLive ? clamp(tHit * invEndFogDelta + startDelta, 0.0, fogCap) * fogOpacity : 0.0; // __FOG_VOXY__",
        )
    } else {
        lod_terrain
    };
    // v6.30.0 RIM MASK PROBE (#360 pre-registered item (2)): the compile-time
    // decode-state paint — every march-owned pixel becomes GREEN (height and
    // color words decoded) / RED (colorless — the color word is absent) /
    // BLUE (height absent at a march hit). One mask frame from a rim vantage
    // (07/12) names the rim fallback class exactly; the production build
    // keeps the marker a comment and the pack byte-stable.
    let lod_terrain = if rim_mask {
        lod_terrain.replace("// __RIM_MASK__", "#define RIM_MASK 1")
    } else {
        lod_terrain
    };
    let clod_blit = include_str!("../assets/glsl/clod_blit.fsh");
    let atmosphere = include_str!("../assets/glsl/atmosphere.fsh");
    // v6.18: the un-fog shader ships in EVERY unfog variant; unfog_debug
    // swaps the chain's pass to the engagement-mask shader (evidence lane).
    let unfog_fsh = if unfog {
        Some(if unfog_debug {
            include_str!("../assets/glsl/unfog_debug.fsh")
        } else {
            include_str!("../assets/glsl/unfog.fsh")
        })
    } else {
        None
    };
    // v6.7: the capture lens ships ONLY in the debug pack.
    let debug_lens_fsh = if debug_lens {
        Some(include_str!("../assets/glsl/debug_lens.fsh"))
    } else {
        None
    };
    // Camera channel: the vanilla sky core-shader override.
    let sky_vsh = include_str!("../assets/glsl/sky.vsh");
    let sky_fsh = include_str!("../assets/glsl/sky.fsh");

    // 26.3 pack.mcmeta schema (see v3 notes): min_format/max_format mandatory
    // for formats > 64; 97 = resource_major of 26.3.
    let mcmeta = "{\"pack\":{\"description\":\"c-LoD v6 live-delta streamed LOD (shader-only pack, zero textures, world data rides dust particles)\",\"min_format\":97,\"max_format\":97}}".to_string();

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
    zip.add("assets/clod/shaders/post/capture.fsh", capture.as_bytes());
    zip.add(
        "assets/clod/shaders/post/dust_mask.fsh",
        dust_mask.as_bytes(),
    );
    zip.add(
        "assets/clod/shaders/post/lod_terrain.fsh",
        lod_terrain.as_bytes(),
    );
    zip.add(
        "assets/clod/shaders/post/clod_blit.fsh",
        clod_blit.as_bytes(),
    );
    if let Some(fsh) = unfog_fsh {
        let name = if unfog_debug {
            "assets/clod/shaders/post/unfog_debug.fsh"
        } else {
            "assets/clod/shaders/post/unfog.fsh"
        };
        zip.add(name, fsh.as_bytes());
    }
    zip.add(
        "assets/clod/shaders/post/atmosphere.fsh",
        atmosphere.as_bytes(),
    );
    if let Some(lens) = debug_lens_fsh {
        zip.add("assets/clod/shaders/post/debug_lens.fsh", lens.as_bytes());
    }
    zip.add("assets/minecraft/shaders/core/sky.vsh", sky_vsh.as_bytes());
    zip.add("assets/minecraft/shaders/core/sky.fsh", sky_fsh.as_bytes());
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
            count, 13,
            "pack.mcmeta + 2 json + 8 fsh + sky.vsh + sky.fsh"
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
        assert!(
            !blob.contains("__MESH_PAINT__"),
            "mesh-paint token must be substituted"
        );
        assert!(
            !blob.contains("__ORIGIN_X__"),
            "grid tokens must be substituted"
        );
        assert!(
            !blob.contains("__PLANE_HI_Y__"),
            "plane tokens must be substituted"
        );
        assert!(
            !blob.contains("__TIERS_LO__"),
            "tier tokens must be substituted"
        );
        assert!(
            blob.contains("80.0, 140.0, 173.0, 200.0"),
            "the tier ladder must ride the pack (ClodTiers UBO)"
        );
        assert!(blob.contains("clod_h_b"), "stream targets must be present");
    }

    #[test]
    fn pack_bytes_are_deterministic() {
        let a = build(&Config::parse(""));
        let b = build(&Config::parse(""));
        assert_eq!(a.zip, b.zip, "pack bytes must be deterministic");
        assert_eq!(a.sha1_hex, b.sha1_hex);
    }

    #[test]
    fn debug_lens_pack_ships_lens_production_does_not() {
        // v6.7: the capture lens is CI-only. The production pack must stay
        // byte-identical to the pre-lens artifact (no debug_lens.fsh entry,
        // no lens pass, no ClodLens UBO); the debug pack carries all three
        // and substitutes the lens window tokens.
        let prod = build(&Config::parse(""));
        let prod_entries = zip_entries(&prod.zip);
        let names: Vec<String> = prod_entries.iter().map(|(n, _)| n.clone()).collect();
        assert_eq!(names.len(), 13, "production entry count fixed");
        assert!(!names.iter().any(|n| n.contains("debug_lens")));
        let chain = String::from_utf8_lossy(
            &prod_entries
                .iter()
                .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
                .unwrap()
                .1,
        );
        assert!(
            !chain.contains("debug_lens"),
            "lens pass in production chain"
        );
        assert!(!chain.contains("ClodLens"), "lens UBO in production chain");

        let dbg = build(&Config::parse(
            "debug_lens=true;debug_cell_x=100;debug_cell_z=200",
        ));
        let entries = zip_entries(&dbg.zip);
        assert_eq!(
            entries.len(),
            14,
            "debug pack = production + debug_lens.fsh"
        );
        assert!(entries
            .iter()
            .any(|(n, _)| n == "assets/clod/shaders/post/debug_lens.fsh"));
        let chain = String::from_utf8_lossy(
            &entries
                .iter()
                .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
                .unwrap()
                .1,
        );
        assert!(chain.contains("clod:post/debug_lens"), "lens pass missing");
        assert!(chain.contains("ClodLens"), "lens UBO missing");
        // Window: center cell (100,200) -> min corner (68,168), win 64.
        assert!(
            chain.contains("[68, 168, 64, 5.0]"),
            "lens window tokens: {chain}"
        );
        // Determinism holds for the debug pack too.
        let dbg2 = build(&Config::parse(
            "debug_lens=true;debug_cell_x=100;debug_cell_z=200",
        ));
        assert_eq!(dbg.zip, dbg2.zip);
        // And its SHA differs from production (one-time re-push on arm).
        assert_ne!(dbg.sha1_hex, prod.sha1_hex);
    }

    #[test]
    fn unfog_pack_ships_pass_production_does_not() {
        // v6.18: the un-fog pass is a parity-session knob. The production
        // pack must stay byte-identical to the pre-unfog artifact (no
        // unfog.fsh entry, no clod_unfog target, march In = minecraft:main);
        // the unfog pack carries all three and retargets the march's In.
        let prod = build(&Config::parse(""));
        let prod_entries = zip_entries(&prod.zip);
        let names: Vec<String> = prod_entries.iter().map(|(n, _)| n.clone()).collect();
        assert!(!names.iter().any(|n| n.contains("unfog")));
        let chain = String::from_utf8_lossy(
            &prod_entries
                .iter()
                .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
                .unwrap()
                .1,
        );
        assert!(
            !chain.contains("clod:post/unfog"),
            "unfog pass in production chain"
        );
        assert!(!chain.contains("clod_unfog"), "unfog target in production");
        assert!(!chain.contains("ClodUnfog"), "unfog UBO in production");
        assert!(
            !chain.contains("__UNFOG"),
            "unfog tokens must never leak into a shipped chain"
        );
        // v6.18.2: the fallback fog pair = the client.jar overworld defaults.
        assert!(
            chain.contains("[0.0, 1024,"),
            "the ClodFog fallback must be the real overworld pair (0, 1024)"
        );

        let unf = build(&Config::parse("unfog=true"));
        let entries = zip_entries(&unf.zip);
        assert!(
            entries
                .iter()
                .any(|(n, _)| n == "assets/clod/shaders/post/unfog.fsh"),
            "unfog pack must ship the shader"
        );
        let chain = String::from_utf8_lossy(
            &entries
                .iter()
                .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
                .unwrap()
                .1,
        );
        assert!(chain.contains("clod:post/unfog"), "unfog pass missing");
        assert!(chain.contains("ClodUnfog"), "unfog UBO missing");
        assert!(
            chain.contains("\"clod_unfog\""),
            "clod_unfog target missing"
        );
        // The march reads the UN-FOGGED frame, not minecraft:main — but ONLY
        // the march pass; the captures must keep reading the raw frame.
        let march = chain.find("clod:post/lod_terrain").unwrap();
        let in_after = &chain[march..march + 700];
        assert!(
            in_after.contains("\"target\": \"clod_unfog\""),
            "march In must read the un-fogged frame"
        );
        // The unfog pass sits between the eraser and the march.
        let mask = chain.find("clod:post/dust_mask").unwrap();
        let unfog = chain.find("clod:post/unfog").unwrap();
        assert!(
            mask < unfog && unfog < march,
            "chain order mask -> unfog -> march"
        );
        // Depth near rides the pass UBO.
        assert!(chain.contains("UnfogInfo"), "UnfogInfo uniform missing");
        assert!(
            !chain.contains("__DEPTH_NEAR__"),
            "depth-near token must be substituted"
        );
        // Determinism holds; and the combo (debug_lens + unfog) ships BOTH —
        // the v6.18.2 §5b evidence lane (lens panels + the un-fog pass).
        let unf2 = build(&Config::parse("unfog=true"));
        assert_eq!(unf.zip, unf2.zip);
        assert_ne!(unf.sha1_hex, prod.sha1_hex);
        let both = build(&Config::parse("debug_lens=true;unfog=true"));
        let both_entries = zip_entries(&both.zip);
        assert!(
            both_entries
                .iter()
                .any(|(n, _)| n == "assets/clod/shaders/post/unfog.fsh"),
            "the combo ships the un-fog shader (the evidence lane)"
        );
        assert!(
            both_entries
                .iter()
                .any(|(n, _)| n == "assets/clod/shaders/post/debug_lens.fsh"),
            "the combo keeps the lens"
        );
        let both_chain = String::from_utf8_lossy(
            &both_entries
                .iter()
                .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
                .unwrap()
                .1,
        );
        assert!(
            both_chain.contains("clod:post/unfog") && both_chain.contains("clod:post/debug_lens"),
            "the combo chain carries both passes"
        );
    }

    #[test]
    fn mesh_paint_flag_rides_the_lod_terrain_ubo() {
        // v6.19 G8d: production ships PaintParams.y = 0.0 (the vanilla path
        // stays pass-through); mesh_paint=true flips the SAME chain variant
        // to 1.0 — the parity-world knob, one re-push on the runner.
        let extract_paint = |zip: &Vec<u8>| -> String {
            let entries = zip_entries(zip);
            let chain = String::from_utf8_lossy(
                &entries
                    .iter()
                    .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
                    .unwrap()
                    .1,
            )
            .to_string();
            assert!(chain.contains("ClodPaint"), "PaintParams block missing");
            assert!(!chain.contains("__MESH_PAINT__"), "token unsubstituted");
            chain
        };
        let prod = build(&Config::parse(""));
        let prod_chain = extract_paint(&prod.zip);
        assert!(
            prod_chain.contains("\"value\": [0.05, 0.0, 0.0, 0.0]"),
            "production PaintParams must be depth-near + disabled: {prod_chain}"
        );
        let painted = build(&Config::parse("mesh_paint=true"));
        let painted_chain = extract_paint(&painted.zip);
        assert!(
            painted_chain.contains("\"value\": [0.05, 1, 0.0, 0.0]"),
            "mesh_paint=true must arm PaintParams.y: {painted_chain}"
        );
        assert_ne!(painted.sha1_hex, prod.sha1_hex, "pack SHA must change");
        // Determinism on the new knob.
        let painted2 = build(&Config::parse("mesh_paint=true"));
        assert_eq!(painted.zip, painted2.zip);
    }

    #[test]
    fn mesh_debug_defines_ship_in_the_march_shader() {
        // v6.19.2: the probe is a compile-time #define in the shipped fsh —
        // the default build keeps the marker line a comment.
        let raw = build(&Config::parse(""));
        let raw_fsh = String::from_utf8_lossy(
            &zip_entries(&raw.zip)
                .iter()
                .find(|(n, _)| n == "assets/clod/shaders/post/lod_terrain.fsh")
                .unwrap()
                .1,
        )
        .to_string();
        assert!(
            !raw_fsh.contains("#define MESH_DEBUG"),
            "probe leaked to prod"
        );
        assert!(raw_fsh.contains("// __MESH_DEBUG__"), "marker must stay");
        let dbg = build(&Config::parse("mesh_debug=true;mesh_paint=true"));
        let dbg_fsh = String::from_utf8_lossy(
            &zip_entries(&dbg.zip)
                .iter()
                .find(|(n, _)| n == "assets/clod/shaders/post/lod_terrain.fsh")
                .unwrap()
                .1,
        )
        .to_string();
        assert!(dbg_fsh.contains("#define MESH_DEBUG 1"), "probe must ship");
        assert!(
            !dbg_fsh.contains("// __MESH_DEBUG__"),
            "marker must be replaced"
        );
        assert_ne!(dbg.sha1_hex, raw.sha1_hex);
    }

    #[test]
    fn quant_ssao_swaps_the_probe_floor() {
        // v6.26.3 candidate (b): quant_ssao=true floor()-quantizes the
        // march_ssao occluder probe (block-stepped field like the voxy depth
        // buffer). Default build keeps the probe line byte-identical to the
        // template — the marker stays a comment, no floor() appears.
        let fsh_of = |cfg: &Config| {
            let p = build(cfg);
            let fsh = String::from_utf8_lossy(
                &zip_entries(&p.zip)
                    .iter()
                    .find(|(n, _)| n == "assets/clod/shaders/post/lod_terrain.fsh")
                    .unwrap()
                    .1,
            )
            .to_string();
            (fsh, p.sha1_hex)
        };
        let (raw_fsh, raw_sha) = fsh_of(&Config::parse(""));
        assert!(
            !raw_fsh.contains("floor(ground_at(q"),
            "quant probe leaked to prod"
        );
        assert!(
            raw_fsh.contains("// __QUANT_SSAO__"),
            "marker must stay in the default build"
        );
        let (q_fsh, q_sha) = fsh_of(&Config::parse("quant_ssao=true"));
        assert!(
            q_fsh.contains("float hq = floor(ground_at(q, 0.0, 0.0, sw, sok));"),
            "quant probe must ship"
        );
        assert_ne!(q_sha, raw_sha, "pack SHA must change");
        // Determinism on the new knob.
        let (_, q_sha2) = fsh_of(&Config::parse("quant_ssao=true"));
        assert_eq!(q_sha, q_sha2);
        // The knob composes with the parity-world lanes (unfog + mesh_paint).
        let (q2_fsh, _) = fsh_of(&Config::parse("unfog=true;mesh_paint=true;quant_ssao=true"));
        assert!(q2_fsh.contains("float hq = floor(ground_at(q, 0.0, 0.0, sw, sok));"));
    }

    #[test]
    fn fog_voxy_swaps_the_fog_cap() {
        // v6.26.7: fog_voxy=true ports the voxy blit fog — the env-pair lerp
        // with the endDistance cap (max(vanRD,320)*sqrt(3); NormalRenderPipeline
        // FOG_AND_FADE default, blit HAS_FOG block). Default build keeps the
        // fog line byte-identical to the template (marker stays a comment).
        let fsh_of = |cfg: &Config| {
            let p = build(cfg);
            let fsh = String::from_utf8_lossy(
                &zip_entries(&p.zip)
                    .iter()
                    .find(|(n, _)| n == "assets/clod/shaders/post/lod_terrain.fsh")
                    .unwrap()
                    .1,
            )
            .to_string();
            (fsh, p.sha1_hex)
        };
        let (raw_fsh, raw_sha) = fsh_of(&Config::parse(""));
        assert!(!raw_fsh.contains("fogCap"), "voxy fog leaked to prod");
        assert!(
            raw_fsh.contains("// __FOG_VOXY__"),
            "marker must stay in the default build"
        );
        let (fv_fsh, fv_sha) = fsh_of(&Config::parse("fog_voxy=true"));
        assert!(
            fv_fsh.contains(
                "float fogCap = clamp(endDistance * invEndFogDelta + startDelta, 0.0, 1.0);"
            ),
            "voxy fog cap must ship"
        );
        assert!(
            fv_fsh.contains(
                "float fog = fogLive ? clamp(tHit * invEndFogDelta + startDelta, 0.0, fogCap) * fogOpacity : 0.0;"
            ),
            "the voxy lerp must replace the linear pair"
        );
        assert_ne!(fv_sha, raw_sha, "pack SHA must change");
        // Determinism on the new knob.
        let (_, fv_sha2) = fsh_of(&Config::parse("fog_voxy=true"));
        assert_eq!(fv_sha, fv_sha2);
        // The knob composes with the parity-world lanes.
        let (fv2_fsh, _) = fsh_of(&Config::parse("unfog=true;mesh_paint=true;fog_voxy=true"));
        assert!(fv2_fsh.contains("float fogCap"));
        // And with the other shader knob: both patches land in one pack.
        let (both_fsh, _) = fsh_of(&Config::parse("fog_voxy=true;quant_ssao=true"));
        assert!(both_fsh.contains("float fogCap"));
        assert!(both_fsh.contains("float hq = floor(ground_at(q"));
    }


    #[test]
    fn rim_mask_swaps_the_decode_state_probe() {
        // v6.30.0 (#360 pre-registered item (2)): rim_mask=true ships the
        // compile-time decode-state paint — every march-owned pixel becomes
        // GREEN (height+color decoded) / RED (colorless: the color word is
        // absent) / BLUE (height absent at a march hit). One mask frame from
        // a rim vantage (07/12) names the rim fallback class exactly. The
        // default build keeps the marker a comment and the pack byte-stable.
        let fsh_of = |cfg: &Config| {
            let p = build(cfg);
            let fsh = String::from_utf8_lossy(
                &zip_entries(&p.zip)
                    .iter()
                    .find(|(n, _)| n == "assets/clod/shaders/post/lod_terrain.fsh")
                    .unwrap()
                    .1,
            )
            .to_string();
            (fsh, p.sha1_hex)
        };
        let (raw_fsh, raw_sha) = fsh_of(&Config::parse(""));
        assert!(
            !raw_fsh.contains("#define RIM_MASK 1"),
            "rim probe leaked to prod"
        );
        assert!(
            raw_fsh.contains("// __RIM_MASK__"),
            "marker must stay in the default build"
        );
        let (rm_fsh, rm_sha) = fsh_of(&Config::parse("rim_mask=true"));
        assert!(
            rm_fsh.contains("#define RIM_MASK 1"),
            "the rim probe define must ship"
        );
        assert!(
            rm_fsh.contains("v6.30.0 RIM MASK PROBE"),
            "the probe block must ride the same swap"
        );
        assert_ne!(rm_sha, raw_sha, "pack SHA must change");
        // Determinism on the new knob.
        let (_, rm_sha2) = fsh_of(&Config::parse("rim_mask=true"));
        assert_eq!(rm_sha, rm_sha2);
        // The probe composes with the parity-world lanes and the other knobs.
        let (rm2_fsh, _) = fsh_of(&Config::parse(
            "unfog=true;mesh_paint=true;sparse_rebeat=true;rim_mask=true",
        ));
        assert!(rm2_fsh.contains("#define RIM_MASK 1"));
        // Three-knob composition with the existing shader knobs.
        let (all_fsh, _) = fsh_of(&Config::parse(
            "rim_mask=true;fog_voxy=true;quant_ssao=true",
        ));
        assert!(all_fsh.contains("#define RIM_MASK 1"));
        assert!(all_fsh.contains("float fogCap"));
        assert!(all_fsh.contains("float hq = floor(ground_at(q"));
    }

    #[test]
    fn unfog_debug_swaps_the_mask_shader() {
        // v6.18.2: unfog_debug=true points the chain's un-fog pass at the
        // engagement-mask shader and ships it under its own name.
        let prod = build(&Config::parse("unfog=true"));
        let dbg = build(&Config::parse("unfog=true;unfog_debug=true"));
        let entries = zip_entries(&dbg.zip);
        assert!(
            entries
                .iter()
                .any(|(n, _)| n == "assets/clod/shaders/post/unfog_debug.fsh"),
            "the mask shader must ship"
        );
        assert!(
            !entries
                .iter()
                .any(|(n, _)| n == "assets/clod/shaders/post/unfog.fsh"),
            "the render shader is replaced, not accompanied"
        );
        let chain = String::from_utf8_lossy(
            &entries
                .iter()
                .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
                .unwrap()
                .1,
        );
        assert!(
            chain.contains("clod:post/unfog_debug"),
            "the pass must reference the mask shader"
        );
        assert!(!chain.contains("clod:post/unfog\""), "no bare unfog ref");
        assert_ne!(dbg.sha1_hex, prod.sha1_hex);
        // The mask without unfog stays inert (nothing ships, chain untouched).
        let solo = build(&Config::parse("unfog_debug=true"));
        assert_eq!(solo.zip, build(&Config::parse("")).zip);
    }

    #[test]
    fn chain_has_stream_passes_and_no_texture_inputs() {
        let cfg = Config::parse("");
        let pack = build(&cfg);
        let entries = zip_entries(&pack.zip);
        let chain = &entries
            .iter()
            .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
            .unwrap()
            .1;
        let chain = String::from_utf8_lossy(chain);
        assert!(chain.contains("clod:post/capture"), "capture pass missing");
        assert!(
            chain.contains("clod:post/lod_terrain"),
            "march pass missing"
        );
        assert!(
            !chain.contains("location"),
            "no texture-input locations may remain (stream-only)"
        );
        assert!(chain.contains("\"clod_h_b\""), "height target missing");
        assert!(chain.contains("\"clod_c_b\""), "color target missing");
        assert!(
            chain.contains("\"persistent\": true"),
            "persistence missing"
        );
        assert!(
            !chain.contains("ScanBox"),
            "the static scan box is gone (sky-grid)"
        );
        // v6.2: the last "atlas" is gone — the marcher's UBO is ClodRegion.
        assert!(
            !chain.contains("ClodAtlas") && !chain.contains("AtlasInfo"),
            "the atlas UBO must stay renamed (ClodRegion/RegionInfo)"
        );
        // v6.2 edit protocol: the reset pass flags edited cells for the
        // frame, so it must run BEFORE the four data captures that read it.
        let reset = chain.find("\"output\": \"clod_reset\"").unwrap();
        let first_capture_read = chain
            .find("\"target\": \"clod_reset\"")
            .expect("data captures must read clod_reset");
        assert!(
            reset < first_capture_read,
            "the reset pass must precede the data captures"
        );
        // v5.3.2 zero-residual: the mask must sit between the last capture
        // and the march, read BOTH capture rounds' fresh words plus the raw
        // frame (color net), and consume the same ClodGrid/ClodStream UBOs.
        let last_capture = chain.rfind("clod:post/capture").unwrap();
        let mask = chain.find("clod:post/dust_mask").unwrap();
        let march = chain.find("clod:post/lod_terrain").unwrap();
        assert!(
            last_capture < mask && mask < march,
            "chain order must be capture..capture -> dust_mask -> lod_terrain"
        );
        let mask_pass = &chain[mask..march];
        for token in [
            "\"target\": \"clod_h_b\"",
            "\"target\": \"clod_c_b\"",
            "\"target\": \"clod_h_a\"",
            "\"target\": \"clod_c_a\"",
            "\"target\": \"minecraft:main\"",
            "\"target\": \"clod_rot_a\"",
        ] {
            assert!(
                mask_pass.contains(token),
                "dust_mask inputs must carry both capture rounds + main + rot: {token}"
            );
        }
    }

    #[test]
    fn camera_override_ships() {
        let cfg = Config::parse("");
        let pack = build(&cfg);
        let entries = zip_entries(&pack.zip);
        let names: Vec<&str> = entries.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"assets/minecraft/shaders/core/sky.fsh"));
        assert!(!names.contains(&"assets/minecraft/shaders/core/text.fsh"));
        assert!(!names.contains(&"assets/clod/shaders/post/scrape.fsh"));
        assert!(names.contains(&"assets/clod/shaders/post/capture.fsh"));
    }

    #[test]
    fn capture_heightref_rides_the_lo_passes() {
        // v6.23 TIER-CONSISTENCY (#316 forensics): the color capture
        // cross-checks its stash rank against the height pair the earlier
        // captures of the SAME frame wrote (c_a reads h_a, c_b reads h_b —
        // chain order HI before LO). The shader declares HeightRefSampler
        // unconditionally, so every capture pass of every chain binds it
        // (the reset/HI passes never read it on their code paths). The
        // shader must carry the gate itself (surface_rank mirror of
        // stream.rs::tier_rank + the merge-honesty blocks).
        let cfg = Config::parse("");
        let pack = build(&cfg);
        let entries = zip_entries(&pack.zip);
        let shader = String::from_utf8_lossy(
            &entries
                .iter()
                .find(|(n, _)| n == "assets/clod/shaders/post/capture.fsh")
                .expect("capture.fsh missing")
                .1,
        );
        for marker in [
            "surface_rank",
            "v6.23 TIER-CONSISTENCY GATE",
            "v6.23 MERGE HONESTY (a)",
            "v6.23 MERGE HONESTY (b)",
            "HeightRefSampler",
        ] {
            assert!(
                shader.contains(marker),
                "capture.fsh must carry the v6.23 decode-honesty marker {marker}"
            );
        }
        // v6.24 BUFFER DUMP: the march's evidence lane carries the raw
        // buffer window under the same compile-time define.
        let march = String::from_utf8_lossy(
            &entries
                .iter()
                .find(|(n, _)| n == "assets/clod/shaders/post/lod_terrain.fsh")
                .expect("lod_terrain.fsh missing")
                .1,
        );
        for marker in [
            "v6.24 BUFFER DUMP",
            "HeightSampler, celli",
            "ColBufSampler, celli",
            // v6.25.4 FOG DISTANCE PARITY (#330 verdict): the env pair
            // (renderDistance-sized) fogged a 150 m forest to mix ~0.9 —
            // that was the ->light class; #330 (fog fully removed) proved
            // the halos are a DATA-COVERAGE hole (our disk ~256 m vs the
            // voxy leg's +/-48 chunks = 768 m: horizon-fogged vanilla sky
            // through the hole) AND that the far summits need the REAL
            // dimension fog back (00/02/04 mean d worsened without it).
            // The march now carries the dimension pair 0 -> 1024.
            "v6.25.4 FOG DISTANCE PARITY",
        ] {
            assert!(
                march.contains(marker),
                "lod_terrain.fsh must carry the v6.24 buffer-dump marker {marker}"
            );
        }
        // The pack ships ONE chain file per build; the four templates are
        // selected by config, so every variant is built and checked.
        for (cfg_s, label) in [
            ("", "plain"),
            ("debug_lens=true", "debug"),
            ("unfog=true", "unfog"),
            ("unfog=true;debug_lens=true", "debug_unfog"),
        ] {
            let cfg = Config::parse(cfg_s);
            let pack = build(&cfg);
            let entries = zip_entries(&pack.zip);
            let chain = String::from_utf8_lossy(
                &entries
                    .iter()
                    .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
                    .unwrap_or_else(|| panic!("{label}: chain missing from the pack"))
                    .1,
            )
            .to_string();
            let bound = chain.matches("\"sampler_name\": \"HeightRef\"").count();
            assert_eq!(
                bound, 5,
                "{label}: all five capture passes must bind HeightRef"
            );
            for (lo, hi) in [("clod_c_a", "clod_h_a"), ("clod_c_b", "clod_h_b")] {
                let pass = chain
                    .split("        {\n")
                    .find(|p| {
                        p.contains("clod:post/capture")
                            && p.contains(&format!("\"output\": \"{lo}\""))
                    })
                    .unwrap_or_else(|| panic!("{label}: no capture pass writes {lo}"));
                assert!(
                    pass.contains(&format!(
                        "\"sampler_name\": \"HeightRef\", \"target\": \"{hi}\""
                    )),
                    "{label}: the {lo} capture must cross-check {hi}"
                );
            }
        }
    }

    #[test]
    fn capture_mirror_arm_decodes_the_canopy_carrier() {
        // v6.25 MIRROR LUMINANCE-DELTA ARM (#318/#319 forensics): the §7b
        // canopy shades the rank-0/1 carriers into NEUTRAL DARK ((35,38,42),
        // 100% of the dot class inside |g-b| <= 0.08 && |g-r| <= 0.08, lum
        // 0.15 vs ring 0.41) — both hue arms are blind to it, the cells
        // never decode, and their dots are never erased: the eastern fake
        // wall on 04/06/02/10. The fix lives in BOTH consumers: the capture
        // strobe (decode -> the eraser's projection path owns the dots) and
        // the dust_mask color net (the strobe-miss safety net). The capture
        // arm carries the FLOATING-CARRIER DEPTH GATE — a dark px the ray
        // reaches before the plane is terrain shading and must never fire
        // (the words are persistent; the net's false positives are
        // self-repairing, the capture's are not).
        let cfg = Config::parse("");
        let pack = build(&cfg);
        let entries = zip_entries(&pack.zip);
        let capture = String::from_utf8_lossy(
            &entries
                .iter()
                .find(|(n, _)| n == "assets/clod/shaders/post/capture.fsh")
                .expect("capture.fsh missing")
                .1,
        );
        for marker in [
            "v6.25 MIRROR LUMINANCE-DELTA ARM",
            "v6.25 FLOATING-CARRIER DEPTH GATE",
            "(ringLum - lum) > 0.06",
            "abs(raw.g - raw.b) <= 0.08",
            "abs(raw.g - raw.r) <= 0.08",
            "zScene >= z",
            "texture(InDepthSampler, sc).r",
        ] {
            assert!(
                capture.contains(marker),
                "capture.fsh must carry the v6.25 mirror-arm marker {marker}"
            );
        }
        // The mode-2 reset pass does not carry the depth near: its probe
        // must pass depthNear = 0 (the arm disabled), the data passes pass
        // StreamInfo.z (the __DEPTH_NEAR__ scalar the chain template fills).
        assert!(
            capture.contains("probe_word(cellWorld, tierY(TiersRs, r), sub, 0.0, w, hit, all)"),
            "the reset walk must disable the mirror arm (no depth near in its UBO)"
        );
        assert!(
            capture.contains(
                "probe_word(cellWorld, tierY(ladders, r), sub, StreamInfo.z, w, hit, all)"
            ),
            "the data tier walk must pass the pass's depth near"
        );
        let mask = String::from_utf8_lossy(
            &entries
                .iter()
                .find(|(n, _)| n == "assets/clod/shaders/post/dust_mask.fsh")
                .expect("dust_mask.fsh missing")
                .1,
        );
        for marker in [
            "(ringLum - lum) > 0.06",
            "abs(raw.g - raw.b) <= 0.08",
            "abs(raw.g - raw.r) <= 0.08",
        ] {
            assert!(
                mask.contains(marker),
                "dust_mask.fsh must carry the v6.25 mirror-net marker {marker}"
            );
        }
        // v6.25.2 REVERT (#323 read): the shape gate FAILED its goal and was
        // REGRESSIONAL — the ->light class lived (01 59.3 / 02 49.3 / 03 55.6
        // / 09 53.4 / 10 71.2 / 11 27.7, flat vs #321) while the gate ate the
        // #321 wins (06 58.3 -> 35.8, 09 59.6 -> 53.4, 08 64.9 -> 61.6): it
        // stops the net on dots over DARK terrain (06/09 are ground vantages
        // — the #321 ->sky win there was the net's own erasing) yet the
        // silhouette halos passed its min-tap test anyway. The net returns to
        // the v6.25.0 mirror semantics (the markers above); the ->light class
        // is handed to the ringLumMin tap forensics on the #323 11_topdown
        // artifact instead of a constant. The capture arm keeps its depth
        // gate (no shape gate there — the tower writes depth closer).
        assert!(
            !mask.contains("ringLumMin"),
            "dust_mask.fsh must NOT carry the reverted v6.25.1 shape gate"
        );
        // The chain templates already bind the depth texture into every
        // capture pass (kept since v5.3 "for the chain schema") — the gate
        // rides the existing binding, no chain change. The count: 5 capture
        // passes + the march's own tVan read (6), +1 more when the unfog
        // variant's extra pass rides the chain (7).
        for cfg_s in [
            "",
            "debug_lens=true",
            "unfog=true",
            "unfog=true;debug_lens=true",
        ] {
            let cfg = Config::parse(cfg_s);
            let pack = build(&cfg);
            let entries = zip_entries(&pack.zip);
            let chain = String::from_utf8_lossy(
                &entries
                    .iter()
                    .find(|(n, _)| n == "assets/clod/post_effect/lod_terrain.json")
                    .expect("chain missing from the pack")
                    .1,
            );
            let expected = if cfg.unfog { 7 } else { 6 };
            assert_eq!(
                chain.matches("\"sampler_name\": \"InDepth\"").count(),
                expected,
                "the capture passes + the march (tVan) must bind the depth texture \
                 (the v6.25 gate reads it)"
            );
        }
    }
}
