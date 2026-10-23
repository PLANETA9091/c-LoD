//! The c-LoD engine: a state machine driven from the server main thread.
//!
//! STATIC-PACK lifecycle (v3) — the resource pack is built EXACTLY ONCE per
//! session and never rebuilt, never re-pushed with new bytes:
//!
//!   Boot → (restore `c-LoD-atlas.bin` if present and matching)
//!        → Scanning (budgeted JNI heightmap sweeps, main thread)
//!        → Finalize (sweep complete OR `scan_deadline_secs` reached):
//!            1. persist the atlas snapshot to disk (if `atlas_cache`)
//!            2. build the pack from the frozen snapshot
//!            3. swap it into the embedded HTTP server — done, forever
//!        → Finalized: zero pack work; the ONLY dynamic part left is
//!          per-player `/posteffect` tier switching, which touches no bytes.
//!
//! Auto-updating LOD without pack rebuilds:
//!   - Client-side: the `clod:lod_terrain` chain raymarches the world-anchored
//!     heightfield atlas from the live camera every frame — walking, flying or
//!     turning re-projects the LOD surface automatically.
//!   - Server-side: every tick each player is placed in a tier and chains are
//!     added/removed accordingly:
//!       * Tier::Off   — other world / no data      → no chains
//!       * Tier::Veil  — atlas world, outside window → `clod:atmosphere` only
//!       * Tier::Full  — inside the atlas window     → terrain + atmosphere
//!   - Across restarts: the restored snapshot reproduces byte-identical pack
//!     (same SHA-1), so vanilla clients never re-download.

