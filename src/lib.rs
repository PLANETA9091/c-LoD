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
        "c-LoD v6.9.2: STAGE EYES v2 — run #231 measured rot=ok scan=15 lens_zero=1 while the carrier column is PLAINLY VISIBLE on the burst shot (4268 green px) and the relative strobe arm fires on 399/400 of the actual dot pixels — the hue test and the camera decode are INNOCENT; the residual suspects are the projection INPUTS. The four lens strips become bucket printers: CAM (the Globals CameraBlockPos the capture sees, in cell-space buckets over the atlas — a wrong bucket convicts the post-chain camera), ROT (az/el buckets from clod_rot_a), HITS (clod_h_b.g strobe-fired cells on a 32x32 grid), MASK (clod_mask marks on a dense 64x36 grid — v6.9.0 32x18 could miss 2-3px discs). The RENDERER DEAD marker decodes the buckets against the known vantage (cam ~8, az ~0 east, el ~3) and names: Globals-camera-wrong vs az-wrong vs strobe-never-fires vs merge-dead. Also #231 taught: the visible column survives the eraser because a DENSE column repaint copies dot colors from dot neighbors (the self-repairing fill assumed isolated dots) — cosmetic, noted. v6.9.1 carried: numpy CI deps, the dynamic prompt click + prompt guard", 
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
