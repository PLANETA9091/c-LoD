//! c-LoD configuration: parsed from the CRUSSTY module options string.
//!
//! Options format: `key1=value1;key2=value2;...` (whatever the runtime hands
//! to `cplugin_init`). Every knob has a working default, so an empty options
//! string produces a functional module.
//!
//! v5.0: the window/revision knobs are GONE — the pack is a static
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
    /// v6.2 LIVE EDITS: engine ticks between rescan waves over the streamed
    /// disk. The wave re-samples stale cells of LOADED chunks (a cheap
    /// height probe; full color sample only when the surface moved), so
    /// blocks placed after server start reach the LOD without a relog.
    /// 0 = off (the v6.1 paint-once behavior). Full-disk wave cost at the
    /// default: ~1638 probes/tick worst case, only inside loaded chunks.
    pub rescan_ticks: u64,
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
    /// v6.18: attach the atmosphere veil effect at all. Production default
    /// true (the veil hides the render-distance edge). The parity sessions
    /// run atmosphere=false: the veil would re-tint the un-fogged far band
    /// (smoothstep(lod_near*0.85, veil_far) * 0.92 = 2.6-6.4% at 64..128 m)
    /// while the voxy reference leg ships no veil — a foreign residual the
    /// pixel gate would measure forever. CLOD_OPTS: atmosphere=false.
    pub atmosphere: bool,
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
    /// v6.0 NATIVE DUST WIRE: deliver the per-tick dust batch as ONE
    /// ClientboundBundlePacket per player built from cached packets (zero
    /// Java objects per particle in the steady state). When false — or when
    /// the bundle surface fails to resolve — the proven v5 per-particle
    /// path handles delivery.
    pub native_dust: bool,
    /// v6.1 SEEDGEN TREES: run the real vanilla VEGETAL_DECORATION pass
    /// (trees, leaves — the exact addVanillaDecorations recipe) on top of
    /// the native terrain walk, so never-generated forest chunks stream
    /// with tree tops from the seed alone. When false — or when any
    /// feature surface fails to resolve — cells stay terrain-only (the
    /// v5.9 output, bit-for-bit).
    pub native_trees: bool,
    /// v6.3 EVENT PUSH: a Bukkit event listener (java/ClodEdits, stable
    /// PluginManager API) feeds every block change into a ring the engine
    /// drains next tick — edits reach the dirty phase in ONE engine tick
    /// instead of waiting rescan_ticks. Needs at least one enabled plugin
    /// as the registration host; without one (or on any failure) the v6.2
    /// rescan wave stays the detector, so this knob can never regress.
    pub native_edits: bool,
    /// v6.5.2 CI FORENSICS: world x/z of a cell window served by
    /// /clod-cells (height/water/valid/rgb/words/last-stream age/dirty).
    /// Lets a CI run compare the SERVER's belief about a cell with what
    /// the client painted. None = the endpoint stays 503 (production
    /// default — no window, no cost beyond one RwLock<String>).
    /// CLOD_OPTS: debug_cell_x=<x>;debug_cell_z=<z> (both or neither).
    pub debug_cell: Option<(i32, i32)>,
    /// v6.7 CAPTURE LENS: ship the debug chain whose final pass draws the
    /// client's committed capture targets (color + height windows) onto the
    /// screen. CI-only observability — the pack SHA changes (one re-push on
    /// the runner), production stays debug_lens=false forever.
    pub debug_lens: bool,
    /// v6.18 UN-FOG: ship the chain variant whose extra pass inverts the
    /// vanilla fog on the vanilla-rendered pixels (assets/glsl/unfog.fsh —
    /// the exact fog.glsl + FogRenderer.bytecode math), so the far terrain
    /// inside the vanilla render distance reads crisp like a Voxy mesh.
    /// Parity-session knob — the pack SHA changes (one re-push on the
    /// runner), production stays unfog=false forever. Combines with nothing:
    /// debug_lens=true keeps the debug chain (the lens variant has no unfog
    /// pass); the parity world runs debug_lens=false + unfog=true.
    pub unfog: bool,
    /// v6.18.2 EVIDENCE: ship the un-fog ENGAGEMENT MASK instead of the
    /// render (unfog_debug.fsh: GREEN = the swap applied, RED = the vanilla
    /// fog saturated, BLUE = the channel blind, DIM = the fogs agree).
    /// Requires unfog=true; the §5b parity_dbg lane reads the masks.
    pub unfog_debug: bool,
    /// v6.19 G8d MESH PAINT: the march depth-tests the vanilla fragments and
    /// paints the unified march look OVER the vanilla band wherever the
    /// streamed surface stands in front (assets/glsl/lod_terrain.fsh — the
    /// run #292 verdict: the representation gap is the 01-10 diff class, the
    /// march is the only painter we own). Pack SHA changes; the parity world
    /// boots with it, production stays mesh_paint=false.
    pub mesh_paint: bool,
    /// v6.7.1 MULTI-BEAT: retry beats a genuine edit gets after its release
    /// (total decode chances = edit_beats + 1). Each retry re-runs the
    /// hold→release sequence with a fresh reset electron, so a beat whose
    /// sparse color dot was lost (the transport's single-point fragility,
    /// RED index 2 = one active slot-dot) gets another complete-decode
    /// chance while the miss probability falls as (1-p)^K. 0 = the v6.6.1
    /// single-shot edit; the cap keeps an episode bounded.
    pub edit_beats: u32,
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
            rescan_ticks: 40,
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
            native_dust: true,
            native_trees: true,
            native_edits: true,
            debug_cell: None,
            debug_lens: false,
            unfog: false,
            unfog_debug: false,
            mesh_paint: false,
            atmosphere: true,
            edit_beats: crate::stream::EDIT_BEATS_DEFAULT,
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
        if let Some(v) = get("rescan_ticks").and_then(|v| v.parse::<u64>().ok()) {
            // 0 keeps the wave off; anything else is at least 5 ticks and
            // at most 2 minutes (a slower wave just delays edit detection).
            cfg.rescan_ticks = if v == 0 { 0 } else { v.clamp(5, 2400) };
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
        if let Some(v) = get("native_dust") {
            cfg.native_dust = matches!(v.trim(), "1" | "true" | "yes" | "on");
        }
        if let Some(v) = get("native_trees") {
            cfg.native_trees = matches!(v.trim(), "1" | "true" | "yes" | "on");
        }
        if let Some(v) = get("native_edits") {
            cfg.native_edits = matches!(v.trim(), "1" | "true" | "yes" | "on");
        }
        // v6.5.2: debug_cell=x,z — the /clod-cells window center.
        if let (Some(x), Some(z)) = (get("debug_cell_x"), get("debug_cell_z")) {
            if let (Ok(x), Ok(z)) = (x.trim().parse::<i32>(), z.trim().parse::<i32>()) {
                cfg.debug_cell = Some((x, z));
            }
        }
        // v6.7: debug_lens=true — ship the capture-lens debug chain.
        if let Some(v) = get("debug_lens") {
            cfg.debug_lens = matches!(v.trim(), "1" | "true" | "yes" | "on");
        }
        // v6.18: unfog=true — ship the un-fog chain variant.
        if let Some(v) = get("unfog") {
            cfg.unfog = matches!(v.trim(), "1" | "true" | "yes" | "on");
        }
        // v6.18.2: unfog_debug=true — the engagement-mask evidence shader.
        if let Some(v) = get("unfog_debug") {
            cfg.unfog_debug = matches!(v.trim(), "1" | "true" | "yes" | "on");
        }
        // v6.19: mesh_paint=true — G8d, the march paints over the vanilla
        // band where the streamed data stands in front (mesh unification).
        if let Some(v) = get("mesh_paint") {
            cfg.mesh_paint = matches!(v.trim(), "1" | "true" | "yes" | "on");
        }
        // v6.18: atmosphere=false — never attach the veil effect.
        if let Some(v) = get("atmosphere") {
            cfg.atmosphere = matches!(v.trim(), "1" | "true" | "yes" | "on");
        }
        // v6.7.1: edit_beats — bounded retry beats per genuine edit.
        if let Some(v) = get("edit_beats").and_then(|v| v.parse::<u32>().ok()) {
            cfg.edit_beats = v.clamp(0, crate::stream::EDIT_BEATS_MAX);
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
    fn native_dust_knob() {
        let c = Config::parse("");
        assert!(c.native_dust, "v6.0 wire defaults ON");
        let c = Config::parse("native_dust=false");
        assert!(!c.native_dust, "legacy per-particle path selectable");
        let c = Config::parse("native_dust=1");
        assert!(c.native_dust);
    }

    #[test]
    fn native_trees_knob() {
        let c = Config::parse("");
        assert!(c.native_trees, "v6.1 seedgen trees default ON");
        let c = Config::parse("native_trees=false");
        assert!(!c.native_trees, "terrain-only v5.9 output selectable");
        let c = Config::parse("native_trees=on");
        assert!(c.native_trees);
    }

    #[test]
    fn native_edits_knob() {
        let c = Config::parse("");
        assert!(c.native_edits, "v6.3 event push defaults ON");
        let c = Config::parse("native_edits=false");
        assert!(!c.native_edits, "wave-only detection selectable");
        let c = Config::parse("native_edits=1");
        assert!(c.native_edits);
    }

    #[test]
    fn debug_cell_window_knob() {
        // v6.5.2 forensics: off by default, both halves required, negatives
        // (the usual disk centers) parse through.
        let c = Config::parse("");
        assert_eq!(c.debug_cell, None, "no window unless configured");
        let c = Config::parse("debug_cell_x=-930");
        assert_eq!(c.debug_cell, None, "a lone x must not arm the window");
        let c = Config::parse("debug_cell_x=-930;debug_cell_z=796");
        assert_eq!(c.debug_cell, Some((-930, 796)));
        let c = Config::parse("debug_cell_x=0;debug_cell_z=64");
        assert_eq!(c.debug_cell, Some((0, 64)));
    }

    #[test]
    fn atmosphere_knob() {
        // v6.18: the veil attaches unless explicitly disabled.
        assert!(Config::parse("").atmosphere, "production default is on");
        assert!(!Config::parse("atmosphere=false").atmosphere);
        assert!(Config::parse("atmosphere=true").atmosphere);
        assert!(!Config::parse("atmosphere=0").atmosphere);
    }

    #[test]
    fn unfog_knob() {
        // v6.18: the un-fog pass ships only when explicitly armed.
        assert!(!Config::parse("").unfog, "production default is off");
        for v in ["1", "true", "yes", "on"] {
            assert!(Config::parse(&format!("unfog={v}")).unfog, "{v}");
        }
        for v in ["0", "false", "off", ""] {
            assert!(!Config::parse(&format!("unfog={v}")).unfog, "{v}");
        }
        // v6.18.2: the evidence mask is a separate knob, default off.
        assert!(!Config::parse("unfog=true").unfog_debug);
        assert!(Config::parse("unfog=true;unfog_debug=true").unfog_debug);
        assert!(!Config::parse("unfog_debug=true").unfog);
    }

    #[test]
    fn debug_lens_knob() {
        // v6.7: the capture lens ships only when explicitly armed.
        assert!(!Config::parse("").debug_lens, "production default is off");
        for v in ["1", "true", "yes", "on"] {
            assert!(Config::parse(&format!("debug_lens={v}")).debug_lens, "{v}");
        }
        for v in ["0", "false", "off", ""] {
            assert!(!Config::parse(&format!("debug_lens={v}")).debug_lens, "{v}");
        }
    }

    #[test]
    fn mesh_paint_knob() {
        // v6.19 G8d: the march's vanilla path ships only when explicitly
        // armed; the parity world boots with mesh_paint=true.
        assert!(!Config::parse("").mesh_paint, "production default is off");
        for v in ["1", "true", "yes", "on"] {
            assert!(Config::parse(&format!("mesh_paint={v}")).mesh_paint, "{v}");
        }
        for v in ["0", "false", "off", ""] {
            assert!(!Config::parse(&format!("mesh_paint={v}")).mesh_paint, "{v}");
        }
        // Combines with the unfog chain flag (the parity boot's full set).
        let c = Config::parse("debug_lens=false;unfog=true;mesh_paint=true;atmosphere=false");
        assert!(c.mesh_paint && c.unfog && !c.debug_lens && !c.atmosphere);
    }

    #[test]
    fn rejects_bad_region() {
        let c = Config::parse("region_blocks=3000");
        assert_eq!(c.region_blocks, 512);
        let c = Config::parse("region_blocks=1024;cell_size=3");
        assert_eq!(c.cell_size, 2, "non-pow2 cell rejected");
    }

    #[test]
    fn rescan_ticks_knob() {
        let c = Config::parse("");
        assert_eq!(c.rescan_ticks, 40, "v6.2 live-edit wave default");
        let c = Config::parse("rescan_ticks=0");
        assert_eq!(c.rescan_ticks, 0, "0 disables the wave (v6.1 behavior)");
        let c = Config::parse("rescan_ticks=1");
        assert_eq!(c.rescan_ticks, 5, "clamp to a sane minimum");
        let c = Config::parse("rescan_ticks=99999");
        assert_eq!(c.rescan_ticks, 2400);
    }

    #[test]
    fn edit_beats_knob() {
        // v6.7.1 MULTI-BEAT: the bounded retry-beat count per genuine edit.
        let c = Config::parse("");
        assert_eq!(
            c.edit_beats,
            crate::stream::EDIT_BEATS_DEFAULT,
            "default: 2 retries (three total decode chances)"
        );
        let c = Config::parse("edit_beats=0");
        assert_eq!(c.edit_beats, 0, "0 restores the v6.6.1 single-shot edit");
        let c = Config::parse("edit_beats=1");
        assert_eq!(c.edit_beats, 1, "1 is the v6.6.2 single rebeat");
        let c = Config::parse("edit_beats=5");
        assert_eq!(c.edit_beats, 5);
        let c = Config::parse("edit_beats=99");
        assert_eq!(
            c.edit_beats,
            crate::stream::EDIT_BEATS_MAX,
            "clamped to the episode bound"
        );
        let c = Config::parse("edit_beats=abc");
        assert_eq!(
            c.edit_beats,
            crate::stream::EDIT_BEATS_DEFAULT,
            "garbage falls back to the default"
        );
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
