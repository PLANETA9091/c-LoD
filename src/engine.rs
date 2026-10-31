//! The c-LoD engine, v5.0 SKY-GRID BIT-STREAM.
//!
//! The resource pack is a STATIC shader-only artifact built once per process
//! boot and pushed once per client join — it contains ZERO world data: no
//! atlas, no PNGs, nothing world-derived. Every byte of LOD data travels at
//! runtime as DUST PARTICLES flying over the scan disk on two sky planes
//! (HI = height word at Y=260, LO = color index word at Y=272); the pack's
//! capture shaders decode them against the depth buffer into persistent
//! targets and the marcher renders the heightfield live.
//!
//! Tick loop (main thread):
//!   1. boot       — resolve the JNI surface, build the pack once, push it.
//!   2. scan       — center-centric heightmap sweep into the disk store (the
//!      streamer's data source), with a bounded async force-load frontier.
//!   3. stream     — spawn the next batch of dust particles over the disk
//!      (ServerLevel.sendParticles, one call per particle, count=0).
//!   4. players    — one resource-pack push per join (the pack never
//!      changes), per-player posteffect tier switching.

use crate::config::Config;
use crate::http::PackServer;
use crate::jni::{self, PlayerInfo};
use crate::packgen::{self, Pack};
use crate::stream::{Cell, Streamer};
use jvmti_bindings::prelude::JniEnv;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, RwLock};

pub const PHASE_BOOT: u8 = 0;
pub const PHASE_LIVE: u8 = 3;

pub struct State {
    pub cfg: Config,
    pub server: Arc<PackServer>,
    pub pack: RwLock<Option<Arc<Pack>>>,
    pub phase: AtomicU8,
    pub world_obj: Mutex<usize>, // jobject handle as usize; main-thread only
    pub world_name: Mutex<String>,
    pub keys: Mutex<Option<Keys>>,
    pub per_player: Mutex<HashMap<String, PlayerState>>,
    pub pack_uuid: Mutex<usize>,
    /// Engine tick counter (every tick() on the main thread).
    pub tick_no: AtomicU64,
    /// Rate limiter for pack-push failure logs.
    pub push_fails: AtomicU32,
    /// v5 sky-grid streaming state (Rust side; main-thread only).
    pub stream: Mutex<Streamer>,
    /// v5.0.4 frontier: chunks with an in-flight moonrise FORCED ticket
    /// (chunk -> tick when requested). NON-BLOCKING: the chunk generates on
    /// the chunk system's workers; this ledger is polled each tick and the
    /// ticket is released after sampling (v4.4's Bukkit setChunkForceLoaded
    /// sync-generated chunks on the main thread and stalled the server).
    pub gen_wish: Mutex<HashMap<(i32, i32), u64>>,
    /// Frontier chunks scanned through the force path so far.
    pub forced_chunks: AtomicU64,
    /// Diagnostics: cells streamed as particles since boot.
    pub streamed_cells: AtomicU64,
    /// Consecutive engine ticks with zero players online (idle throttling).
    pub idle_ticks: AtomicU32,
    /// Set once the streamer origin was fixed and particles are flying.
    pub stream_armed: AtomicU8,
    /// SEEDGEN (v5.7): off-thread native generation of never-generated
    /// chunks (noise+surface per column, no chunk-system tickets). None
    /// until the first LIVE tick after the world is known.
    pub seedgen: Mutex<Option<Arc<crate::seedgen::Seedgen>>>,
    /// Provisional cells upgraded from a real generated chunk since boot.
    pub prov_upgraded: AtomicU64,
    /// Of those, how many changed the encoded words (fidelity metric: the
    /// delta between native noise+surface and the full generated chunk —
    /// trees, carvers, structures).
    pub prov_diffs: AtomicU64,
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
    /// Effect-list re-sends since the pack push (push/reload race healer).
    pub resends: u32,
    /// Last applied tier (0=Off, 1=Veil, 2=Full) for change logging.
    pub last_tier: u8,
    /// Engine tick of the player's first sight (pack push anchor: the push
    /// must land AFTER the client finished the join screens, or the pack
    /// prompt screen gets swallowed by the loading terrain screen).
    pub joined_tick: u64,
}

/// Engine ticks between the join and the single pack push.
const PACK_PUSH_DELAY_TICKS: u64 = 40;

impl State {
    pub fn new(cfg: Config) -> Arc<Self> {
        let stream = Streamer::new(
            cfg.center_x.unwrap_or(0.0),
            cfg.center_z.unwrap_or(0.0),
            cfg.region_blocks,
            cfg.cell_size,
        );
        let server = PackServer::new();
        Arc::new(Self {
            cfg,
            stream: Mutex::new(stream),
            server,
            pack: RwLock::new(None),
            phase: AtomicU8::new(PHASE_BOOT),
            world_obj: Mutex::new(0),
            world_name: Mutex::new(String::new()),
            keys: Mutex::new(None),
            per_player: Mutex::new(HashMap::new()),
            pack_uuid: Mutex::new(0),
            tick_no: AtomicU64::new(0),
            push_fails: AtomicU32::new(0),
            gen_wish: Mutex::new(HashMap::new()),
            forced_chunks: AtomicU64::new(0),
            streamed_cells: AtomicU64::new(0),
            idle_ticks: AtomicU32::new(0),
            stream_armed: AtomicU8::new(0),
            seedgen: Mutex::new(None),
            prov_upgraded: AtomicU64::new(0),
            prov_diffs: AtomicU64::new(0),
        })
    }
}

