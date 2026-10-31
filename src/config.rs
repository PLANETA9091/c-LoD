//! c-LoD configuration: parsed from the CRUSSTY module options string.
//!
//! Options format: `key1=value1;key2=value2;...` (whatever the runtime hands
//! to `cplugin_init`). Every knob has a working default, so an empty options
//! string produces a functional module.
//!
//! v5.0: the atlas/window/revision knobs are GONE — the pack is a static
//! shader-only artifact and the world data streams as dust particles over
//! the sky-grid planes. What remains is the streaming geometry + pacing.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    /// Port of the embedded HTTP server that serves the (static, shader-only)
    /// resource pack. The pack URL becomes `pack_base_url + "/clod-pack.zip"`.
    pub http_port: u16,
    /// Base URL clients can reach this server on, e.g. `http://play.example.com`.
    pub pack_base_url: String,
    /// Resource pack UUID pushed to clients. Stable across reboots so vanilla
    /// clients only ever download once (the SHA-1 is content-fixed).
    pub pack_uuid: String,
    /// Prompt shown by the client for the resource pack.
    pub pack_prompt: String,
    /// World streamed to the LOD (empty = first Bukkit world).
    pub world: String,
    /// Region side length in blocks (power of two, 256..=2048). One region
    /// = one streamed disk of cells.
    pub region_blocks: i32,
    /// One LOD cell covers `cell` x `cell` blocks (1, 2, 4, 8...).
    pub cell_size: i32,
    /// Heightmap cells scanned per engine tick (main-thread JNI budget).
    pub scan_budget: u32,
    /// v5.5 frontier: NEW chunk-gen requests admitted per engine tick. The
    /// old fixed 4 starved the chunk system's worker pool whenever a full
    /// ring's worth of chunks wanted generating at once.
    pub gen_budget: u32,
    /// v5.5 frontier: cap on chunks with an in-flight FORCED ticket. The
    /// old fixed 96 could leave generation workers idle between ring arcs;
    /// a deep window keeps every worker fed while the ledger drips through.
    pub gen_wish: u32,
    /// v5 sky-grid pacing: dust particles spawned per engine tick (each
    /// cell costs 4 height + 4 color particles).
    pub particles_per_tick: u32,
    /// Fixed disk origin (world x/z of the grid's min corner). Empty = the
    /// first player's position at arm time. The disk is the same for every
    /// client; particles fly only over this square.
    pub center_x: Option<f64>,
    pub center_z: Option<f64>,
    /// Camera FOV (vertical, degrees) the GLSL ray reconstruction assumes.
    pub fov: f32,
    /// Distance (blocks) where the LOD raymarch starts. Keep it BELOW the
    /// client view distance: the chain only paints sky-depth pixels, so real
    /// chunks always win by depth and the LOD fills underneath the chunk edge.
    pub lod_near: f32,
    /// Effective maximum distance of the atmosphere veil (blocks).
    pub veil_far: f32,
    /// 26.3 device-depth near plane (blocks): device d = depth_near / eyeZ.
    pub depth_near: f32,
    /// Sea level used for water flagging and LOD water rendering.
    pub sea_level: i32,
    /// Engine tick interval in milliseconds.
    pub tick_ms: u64,
    /// Post-effect ids (namespace) installed into the generated pack.
    pub effect_terrain: String,
    pub effect_atmosphere: String,
    /// Palette: LOD terrain colors (linear RGB 0..1).
    pub color_low: [f32; 3],
    pub color_high: [f32; 3],
    pub color_water: [f32; 3],
    pub snow_line: f32,
    /// Sky/veil color blended over the render-distance edge.
    pub color_veil: [f32; 3],
    /// Upper bound on terrain height used to cull rays early.
    pub max_height: i32,
    /// SEEDGEN v5.7: generate chunks the server never made, on OUR worker
    /// threads (vanilla noise+surface per column — no tickets, no TPS cost).
    /// When false (or when the NMS surface fails to resolve) the pre-5.7
    /// FORCED-ticket frontier handles unloaded chunks.
    pub native_gen: bool,
    /// SEEDGEN worker thread count. 0 = auto (cores-1, clamped 1..8).
    pub native_threads: i32,
    /// SEEDGEN v5.9: chunks per JNI batch call (one helper->Rust crossing
    /// amortized over N chunks; the noise walk still runs per chunk inside
    /// the JVM). 1 = one crossing per chunk (the v5.8 shape).
    pub seedgen_batch: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http_port: 8199,
            pack_base_url: String::new(),
            pack_uuid: "6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100".to_string(),
            pack_prompt: String::new(),
            world: String::new(),
            region_blocks: 512,
            cell_size: 2,
            scan_budget: 4096,
            gen_budget: 16,
            gen_wish: 512,
            particles_per_tick: 2048,
            center_x: None,
            center_z: None,
            fov: 70.0,
            lod_near: 96.0,
            veil_far: 512.0,
            depth_near: 0.05,
            sea_level: 63,
            tick_ms: 1000,
            effect_terrain: "clod:lod_terrain".to_string(),
            effect_atmosphere: "clod:atmosphere".to_string(),
            color_low: [0.35, 0.52, 0.28],
            color_high: [0.48, 0.44, 0.40],
            color_water: [0.12, 0.24, 0.45],
            snow_line: 0.78,
            color_veil: [0.62, 0.74, 0.86],
            max_height: 320,
            native_gen: true,
            native_threads: 0,
            seedgen_batch: 8,
        }
    }
}