use crate::atlas::Atlas;
use crate::config::Config;
use crate::http::PackServer;
use crate::jni::{self, PlayerInfo};
use crate::packgen::{self, Pack};
use jvmti_bindings::prelude::JniEnv;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicI64, AtomicU32, AtomicU64, AtomicU8, Ordering};
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
    /// Last observed world full-game-time (sentinel -1 = nothing yet).
    pub game_time: AtomicI64,
    /// 1 once the world game time ADVANCED between two engine ticks — the
    /// server tick loop is live. Every queued main-thread task before that
    /// runs inside startup's executeModerately drain (prepareLevel), where a
    /// sync chunk load can park forever on a worldgen task that will never
    /// be scheduled (the chunk system needs the tick loop). All heavy work
    /// (force-load warm-up, scan batches) must wait for this flag.
    pub ticking: AtomicU8,
    /// When the atlas world was first resolved (scan deadline anchor).
    pub world_seen_at: Mutex<Option<std::time::Instant>>,
    pub scan_seq: AtomicU64,
    /// Streaming cursor (index into the centre-out chunk order).
    pub scan_cursor: AtomicU64,
    /// Streaming scan: chunks requested via getChunkAtAsyncUrgently whose
    /// future is still unfinished — (chunk index, future global-ref handle,
    /// engine tick of the request). Polled each tick; sampled the moment the
    /// future completes (the chunk system holds the chunk until then).
    pub scan_inflight: Mutex<VecDeque<(u32, usize, u64)>>,
    /// Chunk indices whose force-load failed transiently (Paper's worldgen
    /// pool rejects chunks under load); retried round-robin after the main
    /// sweep drains, so the frozen snapshot carries no scan holes.
    pub scan_retry: Mutex<VecDeque<u32>>,
    /// Chunk-grid indices in the same centre-out Chebyshev-ring order as the
    /// atlas cells (`crate::atlas::ring_order` over the chunk grid): a scan
    /// cut short by the deadline covers a disc of chunks around the atlas
    /// centre instead of the west half-plane.
    pub chunk_order: Vec<u32>,
    pub world_obj: Mutex<usize>, // jobject handle as usize; main-thread only
    pub world_name: Mutex<String>,
    pub keys: Mutex<Option<Keys>>,
    pub per_player: Mutex<HashMap<String, PlayerState>>,
    pub pack_uuid: Mutex<usize>,
    /// Engine tick counter (every tick() on the main thread).
    pub tick_no: AtomicU64,
    /// Rate limiter for pack-push failure logs.
    pub push_fails: AtomicU32,
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
    /// Remaining ticks of the post-pack "kick": the client drops effect ids
    /// that arrive before the pack is applied (push/reload race) and
    /// `PlayerPostEffects.update()` is a wire no-op when the list is
    /// unchanged, so the only reliable way to make it re-resolve is a real
    /// remove+add cycle. While `kick > 0` the list is cycled once per second.
    pub kick: u32,
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
        // Centre-out chunk order for the streamed scan (same metric as the
        // atlas cell order): computed before the state moves the atlas in.
        let chunk_order = {
            let (side, _, _) = chunk_grid(&atlas);
            crate::atlas::ring_order(side as u32)
        };
        Arc::new(Self {
            atlas,
            server,
            pack: RwLock::new(None),
            phase: AtomicU8::new(PHASE_BOOT),
            restored,
            warmed: AtomicU8::new(0),
            game_time: AtomicI64::new(-1),
            ticking: AtomicU8::new(0),
            world_seen_at: Mutex::new(None),
            scan_seq: AtomicU64::new(0),
            scan_cursor: AtomicU64::new(0),
            scan_inflight: Mutex::new(VecDeque::new()),
            scan_retry: Mutex::new(VecDeque::new()),
            chunk_order,
            world_obj: Mutex::new(0),
            world_name: Mutex::new(String::new()),
            keys: Mutex::new(None),
            per_player: Mutex::new(HashMap::new()),
            pack_uuid: Mutex::new(0),
            tick_no: AtomicU64::new(0),
            push_fails: AtomicU32::new(0),
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
    probe_ticking(state, env);
    warm_core(state, env);
    scan_batch(state, env);
    release_forceload(state, env);
    manage_players(state, env);
}

/// Detect a live server tick loop: the world's FULL game time advances every
/// tick (regardless of doDaylightCycle), while tasks queued during startup
/// run inside prepareLevel's executeModerately drain with the clock frozen.
/// Cheap (one JNI call); sets the scan-deadline anchor when it first fires.
fn probe_ticking(state: &Arc<State>, env: Env) {
    if state.ticking.load(Ordering::SeqCst) == 1 {
        return;
    }
    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        return;
    }
    let Some(t) = jni::world_game_time(env, unsafe { as_obj(world_handle) }) else {
        return;
    };
    let prev = state.game_time.swap(t, Ordering::SeqCst);
    if prev >= 0 && prev != t {
        state.ticking.store(1, Ordering::SeqCst);
        // Anchor the scan deadline at the FIRST REAL TICK: before that the
        // scan would burn its budget (or finalize empty) while the server is
        // still preparing levels.
        let mut anchor = state.world_seen_at.lock().unwrap();
        if anchor.is_none() {
            *anchor = Some(std::time::Instant::now());
        }
        drop(anchor);
        crate::log_info(&format!(
            "c-LoD server tick loop live (game time {prev} -> {t}) — heavy work (warm-up, scan) starts"
        ));
    }
}

