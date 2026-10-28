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

use jvmti_bindings::jni::jfieldID;
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

fn jf(v: f32) -> jvalue {
    jvalue {
        f: v as jvmti_bindings::jni::jfloat,
    }
}

fn jd(v: f64) -> jvalue {
    jvalue {
        d: v as jvmti_bindings::jni::jdouble,
    }
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
    /// v5.0.2: CraftWorld.getHandle() -> ServerLevel (moonrise ticket bridge).
    m_get_handle: jmethodID,
    /// v4.2 surface colors: RegionAccessor#getBiome(int,int,int).
    /// Optional: absent -> palette falls back to a neutral tint.
    m_get_biome: Option<jmethodID>,
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
    /// v4.2 surface colors: Block#getType() -> org.bukkit.Material.
    /// Optional: absent -> palette falls back to a neutral tint.
    m_get_type: Option<jmethodID>,
}
// Raw method-ID pointers cached process-lifetime; consumed main-thread-only.
unsafe impl Send for BlockIds {}
static BLOCK_IDS: Mutex<Option<BlockIds>> = Mutex::new(None);

/// Enum#name() resolved on the live Material/Biome classes (both are enums,
/// same method id shape; cached per class).
#[derive(Clone, Copy)]
struct EnumIds {
    m_name: jmethodID,
}
unsafe impl Send for EnumIds {}
static MATERIAL_ENUM: Mutex<Option<(usize, EnumIds)>> = Mutex::new(None);
static BIOME_ENUM: Mutex<Option<(usize, EnumIds)>> = Mutex::new(None);

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
        m_get_handle: bind_method(
            env,
            cls_local,
            cls,
            "getHandle",
            "()Lnet/minecraft/server/level/ServerLevel;",
        )?,
        m_get_block_at: bind_method(
            env,
            cls_local,
            cls,
            "getBlockAt",
            "(III)Lorg/bukkit/block/Block;",
        )?,
        // Color path (v4.2). Resolution failure is not fatal: the world is
        // still scanned, cells just keep the neutral palette color. Probe the
        // local class first (the agent's one-shot class-priming quirk, see
        // method_or_log).
        m_get_biome: {
            let _ = env.get_method_id(cls_local, "getBiome", "(III)Lorg/bukkit/block/Biome;");
            env.exception_clear();
            env.get_method_id(cls, "getBiome", "(III)Lorg/bukkit/block/Biome;")
                .or_else(|| {
                    env.exception_clear();
                    None
                })
        },
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
        m_get_type: {
            let _ = env.get_method_id(cls_local, "getType", "()Lorg/bukkit/Material;");
            env.exception_clear();
            env.get_method_id(cls, "getType", "()Lorg/bukkit/Material;")
                .or_else(|| {
                    env.exception_clear();
                    None
                })
        },
    };
    *BLOCK_IDS.lock().unwrap() = Some(ids);
    Some(ids)
}

