//! The c-LoD engine: a state machine driven from the server main thread.
//!
//! v4.3 STREAMING lifecycle — the pack is a living surface, rebuilt on
//! throttled revisions and re-pushed to vanilla clients (new SHA-1, same
//! URL — clients re-download only when the hash changes, silently after
//! the first acceptance):
//!
//!   Boot → (restore `c-LoD-atlas.bin` if present and matching)
//!        → Scanning (budgeted JNI heightmap sweeps, main thread):
//!            · player-centric spiral (DH-style nearest-first: rings expand
//!              from each online player, so freshly-loaded chunks stream in
//!              exactly where the player is looking)
//!            · radial-from-center filler
//!        → Finalize (sweep complete OR scan deadline) → BASE pack rev 0:
//!            1. persist the atlas snapshot to disk (if `atlas_cache`)
//!            2. build the pack, swap it into the HTTP server
//!        → FINALIZED: with `stream` on the scan KEEPS RUNNING; whenever
//!          ≥`stream_min_cells` new cells accumulated and ≥`stream_min_secs`
//!          elapsed, the pack is REBUILT (engine thread, no JNI) as a new
//!          revision and pushed to every player whose SHA-1 differs.
//!          `stream=off` restores the v3 behavior: frozen after finalize.
//!
//! The ONLY other dynamic surface stays per-player `/posteffect` tier
//! switching (Off/Veil/Full) — it touches no bytes.
//!
//! Client-side, the `clod:lod_terrain` chain raymarches the world-anchored
//! heightfield atlas from the live camera every frame — walking, flying or
//! turning re-projects the LOD surface automatically, and each streamed
//! revision simply makes more cells hit.

use crate::atlas::Atlas;
use crate::config::Config;
use crate::http::PackServer;
use crate::jni::{self, PlayerInfo};
use crate::packgen::{self, Pack};
use jvmti_bindings::prelude::JniEnv;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, RwLock};

pub const PHASE_BOOT: u8 = 0;
pub const PHASE_SCANNING: u8 = 1;
pub const PHASE_PACKING: u8 = 2;
pub const PHASE_FINALIZED: u8 = 3;

pub struct State {
    pub cfg: Config,
    pub atlas: Arc<Atlas>,
    pub server: Arc<PackServer>,
    pub pack: RwLock<Option<Arc<Pack>>>,
    /// Static-pack phase machine (see PHASE_*).
    pub phase: AtomicU8,
    /// True when the atlas came from the disk cache (=> finalize immediately
    /// after the world resolves; bytes must match the previous session).
    pub restored: bool,
    /// Set after the one-time chunk warm-up around the atlas center.
    pub warmed: AtomicU8,
    /// When the atlas world was first resolved (scan deadline anchor).
    pub world_seen_at: Mutex<Option<std::time::Instant>>,
    pub scan_seq: AtomicU64,
    pub world_obj: Mutex<usize>, // jobject handle as usize; main-thread only
    pub world_name: Mutex<String>,
    pub keys: Mutex<Option<Keys>>,
    pub per_player: Mutex<HashMap<String, PlayerState>>,
    pub pack_uuid: Mutex<usize>,
    /// Engine tick counter (every tick() on the main thread).
    pub tick_no: AtomicU64,
    /// Rate limiter for pack-push failure logs.
    pub push_fails: AtomicU32,
    /// v4.3 streaming: scanned-cells watermark at the last pack build (the
    /// delta against `atlas.scanned_cells()` gates a new revision).
    pub built_cells: AtomicU64,
    /// v4.3 streaming: when the last pack build happened (throttle anchor).
    pub last_build: Mutex<Option<std::time::Instant>>,
    /// v4.3 streaming: current pack revision (0 = base, grows on rebuilds).
    pub pack_rev: AtomicU64,
}

#[derive(Clone)]
pub struct Keys {
    pub terrain: usize, // jobject global refs (main-thread only)
    pub atmosphere: usize,
}

#[derive(Default, Clone)]
pub struct PlayerState {
    /// SHA-1 of the pack last pushed to this player. The pack is static, so
    /// after the first successful push this matches forever and we never
    /// push again — no reloads, no flicker, no re-downloads.
    pub pushed_sha: String,
    pub terrain_on: bool,
    pub atmosphere_on: bool,
    /// Effect-list re-sends since the pack push. The client may finish the
    /// pack download/reload AFTER the first effect packet (push/reload race:
    /// the client logs "Requested post effect does not exist" and drops the
    /// chains). A few idempotent `PlayerPostEffects.update()` heartbeats let
    /// the client resolve the chains as soon as the pack is live.
    pub resends: u32,
    /// Last applied tier (0=Off, 1=Veil, 2=Full) for change logging.
    pub last_tier: u8,
}