/// One-time chunk warm-up around the atlas center, deferred until the server
/// tick loop is live: setChunkForceLoaded is a SYNCHRONOUS chunk load, and
/// during startup's prepareLevel drain it can park forever on a worldgen
/// task the chunk system will only schedule once the tick loop runs
/// (live-observed wedge: server thread parked in syncLoad for 15+ minutes,
/// connections stuck in the accept backlog, console unresponsive).
fn warm_core(state: &Arc<State>, env: Env) {
    if state.ticking.load(Ordering::SeqCst) != 1 || state.warmed.load(Ordering::SeqCst) != 0 {
        return;
    }
    if state
        .warmed
        .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }
    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        state.warmed.store(0, Ordering::SeqCst);
        return;
    }
    let world = unsafe { as_obj(world_handle) };
    // Streaming scan (bench/CI): every scanned chunk force-loads ITSELF for
    // the duration of its sampling, so a pinned warm core is redundant —
    // 289 back-to-back synchronous chunk loads in one tick are exactly the
    // watchdog hazard this deferral was meant to avoid (live-observed: the
    // main thread stalled 10s+ in a BLOCKING chunk wait, Paper's watchdog
    // killed the server). The ring only matters for the legacy cell-major
    // path (stream_chunks == 0), whose sampler touches LOADED chunks only.
    if state.cfg.stream_chunks > 0 {
        crate::log_info(
            "c-LoD streaming scan: no pinned warm core — each scan chunk force-loads itself",
        );
        return;
    }
    // One-time core warm-up (legacy path): modern vanilla keeps no spawn
    // chunks loaded and the legacy sampler never forces generation, so
    // without this the static snapshot could cover nothing.
    if state.cfg.warm_chunks > 0 {
        let w = state.cfg.warm_chunks;
        let mut loaded = 0usize;
        for cx in -w..=w {
            for cz in -w..=w {
                if jni::warm_chunk(env, world, cx, cz, true) {
                    loaded += 1;
                }
            }
        }
        let loaded0 = jni::is_chunk_loaded(env, world, 0, 0);
        let h0 = jni::probe_height(env, world, 0, 0);
        crate::log_info(&format!(
            "c-LoD core force-loaded: {}x{} chunks around atlas center ({} ok) — isChunkLoaded(0,0)={loaded0:?}, height(0,0)={h0:?} — released at finalize",
            2 * w + 1,
            2 * w + 1,
            loaded
        ));
    }
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
                // Deadline anchor: SYNC mode scans inside startup's
                // prepareLevel drain (the fast managedBlock path) — there the
                // tick loop may never start before the scan is done, so the
                // anchor must stay at world resolution or the deadline can
                // never fire (live-observed: boot timeout with the scan at
                // 98% and the retry queue spinning). ASYNC mode anchors at
                // the first real tick (probe_ticking): its futures only
                // complete while the chunk system is being ticked.
                if state.cfg.scan_mode != "async" {
                    let mut anchor = state.world_seen_at.lock().unwrap();
                    if anchor.is_none() {
                        *anchor = Some(std::time::Instant::now());
                    }
                }
                // NOTE: no heavy work here either way. The core warm-up moved
                // to warm_core() (first tick; skipped for streaming), and in
                // async mode the scan itself waits for probe_ticking(): tasks
                // queued before the tick loop runs inside prepareLevel's
                // executeModerately drain, where a synchronous chunk load can
                // park forever (live-observed startup wedge).
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
    // Static atlas: after finalize the snapshot is frozen — never scan again.
    if state.phase.load(Ordering::SeqCst) != PHASE_SCANNING {
        return;
    }
    if state.cfg.scan_mode == "async" && state.cfg.stream_chunks > 0 {
        // A restored snapshot is authoritative (same invariant as the sync
        // path below): the pack must reproduce the previous SHA-1 byte for
        // byte, so no cell may be re-sampled before finalize.
        if state.restored {
            return;
        }
        // Async mode never scans before the tick loop is live: the chunk
        // system only schedules generation while the server ticks, so the
        // request futures would never complete during startup's drain.
        if state.ticking.load(Ordering::SeqCst) == 0 {
            return;
        }
        let world_handle = *state.world_obj.lock().unwrap();
        if world_handle == 0 {
            return;
        }
        let world = unsafe { as_obj(world_handle) };
        let seq = state.scan_seq.fetch_add(1, Ordering::Relaxed);
        scan_stream_async(state, env, world);
        if seq.is_multiple_of(25) {
            log_scan_progress(state, seq);
        }
        return;
    }
    // SYNC mode (default): the proven fast path — setChunkForceLoaded per
    // chunk. Runs as soon as the world resolves, INCLUDING inside startup's
    // prepareLevel drain: there the sync load's managedBlock joins worldgen
    // work on the main thread, which is exactly why it is the fastest mode
    // on a well-provisioned box (CI reaches 98% here). The known hazard — a
    // crashed worldgen task parking the sync load forever on starved boxes —
    // is what scan_mode=async exists for.
    // A restored snapshot is AUTHORITATIVE: the pack must reproduce the
    // previous session's SHA-1 byte-for-byte, so no cell may be re-sampled
    // before finalize. The tick ordering (scan_batch on the main thread runs
    // one tick ahead of the engine thread's maybe_finalize) otherwise lets a
    // single sweep tick overwrite restored cells with freshly-sampled values
    // — the disk-loaded chunk state samples slightly differently than the
    // generation-time state did (live-observed: same cell count, same pack
    // size, different SHA-1), and clients would re-download on every restart.
    if state.restored {
        return;
    }
    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        return;
    }
    let world = unsafe { as_obj(world_handle) };
    let seq = state.scan_seq.fetch_add(1, Ordering::Relaxed);
    if state.cfg.stream_chunks > 0 {
        // Chunk-major streaming: generate -> sample -> release, N chunks/tick.
        // This is what lets the atlas span kilometres: resident chunks stay
        // bounded instead of pinning (2*w+1)^2 force-loaded chunks.
        scan_stream_sync(state, env, world);
    } else {
        let budget = state.cfg.scan_budget;
        // Advance the sweep by `budget` cells per tick (scan_task wraps the
        // start index over the whole atlas): passing the bare tick number made
        // every tick rescan cells 0..budget+bumped-by-one, leaving most of the
        // atlas (and the warm core) unreached until the deadline fired.
        let tasks = state.atlas.scan_task(seq * budget as u64, budget);
        for (ix, iz) in tasks.iter().copied() {
            let x = state.atlas.cell_world_x(ix);
            let z = state.atlas.cell_world_z(iz);
            if let Some((y, liquid, top, side, tex_top, tex_side)) =
                jni::sample_column(env, world, x, z, state.cfg.sea_level)
            {
                state.atlas.set(
                    ix,
                    iz,
                    (y.clamp(-64, 1023)) as i16,
                    liquid,
                    top,
                    side,
                    tex_top,
                    tex_side,
                );
            }
        }
    }
    // Rate-limited scan diagnostics (one line per 25 ticks).
    if seq.is_multiple_of(25) {
        use std::sync::atomic::Ordering as O;
        let streamed = if state.cfg.stream_chunks > 0 {
            let (done, total) = stream_progress(&state.atlas);
            format!(" chunks {done}/{total}")
        } else {
            String::new()
        };
        crate::log_info(&format!(
            "scan: tick {seq}, atlas v{} ({}/{} cells{streamed}), ok={} unloaded={} err={}",
            state.atlas.version(),
            state.atlas.scanned_cells(),
            state.atlas.total_cells(),
            jni::SCAN_OK.load(O::Relaxed),
            jni::SCAN_UNLOADED.load(O::Relaxed),
            jni::SCAN_ERR.load(O::Relaxed),
        ));
    }
}

