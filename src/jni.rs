//! JNI bridge to Paper 26.3 (Mojang-mapped, unobfuscated).
//!
//! Everything here runs on the server main thread via
//! `cplug_sdk::run_on_main_thread`. Verified call surface:
//!   - org.bukkit.Bukkit.getServer()            -> Server
//!   - Server.getOnlinePlayers()                -> Collection<Player>
//!   - Server.getWorlds()                       -> List<World>
//!   - World.getName()                          -> String
//!   - World.getHighestBlockYAt(int,int)        -> int
//!   - World.getBlockAt(int,int,int)            -> Block
//!   - Block.isLiquid()                         -> boolean
//!   - Player.getUniqueId()                     -> UUID
//!   - Player.getLocation()                     -> Location
//!   - Location.getBlockX/getBlockY/getBlockZ() -> int
//!   - Location.getWorld()                      -> World
//!   - Player.setResourcePack(UUID,String,byte[],String,boolean)
//!   - Player.postEffects()                     -> PlayerPostEffects
//!   - PlayerPostEffects.add/remove(Key)        -> boolean
//!   - Key.key(String)                          -> Key (static, adventure)
//!   - UUID.fromString(String)                  -> UUID (static)
//!
//! CLASS RESOLUTION RULES (hard-won against Paper 26.3's layered loaders):
//!   1. NEVER plain JNI FindClass from our callbacks — it sees only the
//!      system loader and a pending NoClassDefFoundError escaping the native
//!      frame kills the server tick.
//!   2. The JVMTI GetLoadedClasses scan finds classes across ALL loaders,
//!      but with several loaders hosting same-named API classes it can pick
//!      a duplicate whose method table mismatches (GetMethodID silently
//!      returns NULL). The scan is used ONLY for java.* bootstrap classes
//!      (single copy) and org/bukkit/Bukkit (verified against getServer).
//!   3. Everything else is resolved on the CONCRETE class of a live object
//!      (CraftServer / CraftWorld / CraftPlayer / CraftBlock / Location) via
//!      GetObjectClass — the exact class that actually implements the call.
//!      adventure Key and PlayerPostEffects come from Class.forName through
//!      CraftServer's own loader (exact same-loader Class object).

use jvmti_bindings::jni::{jboolean, jint, jmethodID, jobject, jstring, jvalue};
use jvmti_bindings::prelude::JniEnv;
use std::sync::{Mutex, OnceLock};

type Env<'a> = &'a JniEnv;

fn jv(o: jobject) -> jvalue {
    jvalue { l: o }
}

fn jz(v: bool) -> jvalue {
    jvalue { z: v as jboolean }
}

fn ji(v: i32) -> jvalue {
    jvalue { i: v as jint }
}

fn jobj(o: jobject) -> Option<jobject> {
    if o.is_null() {
        None
    } else {
        Some(o)
    }
}

fn clear_if_pending(env: Env) {
    if env.exception_check() {
        env.exception_clear();
    }
}

fn jstr(env: Env, o: jobject) -> Option<String> {
    jobj(o).and_then(|o| env.get_string_utf(o as jstring))
}

// ---------------------------------------------------------------------------
// Static surface: resolved once (see resolve()).
// ---------------------------------------------------------------------------

pub struct Ids {
    bukkit_cls: jobject,
    m_get_server: jmethodID,
    m_server_players: jmethodID,
    m_server_worlds: jmethodID,
    m_iterator: jmethodID,
    m_has_next: jmethodID,
    m_next: jmethodID,
    m_list_size: jmethodID,
    m_list_get: jmethodID,
    uuid_cls: jobject,
    m_uuid_to_string: jmethodID,
    m_uuid_from_string: jmethodID,
    /// Exact adventure Key class from CraftServer's loader.
    key_cls: jobject,
    m_key_key: jmethodID,
    m_pe_add: jmethodID,
    m_pe_remove: jmethodID,
    m_pe_update: jmethodID,
}

static IDS: OnceLock<Option<SendIds>> = OnceLock::new();

/// JNI IDs are raw pointers: usable only from the main thread they were
/// resolved on, but the static needs Send+Sync to exist. Every bridge
/// function is documented main-thread-only.
struct SendIds(Ids);
unsafe impl Send for SendIds {}
unsafe impl Sync for SendIds {}

fn ids() -> Option<&'static Ids> {
    IDS.get().and_then(|o| o.as_ref()).map(|s| &s.0)
}

// ---------------------------------------------------------------------------
// Lazy surfaces: concrete-class method IDs resolved on first live object.
// All main-thread-only; Mutex<Option<_>> allows cheap retry until available.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct WorldIds {
    m_name: jmethodID,
    m_highest_block: jmethodID,
    m_get_block_at: jmethodID,
    m_chunk_loaded: jmethodID,
    m_set_force_loaded: jmethodID,
    m_get_biome: jmethodID,
    /// World.getChunkAt(int,int) -> Chunk — the DH slice walk reads the
    /// column through a ChunkSnapshot (one JNI object per chunk instead of
    /// one getBlockAt call chain per block).
    m_get_chunk_at: jmethodID,
    // Full game time (ticks since creation). Unlike the day time it advances
    // on every real server tick regardless of doDaylightCycle — the engine
    // uses its growth to detect that the tick loop started (queued tasks run
    // inside prepareLevel's executeModerately long before that, and heavy
    // work queued there wedge the whole startup, see engine.rs probe_ticking).
    m_get_full_time: jmethodID,
    // Paper's URGENT async chunk request (getChunkAtAsyncUrgently): fires
    // generation on the chunk system without ever blocking the calling
    // thread. The streaming scan uses it instead of setChunkForceLoaded — a
    // SYNCHRONOUS force-load of a cold chunk parks the main thread in a
    // BLOCKING neighbour wait that on a small box takes tens of seconds
    // (Paper watchdog kills the server). The plain getChunkAtAsync queues at
    // background priority (live-observed: 0.5 chunks/s); the urgent variant
    // is what a boot-time scan needs.
    m_get_chunk_async: jmethodID,
}
// Raw method-ID pointers cached process-lifetime; consumed main-thread-only.
unsafe impl Send for WorldIds {}
static WORLD_IDS: Mutex<Option<WorldIds>> = Mutex::new(None);

#[derive(Clone, Copy)]
struct PlayerIds {
    m_get_unique_id: jmethodID,
    m_get_location: jmethodID,
    m_post_effects: jmethodID,
    m_set_resource_pack: jmethodID,
}
// Raw method-ID pointers cached process-lifetime; consumed main-thread-only.
unsafe impl Send for PlayerIds {}
static PLAYER_IDS: Mutex<Option<PlayerIds>> = Mutex::new(None);

#[derive(Clone, Copy)]
struct LocationIds {
    m_get_block_x: jmethodID,
    m_get_block_y: jmethodID,
    m_get_block_z: jmethodID,
    m_loc_get_world: jmethodID,
}
// Raw method-ID pointers cached process-lifetime; consumed main-thread-only.
unsafe impl Send for LocationIds {}
static LOCATION_IDS: Mutex<Option<LocationIds>> = Mutex::new(None);

#[derive(Clone, Copy)]
struct BlockIds {
    m_is_liquid: jmethodID,
    m_get_type: jmethodID,
}
// Raw method-ID pointers cached process-lifetime; consumed main-thread-only.
unsafe impl Send for BlockIds {}
static BLOCK_IDS: Mutex<Option<BlockIds>> = Mutex::new(None);