/// Per-player LOD tier, re-evaluated every tick. Switching tiers is pure
/// `/posteffect` add/remove — the pack is never touched.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tier {
    /// Other world or no coverage data: no chains applied.
    Off,
    /// Atlas world but outside the static window: depth veil only.
    Veil,
    /// Inside the static atlas window: terrain + veil.
    Full,
}

impl State {
    pub fn new(cfg: Config) -> Arc<Self> {
        // Static-pack determinism: restore the frozen snapshot when it
        // matches the configured geometry; otherwise start an empty atlas.
        let (atlas, restored) = if cfg.atlas_cache {
            match Atlas::try_restore(
                std::path::Path::new(&cfg.atlas_path),
                cfg.center_x,
                cfg.center_z,
                cfg.atlas_size,
                cfg.cell_size,
                cfg.sea_level,
            ) {
                Some(a) => (Arc::new(a), true),
                None => (
                    Arc::new(Atlas::new(
                        cfg.center_x,
                        cfg.center_z,
                        cfg.atlas_size,
                        cfg.cell_size,
                        cfg.sea_level,
                    )),
                    false,
                ),
            }
        } else {
            (
                Arc::new(Atlas::new(
                    cfg.center_x,
                    cfg.center_z,
                    cfg.atlas_size,
                    cfg.cell_size,
                    cfg.sea_level,
                )),
                false,
            )
        };
        let server = PackServer::new();
        Arc::new(Self {
            atlas,
            server,
            pack: RwLock::new(None),
            phase: AtomicU8::new(PHASE_BOOT),
            restored,
            warmed: AtomicU8::new(0),
            world_seen_at: Mutex::new(None),
            scan_seq: AtomicU64::new(0),
            world_obj: Mutex::new(0),
            world_name: Mutex::new(String::new()),
            keys: Mutex::new(None),
            per_player: Mutex::new(HashMap::new()),
            pack_uuid: Mutex::new(0),
            tick_no: AtomicU64::new(0),
            push_fails: AtomicU32::new(0),
            built_cells: AtomicU64::new(0),
            last_build: Mutex::new(None),
            pack_rev: AtomicU64::new(0),
            cfg,
        })
    }
}

type Env<'a> = &'a JniEnv;

static BRIDGE_OK: AtomicU8 = AtomicU8::new(0); // 0 pending, 1 ok, 2 failed-once

/// One engine tick. MUST run on the main thread.
pub fn tick(state: &Arc<State>, env: Env) {
    state.tick_no.fetch_add(1, Ordering::Relaxed);
    ensure_boot(state, env);
    scan_batch(state, env);
    release_forceload(state, env);
    manage_players(state, env);
}

/// One-time: after finalize, release the temporary force-load on the warm
/// core (the snapshot is taken; a static pack needs no live chunks). Runs on
/// the main thread because it touches JNI; CAS makes it exactly-once.
fn release_forceload(state: &Arc<State>, env: Env) {
    if state.warmed.load(Ordering::SeqCst) != 1
        || state.phase.load(Ordering::SeqCst) != PHASE_FINALIZED
    {
        return;
    }
    if state
        .warmed
        .compare_exchange(1, 2, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }
    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        return;
    }
    let world = unsafe { as_obj(world_handle) };
    let w = state.cfg.warm_chunks;
    let mut released = 0usize;
    for cx in -w..=w {
        for cz in -w..=w {
            if jni::warm_chunk(env, world, cx, cz, false) {
                released += 1;
            }
        }
    }
    crate::log_info(&format!(
        "c-LoD core force-load released ({released} chunks) — static snapshot retained"
    ));
}

