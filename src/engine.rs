//! The c-LoD engine, v5.0 MAP CARRIER STREAMING.
//!
//! The resource pack is a STATIC shader-only artifact built once per process
//! boot and pushed once per client join — it contains ZERO world data: no
//! atlas, no PNGs, nothing world-derived. Every byte of LOD data travels at
//! runtime through vanilla map packets (`ClientboundMapItemDataPacket`) into
//! client-side dynamic map textures; the pack's shaders scrape those into
//! persistent post-chain targets and march the heightfield live.
//!
//! Tick loop (main thread):
//!   1. boot       — resolve the JNI surface, build the pack once, push it.
//!   2. scan    — player-centric heightmap sweep into the region store (the
//!      streamer's data source), with a bounded async force-load frontier.
//!   3. stream  — per player: advance page cursors (budget), ship dirty pages
//!      as raw map packets, cycle the offhand map through the 8 pages so the
//!      client uploads each page to the GPU and the scrape captures it.
//!   4. players — one resource-pack push per join (the pack never changes),
//!      per-player posteffect tier switching.

use crate::config::Config;
use crate::http::PackServer;
use crate::jni::{self, PlayerInfo};
use crate::packgen::{self, Pack};
use crate::stream::{Cell, Streamer, PAGES_PER_REGION};
use jvmti_bindings::prelude::JniEnv;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, AtomicU32, AtomicU64, AtomicU8, Ordering};
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
    /// v5 map-carrier streaming state (Rust side; main-thread only).
    pub stream: Mutex<Streamer>,
    /// Per-player virtual map ids (8 pages), assigned on first sight.
    pub map_ids: Mutex<HashMap<String, [i32; PAGES_PER_REGION]>>,
    /// Monotonic map-id base (vanilla ids live far below this).
    map_base: AtomicI32,
    /// v4.4 frontier: chunks force-loaded and waiting for the async load to
    /// finish (chunk -> tick when queued). Sampled + released a couple of
    /// ticks later.
    pub force_pending: Mutex<HashMap<(i32, i32), u64>>,
    /// Frontier chunks scanned through the force path so far.
    pub forced_chunks: AtomicU64,
    /// Diagnostics: cells streamed as page bytes since boot.
    pub streamed_bytes: AtomicU64,
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
    /// Offhand page currently displayed on the client.
    pub page: u8,
    /// Set once the map ids were assigned and page 0 was handed over.
    pub streaming: bool,
    /// Engine tick of the player's first sight (pack push anchor: the push
    /// must land AFTER the client finished the join screens, or the pack
    /// prompt screen gets swallowed by the loading terrain screen).
    pub joined_tick: u64,
}

/// Engine ticks between the join and the single pack push.
const PACK_PUSH_DELAY_TICKS: u64 = 40;

impl State {
    pub fn new(cfg: Config) -> Arc<Self> {
        let stream = Streamer::new(cfg.region_blocks, cfg.cell_size, cfg.stream_slots);
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
            map_ids: Mutex::new(HashMap::new()),
            map_base: AtomicI32::new(1_000_000_000),
            force_pending: Mutex::new(HashMap::new()),
            forced_chunks: AtomicU64::new(0),
            streamed_bytes: AtomicU64::new(0),
        })
    }

    fn assign_map_ids(&self, uuid: &str) -> [i32; PAGES_PER_REGION] {
        let mut ids = self.map_ids.lock().unwrap();
        if let Some(v) = ids.get(uuid) {
            return *v;
        }
        let base = self
            .map_base
            .fetch_add(PAGES_PER_REGION as i32, Ordering::SeqCst);
        let v: [i32; PAGES_PER_REGION] = core::array::from_fn(|i| base + i as i32);
        ids.insert(uuid.to_string(), v);
        v
    }
}

type Env<'a> = &'a JniEnv;

static BRIDGE_OK: AtomicU8 = AtomicU8::new(0); // 0 pending, 1 ok, 2 failed-once

/// One engine tick. MUST run on the main thread.
pub fn tick(state: &Arc<State>, env: Env) {
    state.tick_no.fetch_add(1, Ordering::Relaxed);
    if state.tick_no.load(Ordering::Relaxed) % 50 == 0 {
        crate::log_info(&format!(
            "engine tick alive: tick {}",
            state.tick_no.load(Ordering::Relaxed)
        ));
    }
    ensure_boot(state, env);
    stream_scan(state, env);
    poll_forced(state, env);
    manage_players(state, env);
}