/// `org.bukkit.Material` (an enum) + `NamespacedKey`.
#[derive(Clone, Copy)]
struct MaterialIds {
    m_ordinal: jmethodID,
    m_get_key: jmethodID,
    // DH avoided-block predicate (FullDataToRenderDataTransformer:
    // ignoreNonSolidBlock = !isSolid && !isLiquid && opacity != opaque;
    // Bukkit isOccluding == NMS opacity 15 for a full cube).
    m_is_solid: jmethodID,
    m_is_occluding: jmethodID,
    // The slice walk cuts bands on Material change and never bands air.
    m_is_air: jmethodID,
}
unsafe impl Send for MaterialIds {}
static MATERIAL_IDS: Mutex<Option<MaterialIds>> = Mutex::new(None);

/// `org.bukkit.Chunk` — only getChunkSnapshot is needed.
#[derive(Clone, Copy)]
struct ChunkIds {
    m_get_snapshot: jmethodID,
}
unsafe impl Send for ChunkIds {}
static CHUNK_IDS: Mutex<Option<ChunkIds>> = Mutex::new(None);

fn chunk_ids(env: Env, chunk: jobject) -> Option<ChunkIds> {
    if let Some(c) = CHUNK_IDS.lock().unwrap().as_ref() {
        return Some(*c);
    }
    let cls_local = env.get_object_class(chunk);
    let cls = {
        let g = env.new_global_ref(cls_local);
        g as jobject
    };
    let ids = ChunkIds {
        m_get_snapshot: bind_method(
            env,
            cls_local,
            cls,
            "getChunkSnapshot",
            "(ZZZ)Lorg/bukkit/ChunkSnapshot;",
        )?,
    };
    *CHUNK_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

/// `org.bukkit.ChunkSnapshot` — the slice walk's block reader.
#[derive(Clone, Copy)]
struct SnapshotIds {
    m_get_block_type: jmethodID,
}
unsafe impl Send for SnapshotIds {}
static SNAPSHOT_IDS: Mutex<Option<SnapshotIds>> = Mutex::new(None);

fn snapshot_ids(env: Env, snap: jobject) -> Option<SnapshotIds> {
    if let Some(s) = SNAPSHOT_IDS.lock().unwrap().as_ref() {
        return Some(*s);
    }
    let cls_local = env.get_object_class(snap);
    let cls = {
        let g = env.new_global_ref(cls_local);
        g as jobject
    };
    let ids = SnapshotIds {
        m_get_block_type: bind_method(
            env,
            cls_local,
            cls,
            "getBlockType",
            "(III)Lorg/bukkit/Material;",
        )?,
    };
    *SNAPSHOT_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

/// `java.util.concurrent.CompletableFuture` — the engine polls isDone() on
/// held getChunkAtAsyncUrgently futures (see engine.rs scan_stream).
#[derive(Clone, Copy)]
struct FutureIds {
    m_is_done: jmethodID,
}
unsafe impl Send for FutureIds {}
static FUTURE_IDS: Mutex<Option<FutureIds>> = Mutex::new(None);

fn future_ids(env: Env, fut: jobject) -> Option<FutureIds> {
    if let Some(f) = FUTURE_IDS.lock().unwrap().as_ref() {
        return Some(*f);
    }
    let cls_local = env.get_object_class(fut);
    let cls = {
        let g = env.new_global_ref(cls_local);
        g as jobject
    };
    let ids = FutureIds {
        m_is_done: bind_method(env, cls_local, cls, "isDone", "()Z")?,
    };
    *FUTURE_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

#[derive(Clone, Copy)]
struct KeyIds {
    m_get_key: jmethodID,
}
unsafe impl Send for KeyIds {}
static KEY_IDS: Mutex<Option<KeyIds>> = Mutex::new(None);

#[derive(Clone, Copy)]
struct BiomeIds {
    m_get_key: jmethodID,
}
unsafe impl Send for BiomeIds {}
static BIOME_IDS: Mutex<Option<BiomeIds>> = Mutex::new(None);

/// Baked average top/side colours + per-face tint kind + ratio-tile set for
/// one Material (DH 3.3.4 textured LODs ride the same table).
#[derive(Clone, Copy)]
struct MatInfo {
    top: u32,
    side: u32,
    tint_top: u8,
    tint_side: u8,
    tex_top: u16,
    tex_side: u16,
    /// blockcolors::lookup hit (a miss keeps the previous band's colour —
    /// the slice walk STOPS at unbaked materials instead of banding gray).
    baked: bool,
    /// DH EDhApiBlockMaterial.GRASS (grass_block) — LodQuadBuilder
    /// FADE_TO_DIRT recolours the wall bottom of grass datapoints.
    grass: bool,
}
static MAT_CACHE: Mutex<Vec<Option<MatInfo>>> = Mutex::new(Vec::new());
/// One-entry biome tint cache: the scan walks chunk-major, so consecutive
/// columns almost always share a biome.
#[allow(clippy::type_complexity)] // (chunkX, chunkZ) -> (grass, foliage, water) tint triple
static BIOME_CACHE: Mutex<Option<((i32, i32), (u32, u32, u32))>> = Mutex::new(None);

fn key_ids(env: Env, key: jobject) -> Option<KeyIds> {
    if let Some(k) = KEY_IDS.lock().unwrap().as_ref() {
        return Some(*k);
    }
    let cls_local = env.get_object_class(key);
    let cls = {
        let g = env.new_global_ref(cls_local);
        g as jobject
    };
    let ids = KeyIds {
        m_get_key: bind_method(env, cls_local, cls, "getKey", "()Ljava/lang/String;")?,
    };
    *KEY_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

fn material_ids(env: Env, mat: jobject) -> Option<MaterialIds> {
    if let Some(m) = MATERIAL_IDS.lock().unwrap().as_ref() {
        return Some(*m);
    }
    let cls_local = env.get_object_class(mat);
    let cls = {
        let g = env.new_global_ref(cls_local);
        g as jobject
    };
    let ids = MaterialIds {
        m_ordinal: bind_method(env, cls_local, cls, "ordinal", "()I")?,
        m_get_key: bind_method(
            env,
            cls_local,
            cls,
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
        )?,
        m_is_solid: bind_method(env, cls_local, cls, "isSolid", "()Z")?,
        m_is_occluding: bind_method(env, cls_local, cls, "isOccluding", "()Z")?,
        m_is_air: bind_method(env, cls_local, cls, "isAir", "()Z")?,
    };
    *MATERIAL_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

fn biome_ids(env: Env, biome: jobject) -> Option<BiomeIds> {
    if let Some(b) = BIOME_IDS.lock().unwrap().as_ref() {
        return Some(*b);
    }
    let cls_local = env.get_object_class(biome);
    let cls = {
        let g = env.new_global_ref(cls_local);
        g as jobject
    };
    let ids = BiomeIds {
        m_get_key: bind_method(
            env,
            cls_local,
            cls,
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
        )?,
    };
    *BIOME_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

fn namespaced_key_path(env: Env, key: jobject, ids: KeyIds) -> Option<String> {
    let s = jobj(env.call_object_method(key, ids.m_get_key, &[]))?;
    let out = jstr(env, s);
    clear_if_pending(env);
    out
}

fn mul_rgb(c: u32, t: u32) -> u32 {
    let r = ((c >> 16) & 0xFF) * ((t >> 16) & 0xFF) / 255;
    let g = ((c >> 8) & 0xFF) * ((t >> 8) & 0xFF) / 255;
    let b = (c & 0xFF) * (t & 0xFF) / 255;
    (r << 16) | (g << 8) | b
}

/// Registry path of a Bukkit Material ("snow", "water", ...) via its Key.
/// Uncached — the surface-layer probe calls this once per scanned column.
fn material_path(env: Env, mat: jobject, ids: &MaterialIds) -> Option<String> {
    let key = jobj(env.call_object_method(mat, ids.m_get_key, &[]))?;
    let kid = key_ids(env, key)?;
    namespaced_key_path(env, key, kid)
}

/// Resolve (and memoise by enum ordinal) the baked colours of one Material.
fn mat_info(env: Env, mat: jobject, ids: &MaterialIds, ordinal: i32) -> Option<MatInfo> {
    if ordinal < 0 {
        return None;
    }
    let idx = ordinal as usize;
    {
        let cache = MAT_CACHE.lock().unwrap();
        if let Some(Some(info)) = cache.get(idx) {
            return Some(*info);
        }
    }
    let key = jobj(env.call_object_method(mat, ids.m_get_key, &[]))?;
    let kid = key_ids(env, key)?;
    let name = namespaced_key_path(env, key, kid)?;
    let info = match crate::blockcolors::lookup(&name) {
        Some((top, side, tint_top, tint_side)) => MatInfo {
            top,
            side,
            tint_top,
            tint_side,
            tex_top: crate::blocktextures::lookup(&name).map_or(0, |(t, _)| t),
            tex_side: crate::blocktextures::lookup(&name).map_or(0, |(_, s)| s),
            baked: true,
            grass: name == "grass_block",
        },
        None => MatInfo {
            top: 0x7F7F7F,
            side: 0x7A746C,
            tint_top: 0,
            tint_side: 0,
            tex_top: 0,
            tex_side: 0,
            baked: false,
            grass: false,
        },
    };
    let mut cache = MAT_CACHE.lock().unwrap();
    if cache.len() <= idx {
        cache.resize(idx + 1, None);
    }
    cache[idx] = Some(info);
    Some(info)
}

/// Biome grass/foliage/water tint for a column (cached per chunk).
fn biome_tint(env: Env, world: jobject, wid: &WorldIds, x: i32, y: i32, z: i32) -> (u32, u32, u32) {
    let key = (x >> 4, z >> 4);
    if let Some((k, v)) = *BIOME_CACHE.lock().unwrap() {
        if k == key {
            return v;
        }
    }
    let resolved = (|| {
        let biome = jobj(env.call_object_method(world, wid.m_get_biome, &[ji(x), ji(y), ji(z)]))?;
        let bid = biome_ids(env, biome)?;
        let kobj = jobj(env.call_object_method(biome, bid.m_get_key, &[]))?;
        let kid = key_ids(env, kobj)?;
        let name = namespaced_key_path(env, kobj, kid)?;
        Some(crate::blockcolors::biome(&name))
    })()
    .unwrap_or((0x91BD59, 0x48B518, 0x3F76E4));
    *BIOME_CACHE.lock().unwrap() = Some((key, resolved));
    resolved
}

/// Sample one column: (blockY, isLiquid, top rgb, side rgb, top tile, side tile)
/// plus the DH vertical slice table (top-down runs cut on Material change).
///
/// The colours come from the baked block table (average texture colour) times
/// the biome tint — Distant Horizons' flat LOD colour. The tile ids point into
/// the ratio-tile atlas (DH 3.3.4 textured LODs); 0 = flat. The slice walk
/// reads the column through a ChunkSnapshot and stops at `walk_depth` blocks
/// or `SLICE_RAW` bands; unbaked materials end the walk (the last band
/// extends to the world floor, exactly like a DH full-data column).
/// (surface tuple..., DH vertical runs) — clippy type_complexity alias.
pub type ColumnTuple = (i32, bool, u32, u32, u16, u16, Vec<crate::slices::SliceRun>);

pub fn sample_column_slices(
    env: Env,
    world: jobject,
    x: i32,
    z: i32,
    sea: i32,
    walk_depth: i32,
) -> Option<ColumnTuple> {
    sample_column_impl(env, world, x, z, sea, walk_depth)
}

fn sample_column_impl(
    env: Env,
    world: jobject,
    x: i32,
    z: i32,
    sea: i32,
    walk_depth: i32,
) -> Option<ColumnTuple> {
    use std::sync::atomic::Ordering;
    let wid = world_ids(env, world)?;
    if env.exception_check() {
        env.exception_clear();
        SCAN_ERR.fetch_add(1, Ordering::Relaxed);
        return None;
    }
    let loaded = env.call_boolean_method(world, wid.m_chunk_loaded, &[ji(x >> 4), ji(z >> 4)]);
    if env.exception_check() {
        env.exception_clear();
        return None;
    }
    if !loaded {
        SCAN_UNLOADED.fetch_add(1, Ordering::Relaxed);
        return None;
    }
    let y = env.call_int_method(world, wid.m_highest_block, &[ji(x), ji(z)]);
    if env.exception_check() {
        env.exception_clear();
        SCAN_ERR.fetch_add(1, Ordering::Relaxed);
        return None;
    }
    if y < -64 {
        return Some((y, false, 0x808080, 0x808080, 0, 0, Vec::new()));
    }
    let gray = 0x808080u32;
    let Some(block) =
        jobj(env.call_object_method(world, wid.m_get_block_at, &[ji(x), ji(y), ji(z)]))
    else {
        clear_if_pending(env);
        return Some((y, false, gray, gray, 0, 0, Vec::new()));
    };
    let Some(bid) = block_ids(env, block) else {
        clear_if_pending(env);
        return Some((y, false, gray, gray, 0, 0, Vec::new()));
    };
    let liquid = env.call_boolean_method(block, bid.m_is_liquid, &[]);
    let mat = jobj(env.call_object_method(block, bid.m_get_type, &[]));
    clear_if_pending(env);
    let Some(mat) = mat else {
        SCAN_OK.fetch_add(1, Ordering::Relaxed);
        return Some((y, liquid, gray, gray, 0, 0, Vec::new()));
    };
    let Some(mid) = material_ids(env, mat) else {
        return Some((y, liquid, gray, gray, 0, 0, Vec::new()));
    };
    let ordinal = env.call_int_method(mat, mid.m_ordinal, &[]);
    clear_if_pending(env);
    let info = mat_info(env, mat, &mid, ordinal).unwrap_or(MatInfo {
        top: gray,
        side: gray,
        tint_top: 0,
        tint_side: 0,
        tex_top: 0,
        tex_side: 0,
        baked: false,
        grass: false,
    });
    let (grass, foliage, water) = biome_tint(env, world, &wid, x, y, z);
    let t_top = match info.tint_top {
        1 => grass,
        2 => foliage,
        3 => water,
        _ => 0xFFFFFF,
    };
    let t_side = match info.tint_side {
        1 => grass,
        2 => foliage,
        3 => water,
        _ => 0xFFFFFF,
    };
    let mut top = mul_rgb(info.top, t_top);
    let mut side = mul_rgb(info.side, t_side);
    let mut tex_top = info.tex_top;
    let mut tex_side = info.tex_side;
    let mut height = y;
    let mut is_liquid = liquid;
    // Band 0's material flags: the surface block's own material, overridden
    // by the reconciliation branches below (a snow/water band never fades
    // to dirt; an avoided-block recolour keeps the GROUND material).
    let mut band0_grass = info.grass;

    // DH surface-layer semantics (FullDataToRenderDataTransformer):
    // whatever sits ON TOP of the heightmap block recolours the column —
    // a snow LAYER whitens the block below it, and water above solid ground
    // means the column renders as a water box. Heightmaps differ in whether
    // they already return that layer as the top block (WORLD_SURFACE does,
    // MOTION_BLOCKING does not), so read one block up and reconcile.
    if let Some(above) =
        jobj(env.call_object_method(world, wid.m_get_block_at, &[ji(x), ji(y + 1), ji(z)]))
    {
        clear_if_pending(env);
        if let Some(aid) = block_ids(env, above) {
            let amat = jobj(env.call_object_method(above, aid.m_get_type, &[]));
            clear_if_pending(env);
            if let Some(amat) = amat {
                if let Some(amid) = material_ids(env, amat) {
                    if let Some(name) = material_path(env, amat, &amid) {
                        match name.as_str() {
                            // snow layer(s): white top (DH tintWithAvoidedBlocks:
                            // the layer recolours the block below), height
                            // unchanged (a 1-block offset is invisible at cell
                            // resolution); snow's own ratio tile rides along
                            "snow" => {
                                top = 0xF9FEFE;
                                band0_grass = false; // a snow band never fades to dirt
                                if let Some((st, _ss)) = crate::blocktextures::lookup("snow") {
                                    tex_top = st;
                                }
                            }
                            // water over ground: the column IS a water box
                            // (ocean/lake floor under a MOTION_BLOCKING
                            // heightmap). Render at sea level with the
                            // water colour; the stored height becomes the
                            // surface the shader marches against.
                            "water" => {
                                is_liquid = true;
                                height = sea.max(y + 1);
                                let wb = crate::blockcolors::WATER_BASE;
                                top = mul_rgb(wb, water);
                                side = mul_rgb(wb, water);
                                if let Some((wt, ws)) = crate::blocktextures::lookup("water") {
                                    tex_top = wt;
                                    tex_side = ws;
                                }
                            }
                            // DH air-gap rule (FullDataToRenderDataTransformer
                            // "air gap leak fix"): AIR RESETS the avoided
                            // queue — a bare column keeps its own identity.
                            // Without this the probe below reads the AIR
                            // above every bare column, "avoids" it (air is
                            // non-colliding) and paints the whole surface
                            // with air's missing-table gray.
                            "air" | "cave_air" | "void_air" => {}
                            // DH blocksToIgnore=NON_COLLIDING +
                            // tintWithAvoidedBlocks=true (both defaults,
                            // Config.java:895/905): a non-colliding,
                            // non-liquid, non-opaque block above the surface
                            // — tall grass, ferns, flowers, torches — is
                            // SKIPPED by DH and its colour + material REPLACE
                            // the block below ("a red flower will tint the
                            // grass below it red"). The height stays at the
                            // ground level: the avoided block contributes no
                            // datapoint, only identity. Skipped when the
                            // ground is liquid (seagrass must not repaint
                            // the water surface; DH keeps the liquid branch).
                            _ => {
                                let above_liquid =
                                    env.call_boolean_method(above, aid.m_is_liquid, &[]);
                                clear_if_pending(env);
                                if !liquid && !above_liquid {
                                    let solid = env.call_boolean_method(amat, amid.m_is_solid, &[]);
                                    clear_if_pending(env);
                                    let occluding =
                                        env.call_boolean_method(amat, amid.m_is_occluding, &[]);
                                    clear_if_pending(env);
                                    if !solid && !occluding {
                                        // Table lookup DIRECTLY (not via
                                        // mat_info, whose miss falls back to
                                        // gray): a missing entry keeps the
                                        // ground block — never gray a live
                                        // surface over a lookup miss.
                                        if let Some((atop, aside, att, ast)) =
                                            crate::blockcolors::lookup(&name)
                                        {
                                            let a_t_top = match att {
                                                1 => grass,
                                                2 => foliage,
                                                3 => water,
                                                _ => 0xFFFFFF,
                                            };
                                            let a_t_side = match ast {
                                                1 => grass,
                                                2 => foliage,
                                                3 => water,
                                                _ => 0xFFFFFF,
                                            };
                                            top = mul_rgb(atop, a_t_top);
                                            side = mul_rgb(aside, a_t_side);
                                            if let Some((st, ss)) =
                                                crate::blocktextures::lookup(&name)
                                            {
                                                tex_top = st;
                                                tex_side = ss;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    SCAN_OK.fetch_add(1, Ordering::Relaxed);

    // ------------------------------------------------------------------
    // DH vertical slices (slices.rs): walk the column DOWN from the surface
    // through a ChunkSnapshot, cutting a band on every Material change
    // (LodDataBuilder cuts on blockstate change; Material is the same
    // identity at LOD resolution). Air starts no band — the shader extends
    // the band above over the gap, which reproduces DH's cave-culled cliff
    // look on a heightfield. Unbaked materials end the walk (the last band
    // reaches the world floor like a DH full-data column).
    // ------------------------------------------------------------------
    let mut runs: Vec<crate::slices::SliceRun> = Vec::new();
    if walk_depth > 0 && height >= -64 {
        let frame_ok = env.push_local_frame(96).is_ok();
        let walked = (|| -> Option<Vec<crate::slices::SliceRun>> {
            const RAW: usize = crate::atlas::SLICE_RAW;
            let chunk =
                jobj(env.call_object_method(world, wid.m_get_chunk_at, &[ji(x >> 4), ji(z >> 4)]))?;
            clear_if_pending(env);
            let cid = chunk_ids(env, chunk)?;
            let snap = jobj(env.call_object_method(
                chunk,
                cid.m_get_snapshot,
                &[jz(false), jz(false), jz(false)],
            ))?;
            clear_if_pending(env);
            let sid = snapshot_ids(env, snap)?;

            let block_at = |y: i32| -> Option<jobject> {
                let m = jobj(env.call_object_method(
                    snap,
                    sid.m_get_block_type,
                    &[ji(x & 15), ji(y), ji(z & 15)],
                ));
                let pending = env.exception_check();
                clear_if_pending(env);
                if pending {
                    None
                } else {
                    m
                }
            };
            let mid = material_ids(env, {
                // resolve MaterialIds once from the surface material
                block_at(height.max(-64))?
            })?;

            let resolve = |m: jobject| -> Option<crate::slices::SliceRun> {
                let ordinal = env.call_int_method(m, mid.m_ordinal, &[]);
                clear_if_pending(env);
                let info = mat_info(env, m, &mid, ordinal)?;
                if !info.baked {
                    return None; // unbaked: stop, the last band reaches down
                }
                let t_top = match info.tint_top {
                    1 => grass,
                    2 => foliage,
                    3 => water,
                    _ => 0xFFFFFF,
                };
                // Sub-band liquid stays false: the water SURFACE band is
                // band 0 (the reconciled surface), and DH never bands water
                // below terrain. Only the liquid flag of band 0 reaches the
                // shader.
                Some(crate::slices::SliceRun {
                    top: 0, // set when the band closes
                    color: mul_rgb(info.top, t_top),
                    tex_top: info.tex_top,
                    tex_side: info.tex_side,
                    liquid: false,
                    grass: info.grass,
                })
            };

            // Band 0 = the reconciled surface band (its material is the
            // heightmap block; the reconciliation may have recoloured it).
            let mut bands: Vec<crate::slices::SliceRun> = Vec::with_capacity(RAW);
            bands.push(crate::slices::SliceRun {
                top: height as i16,
                color: top,
                tex_top,
                tex_side,
                liquid: is_liquid,
                grass: band0_grass,
            });
            if bands.len() >= RAW {
                return Some(bands);
            }

            // The walk's identity anchor: the block AT the surface plane.
            let mut cur_mat = block_at(height.max(-64))?;
            let mut cur_is_air = env.call_boolean_method(cur_mat, mid.m_is_air, &[]);
            clear_if_pending(env);
            // cur holds the OPEN band's appearance (None over air gaps).
            let mut cur: Option<crate::slices::SliceRun> =
                if cur_is_air { None } else { Some(bands[0]) };
            let mut cur_start = height;
            let floor = (height - walk_depth).max(-65);

            let mut yy = height - 1;
            while yy >= floor {
                let Some(m) = block_at(yy) else { break };
                if !env.is_same_object(m, cur_mat) {
                    // Material change: close the open band (its top block is
                    // where it started), open the next one at yy.
                    if !cur_is_air {
                        if let Some(mut band) = cur {
                            band.top = cur_start as i16;
                            if bands.len() >= RAW {
                                break;
                            }
                            bands.push(band);
                        }
                    }
                    cur_mat = m;
                    cur_start = yy;
                    cur_is_air = env.call_boolean_method(m, mid.m_is_air, &[]);
                    clear_if_pending(env);
                    cur = if cur_is_air { None } else { resolve(m) };
                    if cur.is_none() && !cur_is_air {
                        break; // unbaked material: the last band reaches down
                    }
                }
                yy -= 1;
            }
            // Close the final open band (it reaches the world floor).
            if !cur_is_air {
                if let Some(mut band) = cur {
                    band.top = cur_start as i16;
                    if bands.len() < RAW {
                        bands.push(band);
                    }
                }
            }
            // Tops must strictly descend (surface band vs the first cut).
            for w in 1..bands.len() {
                if bands[w - 1].top <= bands[w].top {
                    bands[w].top = bands[w - 1].top.saturating_sub(1);
                }
            }
            Some(bands)
        })();

        if frame_ok {
            env.pop_local_frame(std::ptr::null_mut());
        }
        if let Some(w) = walked {
            runs = w;
        }
    }

    Some((height, is_liquid, top, side, tex_top, tex_side, runs))
}

fn method_or_log(env: Env, cls: jobject, name: &str, sig: &str) -> Option<jmethodID> {
    // The FIRST GetMethodID on a freshly-seen class can fail spuriously
    // under this JVMTI agent (NULL result, no pending exception) while the
    // immediate retry binds — a one-shot class-priming effect observed live.
    // Never give up on the first NULL; retry once before reporting failure.
    if let Some(id) = env.get_method_id(cls, name, sig) {
        clear_if_pending(env);
        return Some(id);
    }
    env.exception_clear();
    match env.get_method_id(cls, name, sig) {
        Some(id) => {
            clear_if_pending(env);
            Some(id)
        }
        None => {
            eprintln!("[c-LoD] method resolve FAILED: {name} {sig}");
            if env.exception_check() {
                env.exception_describe();
                env.exception_clear();
            }
            None
        }
    }
}

fn world_ids(env: Env, world: jobject) -> Option<WorldIds> {
    if let Some(w) = WORLD_IDS.lock().unwrap().as_ref() {
        return Some(*w);
    }
    let cls_local = env.get_object_class(world);
    let cls = {
        let g = env.new_global_ref(cls_local);
        g as jobject
    };
    let ids = WorldIds {
        m_name: bind_method(env, cls_local, cls, "getName", "()Ljava/lang/String;")?,
        m_highest_block: bind_method(env, cls_local, cls, "getHighestBlockYAt", "(II)I")?,
        m_chunk_loaded: bind_method(env, cls_local, cls, "isChunkLoaded", "(II)Z")?,
        m_set_force_loaded: bind_method(env, cls_local, cls, "setChunkForceLoaded", "(IIZ)V")?,
        m_get_full_time: bind_method(env, cls_local, cls, "getFullTime", "()J")?,
        m_get_chunk_async: bind_method(
            env,
            cls_local,
            cls,
            "getChunkAtAsyncUrgently",
            "(II)Ljava/util/concurrent/CompletableFuture;",
        )?,
        m_get_block_at: bind_method(
            env,
            cls_local,
            cls,
            "getBlockAt",
            "(III)Lorg/bukkit/block/Block;",
        )?,
        m_get_chunk_at: bind_method(env, cls_local, cls, "getChunkAt", "(II)Lorg/bukkit/Chunk;")?,
        m_get_biome: bind_method(
            env,
            cls_local,
            cls,
            "getBiome",
            "(III)Lorg/bukkit/block/Biome;",
        )?,
    };
    *WORLD_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

fn player_ids(env: Env, player: jobject) -> Option<PlayerIds> {
    if let Some(p) = PLAYER_IDS.lock().unwrap().as_ref() {
        return Some(*p);
    }
    let cls_local = env.get_object_class(player);
    let cls = {
        let g = env.new_global_ref(cls_local);
        g as jobject
    };
    let ids = PlayerIds {
        m_get_unique_id: bind_method(env, cls_local, cls, "getUniqueId", "()Ljava/util/UUID;")?,
        m_get_location: bind_method(
            env,
            cls_local,
            cls,
            "getLocation",
            "()Lorg/bukkit/Location;",
        )?,
        m_post_effects: bind_method(
            env,
            cls_local,
            cls,
            "postEffects",
            "()Lio/papermc/paper/entity/PlayerPostEffects;",
        )?,
        m_set_resource_pack: bind_method(
            env,
            cls_local,
            cls,
            "setResourcePack",
            "(Ljava/util/UUID;Ljava/lang/String;[BLjava/lang/String;Z)V",
        )?,
    };
    *PLAYER_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

fn location_ids(env: Env, loc: jobject) -> Option<LocationIds> {
    if let Some(l) = LOCATION_IDS.lock().unwrap().as_ref() {
        return Some(*l);
    }
    let cls_local = env.get_object_class(loc);
    let cls = {
        let g = env.new_global_ref(cls_local);
        g as jobject
    };
    let ids = LocationIds {
        m_get_block_x: bind_method(env, cls_local, cls, "getBlockX", "()I")?,
        m_get_block_y: bind_method(env, cls_local, cls, "getBlockY", "()I")?,
        m_get_block_z: bind_method(env, cls_local, cls, "getBlockZ", "()I")?,
        m_loc_get_world: bind_method(env, cls_local, cls, "getWorld", "()Lorg/bukkit/World;")?,
    };
    *LOCATION_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

fn block_ids(env: Env, block: jobject) -> Option<BlockIds> {
    if let Some(b) = BLOCK_IDS.lock().unwrap().as_ref() {
        return Some(*b);
    }
    let cls_local = env.get_object_class(block);
    let cls = {
        let g = env.new_global_ref(cls_local);
        g as jobject
    };
    let ids = BlockIds {
        m_is_liquid: bind_method(env, cls_local, cls, "isLiquid", "()Z")?,
        m_get_type: bind_method(env, cls_local, cls, "getType", "()Lorg/bukkit/Material;")?,
    };
    *BLOCK_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

// ---------------------------------------------------------------------------
// One-time resolve of the static surface.
// ---------------------------------------------------------------------------

/// Bind a virtual method through the probe-then-globalize sequence that
/// works under this agent (see resolve(): local-ref GetMethodID fails
/// silently until the class was forced to link by a local probe).
fn bind_method(
    env: Env,
    local_cls: jobject,
    global_cls: jobject,
    name: &str,
    sig: &str,
) -> Option<jmethodID> {
    let _ = env.get_method_id(local_cls, name, sig); // probe: may fail, primes the class
    env.exception_clear();
    match env.get_method_id(global_cls, name, sig) {
        Some(id) => {
            clear_if_pending(env);
            Some(id)
        }
        None => {
            eprintln!("[c-LoD] method resolve FAILED: {name} {sig}");
            if env.exception_check() {
                env.exception_describe();
                env.exception_clear();
            }
            None
        }
    }
}

fn resolve(env: Env) -> Option<Ids> {
    use cplug_sdk::classes as sdkcls;

    // JVMTI scan (loader-independent) — ONLY for single-copy bootstrap
    // classes and Bukkit. Anything Paper-API-specific uses live objects.
    macro_rules! cls {
        ($name:expr) => {{
            match sdkcls::find_class($name) {
                Some(c) => {
                    clear_if_pending(env);
                    c.as_jclass()
                }
                None => {
                    eprintln!("[c-LoD] class resolve FAILED: {}", $name);
                    return None;
                }
            }
        }};
    }
    macro_rules! m {
        ($c:expr, $n:expr, $s:expr) => {
            method_or_log(env, $c, $n, $s)?
        };
    }
    macro_rules! sm {
        ($c:expr, $n:expr, $s:expr) => {
            match env.get_static_method_id($c, $n, $s) {
                Some(id) => {
                    clear_if_pending(env);
                    id
                }
                None => {
                    eprintln!("[c-LoD] static method resolve FAILED: {} {}", $n, $s);
                    if env.exception_check() {
                        env.exception_describe();
                        env.exception_clear();
                    }
                    return None;
                }
            }
        };
    }

    // Bootstrap classes live in a single stable copy — the JVMTI scan is
    // safe for them. EVERYTHING from paper-api is resolved through
    // Class.forName in CraftServer's own loader instead: the scan can grab
    // an API class mid-redefinition (Paper remaps API classes during boot)
    // and cache a stale global ref whose method table mismatches forever
    // (GetMethodID silently returns NULL for every method).
    let collection_cls = cls!("java/util/Collection");
    let iterator_cls = cls!("java/util/Iterator");
    let list_cls = cls!("java/util/List");
    let uuid_cls = cls!("java/util/UUID");
    let class_cls = cls!("java/lang/Class");

    let m_for_name = sm!(
        class_cls,
        "forName",
        "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;"
    );
    let m_iterator = m!(collection_cls, "iterator", "()Ljava/util/Iterator;");
    let m_has_next = m!(iterator_cls, "hasNext", "()Z");
    let m_next = m!(iterator_cls, "next", "()Ljava/lang/Object;");
    let m_list_size = m!(list_cls, "size", "()I");
    let m_list_get = m!(list_cls, "get", "(I)Ljava/lang/Object;");
    let m_uuid_to_string = m!(uuid_cls, "toString", "()Ljava/lang/String;");
    let m_uuid_from_string = sm!(
        uuid_cls,
        "fromString",
        "(Ljava/lang/String;)Ljava/util/UUID;"
    );

    // Probe the live server through the scanned Bukkit (bootstrap-adjacent,
    // verified working: getServer is its only use) — the probe object is
    // then the ground truth for CraftServer's loader.
    let bukkit_probe = cls!("org/bukkit/Bukkit");
    let m_get_server_probe = sm!(bukkit_probe, "getServer", "()Lorg/bukkit/Server;");
    let server_obj = jobj(env.call_static_object_method(bukkit_probe, m_get_server_probe, &[]));
    clear_if_pending(env);
    let server_obj = server_obj?;

    // Live server -> concrete CraftServer class -> server-surface methods.
    let server_cls = env.get_object_class(server_obj);

    // Force-load + initialize the API classes in CraftServer's loader and
    // keep the EXACT Class objects (no scan ambiguity, no stale cache):
    // server.getClass().getClassLoader() -> Class.forName(name, true, cl).
    let m_get_class = m!(class_cls, "getClass", "()Ljava/lang/Class;");
    let m_get_class_loader = m!(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;");
    let k = jobj(env.call_object_method(server_obj, m_get_class, &[]));
    let loader = k.and_then(|k| jobj(env.call_object_method(k, m_get_class_loader, &[])));
    let loader = loader?;
    let force_load = |dotted: &str| -> Option<jobject> {
        let jn = env.new_string_utf(dotted)?;
        let k =
            env.call_static_object_method(class_cls, m_for_name, &[jv(jn), jz(true), jv(loader)]);
        clear_if_pending(env);
        jobj(k)
    };

    let bukkit_cls = force_load("org.bukkit.Bukkit")?;
    let key_cls = force_load("net.kyori.adventure.key.Key")?;
    let post_effects_cls = force_load("io.papermc.paper.entity.PlayerPostEffects")?;
    // getServer re-resolved on the EXACT Bukkit class from this loader.
    let m_get_server = sm!(bukkit_cls, "getServer", "()Lorg/bukkit/Server;");

    // JNI quirk discovered live under this JVMTI agent: GetMethodID on a
    // LOCAL-ref jclass returns NULL (no pending exception); the SAME class
    // through a GLOBAL ref binds — but only after a failed local-ref attempt
    // forced the class to finish linking (probe-then-globalize,
    // see bind_method). Used for every Paper-API method below.
    let server_cls_local = server_cls;
    let server_cls = {
        let g = env.new_global_ref(server_cls);
        g as jobject
    };

    let m_server_players = bind_method(
        env,
        server_cls_local,
        server_cls,
        "getOnlinePlayers",
        "()Ljava/util/Collection;",
    )?;
    let m_server_worlds = bind_method(
        env,
        server_cls_local,
        server_cls,
        "getWorlds",
        "()Ljava/util/List;",
    )?;
    #[allow(clippy::used_underscore_binding)]
    let m_key_key = sm!(
        key_cls,
        "key",
        "(Ljava/lang/String;)Lnet/kyori/adventure/key/Key;"
    );
    let m_pe_add = m!(post_effects_cls, "add", "(Lnet/kyori/adventure/key/Key;)Z");
    let m_pe_remove = m!(
        post_effects_cls,
        "remove",
        "(Lnet/kyori/adventure/key/Key;)Z"
    );
    let m_pe_update = m!(post_effects_cls, "update", "()V");

    Some(Ids {
        bukkit_cls,
        m_get_server,
        m_server_players,
        m_server_worlds,
        m_iterator,
        m_has_next,
        m_next,
        m_list_size,
        m_list_get,
        uuid_cls,
        m_uuid_to_string,
        m_uuid_from_string,
        key_cls,
        m_key_key,
        m_pe_add,
        m_pe_remove,
        m_pe_update,
    })
}

/// Resolve IDs. Call from the main thread each tick until it returns true.
///
/// The static surface requires a live server object (CraftServer methods),
/// which only exists after Bukkit.setServer — before that, retry next tick.
pub fn warmup(env: Env) -> bool {
    if let Some(s) = IDS.get() {
        return s.is_some();
    }
    match resolve(env) {
        Some(ids) => {
            let _ = IDS.set(Some(SendIds(ids)));
            true
        }
        None => {
            clear_if_pending(env);
            false
        }
    }
}

// ---------------------------------------------------------------------------
// Data returned to the engine.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub uuid: String,
    pub world: String,
    pub bx: i32,
    #[allow(dead_code)] // y is useful for future band filtering
    pub by: i32,
    pub bz: i32,
}

pub struct WorldRef {
    pub name: String,
    pub obj: jobject,
}

// ---------------------------------------------------------------------------
// Bridge functions (main thread, attached env).
// ---------------------------------------------------------------------------

fn server(env: Env, ids: &Ids) -> Option<jobject> {
    jobj(env.call_static_object_method(ids.bukkit_cls, ids.m_get_server, &[]))
}

fn online_players_iter(env: Env, ids: &Ids) -> Option<jobject> {
    let srv = server(env, ids)?;
    let collection = jobj(env.call_object_method(srv, ids.m_server_players, &[]))?;
    let iterator = jobj(env.call_object_method(collection, ids.m_iterator, &[]))?;
    Some(iterator)
}

pub fn players(env: Env) -> Vec<PlayerInfo> {
    let Some(ids) = ids() else { return Vec::new() };
    let Some(iterator) = online_players_iter(env, ids) else {
        clear_if_pending(env);
        return Vec::new();
    };

    let mut out = Vec::new();
    loop {
        if !env.call_boolean_method(iterator, ids.m_has_next, &[]) {
            break;
        }
        let Some(player) = jobj(env.call_object_method(iterator, ids.m_next, &[])) else {
            break;
        };
        let Some(pids) = player_ids(env, player) else {
            clear_if_pending(env);
            continue;
        };

        let Some(uuid) = jobj(env.call_object_method(player, pids.m_get_unique_id, &[]))
            .and_then(|u| jobj(env.call_object_method(u, ids.m_uuid_to_string, &[])))
            .and_then(|s| jstr(env, s))
        else {
            clear_if_pending(env);
            continue;
        };

        let Some(loc) = jobj(env.call_object_method(player, pids.m_get_location, &[])) else {
            clear_if_pending(env);
            continue;
        };
        let Some(lids) = location_ids(env, loc) else {
            clear_if_pending(env);
            continue;
        };
        let bx = env.call_int_method(loc, lids.m_get_block_x, &[]);
        let by = env.call_int_method(loc, lids.m_get_block_y, &[]);
        let bz = env.call_int_method(loc, lids.m_get_block_z, &[]);
        let world = jobj(env.call_object_method(loc, lids.m_loc_get_world, &[]))
            .and_then(|w| {
                let wid = world_ids(env, w)?;
                jobj(env.call_object_method(w, wid.m_name, &[]))
            })
            .and_then(|s| jstr(env, s))
            .unwrap_or_default();
        clear_if_pending(env);

        out.push(PlayerInfo {
            uuid,
            world,
            bx,
            by,
            bz,
        });
    }
    clear_if_pending(env);
    out
}

/// World handle by name (first world when `name` is empty).
pub fn find_world(env: Env, name: &str) -> Option<WorldRef> {
    let ids = ids()?;
    let srv = server(env, ids)?;
    let list = jobj(env.call_object_method(srv, ids.m_server_worlds, &[]))?;
    let n = env.call_int_method(list, ids.m_list_size, &[]);
    for i in 0..n {
        let Some(world) = jobj(env.call_object_method(list, ids.m_list_get, &[ji(i)])) else {
            clear_if_pending(env);
            continue;
        };
        let Some(wid) = world_ids(env, world) else {
            clear_if_pending(env);
            continue;
        };
        let Some(wname) =
            jobj(env.call_object_method(world, wid.m_name, &[])).and_then(|s| jstr(env, s))
        else {
            clear_if_pending(env);
            continue;
        };
        if name.is_empty() || wname == name {
            return Some(WorldRef {
                name: wname,
                obj: world,
            });
        }
    }
    clear_if_pending(env);
    None
}

/// Force-load one chunk (vanilla /forceload semantics: load + generate +
/// RETAIN until un-forced). Paper unloads freshly generated chunks within
/// seconds when no players are near, so plain getChunkAt warm-up evaporates
/// before the sweep reaches the cells.
pub fn warm_chunk(env: Env, world: jobject, cx: i32, cz: i32, retain: bool) -> bool {
    let Some(wid) = world_ids(env, world) else {
        clear_if_pending(env);
        return false;
    };
    if retain {
        env.call_void_method(world, wid.m_set_force_loaded, &[ji(cx), ji(cz), jz(true)]);
        let ok = !env.exception_check();
        clear_if_pending(env);
        return ok;
    }
    // Release pass: un-force (the engine calls this at finalize).
    env.call_void_method(world, wid.m_set_force_loaded, &[ji(cx), ji(cz), jz(false)]);
    let ok = !env.exception_check();
    clear_if_pending(env);
    ok
}

/// URGENT async chunk request (Paper getChunkAtAsyncUrgently) returning the
/// future. The caller HOLDS the future (global ref) and polls isDone(): the
/// chunk system keeps the chunk loaded until the future completes, so the
/// completion -> sample window cannot be missed between engine ticks, and
/// the main thread never blocks on a cold-chunk neighbour wait the way a
/// synchronous setChunkForceLoaded does (live-observed watchdog kill).
pub fn request_chunk_future(env: Env, world: jobject, cx: i32, cz: i32) -> Option<jobject> {
    let wid = world_ids(env, world)?;
    let fut = jobj(env.call_object_method(world, wid.m_get_chunk_async, &[ji(cx), ji(cz)]));
    clear_if_pending(env);
    fut
}

/// True when the held chunk-request future completed (chunk resident or the
/// request answered).
pub fn future_done(env: Env, fut: jobject) -> Option<bool> {
    let ids = future_ids(env, fut)?;
    let done = env.call_boolean_method(fut, ids.m_is_done, &[]);
    let pending = env.exception_check();
    clear_if_pending(env);
    if pending {
        return None;
    }
    Some(done)
}

/// Full game time of the world (ticks since creation). Cheap probe; the
/// engine compares consecutive values to detect a live tick loop.
pub fn world_game_time(env: Env, world: jobject) -> Option<i64> {
    let wid = world_ids(env, world)?;
    let t = env.call_long_method(world, wid.m_get_full_time, &[]);
    let pending = env.exception_check();
    clear_if_pending(env);
    if pending {
        return None;
    }
    Some(t)
}

/// Diagnostic: direct isChunkLoaded probe.
#[allow(dead_code)]
pub fn is_chunk_loaded(env: Env, world: jobject, cx: i32, cz: i32) -> Option<bool> {
    let wid = world_ids(env, world)?;
    let v = env.call_boolean_method(world, wid.m_chunk_loaded, &[ji(cx), ji(cz)]);
    let pending = env.exception_check();
    clear_if_pending(env);
    if pending {
        return None;
    }
    Some(v)
}

/// Diagnostic: direct getHighestBlockYAt probe.
#[allow(dead_code)]
pub fn probe_height(env: Env, world: jobject, x: i32, z: i32) -> Option<i32> {
    let wid = world_ids(env, world)?;
    let y = env.call_int_method(world, wid.m_highest_block, &[ji(x), ji(z)]);
    let pending = env.exception_check();
    clear_if_pending(env);
    if pending {
        return None;
    }
    Some(y)
}

/// Sample the surface height at one column. Returns (blockY, isLiquid).
pub static SCAN_OK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static SCAN_UNLOADED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static SCAN_ERR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[allow(dead_code)] // reserved for the pending-scan probe path
pub fn sample_height(env: Env, world: jobject, x: i32, z: i32) -> Option<(i32, bool)> {
    use std::sync::atomic::Ordering;
    let wid = world_ids(env, world)?;
    if env.exception_check() {
        env.exception_clear();
        SCAN_ERR.fetch_add(1, Ordering::Relaxed);
        return None;
    }
    // Only sample loaded chunks: getHighestBlockYAt would otherwise force
    // synchronous chunk generation and stall the main thread for minutes.
    // Unloaded cells stay invalid and get picked up by later sweeps (the
    // scan task wraps around the atlas) once players load those chunks.
    let loaded = env.call_boolean_method(world, wid.m_chunk_loaded, &[ji(x >> 4), ji(z >> 4)]);
    if env.exception_check() {
        env.exception_clear();
        return None;
    }
    if !loaded {
        SCAN_UNLOADED.fetch_add(1, Ordering::Relaxed);
        return None;
    }
    let y = env.call_int_method(world, wid.m_highest_block, &[ji(x), ji(z)]);
    if env.exception_check() {
        env.exception_clear();
        SCAN_ERR.fetch_add(1, Ordering::Relaxed);
        return None;
    }
    if y < -64 {
        return Some((y, false));
    }
    let Some(block) =
        jobj(env.call_object_method(world, wid.m_get_block_at, &[ji(x), ji(y), ji(z)]))
    else {
        clear_if_pending(env);
        return Some((y, false));
    };
    let Some(bid) = block_ids(env, block) else {
        clear_if_pending(env);
        return Some((y, false));
    };
    let liquid = env.call_boolean_method(block, bid.m_is_liquid, &[]);
    clear_if_pending(env);
    SCAN_OK.fetch_add(1, Ordering::Relaxed);
    Some((y, liquid))
}

// ---------------------------------------------------------------------------
// Actions: resource pack + post effects.
// ---------------------------------------------------------------------------

pub fn parse_uuid(env: Env, s: &str) -> Option<jobject> {
    let ids = ids()?;
    let j = env.new_string_utf(s)?;
    let u = env.call_static_object_method(ids.uuid_cls, ids.m_uuid_from_string, &[jv(j)]);
    clear_if_pending(env);
    jobj(u)
}

pub fn make_key(env: Env, id: &str) -> Option<jobject> {
    let ids = ids()?;
    let jid = env.new_string_utf(id)?;
    let k = env.call_static_object_method(ids.key_cls, ids.m_key_key, &[jv(jid)]);
    clear_if_pending(env);
    jobj(k)
}

/// Player object lookup by uuid string (main thread).
pub fn player_by_uuid(env: Env, uuid: &str) -> Option<jobject> {
    let ids = ids()?;
    let Some(iterator) = online_players_iter(env, ids) else {
        clear_if_pending(env);
        return None;
    };
    while env.call_boolean_method(iterator, ids.m_has_next, &[]) {
        let Some(player) = jobj(env.call_object_method(iterator, ids.m_next, &[])) else {
            break;
        };
        let Some(pids) = player_ids(env, player) else {
            clear_if_pending(env);
            continue;
        };
        let Some(puuid) = jobj(env.call_object_method(player, pids.m_get_unique_id, &[]))
            .and_then(|u| jobj(env.call_object_method(u, ids.m_uuid_to_string, &[])))
            .and_then(|s| jstr(env, s))
        else {
            clear_if_pending(env);
            continue;
        };
        if puuid == uuid {
            return Some(player);
        }
    }
    clear_if_pending(env);
    None
}

pub fn push_pack(
    env: Env,
    player: jobject,
    pack_uuid: jobject,
    url: &str,
    sha1: &[u8; 20],
    prompt: &str,
    force: bool,
) -> bool {
    if ids().is_none() {
        return false; // bridge not resolved yet
    }
    let Some(pids) = player_ids(env, player) else {
        clear_if_pending(env);
        return false;
    };
    let Some(jurl) = env.new_string_utf(url) else {
        return false;
    };
    let Some(jprompt) = env.new_string_utf(prompt) else {
        return false;
    };
    let Some(jarr) = env.new_byte_array(20) else {
        return false;
    };
    let bytes: Vec<jvmti_bindings::jni::jbyte> = sha1.iter().map(|b| *b as i8).collect();
    env.set_byte_array_region(jarr, 0, 20, &bytes);

    let args = [jv(pack_uuid), jv(jurl), jv(jarr), jv(jprompt), jz(force)];
    env.call_void_method(player, pids.m_set_resource_pack, &args);
    if env.exception_check() {
        // Surface the server-side failure instead of swallowing it: without
        // this the pack silently never reaches the client.
        env.exception_describe();
        crate::log_info("pack push: setResourcePack threw (exception above)");
    }
    let ok = !env.exception_check();
    clear_if_pending(env);
    ok
}

/// Re-sends the player's current post-effect list to the client. The pack is
/// static and the effect list is unchanged, so this is a no-op on the wire
/// unless the client just finished applying the pack — in which case it lets
/// the client resolve chains it couldn't load during the push/reload race.
pub fn update_effects(env: Env, player: jobject) -> bool {
    let Some(ids) = ids() else { return false };
    let Some(pids) = player_ids(env, player) else {
        clear_if_pending(env);
        return false;
    };
    let Some(pe) = jobj(env.call_object_method(player, pids.m_post_effects, &[])) else {
        clear_if_pending(env);
        return false;
    };
    env.call_void_method(pe, ids.m_pe_update, &[]);
    if env.exception_check() {
        env.exception_describe();
    }
    let ok = !env.exception_check();
    clear_if_pending(env);
    ok
}

pub fn add_effect(env: Env, player: jobject, key: jobject) -> bool {
    let Some(ids) = ids() else { return false };
    let Some(pids) = player_ids(env, player) else {
        clear_if_pending(env);
        return false;
    };
    let Some(pe) = jobj(env.call_object_method(player, pids.m_post_effects, &[])) else {
        clear_if_pending(env);
        return false;
    };
    let ok = env.call_boolean_method(pe, ids.m_pe_add, &[jv(key)]);
    clear_if_pending(env);
    ok
}

pub fn remove_effect(env: Env, player: jobject, key: jobject) -> bool {
    let Some(ids) = ids() else { return false };
    let Some(pids) = player_ids(env, player) else {
        clear_if_pending(env);
        return false;
    };
    let Some(pe) = jobj(env.call_object_method(player, pids.m_post_effects, &[])) else {
        clear_if_pending(env);
        return false;
    };
    let ok = env.call_boolean_method(pe, ids.m_pe_remove, &[jv(key)]);
    clear_if_pending(env);
    ok
}