impl Config {
    pub fn parse(options: &str) -> Config {
        let mut cfg = Config::default();
        let mut kv: HashMap<&str, &str> = HashMap::new();
        for part in options.split([';', ',', '\n']) {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some((k, v)) = part.split_once('=') {
                kv.insert(k.trim(), v.trim());
            }
        }
        let get = |k: &str| kv.get(k).copied();
        if let Some(v) = get("http_port").and_then(|v| v.parse().ok()) {
            cfg.http_port = v;
        }
        if let Some(v) = get("pack_base_url") {
            cfg.pack_base_url = v.trim_end_matches('/').to_string();
        }
        if let Some(v) = get("pack_uuid") {
            cfg.pack_uuid = v.to_string();
        }
        if let Some(v) = get("pack_prompt") {
            cfg.pack_prompt = v.to_string();
        }
        if let Some(v) = get("world") {
            cfg.world = v.to_string();
        }
        if let Some(v) = get("region_blocks").and_then(|v| v.parse::<i32>().ok()) {
            if (256..=2048).contains(&v) && (v & (v - 1)) == 0 {
                cfg.region_blocks = v;
            }
        }
        if let Some(v) = get("cell_size").and_then(|v| v.parse::<i32>().ok()) {
            if v > 0 && (v & (v - 1)) == 0 && (1..=32).contains(&v) {
                cfg.cell_size = v;
            }
        }
        if let Some(v) = get("scan_budget").and_then(|v| v.parse::<u32>().ok()) {
            cfg.scan_budget = v.clamp(64, 65536);
        }
        if let Some(v) = get("gen_budget").and_then(|v| v.parse::<u32>().ok()) {
            cfg.gen_budget = v.clamp(1, 256);
        }
        if let Some(v) = get("gen_wish").and_then(|v| v.parse::<u32>().ok()) {
            cfg.gen_wish = v.clamp(32, 4096);
        }
        if let Some(v) = get("particles_per_tick")
            .or_else(|| get("stream_budget"))
            .and_then(|v| v.parse::<u32>().ok())
        {
            cfg.particles_per_tick = v.clamp(64, 16384);
        }
        if let Some(v) = get("center_x").and_then(|v| v.parse::<f64>().ok()) {
            cfg.center_x = Some(v);
        }
        if let Some(v) = get("center_z").and_then(|v| v.parse::<f64>().ok()) {
            cfg.center_z = Some(v);
        }
        if let Some(v) = get("fov").and_then(|v| v.parse::<f32>().ok()) {
            if (30.0..=150.0).contains(&v) {
                cfg.fov = v;
            }
        }
        if let Some(v) = get("lod_near").and_then(|v| v.parse::<f32>().ok()) {
            if (64.0..=8192.0).contains(&v) {
                cfg.lod_near = v;
            }
        }
        if let Some(v) = get("veil_far").and_then(|v| v.parse::<f32>().ok()) {
            if (128.0..=16384.0).contains(&v) {
                cfg.veil_far = v;
            }
        }
        if let Some(v) = get("depth_near").and_then(|v| v.parse::<f32>().ok()) {
            if (0.01..=1.0).contains(&v) {
                cfg.depth_near = v;
            }
        }
        if let Some(v) = get("sea_level").and_then(|v| v.parse::<i32>().ok()) {
            cfg.sea_level = v;
        }
        if let Some(v) = get("tick_ms").and_then(|v| v.parse::<u64>().ok()) {
            cfg.tick_ms = v.clamp(50, 60_000);
        }
        if let Some(v) = get("effect_terrain") {
            cfg.effect_terrain = v.to_string();
        }
        if let Some(v) = get("effect_atmosphere") {
            cfg.effect_atmosphere = v.to_string();
        }
        if let Some(v) = parse_color(get("color_low")) {
            cfg.color_low = v;
        }
        if let Some(v) = parse_color(get("color_high")) {
            cfg.color_high = v;
        }
        if let Some(v) = parse_color(get("color_water")) {
            cfg.color_water = v;
        }
        if let Some(v) = parse_color(get("color_veil")) {
            cfg.color_veil = v;
        }
        if let Some(v) = get("snow_line").and_then(|v| v.parse::<f32>().ok()) {
            cfg.snow_line = v.clamp(0.0, 1.0);
        }
        if let Some(v) = get("max_height").and_then(|v| v.parse::<i32>().ok()) {
            cfg.max_height = v.clamp(64, 1024);
        }
        if let Some(v) = get("native_gen") {
            cfg.native_gen = matches!(v.trim(), "1" | "true" | "yes" | "on");
        }
        if let Some(v) = get("native_threads").and_then(|v| v.parse::<i32>().ok()) {
            cfg.native_threads = v.clamp(0, 16);
        }
        if let Some(v) = get("seedgen_batch").and_then(|v| v.parse::<i32>().ok()) {
            cfg.seedgen_batch = v.clamp(1, 64);
        }
        cfg
    }
}