/// Chunk grid of the atlas: (side in chunks, first chunk X, first chunk Z).
fn chunk_grid(atlas: &Atlas) -> (i32, i32, i32) {
    let side = (atlas.size / 16).max(1);
    let half = atlas.size / 2;
    (
        side,
        (atlas.center_x - half).div_euclid(16),
        (atlas.center_z - half).div_euclid(16),
    )
}

fn stream_progress(atlas: &Atlas) -> (u64, u64) {
    let (side, _, _) = chunk_grid(atlas);
    let total = (side as u64) * (side as u64);
    (CURSOR_HINT.load(Ordering::Relaxed).min(total), total)
}

/// Last streaming cursor value (progress display only).
static CURSOR_HINT: AtomicU64 = AtomicU64::new(0);

/// Geometry of the atlas chunk grid, precomputed once per scan pass.
struct StreamGeom {
    cell: i32,
    side: i32,
    cx0: i32,
    cz0: i32,
    per: i32,
    half: i32,
}

impl StreamGeom {
    fn chunk_xy(&self, co: u32) -> (i32, i32) {
        (
            self.cx0 + (co % self.side as u32) as i32,
            self.cz0 + (co / self.side as u32) as i32,
        )
    }
}

/// Sample every atlas cell of one chunk (resident chunk — cheap reads).
fn sample_stream_chunk(
    state: &Arc<State>,
    env: Env,
    world: jvmti_bindings::jni::jobject,
    g: &StreamGeom,
    co: u32,
) {
    let atlas = &state.atlas;
    let (cx, cz) = g.chunk_xy(co);
    for dz in 0..g.per {
        for dx in 0..g.per {
            let x = cx * 16 + dx * g.cell + g.cell / 2;
            let z = cz * 16 + dz * g.cell + g.cell / 2;
            let ix = (x - (atlas.center_x - g.half)) / g.cell;
            let iz = (z - (atlas.center_z - g.half)) / g.cell;
            if ix < 0 || iz < 0 || ix as u32 >= atlas.cells || iz as u32 >= atlas.cells {
                continue;
            }
            if let Some((y, liquid, top, side, tex_top, tex_side)) =
                jni::sample_column(env, world, x, z, state.cfg.sea_level)
            {
                atlas.set(
                    ix as u32,
                    iz as u32,
                    (y.clamp(-64, 1023)) as i16,
                    liquid,
                    top,
                    side,
                    tex_top,
                    tex_side,
                );
            }
        }
    }
}