fn ensure_boot(state: &Arc<State>, env: Env) {
    // Resolve JNI ids once the kernel surface is fully loadable. Paper loads
    // the player-surface classes lazily, so warmup retries each tick until
    // the whole surface resolves (see jni::warmup).
    if BRIDGE_OK.load(Ordering::SeqCst) != 1 {
        if jni::warmup(env) {
            BRIDGE_OK.store(1, Ordering::SeqCst);
            crate::log_info("c-LoD JNI bridge resolved: true");
            // Static-pack phase machine: boot complete -> scanning.
            let _ = state.phase.compare_exchange(
                PHASE_BOOT,
                PHASE_SCANNING,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            // Effect keys (global refs, main thread).
            let terrain = jni::make_key(env, &state.cfg.effect_terrain)
                .map(|k| unsafe { new_global(env, k) })
                .unwrap_or(0);
            let atmosphere = jni::make_key(env, &state.cfg.effect_atmosphere)
                .map(|k| unsafe { new_global(env, k) })
                .unwrap_or(0);
            *state.keys.lock().unwrap() = Some(Keys {
                terrain,
                atmosphere,
            });
            // Pack UUID global.
            let uuid = jni::parse_uuid(env, &state.cfg.pack_uuid)
                .map(|u| unsafe { new_global(env, u) })
                .unwrap_or(0);
            *state.pack_uuid.lock().unwrap() = uuid;
        } else {
            if BRIDGE_OK.swap(2, Ordering::SeqCst) == 0 {
                crate::log_info(
                    "c-LoD JNI surface incomplete — Paper loads kernel classes lazily; retrying each tick",
                );
            }
            return;
        }
    }

    // Resolve the atlas world once (and re-resolve if lost).
    {
        let mut w = state.world_obj.lock().unwrap();
        if *w == 0 {
            if let Some(world) = jni::find_world(env, &state.cfg.world) {
                let name = world.name.clone();
                let g = unsafe { new_global(env, world.obj) };
                *w = g;
                *state.world_name.lock().unwrap() = name.clone();
                crate::log_info(&format!("c-LoD atlas world: {name}"));
                if state.restored {
                    crate::log_info(&format!(
                        "c-LoD atlas restored from {} ({}% coverage) — pack will reproduce the previous session's SHA-1",
                        state.cfg.atlas_path,
                        state.atlas.scanned_cells() * 100 / state.atlas.total_cells().max(1)
                    ));
                }
                // Anchor the scan deadline at world resolution.
                *state.world_seen_at.lock().unwrap() = Some(std::time::Instant::now());
                // One-time core warm-up: modern vanilla keeps no spawn
                // chunks loaded and the scan never forces generation, so
                // without this the static snapshot could cover nothing.
                if state.cfg.warm_chunks > 0 {
                    let w = state.cfg.warm_chunks;
                    let mut loaded = 0usize;
                    for cx in -w..=w {
                        for cz in -w..=w {
                            if jni::warm_chunk(env, unsafe { as_obj(g) }, cx, cz, true) {
                                loaded += 1;
                            }
                        }
                    }
                    state.warmed.store(1, Ordering::SeqCst);
                    let loaded0 = jni::is_chunk_loaded(env, unsafe { as_obj(g) }, 0, 0);
                    let h0 = jni::probe_height(env, unsafe { as_obj(g) }, 0, 0);
                    crate::log_info(&format!(
                        "c-LoD core force-loaded: {}x{} chunks around atlas center ({} ok) — isChunkLoaded(0,0)={loaded0:?}, height(0,0)={h0:?} — released at finalize",
                        2 * w + 1,
                        2 * w + 1,
                        loaded
                    ));
                }
            }
        }
    }
}

/// # Safety
/// `obj` must be a valid local ref; runs on the main thread only.
unsafe fn new_global(env: Env, obj: jvmti_bindings::jni::jobject) -> usize {
    env.new_global_ref(obj) as usize
}

unsafe fn as_obj(handle: usize) -> jvmti_bindings::jni::jobject {
    handle as jvmti_bindings::jni::jobject
}

fn scan_batch(state: &Arc<State>, env: Env) {
    // v4.3: scanning runs in PHASE_SCANNING, and — with streaming on —
    // KEEPS RUNNING after finalize so new revisions accumulate data.
    let phase = state.phase.load(Ordering::SeqCst);
    let streaming = state.cfg.stream;
    if phase != PHASE_SCANNING && !(phase == PHASE_FINALIZED && streaming) {
        return;
    }
    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        return;
    }
    let world = unsafe { as_obj(world_handle) };
    let seq = state.scan_seq.fetch_add(1, Ordering::Relaxed);
    let budget = state.cfg.scan_budget;

    // Budget split (v4.3): HALF goes to the player-centric spiral — the
    // terrain around online players streams first; the other half keeps
    // the radial-from-center sweep moving (it still owns the map when no
    // one is online). The spiral only returns unscanned cells, so in the
    // steady state it costs one RwLock read per visited cell.
    let players = jni::players(env);
    let near_budget = budget / 2;
    let mut near_done = 0usize;
    if near_budget > 0 {
        for p in &players {
            let take = near_budget / players.len().max(1) as u32;
            let tasks = state.atlas.scan_task_near(p.bx, p.bz, take);
            for (ix, iz) in tasks.iter().copied() {
                scan_cell(state, env, world, ix, iz);
                near_done += 1;
            }
            if near_done >= near_budget as usize {
                break;
            }
        }
    }

    // Radial filler: when the atlas is complete the center sweep would
    // only re-sample loaded chunks for nothing — park it.
    let scanned = state.atlas.scanned_cells();
    let total = state.atlas.total_cells();
    if scanned < total {
        let tasks = state.atlas.scan_task(seq * budget as u64, budget);
        for (ix, iz) in tasks.iter().copied() {
            scan_cell(state, env, world, ix, iz);
        }
    }

    // Rate-limited scan diagnostics (one line per 25 ticks).
    if seq.is_multiple_of(25) {
        use std::sync::atomic::Ordering as O;
        crate::log_info(&format!(
            "scan: tick {seq}, atlas v{} ({scanned}/{total} cells, +{near_done} near), rev {}, ok={} unloaded={} err={}",
            state.atlas.version(),
            state.pack_rev.load(O::Relaxed),
            jni::SCAN_OK.load(O::Relaxed),
            jni::SCAN_UNLOADED.load(O::Relaxed),
            jni::SCAN_ERR.load(O::Relaxed),
        ));
    }
}