/// One-time boot: JNI surface + the shader-only pack (built IMMEDIATELY — it
/// holds no world data, so there is nothing to wait for) + effect keys.
fn ensure_boot(state: &Arc<State>, env: Env) {
    if BRIDGE_OK.load(Ordering::SeqCst) != 1 {
        if jni::warmup(env) && jni::nms_warmup(env) {
            BRIDGE_OK.store(1, Ordering::SeqCst);
            crate::log_info("c-LoD JNI bridge resolved: true (incl. NMS map surface)");
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
                "c-LoD shader-only pack built ONCE at boot: {len} bytes, sha1 {sha} — no atlas, no revisions, one push per join"
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
}

/// # Safety
/// `obj` must be a valid local ref; runs on the main thread only.
unsafe fn new_global(env: Env, obj: jvmti_bindings::jni::jobject) -> usize {
    env.new_global_ref(obj) as usize
}

unsafe fn as_obj(handle: usize) -> jvmti_bindings::jni::jobject {
    handle as jvmti_bindings::jni::jobject
}

/// Cap on chunks queued for force-load per engine tick (see v4.4 notes: the
/// generation flood starves the tick loop above these numbers).
const FORCE_MAX_PENDING: usize = 8;
const FORCE_BUDGET_PER_TICK: usize = 4;

/// Player-centric heightmap sweep into the region store. The LOD data source:
/// rings of `cell_size` blocks around each player, skipping cells already
/// valid; unloaded chunks queue for the async force-load frontier.
fn stream_scan(state: &Arc<State>, env: Env) {
    if state.phase.load(Ordering::SeqCst) != PHASE_LIVE {
        return;
    }
    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        return;
    }
    let world = unsafe { as_obj(world_handle) };
    let players = jni::players(env);
    if players.is_empty() {
        return;
    }
    let budget = state.cfg.scan_budget / players.len() as u32;
    for p in &players {
        if p.world.is_empty() {
            continue;
        }
        let (rx, rz) = {
            let s = state.stream.lock().unwrap();
            s.region_of(p.bx, p.bz)
        };
        // Ring sweep around the player, clamped to the region.
        let side = state.cfg.region_blocks / state.cfg.cell_size;
        let pcx = ((p.bx - rx * state.cfg.region_blocks) / state.cfg.cell_size).clamp(0, side - 1);
        let pcz = ((p.bz - rz * state.cfg.region_blocks) / state.cfg.cell_size).clamp(0, side - 1);
        let mut done = 0u32;
        let mut ring: i32 = 0;
        while done < budget && ring <= side {
            let mut step = |dx: i32, dz: i32| -> bool {
                let cx = pcx + dx;
                let cz = pcz + dz;
                if cx < 0 || cz < 0 || cx >= side || cz >= side {
                    return false;
                }
                {
                    let s = state.stream.lock().unwrap();
                    if let Some(cell) = s.region(rx, rz).and_then(|r| r.get(cx as u32, cz as u32)) {
                        if cell.valid {
                            return false;
                        }
                    }
                }
                let wx = rx * state.cfg.region_blocks + cx * state.cfg.cell_size;
                let wz = rz * state.cfg.region_blocks + cz * state.cfg.cell_size;
                scan_cell(state, env, world, rx, rz, cx as u32, cz as u32, wx, wz);
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
}

/// Sample one world column into the region store (or queue its chunk for the
/// async force-load frontier when unloaded).
#[allow(clippy::too_many_arguments)]
fn scan_cell(
    state: &Arc<State>,
    env: Env,
    world: jvmti_bindings::jni::jobject,
    rx: i32,
    rz: i32,
    cx: u32,
    cz: u32,
    wx: i32,
    wz: i32,
) {
    if let Some((y, liquid, material, biome)) = jni::sample_surface(env, world, wx, wz) {
        let rgb = crate::palette::surface_color(&material, &biome, liquid);
        let mut s = state.stream.lock().unwrap();
        s.set_cell(
            rx,
            rz,
            cx,
            cz,
            Cell {
                height: y.clamp(-64, 1023) as i16,
                water: liquid,
                valid: true,
                rgb,
            },
        );
    } else {
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
    let mut pend = state.force_pending.lock().unwrap();
    if pend.contains_key(&(cx, cz)) || pend.len() >= FORCE_MAX_PENDING {
        return;
    }
    let now = state.tick_no.load(Ordering::SeqCst);
    let fresh = pend.values().filter(|t| **t == now).count();
    if fresh >= FORCE_BUDGET_PER_TICK {
        return;
    }
    if jni::warm_chunk(env, world, cx, cz, true) {
        pend.insert((cx, cz), now);
    }
}

/// Sample + release chunks queued by [`force_chunk_for`] once the async load
/// finished. One chunk covers (16/cell)^2 cells — every still-invalid cell
/// inside it is sampled in one pass.
fn poll_forced(state: &Arc<State>, env: Env) {
    const READY_TICKS: u64 = 2;
    const MAX_SAMPLES: usize = 128;
    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        return;
    }
    let world = unsafe { as_obj(world_handle) };
    let now = state.tick_no.load(Ordering::SeqCst);
    let mut pend = state.force_pending.lock().unwrap();
    let ready: Vec<(i32, i32)> = pend
        .iter()
        .filter(|(_, t)| now.saturating_sub(**t) >= READY_TICKS)
        .map(|(c, _)| *c)
        .collect();
    let mut samples = 0usize;
    for (cx, cz) in ready {
        if !jni::is_chunk_loaded(env, world, cx, cz).unwrap_or(false) {
            pend.insert((cx, cz), now);
            continue;
        }
        let side = state.cfg.region_blocks / state.cfg.cell_size;
        let x0 = cx * 16;
        let z0 = cz * 16;
        for dz in 0..(16 / state.cfg.cell_size.max(1)).max(1) {
            for dx in 0..(16 / state.cfg.cell_size.max(1)).max(1) {
                if samples >= MAX_SAMPLES {
                    break;
                }
                let wx = x0 + dx * state.cfg.cell_size;
                let wz = z0 + dz * state.cfg.cell_size;
                let (rx, rz) = {
                    let s = state.stream.lock().unwrap();
                    s.region_of(wx, wz)
                };
                let ccx =
                    ((wx - rx * state.cfg.region_blocks) / state.cfg.cell_size).clamp(0, side - 1);
                let ccz =
                    ((wz - rz * state.cfg.region_blocks) / state.cfg.cell_size).clamp(0, side - 1);
                let valid = {
                    let s = state.stream.lock().unwrap();
                    s.region(rx, rz)
                        .and_then(|r| r.get(ccx as u32, ccz as u32))
                        .map(|c| c.valid)
                        .unwrap_or(false)
                };
                if valid {
                    continue;
                }
                scan_cell(state, env, world, rx, rz, ccx as u32, ccz as u32, wx, wz);
                samples += 1;
            }
        }
        jni::warm_chunk(env, world, cx, cz, false); // release the force ticket
        pend.remove(&(cx, cz));
        state.forced_chunks.fetch_add(1, Ordering::SeqCst);
        if samples >= MAX_SAMPLES {
            break;
        }
    }
}

/// Tier for one player from the world only: streaming follows the player, so
/// any position in the streaming world is Full; other worlds keep the veil.
fn tier_for(state: &Arc<State>, p: &PlayerInfo) -> Tier {
    if p.world.is_empty() {
        return Tier::Off;
    }
    let wname = state.world_name.lock().unwrap().clone();
    if !wname.is_empty() && p.world != wname {
        return Tier::Veil;
    }
    Tier::Full
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

            // 3. MAP-CARRIER STREAMING: raw map packets + offhand page cycle.
            if have_pack && tier == Tier::Full {
                stream_player(state, env, handle, &p, &mut st);
            }
        }

        state.per_player.lock().unwrap().insert(p.uuid.clone(), st);
    }

    // Drop state for players who left (their map ids stay reserved — the
    // client textures die with the connection anyway).
    state
        .per_player
        .lock()
        .unwrap()
        .retain(|k, _| seen.contains(k));
}

/// One player's streaming step: page patches + offhand cycling.
fn stream_player(
    state: &Arc<State>,
    env: Env,
    handle: jvmti_bindings::jni::jobject,
    p: &PlayerInfo,
    st: &mut PlayerState,
) {
    let uuid = &p.uuid;
    let ids = state.assign_map_ids(uuid);
    let (rx, rz) = {
        let s = state.stream.lock().unwrap();
        s.region_of(p.bx, p.bz)
    };

    if !st.streaming {
        st.streaming = true;
        st.page = 0;
        unsafe {
            jni::give_offhand_map(env, handle, ids[0]);
        }
        crate::log_info(&format!(
            "stream: map ids {}..{} assigned to {uuid}, page 0 in offhand, region ({rx},{rz})",
            ids[0],
            ids[PAGES_PER_REGION - 1]
        ));
    }

    // Page patches: the per-tick byte budget splits across the 8 pages.
    let budget = (state.cfg.stream_budget / PAGES_PER_REGION as u32).max(64) as usize;
    let mut sent_pages = 0u32;
    let mut sent_bytes = 0u64;
    for (page, map_id) in ids.iter().enumerate() {
        let patch = state
            .stream
            .lock()
            .unwrap()
            .page_patch(rx, rz, page, budget);
        let Some(patch) = patch else { continue };
        let p = (patch.x, patch.y, patch.w, patch.h, patch.bytes.as_slice());
        unsafe {
            let Some(pkt) = jni::map_page_packet(env, *map_id, Some(p)) else {
                continue;
            };
            if jni::send_packet(env, handle, pkt) {
                sent_pages += 1;
                sent_bytes += patch.bytes.len() as u64;
            }
        }
    }
    if sent_bytes > 0 {
        state
            .streamed_bytes
            .fetch_add(sent_bytes, Ordering::Relaxed);
    }

    // Offhand page cycle: the client uploads the DISPLAYED map to the GPU,
    // the scrape pass captures it into the persistent target. Cycle even
    // without patches — freshly joined clients must see every page once.
    let tick = state.tick_no.load(Ordering::Relaxed);
    let want_page = ((tick / state.cfg.page_ticks.max(1) as u64) % PAGES_PER_REGION as u64) as u8;
    if want_page != st.page {
        unsafe {
            if jni::give_offhand_map(env, handle, ids[want_page as usize]) {
                st.page = want_page;
            }
        }
    }

    // Rate-limited diagnostics.
    if tick.is_multiple_of(100) {
        let (cells, scanned) = {
            let s = state.stream.lock().unwrap();
            match s.region(rx, rz) {
                Some(r) => (s.region_cells() * s.region_cells(), r.valid_cells()),
                None => (0, 0),
            }
        };
        crate::log_info(&format!(
            "stream: tick {tick} region ({rx},{rz}) {scanned}/{cells} cells, page {}, +{sent_pages} pkts (+{sent_bytes} B), total {} B / {} pkts",
            st.page,
            state.streamed_bytes.load(Ordering::Relaxed),
            jni::STREAM_PACKETS.load(Ordering::Relaxed),
        ));
    }
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

    #[test]
    fn pack_url_uses_base() {
        let st = State::new(Config::parse("pack_base_url=http://x.example;http_port=1"));
        assert_eq!(pack_url(&st), "http://x.example/clod-pack.zip");
    }

    #[test]
    fn coverage_matches_world_and_bounds() {
        let st = State::new(Config::parse(""));
        *st.world_name.lock().unwrap() = "world".to_string();
        let p = PlayerInfo {
            uuid: "u".into(),
            world: "world".into(),
            bx: 100,
            by: 64,
            bz: 100,
        };
        assert_eq!(tier_for(&st, &p), Tier::Full);
        let p_far = PlayerInfo {
            uuid: "u".into(),
            world: "world".into(),
            bx: 90_000,
            by: 64,
            bz: 0,
        };
        // Streaming follows the player anywhere in the world.
        assert_eq!(tier_for(&st, &p_far), Tier::Full);
        let p_world = PlayerInfo {
            uuid: "u".into(),
            world: "world_nether".into(),
            bx: 0,
            by: 64,
            bz: 0,
        };
        assert_eq!(tier_for(&st, &p_world), Tier::Veil);
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
    fn map_ids_unique_per_player() {
        let st = State::new(Config::parse(""));
        let a = st.assign_map_ids("a");
        let b = st.assign_map_ids("b");
        let a2 = st.assign_map_ids("a");
        assert_eq!(a, a2);
        assert_ne!(a[0], b[0]);
        assert_eq!(a[7] - a[0], 7);
    }

    #[test]
    fn pack_builds_at_boot_without_world() {
        // The v5 pack is shader-only: it must build with NO world, NO atlas,
        // NO scan data — the whole point of map-carrier streaming.
        let st = State::new(Config::parse(""));
        assert!(st.pack.read().unwrap().is_none());
        assert_eq!(st.phase.load(Ordering::SeqCst), PHASE_BOOT);
    }
}