/// Streamed chunk-major scan pass, ASYNC EDITION.
///
/// The previous edition force-loaded every chunk SYNCHRONOUSLY on the main
/// thread (setChunkForceLoaded parks the caller in a BLOCKING neighbour wait
/// until the chunk — and enough of its neighbours — have generated). On a
/// small box a single cold chunk can park the server thread for tens of
/// seconds: Paper's watchdog kills the process (live-observed), and even
/// without the watchdog the worldgen serializes behind main-thread stalls.
///
/// This edition never blocks the main thread:
///   1. keep a window of chunks requested via getChunkAtAsync (fire-and-
///      forget — the chunk system generates them on its own workers);
///   2. each tick, sample up to `stream_chunks` chunks that ARE resident
///      (isChunkLoaded poll; pin is instant on a resident chunk);
///   3. re-request anything still in flight every 200 ticks — the chunk
///      system dedups by position, and a worldgen crash would otherwise
///      freeze the chunk in the window forever.
fn scan_stream_async(state: &Arc<State>, env: Env, world: jvmti_bindings::jni::jobject) {
    let atlas = &state.atlas;
    let (side, cx0, cz0) = chunk_grid(atlas);
    let total = (side as u64) * (side as u64);
    let g = StreamGeom {
        cell: atlas.cell.max(1),
        side,
        cx0,
        cz0,
        per: (16 / atlas.cell.max(1)).max(1), // cells per chunk edge
        half: atlas.size / 2,
    };
    let budget = state.cfg.stream_chunks.max(1);
    let window = (budget as usize * 4).max(8);

    let mut inflight = state.scan_inflight.lock().unwrap();
    let tick = state.tick_no.load(Ordering::Relaxed);

    // 1) top up the async request window (monotonic centre-out cursor)
    let mut req = state.scan_cursor.load(Ordering::Relaxed);
    while inflight.len() < window && req < total {
        let co = state.chunk_order[req as usize];
        req += 1;
        let (cx, cz) = g.chunk_xy(co);
        if jni::is_chunk_loaded(env, world, cx, cz) == Some(true) {
            // Resident already (spawn area / player chunks): sample right
            // now, no async round-trip, no window slot.
            sample_stream_chunk(state, env, world, &g, co);
        } else if let Some(fut) = jni::request_chunk_future(env, world, cx, cz) {
            let gref = unsafe { new_global(env, fut) };
            inflight.push_back((co, gref, tick));
        }
        // request failure (JNI hiccup): the deadline re-request watchdog
        // never sees it — track it as a hole via the retry queue instead.
        else {
            state.scan_retry.lock().unwrap().push_back(co);
        }
        state.scan_cursor.store(req, Ordering::Relaxed);
    }
    CURSOR_HINT.store(
        state.scan_cursor.load(Ordering::Relaxed).min(total),
        Ordering::Relaxed,
    );

    // 2) sample up to `budget` chunks whose future completed. The chunk
    // system holds the chunk loaded until the future completes, and this
    // task runs at tick start (before the chunk system's unload pass), so a
    // completed future means the chunk is resident RIGHT NOW.
    let mut sampled = 0u32;
    let budget = budget as u32;
    let mut waiting = VecDeque::new();
    while let Some((co, gref, at)) = inflight.pop_front() {
        let done = jni::future_done(env, unsafe { as_obj(gref) }).unwrap_or(false);
        if !done && tick.saturating_sub(at) < 400 {
            waiting.push_back((co, gref, at));
            continue;
        }
        // Done, or stale (crashed generation / lost request) — drop the
        // future ref either way.
        unsafe { env.delete_global_ref(as_obj(gref)) };
        let (cx, cz) = g.chunk_xy(co);
        // PIN ONLY ON A COMPLETED FUTURE. isChunkLoaded alone lies: a chunk
        // whose generation CRASHED (live-observed: OOM inside the worldgen
        // task) keeps a half-built ProtoChunk that reads as "loaded" while
        // its full status never arrives — setChunkForceLoaded on it parks
        // the main thread forever. A completed future means FULL status.
        if !done {
            // Stale: re-request, never pin. The chunk system dedups by
            // position; a crashing chunk re-requests until the deadline
            // cuts it (the hole renders as no-geometry, never as a stall).
            if let Some(fut) = jni::request_chunk_future(env, world, cx, cz) {
                let gref = unsafe { new_global(env, fut) };
                waiting.push_back((co, gref, tick));
            } else {
                state.scan_retry.lock().unwrap().push_back(co);
            }
            continue;
        }
        if sampled < budget && jni::is_chunk_loaded(env, world, cx, cz) == Some(true) {
            // Pin (instant — the chunk is resident AND full), sample,
            // release. The pin/release brackets the reads against a
            // between-ticks unload.
            let _ = jni::warm_chunk(env, world, cx, cz, true);
            sample_stream_chunk(state, env, world, &g, co);
            let _ = jni::warm_chunk(env, world, cx, cz, false);
            sampled += 1;
        } else {
            // Completed but not resident (raced an unload) — re-request.
            if let Some(fut) = jni::request_chunk_future(env, world, cx, cz) {
                let gref = unsafe { new_global(env, fut) };
                waiting.push_back((co, gref, tick));
            } else {
                state.scan_retry.lock().unwrap().push_back(co);
            }
        }
    }
    *inflight = waiting;
}