/// Sample one atlas cell from the world and store it (shared by the
/// player-centric spiral and the radial sweep).
fn scan_cell(state: &Arc<State>, env: Env, world: jvmti_bindings::jni::jobject, ix: u32, iz: u32) {
    let x = state.atlas.cell_world_x(ix);
    let z = state.atlas.cell_world_z(iz);
    if let Some((y, liquid, material, biome)) = jni::sample_surface(env, world, x, z) {
        // v4.2 Voxy look: bake the REAL surface color (block palette +
        // biome tint) into the cell so the pack ships colored terrain.
        let rgb = crate::palette::surface_color(&material, &biome, liquid);
        state
            .atlas
            .set(ix, iz, (y.clamp(-64, 1023)) as i16, liquid, rgb);
    }
}

/// Tier for one player from world + static window bounds.
fn tier_for(state: &Arc<State>, p: &PlayerInfo) -> Tier {
    if p.world.is_empty() {
        return Tier::Off;
    }
    let wname = state.world_name.lock().unwrap().clone();
    if !wname.is_empty() && p.world != wname {
        return Tier::Off;
    }
    if state.atlas.cell_index(p.bx, p.bz).is_some() {
        Tier::Full
    } else {
        Tier::Veil
    }
}

fn manage_players(state: &Arc<State>, env: Env) {
    let keys = state.keys.lock().unwrap().clone();
    let Some(keys) = keys else { return };
    if keys.terrain == 0 && keys.atmosphere == 0 {
        return;
    }
    let pack = state.pack.read().unwrap().clone();

    let players = jni::players(env);
    let mut seen: Vec<String> = Vec::new();

    for p in players {
        seen.push(p.uuid.clone());
        let mut st = state
            .per_player
            .lock()
            .unwrap()
            .entry(p.uuid.clone())
            .or_default()
            .clone();

        // 1. Resource pack — pushed whenever the player's last-pushed SHA-1
        //    differs from the current pack. v3: exactly once per session
        //    (static pack). v4.3: ALSO on every streamed revision — the
        //    client re-downloads from the same URL only when the hash
        //    changed, silently after the first acceptance.
        if let Some(pack) = &pack {
            if st.pushed_sha != pack.sha1_hex {
                if let Some(handle) = jni::player_by_uuid(env, &p.uuid) {
                    let pu = *state.pack_uuid.lock().unwrap();
                    if pu != 0 {
                        let url = pack_url(state);
                        let ok = jni::push_pack(
                            env,
                            handle,
                            unsafe { as_obj(pu) },
                            &url,
                            &pack.sha1,
                            &state.cfg.pack_prompt,
                            false,
                        );
                        if ok {
                            let rev = state.pack_rev.load(Ordering::Relaxed);
                            st.pushed_sha = pack.sha1_hex.clone();
                            st.resends = 0;
                            crate::log_info(&format!(
                                "pack rev {rev} pushed to {} ({}): {} bytes, sha1 {}",
                                p.uuid,
                                url,
                                pack.zip.len(),
                                pack.sha1_hex
                            ));
                        } else if state.push_fails.fetch_add(1, Ordering::Relaxed) < 10 {
                            crate::log_info("pack push FAILED (see exception above)");
                        }
                    }
                }
            } else if st.resends < 12 && state.tick_no.load(Ordering::Relaxed).is_multiple_of(20) {
                // Self-healing: re-send the (unchanged) effect list a few
                // times after EACH push so a client that was mid-reload when
                // the first packet landed picks the chains up. Identical
                // lists are a wire no-op — zero flicker, zero reloads.
                if let Some(handle) = jni::player_by_uuid(env, &p.uuid) {
                    if jni::update_effects(env, handle) {
                        st.resends += 1;
                    }
                }
            }
        }

        // 2. Tier switching — the only dynamic surface, pure /posteffect.
        let tier = tier_for(state, &p);
        let tier_no = match tier {
            Tier::Full => 2u8,
            Tier::Veil => 1,
            Tier::Off => 0,
        };
        if st.last_tier != tier_no {
            crate::log_info(&format!(
                "tier {} -> {} for {} at ({}, {}) {}",
                match st.last_tier {
                    2 => "Full",
                    1 => "Veil",
                    _ => "Off",
                },
                match tier {
                    Tier::Full => "Full",
                    Tier::Veil => "Veil",
                    Tier::Off => "Off",
                },
                p.uuid,
                p.bx,
                p.bz,
                if p.world.is_empty() {
                    "(no world)"
                } else {
                    &p.world
                }
            ));
            st.last_tier = tier_no;
        }
        let have_pack = pack.is_some();
        if let Some(handle) = jni::player_by_uuid(env, &p.uuid) {
            let want_terrain = have_pack && tier == Tier::Full && keys.terrain != 0;
            let want_atmo =
                have_pack && (tier == Tier::Full || tier == Tier::Veil) && keys.atmosphere != 0;

            if want_terrain && !st.terrain_on {
                st.terrain_on = jni::add_effect(env, handle, unsafe { as_obj(keys.terrain) });
            } else if !want_terrain && st.terrain_on {
                st.terrain_on = !jni::remove_effect(env, handle, unsafe { as_obj(keys.terrain) });
            }
            if want_atmo && !st.atmosphere_on {
                st.atmosphere_on = jni::add_effect(env, handle, unsafe { as_obj(keys.atmosphere) });
            } else if !want_atmo && st.atmosphere_on {
                st.atmosphere_on =
                    !jni::remove_effect(env, handle, unsafe { as_obj(keys.atmosphere) });
            }
        }

        state.per_player.lock().unwrap().insert(p.uuid.clone(), st);
    }

    // Drop state for players who left.
    state
        .per_player
        .lock()
        .unwrap()
        .retain(|k, _| seen.contains(k));
}

