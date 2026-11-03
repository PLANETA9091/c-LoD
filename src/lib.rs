//! c-LoD — true server-side LOD for Paper 26.3, as a CRUSSTY module.
//!
//! Architecture (NO Display entities, NO client mods, NO runtime repack):
//!   1. The module scans world heightmaps into a fixed cell disk (JNI, main
//!      thread, budgeted per tick).
//!   2. It generates a SHADER-ONLY server resource pack (two `/posteffect`
//!      chains + GLSL + the sky camera override; ZERO textures, zero world
//!      data) and serves it over an embedded HTTP server. The pack is built
//!      EXACTLY ONCE per session — never rebuilt, never re-pushed.
//!   3. Players receive the pack once (`setResourcePack`, stable SHA-1).
//!   4. Every LOD byte streams at runtime as DUST PARTICLES over two sky
//!      planes above the disk (SKY-GRID BIT-STREAM): HI Y=260 carries the
//!      8-bit height word, LO Y=272 the 8-bit color index; 4 particles =
//!      one 16-bit word, sub-cell position = bits. The pack's capture
//!      shaders decode them against the depth buffer into persistent
//!      targets; `clod:lod_terrain` raymarches the heightfield from the
//!      live camera every frame and paints distant terrain.
//!   5. `clod:atmosphere` hides the render-distance edge with a depth veil.
//!      Per tick the engine switches chains per player (Off / Veil / Full
//!      tiers) — the only dynamic part, pure `/posteffect`.

mod config;
mod edits;
mod engine;
mod http;
mod jni;
mod packgen;
mod palette;
mod seedgen;
mod sha1;
mod stream;
mod wire;
mod zipw;

use cplug_abi::{CPluginApi, JavaVmPtr};
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub(crate) fn log_info(msg: &str) {
    eprintln!("[c-LoD] {msg}");
    cplug_sdk::log::info(&format!("[c-LoD] {msg}"));
}

static STATE: std::sync::OnceLock<Arc<engine::State>> = std::sync::OnceLock::new();

fn state() -> Option<&'static Arc<engine::State>> {
    STATE.get()
}

/// The single required export (cplug-abi contract).
///
/// # Safety
/// `api` must be a valid CPluginApi owned by the agent, `vm` the live
/// JavaVM pointer, `options` a NUL-terminated string for the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cplugin_init(
    api: *const CPluginApi,
    vm: JavaVmPtr,
    options: *const std::ffi::c_char,
) -> i32 {
    cplug_sdk::init(api, vm);

    let opts = cplug_sdk::jni_util::cstr(options).unwrap_or("").to_string();
    // CI/ops escape hatch: the CRUSSTY launcher hardcodes the agent options
    // string, so extra module knobs ride in via CLOD_OPTS ("k=v;k2=v2")
    // appended after the runtime's own keys (unknown keys are ignored by the
    // runtime's parser, ours are parsed below).
    let opts = match std::env::var("CLOD_OPTS") {
        Ok(extra) if !extra.trim().is_empty() => format!("{opts};{}", extra.trim()),
        _ => opts,
    };
    let cfg = config::Config::parse(&opts);
    log_info(&format!(
        "booting: sky-grid disk {} blocks (cell {}), http :{}, fov {}, particles/tick {}",
        cfg.region_blocks, cfg.cell_size, cfg.http_port, cfg.fov, cfg.particles_per_tick
    ));
    log_info(
        "c-LoD v6.9.0: THE RENDERER-DEAD MYTH BUSTED BY PIXEL FORENSICS — the #228 client.log + artifact PNGs show a HEALTHY client (no GL errors, no failed-to-apply warns, tick loop alive) whose MC window is 640x360 at (320,176) inside the 1280x720 Xvfb (run_client.sh --width 640 since v4.1): so the v6.8.x debug-lens panels rendered at window-relative x 312..632 (screen 632..952) and the harness lens_metrics (PX0=952, rows 392..712) measured the LETTERBOX — every lens_zero since v6.7.3 was a letterbox artifact (the panels are genuinely empty, the verdict right for the wrong reason); the burst white-count died because the color panel (screen x 632..952) covers the dust column zone (screen x 634..668 measured on the control arm) since v6.8.2 — the lens era IS the renderer-dead era. Fixes: run_client.sh launches --width 1280 --height 720 (documented geometry returns, burst zone uncovered, both arms one ruler); debug_lens.fsh gains three STAGE-EYE strips under the color panel's top frame (rot: green=conf>=0.25 / magenta=decayed / yellow=never decoded; scan: blue = col_scan payload rows found in main; carriers: yellow = 32x18 grid hits on clod_mask) so the next run names the blind stage (no-payload vs camera-dead vs capture-gates) instead of guessing; the debug chain feeds the lens Scan/Rot/Mask; lens_metrics LOCATES the panels via their green frames — window-size-proof forever", 
    );

    let st = engine::State::new(cfg);
    let _ = STATE.set(Arc::clone(&st));

    // Embedded HTTP server for the resource pack (own thread, no JVM).
    // v6.4.2 FIX: bind + the "listening" marker happen synchronously HERE —
    // the marker used to print from the accept thread and could interleave
    // mid-line with the runtime's "pipeline ready: N module hook(s)" (the
    // CI e2e G2 gate failed on a perfectly healthy boot).
    {
        let srv = Arc::clone(&st.server);
        let port = st.cfg.http_port;
        match srv.bind(port) {
            Ok(listener) => {
                std::thread::Builder::new()
                    .name("clod-http".into())
                    .spawn(move || {
                        srv.accept_loop(listener);
                    })
                    .ok();
            }
            Err(e) => {
                eprintln!("[c-LoD] http server failed: {e}");
            }
        }
    }

    // Everything JVM-side waits until the kernel (Bukkit) is up.
    cplug_sdk::on_kernel_ready("org.bukkit.Bukkit", move || {
        log_info("kernel ready — starting engine loop");
        let st = state().expect("state").clone();
        let tick_ms = st.cfg.tick_ms;

        std::thread::Builder::new()
            .name("clod-engine".into())
            .spawn(move || loop {
                // Idle throttling: 1 s between ticks with no players online
                // (v5.0.3: the 100 ms scheduling churn on an idle server was
                // implicated in native crashes after the last player left).
                let idle = st.idle_ticks.load(Ordering::Relaxed);
                let ms = if idle > 10 { tick_ms * 10 } else { tick_ms };
                std::thread::sleep(std::time::Duration::from_millis(ms));
                let st = state().expect("state").clone();
                // v5: one engine tick — boot/pack (once), disk scan,
                // dust-particle streaming, per-player tiers. All main thread.
                cplug_sdk::run_on_main_thread({
                    let st = Arc::clone(&st);
                    move |_env| {
                        engine::tick(&st, _env);
                    }
                });
            })
            .ok();
    });

    0
}
