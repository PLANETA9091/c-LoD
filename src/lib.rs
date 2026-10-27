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
mod engine;
mod http;
mod jni;
mod packgen;
mod palette;
mod sha1;
mod stream;
mod zipw;

use cplug_abi::{CPluginApi, JavaVmPtr};
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
        "c-LoD v5.0: SKY-GRID BIT-STREAM — shader-only pack pushed once, world data rides dust particles",
    );

    let st = engine::State::new(cfg);
    let _ = STATE.set(Arc::clone(&st));

    // Embedded HTTP server for the resource pack (own thread, no JVM).
    {
        let srv = Arc::clone(&st.server);
        let port = st.cfg.http_port;
        std::thread::Builder::new()
            .name("clod-http".into())
            .spawn(move || {
                if let Err(e) = srv.serve(port) {
                    eprintln!("[c-LoD] http server failed: {e}");
                }
            })
            .ok();
    }

    // Everything JVM-side waits until the kernel (Bukkit) is up.
    cplug_sdk::on_kernel_ready("org.bukkit.Bukkit", move || {
        log_info("kernel ready — starting engine loop");
        let st = state().expect("state").clone();
        let tick_ms = st.cfg.tick_ms;

        std::thread::Builder::new()
            .name("clod-engine".into())
            .spawn(move || loop {
                std::thread::sleep(std::time::Duration::from_millis(tick_ms));
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