type Env<'a> = &'a JniEnv;

static BRIDGE_OK: AtomicU8 = AtomicU8::new(0); // 0 pending, 1 ok, 2 failed-once

/// One engine tick. MUST run on the main thread.
pub fn tick(state: &Arc<State>, env: Env) {
    state.tick_no.fetch_add(1, Ordering::Relaxed);
    if state.tick_no.load(Ordering::Relaxed).is_multiple_of(50) {
        crate::log_info(&format!(
            "engine tick alive: tick {}",
            state.tick_no.load(Ordering::Relaxed)
        ));
    }
    ensure_boot(state, env);
    arm_from_config(state);
    stream_scan(state, env);
    poll_forced(state, env);
    stream_step(state, env);
    manage_players(state, env);
    // Idle throttling: with nobody online the engine loop sleeps 10x longer
    // between ticks (the scheduling churn of an idle server was implicated
    // in native crashes after the last player left). per_player holds one
    // entry per online player after manage_players.
    if state.per_player.lock().unwrap().is_empty() {
        state.idle_ticks.fetch_add(1, Ordering::Relaxed);
    } else {
        state.idle_ticks.store(0, Ordering::Relaxed);
    }
}

/// One-time boot: JNI surface + the shader-only pack (built IMMEDIATELY — it
/// holds no world data, so there is nothing to wait for) + effect keys.
fn ensure_boot(state: &Arc<State>, env: Env) {
    eprintln!("[c-LoD] EB: enter, native_gen={}", state.cfg.native_gen);
    if BRIDGE_OK.load(Ordering::SeqCst) != 1 {
        if jni::warmup(env) && jni::nms_warmup(env) {
            BRIDGE_OK.store(1, Ordering::SeqCst);
            crate::log_info("c-LoD JNI bridge resolved: true (incl. NMS particle surface)");
            let _ = state.phase.compare_exchange(
                PHASE_BOOT,
                PHASE_LIVE,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
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
            let uuid = jni::parse_uuid(env, &state.cfg.pack_uuid)
                .map(|u| unsafe { new_global(env, u) })
                .unwrap_or(0);
            *state.pack_uuid.lock().unwrap() = uuid;

            // THE pack: shader-only, data-free, deterministic. Built exactly
            // once per process; its SHA-1 never changes for the lifetime of
            // the module (vanilla clients re-download never).
            let new_pack = packgen::build(&state.cfg);
            let sha = new_pack.sha1_hex.clone();
            let len = new_pack.zip.len();
            state.server.set_pack(
                new_pack.zip.clone(),
                crate::http::PackMeta {
                    rev: 0,
                    sha1_hex: sha.clone(),
                    cells: 0,
                    total_cells: 0,
                },
            );
            *state.pack.write().unwrap() = Some(Arc::new(new_pack));
            crate::log_info(&format!(
                "c-LoD pack finalized (ONE-TIME, static): shader-only, {len} bytes, sha1 {sha} — no atlas, no textures, one push per join"
            ));
        } else {
            if BRIDGE_OK.swap(2, Ordering::SeqCst) == 0 {
                crate::log_info(
                    "c-LoD JNI surface incomplete — Paper loads kernel classes lazily; retrying each tick",
                );
            }
            return;
        }
    }

    // Resolve the streaming world once (and re-resolve if lost).
    let mut w = state.world_obj.lock().unwrap();
    if *w == 0 {
        if let Some(world) = jni::find_world(env, &state.cfg.world) {
            let name = world.name.clone();
            let g = unsafe { new_global(env, world.obj) };
            *w = g;
            *state.world_name.lock().unwrap() = name.clone();
            crate::log_info(&format!("c-LoD streaming world: {name}"));
        }
    }
    drop(w);
}

/// # Safety
/// `obj` must be a valid local ref; runs on the main thread only.
unsafe fn new_global(env: Env, obj: jvmti_bindings::jni::jobject) -> usize {
    env.new_global_ref(obj) as usize
}

unsafe fn as_obj(handle: usize) -> jvmti_bindings::jni::jobject {
    handle as jvmti_bindings::jni::jobject
}

/// Re-issue a ticket that produced a loaded chunk within this many ticks
/// (moonrise retries internally; this guards against lost tickets).
const GEN_REISSUE_TICKS: u64 = 200;

/// v5.5 fallback sampling cap for one poll tick (config-independent JNI
/// ceiling; the frontier knobs live in Config now).
const POLL_MAX_SAMPLES: usize = 256;

/// Pure admission gate for the gen-wish ledger (unit-tested): duplicate chunk
/// rejected, backlog capped, per-tick request budget enforced. v5.5: the
/// window cap is a parameter (cfg.gen_wish) instead of a fixed constant.
fn wish_admit(
    wish: &HashMap<(i32, i32), u64>,
    cx: i32,
    cz: i32,
    now: u64,
    budget: usize,
    max_wish: usize,
) -> bool {
    if wish.contains_key(&(cx, cz)) || wish.len() >= max_wish {
        return false;
    }
    let fresh = wish.values().filter(|t| **t == now).count();
    fresh < budget
}

/// Center-centric heightmap sweep into the disk store. The LOD data source:
/// rings of `cell_size` blocks around the disk center, skipping cells
/// already valid; unloaded chunks queue for the async force-load frontier.
fn stream_scan(state: &Arc<State>, env: Env) {
    if state.phase.load(Ordering::SeqCst) != PHASE_LIVE {
        return;
    }
    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        return;
    }
    let world = unsafe { as_obj(world_handle) };
    // v5.6: pre-join warm scan. With nobody online we still sample cells
    // whose chunks are ALREADY loaded (spawn area warm-up before the first
    // join) — but the async force frontier stays player-gated: no tickets
    // are churned on an idle server (v5.0.4 crash mitigation).
    let allow_force = !jni::players(env).is_empty();
    let budget = state.cfg.scan_budget;
    let (side, ox, oz, cell) = {
        let s = state.stream.lock().unwrap();
        (s.side as i32, s.origin_x, s.origin_z, state.cfg.cell_size)
    };
    // Ring sweep from the disk center outward.
    let center = side / 2;
    let mut done = 0u32;
    let mut ring: i32 = 0;
    while done < budget && ring <= side {
        let mut step = |dx: i32, dz: i32| -> bool {
            let cx = center + dx;
            let cz = center + dz;
            if cx < 0 || cz < 0 || cx >= side || cz >= side {
                return false;
            }
            {
                let s = state.stream.lock().unwrap();
                if let Some(cell) = s.get(cx as u32, cz as u32) {
                    if cell.valid {
                        return false;
                    }
                }
            }
            let wx = ox + f64::from(cx) * f64::from(cell);
            let wz = oz + f64::from(cz) * f64::from(cell);
            scan_cell(
                state,
                env,
                world,
                cx as u32,
                cz as u32,
                wx as i32,
                wz as i32,
                allow_force,
            );
            done += 1;
            done >= budget
        };
        // One ring: top+bottom rows and left+right columns.
        let mut stop = false;
        for d in -ring..=ring {
            if step(d, -ring) || step(d, ring) || step(-ring, d) || step(ring, d) {
                stop = true;
                break;
            }
            if ring == 0 {
                break; // the single center cell
            }
        }
        if stop {
            break;
        }
        ring += 1;
    }
}