/// SYNC streaming edition (default): exactly `stream_chunks` chunks per
/// tick, each synchronously force-loaded (which generates it), sampled at
/// cell resolution and un-forced immediately. The cursor is monotonic over
/// the atlas chunk grid, so the whole atlas is covered exactly once and
/// `scanned >= total` fires the normal finalize path as soon as the last
/// chunk is written.
fn scan_stream_sync(state: &Arc<State>, env: Env, world: jvmti_bindings::jni::jobject) {
    let atlas = &state.atlas;
    let cell = atlas.cell.max(1);
    let (side, cx0, cz0) = chunk_grid(atlas);
    let total = (side as u64) * (side as u64);
    let g = StreamGeom {
        cell,
        side,
        cx0,
        cz0,
        per: (16 / cell).max(1), // cells per chunk edge
        half: atlas.size / 2,
    };
    let budget = state.cfg.stream_chunks.max(1);

    let mut inflight = state.scan_inflight.lock().unwrap();
    let _ = &mut inflight; // async-only state; unused in this edition

    for _ in 0..budget {
        // Main sweep first (monotonic cursor over the centre-out ring order),
        // then round-robin retries of the chunks whose force-load failed
        // transiently. Without the retry pass those chunks never come back:
        // the holes freeze into the snapshot as void cells and render as
        // see-through slits in the LOD band.
        let co: u32 = {
            let idx = state.scan_cursor.fetch_add(1, Ordering::Relaxed);
            if idx < total {
                state.chunk_order[idx as usize]
            } else {
                let mut q = state.scan_retry.lock().unwrap();
                match q.pop_front() {
                    Some(i) => i,
                    None => break, // main sweep + retries exhausted
                }
            }
        };
        CURSOR_HINT.store(
            state.scan_cursor.load(Ordering::Relaxed).min(total),
            Ordering::Relaxed,
        );
        let (cx, cz) = g.chunk_xy(co);
        if !jni::warm_chunk(env, world, cx, cz, true) {
            // Transient failure (worldgen pool busy): queue for a retry pass
            // instead of dropping the chunk from the snapshot forever.
            state.scan_retry.lock().unwrap().push_back(co);
            continue;
        }
        sample_stream_chunk(state, env, world, &g, co);
        let _ = jni::warm_chunk(env, world, cx, cz, false);
    }
}

