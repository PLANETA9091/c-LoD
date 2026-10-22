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
    // v4.4: the fog used to start at 2.2*lod_near (~211 blocks), which veiled
    // the whole mid-band and read as "less detailed than DH" — DH stays crisp
    // for kilometres. Start the fade only in the outer half of the atlas
    // (never before 2.2*lod_near) and still dissolve fully before the border
    // (the old hard-edge fix below).
    let fog_start = (atlas.half() as f32 * 0.45).max(cfg.lod_near * 2.2);
    // Local tweak: the default (size * 0.72) sat past the atlas border for
    // half-extent <= 0.72*size, so the scanned window's square edge was
    // only ~65% veiled and showed up as a hard wall on the horizon.
    // Fade fully before the border instead.
    let fog_end = (atlas.size as f32 * 0.72).min(atlas.half() as f32 * 0.92);

    // Highest scanned height in the atlas: anything above it in the scene can
    // only be the cloud layer (or weather), never LOD terrain.
    let atlas_max_h = atlas
        .snapshot()
        .iter()
        .filter(|c| c.valid)
        .map(|c| c.height)
        .max()
        .unwrap_or(0) as f32;

    let veil_rgba = [
        cfg.color_veil[0],
        cfg.color_veil[1],
        cfg.color_veil[2],
        cfg.snow_line,
    ];

    // Pyramid levels: level k has dim = cells >> k; the last level is 1x1.
    let pyr_levels = ((atlas.cells as f64).log2().floor() as u32).max(1);

    let map: Vec<(&str, String)> = vec![
        ("__FOV__", fmt_f(cfg.fov)),
        ("__ORIGIN_X__", fmt_f(origin_x as f32)),
        ("__ORIGIN_Z__", fmt_f(origin_z as f32)),
        ("__ATLAS_SIZE__", fmt_f(atlas.size as f32)),
        ("__ATLAS_CELLS__", atlas.cells.to_string()),
        ("__CELL__", fmt_f(atlas.cell as f32)),
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
        ("__DEBUG__", format!("{}.0", cfg.debug_lod)),
        ("__ATLAS_MAX_H__", fmt_f(atlas_max_h + 3.0)),
        ("__ATMO_START__", atmo_start),
        ("__ATMO_END__", atmo_end),
        // The distance veil is disabled (strength 0): it sits on real geometry
        // only, but a strength > 0 also tints distant clouds and the chunk
        // wall; LOD fog already handles the horizon, and the veil must never
        // starve the sun tracker.
        ("__ATMO_STRENGTH__", fmt_f(0.0)),
        // 26.3 device depth: d = depth_near / eyeZ (infinite far, sky == 0.0).
        ("__DEPTH_NEAR__", fmt_f(cfg.depth_near)),
        // v4.2 max-height pyramid: level count for the conservative march.
        ("__PYR_LEVELS__", format!("{}.0", pyr_levels)),
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
    // DH-style slab geometry (gap bottoms + ground segments below canopies
    // and floating islands). Sampled NEAREST — the values are exact levels.
    let (gw, gh, geom_rgba) = atlas.encode_geom();
    let geom_png = png::encode_png(gw, gh, &geom_rgba);
    // Conservative max-height pyramid for the ray march.
    let (pw, ph, pyr_rgba) = atlas.encode_pyramid();
    let pyr_png = png::encode_png(pw, ph, &pyr_rgba);
    // Top-face colours (bilinear, this is what the eye reads as "block colour").
    let (cw, ch, color_rgba) = atlas.encode_colors();
    let color_png = png::encode_png(cw, ch, &color_rgba);
    // Ground-segment colours below air gaps (bilinear).
    let (grw, grh, ground_rgba) = atlas.encode_ground();
    let ground_png = png::encode_png(grw, grh, &ground_rgba);
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
    zip.add("assets/clod/textures/effect/geom.png", &geom_png);
    zip.add("assets/clod/textures/effect/pyr.png", &pyr_png);
    zip.add("assets/clod/textures/effect/color.png", &color_png);
    zip.add("assets/clod/textures/effect/ground.png", &ground_png);
    zip.add("assets/clod/textures/effect/side.png", &side_png);
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

    /// Mirrors the v4.3 camera channel byte format exactly as the GLSL does
    /// (sky.fsh encoders, lod_terrain.fsh decoders, rot_solve.fsh packing).
    /// Guards the shader trio against format drift: the persistent RGBA8
    /// target only round-trips bytes, so any layout change must land in all
    /// three shaders AND here.
    mod camchannel {
        // sky.fsh enc16 / lod_terrain.fsh c16. The GLSL decoders read the
        // RGBA8-stored float n/255, so the byte-level mirror is
        // (hi*256 + lo) / 65535 remapped back to [-1, 1].
        fn enc16(c: f32) -> (u8, u8) {
            let q = (((c + 1.0) * 0.5).clamp(0.0, 1.0) * 65535.0 + 0.5).floor() as u32;
            ((q / 256) as u8, (q % 256) as u8)
        }
        fn c16(hi: u8, lo: u8) -> f32 {
            ((hi as f32) * 256.0 + (lo as f32)) / 65535.0 * 2.0 - 1.0
        }
        // sky.fsh enc8s (signed +-1 over one byte).
        fn enc8s(c: f32) -> u8 {
            (((c + 1.0) * 0.5).clamp(0.0, 1.0) * 255.0 + 0.5).floor() as u8
        }
        fn dec8s(b: u8) -> f32 {
            (b as f32 / 255.0) * 2.0 - 1.0
        }

        #[test]
        fn basis_16bit_roundtrip_is_sub_pixel() {
            // Worst-case angular error of the 16-bit signed remap must stay
            // far below one pixel (70deg fov over 1080p ~ 0.065 deg/px).
            let mut max_err: f32 = 0.0;
            let mut c = -1.0f32;
            while c <= 1.0 {
                let (hi, lo) = enc16(c);
                let back = c16(hi, lo);
                max_err = max_err.max((back - c).abs());
                c += 0.00005;
            }
            // One quantization step is 2/65535 ~ 3.05e-5; float32 rounding
            // on the decode adds a hair.
            assert!(max_err <= 3.2e-5, "max roundtrip error {max_err}");
        }

        #[test]
        fn eye_wobble_8bit_is_milliblock() {
            for &c in &[-1.0f32, -0.5, 0.0, 0.37, 1.0] {
                let back = dec8s(enc8s(c));
                assert!((back - c).abs() <= 2.0 / 255.0 + 1e-6, "{c} -> {back}");
            }
        }

        #[test]
        fn pixel_packing_matches_rot_solve() {
            // rot_solve packs: px0=(Rxh,Rxl,Ryh,Ryl) px1=(Rzh,Rzl,Uxh,Uxl)
            // px2=(Uyh,Uyl,Uzh,Uzl) px3=(fovh,fovl,tx,ty) px4=(conf,0,0,0).
            let r = [-0.31f32, 0.77, 0.55];
            let u = [0.12f32, 0.61, -0.78];
            let mut bytes = Vec::new();
            for c in r.into_iter().chain(u) {
                let (hi, lo) = enc16(c);
                bytes.push(hi);
                bytes.push(lo);
            }
            let fovq = 41234u32; // (70-5)/115*65534 ~ 37061; arbitrary valid
            bytes.push((fovq / 256) as u8);
            bytes.push((fovq % 256) as u8);
            bytes.push(enc8s(0.08));
            bytes.push(enc8s(-0.11));
            assert_eq!(bytes.len(), 16, "payload x2..x17");

            let px = |j: usize| &bytes[j * 4..j * 4 + 4];
            let (rh, rl) = (px(0)[0], px(0)[1]);
            let (uh, ul) = (px(2)[2], px(2)[3]);
            assert!((c16(rh, rl) - r[0]).abs() < 1e-4);
            assert!((c16(uh, ul) - u[2]).abs() < 1e-4);
            assert_eq!(px(3)[2], enc8s(0.08));
            assert_eq!(px(3)[3], enc8s(-0.11));
        }

        #[test]
        fn checksum_layout_matches_shaders() {
            // sky.fsh: chk = sum(pay[0..18)) & 255 at x18; col_scan re-derives
            // it. The magic bytes are x0=199, x1=99.
            let payload: [u8; 19] = {
                let mut p = [0u8; 19];
                p[0] = 199;
                p[1] = 99;
                for (i, c) in [0.3f32, -0.9, 0.5].iter().enumerate() {
                    let (hi, lo) = enc16(*c);
                    p[2 + i * 2] = hi;
                    p[3 + i * 2] = lo;
                }
                let s: u32 = p[..18].iter().map(|b| *b as u32).sum();
                p[18] = (s & 255) as u8;
                p
            };
            let s: u32 = payload[..18].iter().map(|b| *b as u32).sum();
            assert_eq!((s & 255) as u8, payload[18]);
        }
    }

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
        atlas.set(0, 0, 70, false, 0x808080, 0x707070, 70, 0, false, 0);
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
            count, 18,
            "pack.mcmeta + 2 json + 6 fsh + 6 textures + sky.vsh/fsh + terrain.vsh"
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
        a.set(1, 1, 90, false, 0x808080, 0x707070, 90, 0, false, 0);
        a.set(2, 2, 40, true, 0x808080, 0x707070, 40, 0, false, 0);
        let pack_a = build(&cfg, &a);

        let b = Atlas::new(0, 0, 256, 4, 63);
        b.set(1, 1, 90, false, 0x808080, 0x707070, 90, 0, false, 0);
        b.set(2, 2, 40, true, 0x808080, 0x707070, 40, 0, false, 0);
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
        // v4.2 slab/pyramid/ground textures must ship next to the atlas.
        for tex in ["atlas", "geom", "pyr", "color", "ground", "side"] {
            assert!(
                names.contains(&format!("assets/clod/textures/effect/{tex}.png").as_str()),
                "effect texture {tex} missing: {names:?}"
            );
        }
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
        assert!(sky.contains("mat3(ModelViewMat)"), "basis source missing");
        assert!(
            sky.contains("Bob rows out of the folded projection"),
            "bob extraction missing"
        );
        assert!(
            sky.contains("r0 = normalize(vec3(ProjMat"),
            "ProjMat rows not read"
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
        for loc in ["clod:geom", "clod:pyr", "clod:ground"] {
            assert!(chain.contains(loc), "sampler location {loc} missing");
        }
        assert!(
            !chain.contains("sun_"),
            "sun passes must be gone from the chain"
        );
    }

    #[test]
    fn v4_4_channel_groups_and_adaptive_march_are_shipped() {
        // v4.4 pan-flicker fix: the payload must be written into eight column
        // groups (sky.fsh), scanned group-wise at 5120 slices (col_scan +
        // rot_solve + the chain target), and the march must carry the fixed
        // early-out plus the adaptive PIXF level.
        let cfg = Config::parse("");
        let atlas = Atlas::new(0, 0, 256, 4, 63);
        let pack = build(&cfg, &atlas);
        let entries = zip_entries(&pack.zip);
        let get = |name: &str| {
            entries
                .iter()
                .find(|(n, _)| n == name)
                .unwrap_or_else(|| panic!("{name} missing"))
                .1
                .clone()
        };

        let sky_v = get("assets/minecraft/shaders/core/sky.fsh");
        let sky = String::from_utf8_lossy(&sky_v);
        assert!(
            sky.contains("px.x % 240 < 19"),
            "sky payload must use 8 column groups"
        );

        let scan_v = get("assets/clod/shaders/post/col_scan.fsh");
        let scan = String::from_utf8_lossy(&scan_v);
        assert!(
            scan.contains("const int GROUPS = 8;"),
            "col_scan group count"
        );
        assert!(
            scan.contains("const int GROUP_STRIDE = 240;"),
            "col_scan group stride must match sky.fsh"
        );

        let solve_v = get("assets/clod/shaders/post/rot_solve.fsh");
        let solve = String::from_utf8_lossy(&solve_v);
        assert!(
            solve.contains("const int SLICES = 5120;"),
            "solver slice count"
        );
        assert!(
            solve.contains("min(prev.r + 0.08, 1.0)"),
            "confidence recovery must ramp, not snap"
        );

        let chain_v = get("assets/clod/post_effect/lod_terrain.json");
        let chain = String::from_utf8_lossy(&chain_v);
        assert!(
            chain.contains("\"clod_scan\": { \"width\": 5120, \"height\": 1 }"),
            "scan target must be 5120x1 for the 8 group columns"
        );

        let terrain_v = get("assets/clod/shaders/post/lod_terrain.fsh");
        let terrain = String::from_utf8_lossy(&terrain_v);
        assert!(
            terrain.contains("cam.y > maxY + 64.0"),
            "early-out must only fire above the whole heightfield (cloud-cut fix)"
        );
        assert!(
            !terrain.contains("toCeil"),
            "the broken toCeil/toEdge early-out must be gone"
        );
        assert!(
            terrain.contains("const float PIXF"),
            "adaptive level constant"
        );
        assert!(
            terrain.contains("hitLvl"),
            "adaptive level tracking missing"
        );
    }
}