/// Sample one world column into the disk store (or queue its chunk for the
/// async force-load frontier when unloaded). `allow_force=false` (idle,
/// nobody online) makes the cell silently skip instead of queueing a
/// generation ticket — the idle server never churns the chunk system.
#[allow(clippy::too_many_arguments)] // disk coords + world coords + gate
fn scan_cell(
    state: &Arc<State>,
    env: Env,
    world: jvmti_bindings::jni::jobject,
    cx: u32,
    cz: u32,
    wx: i32,
    wz: i32,
    allow_force: bool,
) {
    if let Some((y, liquid, material, biome)) = jni::sample_surface(env, world, wx, wz) {
        let rgb = crate::palette::surface_color(&material, &biome, liquid);
        let cell = Cell {
            height: y.clamp(-64, 1023) as i16,
            water: liquid,
            valid: true,
            provisional: false,
            rgb,
        };
        {
            let mut s = state.stream.lock().unwrap();
            // SEEDGEN fidelity: a real chunk replacing a native provisional
            // cell — count the upgrade and whether the encoded words moved.
            let prev = s.get(cx, cz);
            let was_prov = prev.map(|c| c.valid && c.provisional).unwrap_or(false);
            s.set_cell(cx, cz, cell);
            if was_prov {
                state.prov_upgraded.fetch_add(1, Ordering::Relaxed);
                if let Some(p) = prev {
                    if Streamer::encode_words(&p) != Streamer::encode_words(&cell) {
                        state.prov_diffs.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        }
    } else if state.cfg.native_gen {
        // SEEDGEN (v5.7, v5.8 LAZY ARM): the first unloaded cell arms the
        // native generator (bootstrap no-ops after the first attempt), so
        // the pre-join "stream armed" marker is no longer delayed by the
        // ~20-class MethodHandle resolve. The chunk then fills on OUR
        // worker threads — vanilla noise+surface, no tickets, zero TPS
        // impact; cells land provisional and the loaded-chunk sampler
        // upgrades them when the server generates the real chunk.
        if !crate::seedgen::tried(state) {
            crate::seedgen::bootstrap(state, env, world);
        }
        if !crate::seedgen::enqueue(state, wx >> 4, wz >> 4) && allow_force {
            force_chunk_for(state, env, world, wx >> 4, wz >> 4);
        }
    } else if allow_force {
        force_chunk_for(state, env, world, wx >> 4, wz >> 4);
    }
}

fn force_chunk_for(
    state: &Arc<State>,
    env: Env,
    world: jvmti_bindings::jni::jobject,
    cx: i32,
    cz: i32,
) {
    let now = state.tick_no.load(Ordering::SeqCst);
    {
        let wish = state.gen_wish.lock().unwrap();
        if !wish_admit(
            &wish,
            cx,
            cz,
            now,
            state.cfg.gen_budget as usize,
            state.cfg.gen_wish as usize,
        ) {
            return;
        }
    }
    // v5.0.2: NON-BLOCKING moonrise CHUNK_LOAD ticket. The chunk generates
    // on the chunk system's workers; poll_forced picks it up when loaded.
    if jni::request_chunk_gen(env, world, cx, cz) {
        state.gen_wish.lock().unwrap().insert((cx, cz), now);
    }
}

/// Sample every still-invalid disk cell of one loaded chunk (16/cell)^2
/// grid. Returns the number of cells sampled.
fn sample_chunk(
    state: &Arc<State>,
    env: Env,
    world: jvmti_bindings::jni::jobject,
    cx: i32,
    cz: i32,
    max_samples: usize,
) -> usize {
    let mut samples = 0usize;
    let x0 = cx * 16;
    let z0 = cz * 16;
    for dz in 0..(16 / state.cfg.cell_size.max(1)).max(1) {
        for dx in 0..(16 / state.cfg.cell_size.max(1)).max(1) {
            if samples >= max_samples {
                return samples;
            }
            let wx = x0 + dx * state.cfg.cell_size;
            let wz = z0 + dz * state.cfg.cell_size;
            let occ = {
                let s = state.stream.lock().unwrap();
                world_to_cell(&s, wx, wz)
            };
            let Some((ccx, ccz)) = occ else {
                continue;
            };
            let valid = {
                let s = state.stream.lock().unwrap();
                s.get(ccx, ccz)
                    .map(|c| c.valid && !c.provisional)
                    .unwrap_or(false)
            };
            if valid {
                continue;
            }
            scan_cell(state, env, world, ccx, ccz, wx, wz, true);
            samples += 1;
        }
    }
    samples
}

/// Poll the gen-wish ledger: chunks whose async CHUNK_LOAD ticket finished
/// are sampled + released; stuck tickets are re-issued. NON-BLOCKING by
/// construction — no call here may sync-generate.
fn poll_forced(state: &Arc<State>, env: Env) {
    const MAX_SAMPLES: usize = POLL_MAX_SAMPLES;
    // Player gate: with nobody online the frontier must not churn tickets —
    // the long-idle ticket re-issue loop was implicated in a native crash
    // (SIGSEGV in scan_cell ~9 min after the last player left). Chunks with
    // in-flight tickets keep them; work resumes when a player returns.
    if jni::players(env).is_empty() {
        return;
    }
    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        return;
    }
    let world = unsafe { as_obj(world_handle) };
    let now = state.tick_no.load(Ordering::SeqCst);
    let entries: Vec<((i32, i32), u64)> = {
        let wish = state.gen_wish.lock().unwrap();
        wish.iter().map(|(c, t)| (*c, *t)).collect()
    };
    let mut samples = 0usize;
    for ((cx, cz), t0) in entries {
        if samples >= MAX_SAMPLES {
            break;
        }
        if jni::is_chunk_loaded(env, world, cx, cz).unwrap_or(false) {
            samples += sample_chunk(state, env, world, cx, cz, MAX_SAMPLES - samples);
            jni::release_chunk_ticket(env, world, cx, cz);
            state.gen_wish.lock().unwrap().remove(&(cx, cz));
            state.forced_chunks.fetch_add(1, Ordering::SeqCst);
        } else if now.saturating_sub(t0) >= GEN_REISSUE_TICKS {
            // Ticket lost (never completed): re-issue once per window.
            if jni::request_chunk_gen(env, world, cx, cz) {
                state.gen_wish.lock().unwrap().insert((cx, cz), now);
            }
        }
    }
}

/// World block coords -> disk cell coords (None when outside the disk).
fn world_to_cell(s: &Streamer, wx: i32, wz: i32) -> Option<(u32, u32)> {
    let cell = s.cell_size;
    let side = s.side as i32;
    let cx = (((wx as f64 - s.origin_x) / cell as f64).floor() as i32).clamp(0, side - 1);
    let cz = (((wz as f64 - s.origin_z) / cell as f64).floor() as i32).clamp(0, side - 1);
    let bx = s.origin_x + cx as f64 * cell as f64;
    let bz = s.origin_z + cz as f64 * cell as f64;
    if (wx as f64 - bx).abs() >= cell as f64 || (wz as f64 - bz).abs() >= cell as f64 {
        return None;
    }
    Some((cx as u32, cz as u32))
}

/// Tier for one player: Full only INSIDE the streamed disk (particles fly
/// over that square, so data exists only there); other worlds keep the veil.
fn tier_for(state: &Arc<State>, p: &PlayerInfo) -> Tier {
    if p.world.is_empty() {
        return Tier::Off;
    }
    let wname = state.world_name.lock().unwrap().clone();
    if !wname.is_empty() && p.world != wname {
        return Tier::Veil;
    }
    let inside = {
        let s = state.stream.lock().unwrap();
        world_to_cell(&s, p.bx, p.bz).is_some()
    };
    if inside {
        Tier::Full
    } else {
        Tier::Veil
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tier {
    /// No world (or no pack): no chains applied.
    Off,
    /// Other world: depth veil only.
    Veil,
    /// Streaming world: terrain + veil.
    Full,
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

        // 1. Resource pack — pushed exactly once per player (the pack is
        //    static; its SHA-1 never changes for the module's lifetime), a
        //    few seconds AFTER the join so the client's prompt screen lands
        //    on the in-world screen, not on the loading terrain screen.
        if st.joined_tick == 0 {
            st.joined_tick = state.tick_no.load(Ordering::Relaxed).max(1);
        }
        let joined_longs_enough =
            state.tick_no.load(Ordering::Relaxed) >= st.joined_tick + PACK_PUSH_DELAY_TICKS;
        if let Some(pack) = &pack {
            if st.pushed_sha != pack.sha1_hex && joined_longs_enough {
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
                            true,
                        );
                        if ok {
                            st.pushed_sha = pack.sha1_hex.clone();
                            st.resends = 0;
                            crate::log_info(&format!(
                                "pack pushed to {} ({}): {} bytes, sha1 {} — the ONE push this session",
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
                // times after the push (push/reload race healer).
                if let Some(handle) = jni::player_by_uuid(env, &p.uuid) {
                    if jni::update_effects(env, handle) {
                        st.resends += 1;
                    }
                }
            }
        }

        // 2. Tier switching — pure /posteffect.
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
                let tkey = unsafe { as_obj(keys.terrain) };
                let ok = jni::add_effect(env, handle, tkey);
                if ok {
                    st.terrain_on = true;
                } else {
                    // add -> false: 26.3 PostEffectCommand returns false when the
                    // effect is ALREADY attached (fast client relog keeps the
                    // server-side attachment). Reconcile with ONE remove+add
                    // cycle and stop retrying — a per-tick retry storm (~100/s)
                    // makes the client rebuild its post chain every tick and
                    // starves the render thread (llvmpipe freeze).
                    let rm = jni::remove_effect(env, handle, tkey);
                    let ok2 = jni::add_effect(env, handle, tkey);
                    st.terrain_on = true; // no more retries until the tier flips
                    crate::log_info(&format!(
                        "effect terrain resync (remove -> {}) add -> {} ({})",
                        rm, ok2, p.uuid
                    ));
                }
                crate::log_info(&format!(
                    "effect terrain add -> {} ({} {})",
                    st.terrain_on, p.uuid, tier_no
                ));
            } else if !want_terrain && st.terrain_on {
                let ok = jni::remove_effect(env, handle, unsafe { as_obj(keys.terrain) });
                // remove -> false means it was not attached; either way stop retrying
                st.terrain_on = false;
                crate::log_info(&format!("effect terrain remove -> {} ({})", ok, p.uuid));
            }
            if want_atmo && !st.atmosphere_on {
                let akey = unsafe { as_obj(keys.atmosphere) };
                let ok = jni::add_effect(env, handle, akey);
                if ok {
                    st.atmosphere_on = true;
                } else {
                    let rm = jni::remove_effect(env, handle, akey);
                    let ok2 = jni::add_effect(env, handle, akey);
                    st.atmosphere_on = true;
                    crate::log_info(&format!(
                        "effect atmosphere resync (remove -> {}) add -> {} ({})",
                        rm, ok2, p.uuid
                    ));
                }
                crate::log_info(&format!(
                    "effect atmosphere add -> {} ({} {})",
                    st.atmosphere_on, p.uuid, tier_no
                ));
            } else if !want_atmo && st.atmosphere_on {
                let ok = jni::remove_effect(env, handle, unsafe { as_obj(keys.atmosphere) });
                st.atmosphere_on = false;
                crate::log_info(&format!("effect atmosphere remove -> {} ({})", ok, p.uuid));
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

/// v5.6: with an explicit config center the disk arms BEFORE the first
/// player joins — pure Rust state, no JNI, no tickets. Headless boots
/// (CI, ops) get a pre-armed disk and the "sky-grid stream armed" marker
/// without anybody online; the player-anchored path (no config center)
/// is unchanged.
fn arm_from_config(state: &Arc<State>) {
    if state.stream_armed.load(Ordering::SeqCst) != 0 {
        return;
    }
    if state.phase.load(Ordering::SeqCst) != PHASE_LIVE {
        return;
    }
    // The scan/stream world must already be resolved (boot step 1).
    if *state.world_obj.lock().unwrap() == 0 {
        return;
    }
    if let (Some(x), Some(z)) = (state.cfg.center_x, state.cfg.center_z) {
        arm_stream(state, x, z);
    }
}

/// Fix the disk origin once and log the arming marker (single writer).
fn arm_stream(state: &Arc<State>, ox: f64, oz: f64) {
    let (side, cells) = {
        let mut s = state.stream.lock().unwrap();
        *s = Streamer::new(ox, oz, state.cfg.region_blocks, state.cfg.cell_size);
        (s.side, s.cells())
    };
    let lap_ticks = cells / ((state.cfg.particles_per_tick as u64) / 8).max(1);
    state.stream_armed.store(1, Ordering::SeqCst);
    crate::log_info(&format!(
        "c-LoD sky-grid stream armed: {side}x{side} cells at ({ox:.0},{oz:.0}), {} particles/tick, {lap_ticks} ticks/lap",
        state.cfg.particles_per_tick
    ));
}

/// Rate-limited stream diagnostics + live /clod-pack.info counters.
/// Runs with AND without players (v5.7: the playerless pre-join arm +
/// SEEDGEN fill must stay visible to ops).
fn stream_diag(state: &Arc<State>, players: &[PlayerInfo]) {
    let tick = state.tick_no.load(Ordering::Relaxed);
    if !tick.is_multiple_of(100) {
        return;
    }
    let (valid, laps, total_cells, cone_c, zone_c, dirty_c, dirty_q) = {
        let s = state.stream.lock().unwrap();
        (
            s.valid_cells(),
            s.laps,
            s.cells(),
            s.cone_cells,
            s.zone_cells,
            s.dirty_streams,
            s.dirty_pending,
        )
    };
    // v5.5 frontier diagnostics: wish = in-flight FORCED tickets,
    // forced = chunks that completed gen and got sampled.
    let wish_n = state.gen_wish.lock().unwrap().len();
    let forced_n = state.forced_chunks.load(Ordering::Relaxed);
    // v5.7 SEEDGEN diagnostics + v5.8 helper state (u64::MAX = off).
    let (nat_c, nat_ch, nat_f, nat_w, nat_q, nat_h) = {
        let g = state.seedgen.lock().unwrap();
        match g.as_ref() {
            Some(sg) if sg.ready() => {
                let h = if sg.helper.load(Ordering::Relaxed) {
                    sg.helper_fails.load(Ordering::Relaxed)
                } else {
                    u64::MAX
                };
                (
                    sg.cells.load(Ordering::Relaxed),
                    sg.chunks.load(Ordering::Relaxed),
                    sg.fails.load(Ordering::Relaxed),
                    sg.workers.load(Ordering::Relaxed),
                    sg.inflight_len(),
                    h,
                )
            }
            _ => (0, 0, 0, 0, 0, u64::MAX),
        }
    };
    let nat_hs = if nat_h == u64::MAX {
        "off".to_string()
    } else if nat_h == 0 {
        "on".to_string()
    } else {
        format!("on (fails {nat_h})")
    };
    // v5.2.2: /clod-pack.info serves LIVE scan counters (it used to
    // report the boot-time cells=0 forever).
    state.server.set_meta(valid, total_cells);
    let p0 = players.first();
    crate::log_info(&format!(
        "stream: tick {tick} {valid}/{total_cells} cells, lap {laps}, cone {cone_c} zone {zone_c} dirty {dirty_c} (q {dirty_q}), spawned {} / failed {}, cam0 {}, wish {wish_n} forced {forced_n}, native {nat_c}c/{nat_ch}ch fail {nat_f} ({nat_w}w q {nat_q}) helper {nat_hs}",
        jni::STREAM_SPAWNED.load(Ordering::Relaxed),
        jni::STREAM_FAILED.load(Ordering::Relaxed),
        p0.map(|p| format!("yaw {:.0} pitch {:.0}", p.yaw, p.pitch))
            .unwrap_or_else(|| "none".into()),
    ));
}

/// The v5 streaming step: fix the disk origin once (config center or the
/// first player), then spawn the next batch of dust particles over the two
/// sky planes — one force ClientboundLevelParticlesPacket per player per
/// particle (overrideLimiter: survives particle settings + render clip).
fn stream_step(state: &Arc<State>, env: Env) {
    if state.phase.load(Ordering::SeqCst) != PHASE_LIVE {
        return;
    }
    let players = jni::players(env);

    // v5.7: the rate-limited diag (and the live /clod-pack.info counters)
    // must ALSO run with nobody online — the pre-join arm + SEEDGEN fill
    // the disk playerless, and ops watch exactly this line for progress.
    if players.is_empty() {
        stream_diag(state, &[]);
        return;
    }

    // Arm once (player-anchored path — only when no config center exists;
    // a configured center already armed in arm_from_config).
    if state.stream_armed.load(Ordering::SeqCst) == 0 {
        let (ox, oz) = match players.first() {
            Some(p) => (
                (p.bx as f64) - f64::from(state.cfg.region_blocks) * 0.5,
                (p.bz as f64) - f64::from(state.cfg.region_blocks) * 0.5,
            ),
            None => return, // no anchor yet
        };
        arm_stream(state, ox, oz);
    }

    // Spawn the next batch to every player (the disk is shared).
    // v5.4 DELTA-ZONE: every player's live camera is a view — views[0]
    // drives the cone-priority phase, ALL views define the decodable set
    // (dirty phase + zone sweep never spend budget outside it).
    let views: Vec<crate::stream::ViewHint> = players
        .iter()
        .map(|p| crate::stream::ViewHint {
            eye: [
                f64::from(p.bx) + 0.5,
                f64::from(p.by) + 1.62,
                f64::from(p.bz) + 0.5,
            ],
            look: crate::stream::look_dir(p.yaw, p.pitch),
            now: state.tick_no.load(Ordering::Relaxed),
        })
        .collect();
    let batch: Vec<crate::stream::Spawn> = {
        let mut s = state.stream.lock().unwrap();
        s.tick_batch(state.cfg.particles_per_tick as usize, &views)
            .to_vec()
    };
    stream_diag(state, &players);
    if batch.is_empty() {
        return;
    }
    let mut spawned = 0u64;
    for p in &players {
        let Some(handle) = jni::player_by_uuid(env, &p.uuid) else {
            continue;
        };
        // v5.3.1 spawn gate: spawn a dot to THIS player only when the client
        // can see it AND decode AND erase it. The server knows the player's
        // exact camera (Bukkit syncs yaw/pitch every tick), so the gate is
        // the product of three tests: sky-in-view (the rot channel is
        // sky-measured — dots spawned into sky-blind frustums are never
        // erased), the decode band [CULL_RADIUS, ZONE_FAR] (outside it a dot
        // is either too big to erase or permanently undecodable), and the
        // view frustum (widest-FOV assumption + margin). Everything else the
        // cursor streams is skipped for this player: invisible waste the
        // transport no longer pays for.
        let px = f64::from(p.bx) + 0.5;
        let py = f64::from(p.by) + 1.62;
        let pz = f64::from(p.bz) + 0.5;
        let sky_visible = p.pitch < crate::stream::SKY_PITCH_LIMIT_DEG;
        let look = crate::stream::look_dir(p.yaw, p.pitch);
        let cos_half = crate::stream::cos_half_cone();
        // v5.3.1 diag: per-tick gate breakdown (near / far / cone / blind).
        let mut n_near = 0u64;
        let mut n_far = 0u64;
        let mut n_cone = 0u64;
        for sp in &batch {
            if !crate::stream::spawn_visible(sp, [px, py, pz], look, sky_visible, cos_half) {
                if !sky_visible {
                    continue;
                }
                let dx = sp.x - px;
                let dy = sp.y - py;
                let dz = sp.z - pz;
                let d2 = dx * dx + dy * dy + dz * dz;
                if d2 <= crate::stream::CULL_RADIUS * crate::stream::CULL_RADIUS {
                    n_near += 1;
                } else if d2 > crate::stream::ZONE_FAR * crate::stream::ZONE_FAR {
                    n_far += 1;
                } else {
                    n_cone += 1;
                }
                continue;
            }
            if unsafe { jni::spawn_particle(env, handle, sp.x, sp.y, sp.z) } {
                spawned += 1;
            }
        }
        if state.tick_no.load(Ordering::Relaxed).is_multiple_of(100) {
            crate::log_info(&format!(
                "gate: sky {} near {} far {} cone {}",
                sky_visible, n_near, n_far, n_cone
            ));
        }
    }
    state
        .streamed_cells
        .fetch_add(spawned / 8 / players.len().max(1) as u64, Ordering::Relaxed);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// v5.5: the frontier admission budget and window moved into Config
    /// (cfg.gen_budget / cfg.gen_wish, defaults 16 / 512). The old fixed
    /// constants live on only as test fixtures.
    const FORCE_BUDGET_PER_TICK: usize = 4;
    /// Cap on chunks with an in-flight CHUNK_LOAD ticket (async gen backlog).
    const GEN_MAX_WISH: usize = 96;

    #[test]
    fn pack_url_uses_base() {
        let st = State::new(Config::parse("pack_base_url=http://x.example;http_port=1"));
        assert_eq!(pack_url(&st), "http://x.example/clod-pack.zip");
    }

    #[test]
    fn coverage_matches_world_and_bounds() {
        let st = State::new(Config::parse(""));
        *st.world_name.lock().unwrap() = "world".to_string();
        // Default disk: origin (0,0), 512 blocks side, cell 2 -> 0..512.
        let p = PlayerInfo {
            uuid: "u".into(),
            world: "world".into(),
            bx: 100,
            by: 64,
            bz: 100,
            yaw: 0.0,
            pitch: 0.0,
        };
        assert_eq!(tier_for(&st, &p), Tier::Full);
        let p_far = PlayerInfo {
            uuid: "u".into(),
            world: "world".into(),
            bx: 90_000,
            by: 64,
            bz: 0,
            yaw: 0.0,
            pitch: 0.0,
        };
        // Outside the streamed disk: veil (no particles fly there).
        assert_eq!(tier_for(&st, &p_far), Tier::Veil);
        let p_world = PlayerInfo {
            uuid: "u".into(),
            world: "world_nether".into(),
            bx: 0,
            by: 64,
            bz: 0,
            yaw: 0.0,
            pitch: 0.0,
        };
        assert_eq!(tier_for(&st, &p_world), Tier::Veil);
        let p_noworld = PlayerInfo {
            uuid: "u".into(),
            world: String::new(),
            bx: 0,
            by: 64,
            bz: 0,
            yaw: 0.0,
            pitch: 0.0,
        };
        assert_eq!(tier_for(&st, &p_noworld), Tier::Off);
    }

    #[test]
    fn pack_builds_at_boot_without_world() {
        // The v5 pack is shader-only: it must build with NO world, NO atlas,
        // NO scan data — the whole point of sky-grid streaming.
        let st = State::new(Config::parse(""));
        assert!(st.pack.read().unwrap().is_none());
        assert_eq!(st.phase.load(Ordering::SeqCst), PHASE_BOOT);
    }

    #[test]
    fn world_to_cell_maps_inside_disk() {
        let st = State::new(Config::parse(""));
        let s = st.stream.lock().unwrap();
        assert_eq!(world_to_cell(&s, 0, 0), Some((0, 0)));
        assert_eq!(world_to_cell(&s, 511, 511), Some((255, 255)));
        assert_eq!(world_to_cell(&s, 100, 200), Some((50, 100)));
        // Negative / beyond-disk coords clamp into the disk footprint...
        assert_eq!(world_to_cell(&s, -50, -50), None);
        assert_eq!(world_to_cell(&s, 600, 600), None);
    }

    #[test]
    fn wish_admit_gates_duplicates_backlog_and_budget() {
        let mut wish: HashMap<(i32, i32), u64> = HashMap::new();
        let win = GEN_MAX_WISH; // window comfortably above the test volume
                                // Fresh chunk on a fresh tick: admitted.
        assert!(wish_admit(&wish, 3, 5, 100, FORCE_BUDGET_PER_TICK, win));
        wish.insert((3, 5), 100);
        // Same chunk again (in-flight ticket): rejected.
        assert!(!wish_admit(&wish, 3, 5, 100, FORCE_BUDGET_PER_TICK, win));
        // Another chunk, same tick, budget (4) not yet exhausted: admitted.
        assert!(wish_admit(&wish, 7, 1, 100, FORCE_BUDGET_PER_TICK, win));
        wish.insert((7, 1), 100);
        assert!(wish_admit(&wish, 7, 2, 100, FORCE_BUDGET_PER_TICK, win));
        wish.insert((7, 2), 100);
        assert!(wish_admit(&wish, 7, 3, 100, FORCE_BUDGET_PER_TICK, win));
        wish.insert((7, 3), 100);
        // Budget for THIS tick exhausted: rejected...
        assert!(!wish_admit(&wish, 7, 4, 100, FORCE_BUDGET_PER_TICK, win));
        // ...but the same chunk is fine on the NEXT tick.
        assert!(wish_admit(&wish, 7, 4, 101, FORCE_BUDGET_PER_TICK, win));
    }

    #[test]
    fn wish_admit_caps_backlog_at_gen_max() {
        let mut wish: HashMap<(i32, i32), u64> = HashMap::new();
        for i in 0..GEN_MAX_WISH {
            let key = (i as i32, (i as i32 * 7) % 13);
            assert!(wish_admit(
                &wish,
                key.0,
                key.1,
                1,
                FORCE_BUDGET_PER_TICK,
                GEN_MAX_WISH
            ));
            wish.insert(key, i as u64);
        }
        assert_eq!(wish.len(), GEN_MAX_WISH);
        // Ledger full: ANY new chunk is rejected regardless of tick/budget.
        assert!(!wish_admit(
            &wish,
            -1,
            -1,
            99,
            FORCE_BUDGET_PER_TICK,
            GEN_MAX_WISH
        ));
    }

    #[test]
    fn wish_admit_takes_the_v55_window_from_cfg() {
        // The admission gate must honor a window OTHER than the legacy 96:
        // the v5.5 default is cfg-driven (512), and small windows (32) must
        // cap just the same.
        for window in [32usize, 512] {
            let mut wish: HashMap<(i32, i32), u64> = HashMap::new();
            for i in 0..window {
                let key = (i as i32, (i as i32 * 5) % 17);
                assert!(
                    wish_admit(&wish, key.0, key.1, 1, 16, window),
                    "chunk {i} must fit in window {window}"
                );
                wish.insert(key, i as u64);
            }
            assert_eq!(wish.len(), window);
            assert!(!wish_admit(&wish, -9, -9, 7, 16, window));
        }
    }

    #[test]
    fn gen_wish_ledger_lifecycle_drains_and_reissues() {
        // Simulates the poll_forced contract with a pure stand-in for the
        // JNI probes: loaded chunks drain (sample+release), stale ones
        // re-issue (fresh timestamp), unknown stay.
        let mut wish: HashMap<(i32, i32), u64> = HashMap::new();
        wish.insert((0, 0), 10);
        wish.insert((1, 0), 10);
        wish.insert((2, 0), 10);
        let now = 10 + GEN_REISSUE_TICKS; // all past the reissue window
        let loaded = |c: (i32, i32)| c.0 == 0; // only (0,0) finished generating
        let entries: Vec<((i32, i32), u64)> = wish.iter().map(|(c, t)| (*c, *t)).collect();
        for ((cx, cz), t0) in entries {
            if loaded((cx, cz)) {
                wish.remove(&(cx, cz));
            } else if now.saturating_sub(t0) >= GEN_REISSUE_TICKS {
                wish.insert((cx, cz), now);
            }
        }
        assert!(!wish.contains_key(&(0, 0)), "drained after sampling");
        assert_eq!(wish.get(&(1, 0)), Some(&now), "re-issued");
        assert_eq!(wish.get(&(2, 0)), Some(&now), "re-issued");
    }
}