fn parse_color(v: Option<&str>) -> Option<[f32; 3]> {
    let v = v?;
    let mut out = [0f32; 3];
    for (i, c) in v.split(['/', ':', ',']).take(3).enumerate() {
        out[i] = c.trim().parse::<f32>().ok()?;
    }
    // Accept 0..255 ranges too: if any component > 1, scale all.
    if out.iter().any(|c| *c > 1.0) {
        for c in &mut out {
            *c /= 255.0;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sane() {
        let c = Config::parse("");
        assert_eq!(c.http_port, 8199);
        assert_eq!(c.region_blocks, 512);
        assert_eq!(c.cell_size, 2);
        assert_eq!(c.particles_per_tick, 2048);
        assert_eq!(c.gen_budget, 16, "v5.5 frontier admission default");
        assert_eq!(c.gen_wish, 512, "v5.5 in-flight window default");
        assert!(c.center_x.is_none());
        assert_eq!(c.fov, 70.0);
    }

    #[test]
    fn parses_everything() {
        let c = Config::parse(
            "http_port=9001;fov=90;region_blocks=1024;cell_size=2;color_low=0.1/0.8/0.2;world=world_nether;particles_per_tick=4096;center_x=-930.5;center_z=796",
        );
        assert_eq!(c.http_port, 9001);
        assert_eq!(c.fov, 90.0);
        assert_eq!(c.region_blocks, 1024);
        assert_eq!(c.cell_size, 2);
        assert_eq!(c.color_low, [0.1, 0.8, 0.2]);
        assert_eq!(c.world, "world_nether");
        assert_eq!(c.particles_per_tick, 4096);
        assert_eq!(c.center_x, Some(-930.5));
        assert_eq!(c.center_z, Some(796.0));
    }

    #[test]
    fn frontier_knobs_parse_and_clamp() {
        let c = Config::parse("gen_budget=64;gen_wish=1024");
        assert_eq!(c.gen_budget, 64);
        assert_eq!(c.gen_wish, 1024);
        // Clamps: 0 admission would deadlock the frontier, a giant window
        // would churn thousands of tickets.
        let c = Config::parse("gen_budget=0;gen_wish=100000");
        assert_eq!(c.gen_budget, 1);
        assert_eq!(c.gen_wish, 4096);
        let c = Config::parse("gen_wish=10");
        assert_eq!(c.gen_wish, 32);
    }

    #[test]
    fn seedgen_knobs_parse_and_clamp() {
        let c = Config::parse("native_threads=12;seedgen_batch=32");
        assert_eq!(c.native_threads, 12);
        assert_eq!(c.seedgen_batch, 32);
        // native_threads=0 keeps the auto path; batch clamps hard.
        let c = Config::parse("native_threads=0;seedgen_batch=1000");
        assert_eq!(c.native_threads, 0);
        assert_eq!(c.seedgen_batch, 64);
        let c = Config::parse("seedgen_batch=0");
        assert_eq!(c.seedgen_batch, 1, "batch 0/1 mean the single-chunk shape");
        let c = Config::default();
        assert_eq!(c.seedgen_batch, 8, "v5.9 batch default");
    }

    #[test]
    fn rejects_bad_region() {
        let c = Config::parse("region_blocks=3000");
        assert_eq!(c.region_blocks, 512);
        let c = Config::parse("region_blocks=1024;cell_size=3");
        assert_eq!(c.cell_size, 2, "non-pow2 cell rejected");
    }

    #[test]
    fn legacy_keys_are_ignored_silently() {
        // Old option strings must not crash the parser.
        let c = Config::parse(
            "atlas_size=1024;scan_deadline_secs=500;center_x=-930;stream_min_secs=90",
        );
        assert_eq!(c.region_blocks, 512);
        assert_eq!(c.http_port, 8199);
    }

    #[test]
    fn color_scale_255() {
        let c = Config::parse("color_veil=160/190/220");
        assert!((c.color_veil[0] - 160.0 / 255.0).abs() < 1e-4);
    }
}