/// Enum#name() resolved once per concrete enum class (Material, Biome).
fn enum_name_ids(
    env: Env,
    obj: jobject,
    cache: &Mutex<Option<(usize, EnumIds)>>,
) -> Option<EnumIds> {
    let cls = env.get_object_class(obj);
    let key = cls as usize;
    if let Some((k, ids)) = cache.lock().unwrap().as_ref() {
        if *k == key {
            return Some(*ids);
        }
    }
    let ids = EnumIds {
        m_name: bind_method(env, cls, cls, "name", "()Ljava/lang/String;")?,
    };
    *cache.lock().unwrap() = Some((key, ids));
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
    let server_obj = server_obj.or_else(|| {
        eprintln!("[c-LoD] resolve FAILED at: Bukkit.getServer probe");
        None
    })?;

    // Live server -> concrete CraftServer class -> server-surface methods.
    let server_cls = env.get_object_class(server_obj);

    // Force-load + initialize the API classes in CraftServer's loader and
    // keep the EXACT Class objects (no scan ambiguity, no stale cache):
    // server.getClass().getClassLoader() -> Class.forName(name, true, cl).
    let m_get_class = m!(class_cls, "getClass", "()Ljava/lang/Class;");
    let m_get_class_loader = m!(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;");
    let k = jobj(env.call_object_method(server_obj, m_get_class, &[]));
    let loader = k.and_then(|k| jobj(env.call_object_method(k, m_get_class_loader, &[])));
    let loader = loader.or_else(|| {
        eprintln!("[c-LoD] resolve FAILED at: CraftServer classloader");
        None
    })?;
    stash_loader(env, loader);
    let force_load = |dotted: &str| -> Option<jobject> {
        let jn = env.new_string_utf(dotted)?;
        let k =
            env.call_static_object_method(class_cls, m_for_name, &[jv(jn), jz(true), jv(loader)]);
        clear_if_pending(env);
        jobj(k)
    };

    let bukkit_cls = force_load("org.bukkit.Bukkit").or_else(|| {
        eprintln!("[c-LoD] resolve FAILED at: force_load org.bukkit.Bukkit");
        None
    })?;
    let key_cls = force_load("net.kyori.adventure.key.Key").or_else(|| {
        eprintln!("[c-LoD] resolve FAILED at: force_load Key");
        None
    })?;
    let post_effects_cls =
        force_load("io.papermc.paper.entity.PlayerPostEffects").or_else(|| {
            eprintln!("[c-LoD] resolve FAILED at: force_load PlayerPostEffects");
            None
        })?;
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
    )
    .or_else(|| {
        eprintln!("[c-LoD] resolve FAILED at: getOnlinePlayers");
        None
    })?;
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
    eprintln!("[c-LoD] warmup: resolve attempt");
    match resolve(env) {
        Some(ids) => {
            eprintln!("[c-LoD] warmup: resolve OK");
            let _ = IDS.set(Some(SendIds(ids)));
            true
        }
        None => {
            eprintln!("[c-LoD] warmup: resolve returned None");
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

// ---------------------------------------------------------------------------
// v5.0.2 moonrise ticket bridge — NON-BLOCKING chunk generation requests.
//
// v4.4's Bukkit setChunkForceLoaded surface was REMOVED: on a fresh world it
// SYNCHRONOUSLY generated each frontier chunk on the main thread (BLOCKING
// priority syncLoad, ServerLevel.setChunkForced -> getChunk -> syncLoad) and
// stalled the server for minutes (watchdog "has not responded for 10
// seconds"), starving the pack HTTP surface with it. The ticket bridge below
// rides the engine Paper itself uses for async work:
//
// The fix rides the engine Paper itself uses for async chunk work:
//   ServerChunkCache.addTicketAndLoadWithRadius(TicketType, ChunkPos, int)
//     -> ChunkHolderManager ticket + ASYNC gen at NORMAL priority,
//        returns a CompletableFuture we do not even need to keep (completion
//        is polled via isChunkLoaded by the engine tick).
//   ServerChunkCache.removeTicketWithRadius(TicketType, ChunkPos, int)
//     -> release after sampling (or at finalize).
// TicketType.CHUNK_LOAD is the load-only, non-persistent type (flags=2:
// FLAG_LOADING) — the chunk is loaded to FULL for data access without
// ticking, and nothing leaks into level.dat.
//
// Everything resolves through CraftServer's loader (same as the NMS surface
// below); every call is main-thread-only.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct GenIds {
    /// net.minecraft.server.level.ServerLevel#getChunkSource()
    m_get_chunk_source: jmethodID,
    /// addTicketAndLoadWithRadius(TicketType, ChunkPos, int) -> CompletableFuture
    m_add_ticket: jmethodID,
    /// removeTicketWithRadius(TicketType, ChunkPos, int)
    m_remove_ticket: jmethodID,
    /// TicketType.FORCED (static field): flags=15 — guaranteed generation
    /// (vanilla /forceload semantics), non-persistent once removed.
    ticket_type_cls: jobject,
    f_chunk_load: jfieldID,
    /// ChunkPos.<init>(II)V + its class.
    chunk_pos_cls: jobject,
    m_chunk_pos_new: jmethodID,
}
struct SendGenIds(GenIds);
unsafe impl Send for SendGenIds {}
unsafe impl Sync for SendGenIds {}
static GEN_IDS: OnceLock<Option<SendGenIds>> = OnceLock::new();

fn gen() -> Option<&'static GenIds> {
    GEN_IDS.get().and_then(|o| o.as_ref()).map(|s| &s.0)
}

macro_rules! gstep {
    ($name:expr, $e:expr) => {
        match $e {
            Some(v) => v,
            None => {
                eprintln!("[c-LoD] gen resolve FAILED at: {}", $name);
                return None;
            }
        }
    };
}

fn resolve_gen(env: Env) -> Option<GenIds> {
    use cplug_sdk::classes as sdkcls;
    let loader = *LOADER.lock().unwrap();
    if loader == 0 {
        eprintln!("[c-LoD] gen resolve FAILED at: loader stash empty");
        return None;
    }
    let class_cls = sdkcls::find_class("java/lang/Class")
        .map(|c| c.as_jclass())
        .or_else(|| {
            eprintln!("[c-LoD] gen resolve FAILED at: find_class Class");
            None
        })?;
    let m_for_name = env
        .get_static_method_id(
            class_cls,
            "forName",
            "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
        )
        .or_else(|| {
            eprintln!("[c-LoD] gen resolve FAILED at: forName method id");
            None
        })?;
    let load = |dotted: &str| -> Option<jobject> {
        let jn = env.new_string_utf(dotted)?;
        let k = env.call_static_object_method(
            class_cls,
            m_for_name,
            &[jv(jn), jz(true), jv(loader as jobject)],
        );
        clear_if_pending(env);
        match jobj(k) {
            Some(c) => Some(env.new_global_ref(c) as jobject),
            None => {
                eprintln!("[c-LoD] gen resolve FAILED at: forName({dotted})");
                None
            }
        }
    };

    let server_level_cls = gstep!(
        "server_level_cls",
        load("net.minecraft.server.level.ServerLevel")
    );
    let chunk_cache_cls = gstep!(
        "chunk_cache_cls",
        load("net.minecraft.server.level.ServerChunkCache")
    );
    let ticket_type_cls = gstep!(
        "ticket_type_cls",
        load("net.minecraft.server.level.TicketType")
    );
    let chunk_pos_cls = gstep!("chunk_pos_cls", load("net.minecraft.world.level.ChunkPos"));

    let m_get_chunk_source = gstep!(
        "m_get_chunk_source",
        bind_method(
            env,
            server_level_cls,
            server_level_cls,
            "getChunkSource",
            "()Lnet/minecraft/server/level/ServerChunkCache;",
        )
    );
    let m_add_ticket = gstep!(
        "m_add_ticket",
        bind_method(
            env,
            chunk_cache_cls,
            chunk_cache_cls,
            "addTicketAndLoadWithRadius",
            "(Lnet/minecraft/server/level/TicketType;Lnet/minecraft/world/level/ChunkPos;I)Ljava/util/concurrent/CompletableFuture;",
        )
    );
    let m_remove_ticket = gstep!(
        "m_remove_ticket",
        bind_method(
            env,
            chunk_cache_cls,
            chunk_cache_cls,
            "removeTicketWithRadius",
            "(Lnet/minecraft/server/level/TicketType;Lnet/minecraft/world/level/ChunkPos;I)V",
        )
    );
    let f_chunk_load = gstep!(
        "f_chunk_load",
        env.get_static_field_id(
            ticket_type_cls,
            "FORCED",
            "Lnet/minecraft/server/level/TicketType;",
        )
    );
    let m_chunk_pos_new = gstep!(
        "m_chunk_pos_new",
        env.get_method_id(chunk_pos_cls, "<init>", "(II)V")
    );

    Some(GenIds {
        m_get_chunk_source,
        m_add_ticket,
        m_remove_ticket,
        ticket_type_cls,
        f_chunk_load,
        chunk_pos_cls,
        m_chunk_pos_new,
    })
}

/// The FORCED TicketType object (generation guaranteed) (static field read; local ref, valid for
/// the current call frame only).
fn chunk_load_type(env: Env, g: &GenIds) -> Option<jobject> {
    let t = env.get_static_object_field(g.ticket_type_cls, g.f_chunk_load);
    jobj(t)
}

/// Request an ASYNC chunk generation + retention ticket (moonrise FORCED).
/// Non-blocking: the chunk generates at NORMAL priority on the chunk system's
/// workers; the engine polls isChunkLoaded. Returns false when the bridge is
/// unresolved or the call raised.
pub fn request_chunk_gen(env: Env, world: jobject, cx: i32, cz: i32) -> bool {
    if GEN_IDS.get().is_none() {
        let _ = GEN_IDS.set(match resolve_gen(env) {
            Some(g) => Some(SendGenIds(g)),
            None => {
                clear_if_pending(env);
                None
            }
        });
    }
    let Some(g) = gen() else {
        clear_if_pending(env);
        return false;
    };
    let Some(wid) = world_ids(env, world) else {
        clear_if_pending(env);
        return false;
    };
    let Some(level) = jobj(env.call_object_method(world, wid.m_get_handle, &[])) else {
        clear_if_pending(env);
        return false;
    };
    let Some(cache) = jobj(env.call_object_method(level, g.m_get_chunk_source, &[])) else {
        clear_if_pending(env);
        return false;
    };
    let Some(ttype) = chunk_load_type(env, g) else {
        clear_if_pending(env);
        return false;
    };
    let Some(pos) = env.new_object(g.chunk_pos_cls, g.m_chunk_pos_new, &[ji(cx), ji(cz)]) else {
        clear_if_pending(env);
        return false;
    };
    // The returned future is intentionally dropped: ticket retention is
    // tracked by the engine's gen_wish ledger + isChunkLoaded polling.
    let _future = env.call_object_method(cache, g.m_add_ticket, &[jv(ttype), jv(pos), ji(0)]);
    let ok = !env.exception_check();
    if !ok {
        env.exception_describe();
    }
    clear_if_pending(env);
    ok
}

/// Release the chunk's FORCED ticket (after sampling or at finalize).
pub fn release_chunk_ticket(env: Env, world: jobject, cx: i32, cz: i32) -> bool {
    let Some(g) = gen() else {
        clear_if_pending(env);
        return false;
    };
    let Some(wid) = world_ids(env, world) else {
        clear_if_pending(env);
        return false;
    };
    let Some(level) = jobj(env.call_object_method(world, wid.m_get_handle, &[])) else {
        clear_if_pending(env);
        return false;
    };
    let Some(cache) = jobj(env.call_object_method(level, g.m_get_chunk_source, &[])) else {
        clear_if_pending(env);
        return false;
    };
    let Some(ttype) = chunk_load_type(env, g) else {
        clear_if_pending(env);
        return false;
    };
    let Some(pos) = env.new_object(g.chunk_pos_cls, g.m_chunk_pos_new, &[ji(cx), ji(cz)]) else {
        clear_if_pending(env);
        return false;
    };
    env.call_void_method(cache, g.m_remove_ticket, &[jv(ttype), jv(pos), ji(0)]);
    let ok = !env.exception_check();
    clear_if_pending(env);
    ok
}

/// Diagnostic: direct isChunkLoaded probe.
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

/// Scan diagnostics counters.
#[allow(dead_code)]
pub static SCAN_OK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[allow(dead_code)]
pub static SCAN_UNLOADED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[allow(dead_code)]
pub static SCAN_ERR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Sample the surface height at one column. Returns (blockY, isLiquid).
/// Superseded by [`sample_surface`] (v4.2 color payload); kept as a
/// minimal-surface diagnostic probe.
#[allow(dead_code)]
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

/// Sample the surface at one column WITH the v4.2 color payload.
/// Returns (blockY, isLiquid, material name, biome name). The material /
/// biome names may be empty when the optional lookups are unavailable —
/// the palette falls back to a neutral surface color in that case.
pub fn sample_surface(
    env: Env,
    world: jobject,
    x: i32,
    z: i32,
) -> Option<(i32, bool, String, String)> {
    use std::sync::atomic::Ordering;
    let wid = world_ids(env, world)?;
    if env.exception_check() {
        env.exception_clear();
        SCAN_ERR.fetch_add(1, Ordering::Relaxed);
        return None;
    }
    // Same loaded-chunk guard as sample_height: never force generation.
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
        return Some((y, false, String::new(), String::new()));
    }
    let Some(block) =
        jobj(env.call_object_method(world, wid.m_get_block_at, &[ji(x), ji(y), ji(z)]))
    else {
        clear_if_pending(env);
        return Some((y, false, String::new(), String::new()));
    };
    let Some(bid) = block_ids(env, block) else {
        clear_if_pending(env);
        return Some((y, false, String::new(), String::new()));
    };
    let liquid = env.call_boolean_method(block, bid.m_is_liquid, &[]);
    if env.exception_check() {
        env.exception_clear();
    }

    // Material name (Block#getType().name()).
    let mut material = String::new();
    if let Some(m_get_type) = bid.m_get_type {
        if let Some(mat) = jobj(env.call_object_method(block, m_get_type, &[])) {
            if env.exception_check() {
                env.exception_clear();
            } else if let Some(eid) = enum_name_ids(env, mat, &MATERIAL_ENUM) {
                material =
                    jstr(env, env.call_object_method(mat, eid.m_name, &[])).unwrap_or_default();
                clear_if_pending(env);
            }
        } else {
            clear_if_pending(env);
        }
    }

    // Biome name (World#getBiome(x, y, z).name()) — for grass/foliage/water
    // tints. Sampled at the surface block.
    let mut biome = String::new();
    if let Some(m_get_biome) = wid.m_get_biome {
        if let Some(bio) = jobj(env.call_object_method(world, m_get_biome, &[ji(x), ji(y), ji(z)]))
        {
            if env.exception_check() {
                env.exception_clear();
            } else if let Some(eid) = enum_name_ids(env, bio, &BIOME_ENUM) {
                biome = jstr(env, env.call_object_method(bio, eid.m_name, &[])).unwrap_or_default();
                clear_if_pending(env);
            }
        } else {
            clear_if_pending(env);
        }
    }

    SCAN_OK.fetch_add(1, Ordering::Relaxed);
    Some((y, liquid, material, biome))
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

// ---------------------------------------------------------------------------
// v5.0 MAP CARRIER: raw NMS surface for streaming LOD pages to the vanilla
// client as ClientboundMapItemDataPackets + a FILLED_MAP item in the offhand.
// Paper 26.3 runs Mojang-mapped at runtime, so every net.minecraft.* name
// below is the real runtime class, resolved through CraftServer's loader
// (same force_load path as the Bukkit surface).
// ---------------------------------------------------------------------------

pub struct Nms {
    /// ParticleTypes.HAPPY_VILLAGER (SimpleParticleType static) — the ONLY
    /// particle texture the 26.3 client actually renders on software raster
    /// backends (dust/explosion/flame stay invisible: broken sprite UVs).
    happy_villager: jobject,
    /// ClientboundLevelParticlesPacket(ParticleOptions,ZZDDDFFFI)V —
    /// overrideLimiter=true rides like /particle force (never clipped by
    /// the client's render distance, always shown).
    particle_packet_cls: jobject,
    m_particle_packet_new: jmethodID,
    /// ServerPlayer.connection field + ServerGamePacketListenerImpl.send
    f_connection: jfieldID,
    m_send: jmethodID,
}

struct SendNms(Nms);
unsafe impl Send for SendNms {}
unsafe impl Sync for SendNms {}

static NMS: OnceLock<Option<SendNms>> = OnceLock::new();
/// Raw pointer cache of the CraftServer classloader (global ref), resolved
/// together with the NMS surface. Main-thread-only.
static LOADER: Mutex<usize> = Mutex::new(0);

/// Stream diagnostics: dust spawns handed to ServerLevel + failures.
pub static STREAM_SPAWNED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static STREAM_FAILED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Resolve the NMS surface once (main thread). Returns true when usable.
pub fn nms_warmup(env: Env) -> bool {
    if let Some(s) = NMS.get() {
        eprintln!("[c-LoD] NMS warmup: cached = {}", s.is_some());
        return s.is_some();
    }
    eprintln!("[c-LoD] NMS warmup: resolving...");
    match resolve_nms(env) {
        Some(n) => {
            let _ = NMS.set(Some(SendNms(n)));
            true
        }
        None => {
            clear_if_pending(env);
            false
        }
    }
}

macro_rules! step {
    ($name:expr, $e:expr) => {
        match $e {
            Some(v) => v,
            None => {
                eprintln!("[c-LoD] NMS resolve FAILED at: {}", $name);
                return None;
            }
        }
    };
}

fn resolve_nms(env: Env) -> Option<Nms> {
    use cplug_sdk::classes as sdkcls;
    // Classloader reuse: the Bukkit resolve() stored it (see resolve()).
    let loader = *LOADER.lock().unwrap();
    eprintln!("[c-LoD] NMS resolve: loader = {}", loader);
    if loader == 0 {
        eprintln!("[c-LoD] NMS resolve FAILED at: loader stash empty");
        return None;
    }
    let class_cls = sdkcls::find_class("java/lang/Class")
        .map(|c| c.as_jclass())
        .or_else(|| {
            eprintln!("[c-LoD] NMS resolve FAILED at: find_class Class");
            None
        })?;
    let m_for_name = env
        .get_static_method_id(
            class_cls,
            "forName",
            "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
        )
        .or_else(|| {
            eprintln!("[c-LoD] NMS resolve FAILED at: forName method id");
            None
        })?;
    let load = |dotted: &str| -> Option<jobject> {
        let jn = step!("jn", env.new_string_utf(dotted));
        let k = env.call_static_object_method(
            class_cls,
            m_for_name,
            &[jv(jn), jz(true), jv(loader as jobject)],
        );
        clear_if_pending(env);
        match jobj(k) {
            Some(c) => Some(env.new_global_ref(c) as jobject),
            None => {
                eprintln!("[c-LoD] NMS resolve FAILED at: forName({dotted})");
                None
            }
        }
    };

    let particle_types_cls = step!(
        "particle_types_cls",
        load("net.minecraft.core.particles.ParticleTypes")
    );
    let happy_villager = step!(
        "happy_villager",
        env.get_static_field_id(
            particle_types_cls,
            "HAPPY_VILLAGER",
            "Lnet/minecraft/core/particles/SimpleParticleType;",
        )
        .map(|fid| env.get_static_object_field(particle_types_cls, fid))
        .map(|o| env.new_global_ref(o) as jobject)
    );
    let particle_packet_cls = step!(
        "particle_packet_cls",
        load("net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket")
    );
    let server_player_cls = step!(
        "server_player_cls",
        load("net.minecraft.server.level.ServerPlayer")
    );
    let sgp_listener_cls = step!(
        "sgp_listener_cls",
        load("net.minecraft.server.network.ServerGamePacketListenerImpl")
    );

    let _ = &happy_villager; // consumed below in the Nms struct
    let m_particle_packet_new = step!(
        "m_particle_packet_new",
        env.get_method_id(
            particle_packet_cls,
            "<init>",
            "(Lnet/minecraft/core/particles/ParticleOptions;ZZDDDFFFFI)V",
        )
    );
    let f_connection = step!(
        "f_connection",
        env.get_field_id(
            server_player_cls,
            "connection",
            "Lnet/minecraft/server/network/ServerGamePacketListenerImpl;",
        )
    );
    let m_send = step!(
        "m_send",
        bind_method(
            env,
            sgp_listener_cls,
            sgp_listener_cls,
            "send",
            "(Lnet/minecraft/network/protocol/Packet;)V",
        )
    );
    Some(Nms {
        happy_villager,
        particle_packet_cls,
        m_particle_packet_new,
        f_connection,
        m_send,
    })
}

fn nms() -> Option<&'static Nms> {
    NMS.get().and_then(|o| o.as_ref()).map(|s| &s.0)
}