pub fn pack_url(state: &Arc<State>) -> String {
    let base = if state.cfg.pack_base_url.is_empty() {
        detect_base_url(state.cfg.http_port)
    } else {
        state.cfg.pack_base_url.clone()
    };
    format!("{base}/clod-pack.zip")
}

fn detect_base_url(port: u16) -> String {
    // Best-effort: pick the first non-loopback IPv4 via a UDP connect trick
    // (no packets are sent), else localhost.
    let ip = std::net::UdpSocket::bind("0.0.0.0:0")
        .and_then(|s| {
            s.connect("10.254.254.254:1")?;
            Ok(s)
        })
        .and_then(|s| s.local_addr())
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    format!("http://{ip}:{port}")
}

/// Engine-thread finalize driver. Runs `finalize` exactly once when the
/// static snapshot is complete (restored, swept fully, or deadline reached).
/// Pure Rust + HTTP swap: no JNI, safe off the main thread.
pub fn maybe_finalize(state: &Arc<State>) {
    if state.phase.load(Ordering::SeqCst) != PHASE_SCANNING {
        return;
    }
    // Wait for the atlas world so logs/coverage are coherent before the pack
    // goes live (one engine tick after world resolution at most).
    if *state.world_obj.lock().unwrap() == 0 {
        return;
    }
    let complete = state.atlas.scanned_cells() >= state.atlas.total_cells();
    let deadline_hit = match *state.world_seen_at.lock().unwrap() {
        Some(t) => t.elapsed().as_secs() >= state.cfg.scan_deadline_secs || state.restored,
        None => state.restored,
    };
    if !complete && !deadline_hit {
        return;
    }
    finalize(state);
}