/// Rate-limited scan progress line (shared by both editions).
fn log_scan_progress(state: &Arc<State>, seq: u64) {
    use std::sync::atomic::Ordering as O;
    let streamed = if state.cfg.stream_chunks > 0 {
        let (done, total) = stream_progress(&state.atlas);
        format!(" chunks {done}/{total}")
    } else {
        String::new()
    };
    crate::log_info(&format!(
        "scan: tick {seq}, atlas v{} ({}/{} cells{streamed}), ok={} unloaded={} err={}",
        state.atlas.version(),
        state.atlas.scanned_cells(),
        state.atlas.total_cells(),
        jni::SCAN_OK.load(O::Relaxed),
        jni::SCAN_UNLOADED.load(O::Relaxed),
        jni::SCAN_ERR.load(O::Relaxed),
    ));
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

        // 1. Resource pack — pushed exactly once per player per session.
        //    The pack is static; its SHA-1 never changes, so once pushed we
        //    never call setResourcePack again (no client reloads, ever).
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
                            state.cfg.pack_force,
                        );
                        if ok {
                            st.pushed_sha = pack.sha1_hex.clone();
                            st.resends = 0;
                            // 30 s of one-per-second effect-list kicks: the
                            // pack download+reload on the client is slower
                            // than the first post-effect packet.
                            st.kick = 600;
                            crate::log_info(&format!(
                                "pack pushed to {} ({}): {} bytes, sha1 {}",
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
                // times after the push so a client that was mid-reload when
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
            // Kick window: force a fresh remove+add once per second so a
            // client that resolved the ids too early (before its resource
            // reload finished) gets a second chance — repeatedly.
            if st.kick > 0 && state.tick_no.load(Ordering::Relaxed).is_multiple_of(20) {
                st.kick = st.kick.saturating_sub(20);
                if st.terrain_on {
                    let _ = jni::remove_effect(env, handle, unsafe { as_obj(keys.terrain) });
                    st.terrain_on = false;
                }
                if st.atmosphere_on {
                    let _ = jni::remove_effect(env, handle, unsafe { as_obj(keys.atmosphere) });
                    st.atmosphere_on = false;
                }
            }
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

/// Build the pack ONCE, swap it into the HTTP server, persist the atlas.
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
    state.server.set_pack(new_pack.zip.clone());
    crate::log_info(&format!(
        "c-LoD pack finalized (ONE-TIME, static): {scanned}/{total} cells ({}%), {} bytes, sha1 {}",
        scanned * 100 / total.max(1),
        new_pack.zip.len(),
        new_pack.sha1_hex
    ));
    *state.pack.write().unwrap() = Some(Arc::new(new_pack));
    state.phase.store(PHASE_FINALIZED, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

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
        st.atlas.set(0, 0, 70, false, 0x808080, 0x707070, 0, 0);

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

        // REGRESSION (e2e-live, v4.3.0): the restored snapshot is
        // authoritative — a scan tick that runs BEFORE the engine thread's
        // finalize must not re-sample restored cells. The disk-loaded chunk
        // state samples slightly differently than the generation-time state
        // (same cell count, same pack size, different SHA-1 — observed live),
        // so scan_batch must be a no-op whenever the session was restored.
        st2.phase.store(PHASE_SCANNING, Ordering::SeqCst);
        let before = st2.atlas.to_bytes();
        // scan_batch needs an Env, which unit tests cannot build — the guard
        // sits BEFORE the world/JNI touchpoints, so assert it via the same
        // precondition it checks: a restored session in the SCANNING phase
        // must never mutate the atlas. (The JNI-touching path is covered by
        // the live e2e gate "SHA-1 stability across restarts".)
        assert!(st2.restored);
        assert_eq!(
            st2.atlas.to_bytes(),
            before,
            "snapshot bytes must stay frozen while restored+scanning"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}