/// Cached CraftPlayer.getHandle method id (resolved on the first live player).
static M_GET_HANDLE: Mutex<usize> = Mutex::new(0);

/// # Safety
/// Main thread only; `player` must be a live CraftPlayer local ref.
unsafe fn get_handle(env: Env, player: jobject) -> Option<jobject> {
    let mid = {
        let cached = *M_GET_HANDLE.lock().unwrap();
        if cached != 0 {
            cached as jmethodID
        } else {
            let cls = env.get_object_class(player);
            let probe = env.get_method_id(
                cls,
                "getHandle",
                "()Lnet/minecraft/server/level/ServerPlayer;",
            );
            env.exception_clear();
            let g = env.new_global_ref(cls);
            let mid = probe.or_else(|| {
                env.exception_clear();
                env.get_method_id(
                    g as jobject,
                    "getHandle",
                    "()Lnet/minecraft/server/level/ServerPlayer;",
                )
            })?;
            clear_if_pending(env);
            *M_GET_HANDLE.lock().unwrap() = mid as usize;
            mid
        }
    };
    jobj(env.call_object_method(player, mid, &[]))
}

/// Spawn ONE FORCE villager-sparkle particle for ONE player: build a
/// ClientboundLevelParticlesPacket (overrideLimiter = true — the particle
/// survives the client's particle setting AND its render-distance clip,
/// exactly like `/particle ... force`) and push it straight into the
/// connection. This IS the LOD transport (SKY-GRID BIT-STREAM).
///
/// happy_villager replaces the v5.0 dust carrier: dust quads render with
/// degenerate UVs on every 26.3 software backend (verified live on GL
/// llvmpipe AND Vulkan lavapipe — dust stays 1-2 px while explosions and
/// town-sparkles render full-size).
///
/// # Safety
/// Main thread only; `player` must be a live CraftPlayer local ref.
pub unsafe fn spawn_particle(env: Env, player: jobject, x: f64, y: f64, z: f64) -> bool {
    let Some(n) = nms() else {
        return false;
    };
    let Some(handle) = get_handle(env, player) else {
        clear_if_pending(env);
        return false;
    };
    let Some(conn) = jobj(env.get_object_field(handle, n.f_connection)) else {
        clear_if_pending(env);
        return false;
    };
    // SimpleParticleType is a process-lifetime JVM static — borrow it directly.
    let opts = n.happy_villager;
    // (options, overrideLimiter=true, alwaysShow=true, x, y, z,
    //  xDist=0, yDist=0, zDist=0, maxSpeed=0, count=0 -> exactly one
    //  particle at the point, zero drift)
    let args = [
        jv(opts),
        jz(true),
        jz(true),
        jd(x),
        jd(y),
        jd(z),
        jf(0.0),
        jf(0.0),
        jf(0.0),
        jf(0.0),
        ji(0),
    ];
    let Some(pkt) = jobj(
        env.new_object(n.particle_packet_cls, n.m_particle_packet_new, &args)
            .unwrap_or(std::ptr::null_mut()),
    ) else {
        clear_if_pending(env);
        STREAM_FAILED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return false;
    };
    env.call_void_method(conn, n.m_send, &[jv(pkt)]);
    if env.exception_check() {
        if STREAM_FAILED.load(std::sync::atomic::Ordering::Relaxed) < 5 {
            env.exception_describe();
        }
        env.exception_clear();
        STREAM_FAILED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return false;
    }
    STREAM_SPAWNED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    true
}

/// Remember the CraftServer classloader global for the NMS loader path
/// (called from resolve(), main thread).
pub(crate) fn stash_loader(env: Env, loader_local: jobject) {
    let g = env.new_global_ref(loader_local) as usize;
    *LOADER.lock().unwrap() = g;
}