/// Build the BASE pack (rev 0), swap it into the HTTP server, persist the
/// atlas. v4.3: this is no longer the end of the story — with `stream` on,
/// `maybe_rebuild` keeps shipping revisions from here on.
fn finalize(state: &Arc<State>) {
    // Exactly-once guarantee even under racy callers.
    if state
        .phase
        .compare_exchange(
            PHASE_SCANNING,
            PHASE_PACKING,
            Ordering::SeqCst,
            Ordering::SeqCst,
        )
        .is_err()
    {
        return;
    }
    let (scanned, total) = (state.atlas.scanned_cells(), state.atlas.total_cells());

    if state.cfg.atlas_cache {
        let path = std::path::Path::new(&state.cfg.atlas_path);
        match state.atlas.save(path) {
            Ok(()) => crate::log_info(&format!(
                "c-LoD atlas snapshot saved: {} ({} cells, {} bytes)",
                state.cfg.atlas_path,
                total,
                std::fs::metadata(path).map(|m| m.len()).unwrap_or(0)
            )),
            Err(e) => crate::log_info(&format!(
                "c-LoD atlas snapshot save FAILED ({e}) — restarts will rescan"
            )),
        }
    }

    let new_pack = packgen::build(&state.cfg, &state.atlas);
    let total_c = state.atlas.total_cells();
    state.server.set_pack(
        new_pack.zip.clone(),
        crate::http::PackMeta {
            rev: 0,
            sha1_hex: new_pack.sha1_hex.clone(),
            cells: scanned,
            total_cells: total_c,
        },
    );
    crate::log_info(&format!(
        "c-LoD pack finalized (base rev 0): {scanned}/{total} cells ({}%), {} bytes, sha1 {}{}",
        scanned * 100 / total.max(1),
        new_pack.zip.len(),
        new_pack.sha1_hex,
        if state.cfg.stream {
            " — streaming ON"
        } else {
            " — static (stream off)"
        }
    ));
    *state.pack.write().unwrap() = Some(Arc::new(new_pack));
    state.built_cells.store(scanned, Ordering::SeqCst);
    *state.last_build.lock().unwrap() = Some(std::time::Instant::now());
    state.phase.store(PHASE_FINALIZED, Ordering::SeqCst);
}

/// Engine-thread revision driver (v4.3 streaming). Runs AFTER finalize:
/// whenever ≥`stream_min_cells` NEW cells accumulated since the last build
/// and ≥`stream_min_secs` elapsed, rebuild the pack from the live atlas,
/// swap it into the HTTP server and bump the revision. Pure Rust (PNG, zip,
/// sha1) — no JNI, safe off the main thread. The next engine tick pushes
/// the new SHA-1 to every player whose client still holds an older pack.
pub fn maybe_rebuild(state: &Arc<State>) {
    if !state.cfg.stream || state.phase.load(Ordering::SeqCst) != PHASE_FINALIZED {
        return;
    }
    if *state.world_obj.lock().unwrap() == 0 {
        return;
    }
    let delta = state
        .atlas
        .scanned_cells()
        .saturating_sub(state.built_cells.load(Ordering::SeqCst));
    if delta < state.cfg.stream_min_cells {
        return;
    }
    let due = match *state.last_build.lock().unwrap() {
        Some(t) => t.elapsed().as_secs() >= state.cfg.stream_min_secs,
        None => true,
    };
    if !due {
        return;
    }
    rebuild(state);
}

fn rebuild(state: &Arc<State>) {
    // One rebuild at a time; the watermark update makes re-entry a no-op.
    let scanned = state.atlas.scanned_cells();
    let total = state.atlas.total_cells();
    if state.cfg.atlas_cache {
        let path = std::path::Path::new(&state.cfg.atlas_path);
        if let Err(e) = state.atlas.save(path) {
            crate::log_info(&format!(
                "c-LoD atlas snapshot save FAILED ({e}) — continuing to stream"
            ));
        }
    }
    let new_pack = packgen::build(&state.cfg, &state.atlas);
    let rev = state.pack_rev.fetch_add(1, Ordering::SeqCst) + 1;
    state.server.set_pack(
        new_pack.zip.clone(),
        crate::http::PackMeta {
            rev,
            sha1_hex: new_pack.sha1_hex.clone(),
            cells: scanned,
            total_cells: total,
        },
    );
    let delta = scanned.saturating_sub(state.built_cells.load(Ordering::SeqCst));
    crate::log_info(&format!(
        "c-LoD pack rebuilt (stream rev {rev}): +{delta} cells, {scanned}/{total} ({}%), {} bytes, sha1 {}",
        scanned * 100 / total.max(1),
        new_pack.zip.len(),
        new_pack.sha1_hex
    ));
    *state.pack.write().unwrap() = Some(Arc::new(new_pack));
    state.built_cells.store(scanned, Ordering::SeqCst);
    *state.last_build.lock().unwrap() = Some(std::time::Instant::now());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_revision_lifecycle() {
        // v4.3: after finalize, accumulating stream_min_cells new cells (plus
        // stream_min_secs) must produce exactly one revision with new bytes;
        // below the thresholds nothing may rebuild.
        let dir = std::env::temp_dir().join(format!("clod-str-{}", std::process::id()));
        std::fs::create_dir_all(&dir).ok();
        let st = State::new(Config::parse(&format!(
            "atlas_size=512;cell_size=8;atlas_cache=true;atlas_path={}/a.bin;stream_min_cells=4",
            dir.display()
        )));
        st.phase.store(PHASE_SCANNING, Ordering::SeqCst);
        *st.world_obj.lock().unwrap() = 1;
        // Deadline already reached -> base finalize fires on first call.
        *st.world_seen_at.lock().unwrap() =
            Some(std::time::Instant::now() - std::time::Duration::from_secs(10_000));
        st.atlas.set(0, 0, 70, false, [0x91, 0xBD, 0x59]);
        maybe_finalize(&st);
        assert_eq!(st.phase.load(Ordering::SeqCst), PHASE_FINALIZED);
        let base_sha = st.pack.read().unwrap().as_ref().unwrap().sha1_hex.clone();
        assert_eq!(st.pack_rev.load(Ordering::SeqCst), 0);

        // Below the cell gate: no revision (throttle backdated so ONLY the
        // cell gate can block here). stream_min_cells clamps to a 16 floor,
        // so 14 new cells (delta 14) must stay below it.
        *st.last_build.lock().unwrap() =
            Some(std::time::Instant::now() - std::time::Duration::from_secs(600));
        for ix in 1..=14u32 {
            st.atlas
                .set(ix, 0, 70 + ix as i16, false, [0x91, 0xBD, 0x59]);
        }
        maybe_rebuild(&st);
        assert_eq!(st.pack_rev.load(Ordering::SeqCst), 0, "gate not reached");

        // Gate reached: one revision, new bytes, HTTP + meta swapped.
        // delta = 17 scanned - 1 built = 16 >= stream_min_cells.
        st.atlas.set(15, 0, 85, false, [0x91, 0xBD, 0x59]);
        st.atlas.set(16, 0, 86, false, [0x91, 0xBD, 0x59]);
        *st.last_build.lock().unwrap() =
            Some(std::time::Instant::now() - std::time::Duration::from_secs(600));
        maybe_rebuild(&st);
        assert_eq!(
            st.pack_rev.load(Ordering::SeqCst),
            1,
            "one streamed revision"
        );
        let rev_sha = st.pack.read().unwrap().as_ref().unwrap().sha1_hex.clone();
        assert_ne!(rev_sha, base_sha, "revision must change the pack bytes");
        assert_eq!(st.server.meta().rev, 1);
        assert_eq!(st.server.meta().cells, 17);
        assert_eq!(
            st.server.pack_bytes(),
            st.pack.read().unwrap().as_ref().unwrap().zip.clone()
        );

        // No NEW cells since the last build: idempotent, no revision churn.
        maybe_rebuild(&st);
        assert_eq!(st.pack_rev.load(Ordering::SeqCst), 1);

        // Streaming off: scanning stops after finalize, no rebuilds.
        let st2 = State::new(Config::parse(
            "atlas_size=512;cell_size=8;atlas_cache=false;stream=false",
        ));
        st2.phase.store(PHASE_SCANNING, Ordering::SeqCst);
        *st2.world_obj.lock().unwrap() = 1;
        *st2.world_seen_at.lock().unwrap() =
            Some(std::time::Instant::now() - std::time::Duration::from_secs(10_000));
        st2.atlas.set(0, 0, 70, false, [0x91, 0xBD, 0x59]);
        maybe_finalize(&st2);
        assert_eq!(st2.phase.load(Ordering::SeqCst), PHASE_FINALIZED);
        st2.atlas.set(1, 0, 71, false, [0x91, 0xBD, 0x59]);
        st2.atlas.set(2, 0, 72, false, [0x91, 0xBD, 0x59]);
        st2.atlas.set(3, 0, 73, false, [0x91, 0xBD, 0x59]);
        st2.atlas.set(4, 0, 74, false, [0x91, 0xBD, 0x59]);
        st2.atlas.set(5, 0, 75, false, [0x91, 0xBD, 0x59]);
        maybe_rebuild(&st2);
        assert_eq!(
            st2.pack_rev.load(Ordering::SeqCst),
            0,
            "stream=off stays frozen"
        );

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn scan_task_near_spreads_from_player() {
        let a = Atlas::new(0, 0, 512, 8, 63); // 64x64 cells
                                              // Player cell center: 512/8 = 64 cells -> center cell (32, 32).
                                              // Ring 0 (1 cell) + ring 1 (8 cells) = 9 cells — exactly `count`.
        let tasks = a.scan_task_near(0, 0, 9);
        assert_eq!(tasks.len(), 9);
        // All returned cells must sit inside the first rings around (32,32).
        for (ix, iz) in &tasks {
            let r = ((*ix as i32) - 32).abs().max(((*iz as i32) - 32).abs());
            assert!(r <= 1, "cell {:?} too far from the player", (ix, iz));
        }
        // Second call with the same count returns the same set (nothing was
        // marked valid in between — the spiral is deterministic).
        let again = a.scan_task_near(0, 0, 9);
        assert_eq!(tasks, again);
        // Outside the atlas: empty.
        assert!(a.scan_task_near(99_999, 99_999, 5).is_empty());
    }

    #[test]
    fn pack_url_uses_base() {
        let st = State::new(Config::parse("pack_base_url=http://x.example;http_port=1"));
        assert_eq!(pack_url(&st), "http://x.example/clod-pack.zip");
    }

    #[test]
    fn coverage_matches_world_and_bounds() {
        let st = State::new(Config::parse(
            "center_x=0;center_z=0;atlas_size=1024;cell_size=4",
        ));
        *st.world_name.lock().unwrap() = "world".to_string();
        let p = PlayerInfo {
            uuid: "u".into(),
            world: "world".into(),
            bx: 100,
            by: 64,
            bz: 100,
        };
        assert_eq!(tier_for(&st, &p), Tier::Full);
        let p_out = PlayerInfo {
            uuid: "u".into(),
            world: "world".into(),
            bx: 90_000,
            by: 64,
            bz: 0,
        };
        assert_eq!(tier_for(&st, &p_out), Tier::Veil);
        let p_world = PlayerInfo {
            uuid: "u".into(),
            world: "world_nether".into(),
            bx: 0,
            by: 64,
            bz: 0,
        };
        assert_eq!(tier_for(&st, &p_world), Tier::Off);
        let p_noworld = PlayerInfo {
            uuid: "u".into(),
            world: String::new(),
            bx: 0,
            by: 64,
            bz: 0,
        };
        assert_eq!(tier_for(&st, &p_noworld), Tier::Off);
    }

    #[test]
    fn finalize_happens_exactly_once() {
        let dir = std::env::temp_dir().join(format!("clod-eng-{}", std::process::id()));
        std::fs::create_dir_all(&dir).ok();
        let st = State::new(Config::parse(&format!(
            "atlas_size=512;cell_size=8;atlas_cache=true;atlas_path={}/a.bin",
            dir.display()
        )));
        st.phase.store(PHASE_SCANNING, Ordering::SeqCst);
        *st.world_obj.lock().unwrap() = 1; // pretend the world resolved
        *st.world_seen_at.lock().unwrap() = Some(std::time::Instant::now());
        st.atlas.set(0, 0, 70, false, [0x91, 0xBD, 0x59]);

        // Deadline NOT reached yet, atlas incomplete -> no finalize.
        maybe_finalize(&st);
        assert_eq!(st.phase.load(Ordering::SeqCst), PHASE_SCANNING);
        assert!(st.pack.read().unwrap().is_none());

        // Deadline reached -> finalize; second call is a no-op.
        *st.world_seen_at.lock().unwrap() =
            Some(std::time::Instant::now() - std::time::Duration::from_secs(10_000));
        maybe_finalize(&st);
        assert_eq!(st.phase.load(Ordering::SeqCst), PHASE_FINALIZED);
        let sha1 = st.pack.read().unwrap().as_ref().unwrap().sha1_hex.clone();
        maybe_finalize(&st);
        assert_eq!(st.phase.load(Ordering::SeqCst), PHASE_FINALIZED);
        assert_eq!(st.pack.read().unwrap().as_ref().unwrap().sha1_hex, sha1);

        // HTTP server serves the finalized pack bytes.
        let served = st.server.pack_bytes();
        assert!(!served.is_empty());
        assert_eq!(
            served.len(),
            st.pack.read().unwrap().as_ref().unwrap().zip.len()
        );

        // Cache file exists and restores to the same SHA-1 (restart sim).
        let path = dir.join("a.bin");
        assert!(path.exists());
        drop(st);
        let st2 = State::new(Config::parse(&format!(
            "atlas_size=512;cell_size=8;atlas_cache=true;atlas_path={}/a.bin",
            dir.display()
        )));
        assert!(st2.restored);
        assert_eq!(st2.atlas.scanned_cells(), 1);
        st2.phase.store(PHASE_SCANNING, Ordering::SeqCst);
        *st2.world_obj.lock().unwrap() = 1;
        *st2.world_seen_at.lock().unwrap() = Some(std::time::Instant::now());
        maybe_finalize(&st2);
        assert_eq!(
            st2.pack.read().unwrap().as_ref().unwrap().sha1_hex,
            sha1,
            "restored session must reproduce the exact same pack SHA-1"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}
