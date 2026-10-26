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
        m_set_force_loaded: bind_method(env, cls_local, cls, "setChunkForceLoaded", "(IIZ)V")?,
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

    let bukkit_cls = force_load("org.bukkit.Bukkit")
        .or_else(|| { eprintln!("[c-LoD] resolve FAILED at: force_load org.bukkit.Bukkit"); None })?;
    let key_cls = force_load("net.kyori.adventure.key.Key")
        .or_else(|| { eprintln!("[c-LoD] resolve FAILED at: force_load Key"); None })?;
    let post_effects_cls = force_load("io.papermc.paper.entity.PlayerPostEffects")
        .or_else(|| { eprintln!("[c-LoD] resolve FAILED at: force_load PlayerPostEffects"); None })?;
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
    .or_else(|| { eprintln!("[c-LoD] resolve FAILED at: getOnlinePlayers"); None })?;
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
    /// net.minecraft.world.level.saveddata.maps.MapId(int)
    map_id_cls: jobject,
    m_map_id_new: jmethodID,
    /// MapItemSavedData$MapPatch(int,int,int,int,byte[])
    map_patch_cls: jobject,
    m_map_patch_new: jmethodID,
    /// ClientboundMapItemDataPacket(MapId, byte, boolean, Collection, MapPatch)
    map_packet_cls: jobject,
    m_map_packet_new: jmethodID,
    /// ServerGamePacketListenerImpl.send(Packet)
    m_send: jmethodID,
    /// ServerPlayer.connection field
    f_connection: jfieldID,
    /// ItemStack(ItemLike) + set(DataComponentType, Object)
    item_stack_cls: jobject,
    m_item_stack_new: jmethodID,
    m_item_stack_set: jmethodID,
    /// Items.FILLED_MAP static field + the Items class itself
    items_cls: jobject,
    f_filled_map: jfieldID,
    /// DataComponents.MAP_ID static field + the DataComponents class itself
    data_components_cls: jobject,
    f_map_id: jfieldID,
    /// EquipmentSlot.OFFHAND static field
    f_offhand: jfieldID,
}

struct SendNms(Nms);
unsafe impl Send for SendNms {}
unsafe impl Sync for SendNms {}

static NMS: OnceLock<Option<SendNms>> = OnceLock::new();
/// Raw pointer cache of the CraftServer classloader (global ref), resolved
/// together with the NMS surface. Main-thread-only.
static LOADER: Mutex<usize> = Mutex::new(0);

/// Stream diagnostics: raw map packets handed to the connection + offhand swaps.
pub static STREAM_PACKETS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static STREAM_PAGE_SWAPS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

use jvmti_bindings::jni::jfieldID;

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
        .or_else(|| { eprintln!("[c-LoD] NMS resolve FAILED at: find_class Class"); None })?;
    let m_for_name = env.get_static_method_id(
        class_cls,
        "forName",
        "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
    )
    .or_else(|| { eprintln!("[c-LoD] NMS resolve FAILED at: forName method id"); None })?;
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

    let map_id_cls = step!("map_id_cls", load("net.minecraft.world.level.saveddata.maps.MapId"));
    let map_patch_cls = step!("map_patch_cls", load("net.minecraft.world.level.saveddata.maps.MapItemSavedData$MapPatch"));
    let map_packet_cls = step!("map_packet_cls", load("net.minecraft.network.protocol.game.ClientboundMapItemDataPacket"));
    let item_stack_cls = step!("item_stack_cls", load("net.minecraft.world.item.ItemStack"));
    let items_cls = step!("items_cls", load("net.minecraft.world.item.Items"));
    let data_components_cls = step!("data_components_cls", load("net.minecraft.core.component.DataComponents"));
    let equipment_slot_cls = step!("equipment_slot_cls", load("net.minecraft.world.entity.EquipmentSlot"));
    let server_player_cls = step!("server_player_cls", load("net.minecraft.server.level.ServerPlayer"));
    let sgp_listener_cls = step!("sgp_listener_cls", load("net.minecraft.server.network.ServerGamePacketListenerImpl"));

    let m_map_id_new = step!("m_map_id_new", env.get_method_id(map_id_cls, "<init>", "(I)V"));
    let m_map_patch_new = step!("m_map_patch_new", env.get_method_id(map_patch_cls, "<init>", "(IIII[B)V"));
    let m_map_packet_new = step!("m_map_packet_new", env.get_method_id(
        map_packet_cls,
        "<init>",
        "(Lnet/minecraft/world/level/saveddata/maps/MapId;BZLjava/util/Collection;Lnet/minecraft/world/level/saveddata/maps/MapItemSavedData$MapPatch;)V",
    ));
    let m_send = step!("m_send", bind_method(
        env,
        sgp_listener_cls,
        sgp_listener_cls,
        "send",
        "(Lnet/minecraft/network/protocol/Packet;)V",
    ));
    let f_connection = step!("f_connection", env.get_field_id(
        server_player_cls,
        "connection",
        "Lnet/minecraft/server/network/ServerGamePacketListenerImpl;",
    ));
    let m_item_stack_new = step!("m_item_stack_new", env.get_method_id(
        item_stack_cls,
        "<init>",
        "(Lnet/minecraft/world/level/ItemLike;)V",
    ));
    // public <T> T set(DataComponentType<T>, T) — T erases to Object.
    let m_item_stack_set = step!("m_item_stack_set", env.get_method_id(
        item_stack_cls,
        "set",
        "(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ljava/lang/Object;",
    ));
    let f_filled_map = step!("f_filled_map",
        env.get_static_field_id(items_cls, "FILLED_MAP", "Lnet/minecraft/world/item/Item;"));
    let f_map_id = step!("f_map_id", env.get_static_field_id(
        data_components_cls,
        "MAP_ID",
        "Lnet/minecraft/core/component/DataComponentType;",
    ));
    let f_offhand = step!("f_offhand", env.get_static_field_id(
        equipment_slot_cls,
        "OFFHAND",
        "Lnet/minecraft/world/entity/EquipmentSlot;",
    ));
    // setItemSlot lives on Entity; resolve through the concrete ServerPlayer.
    let m_set_item_slot = step!("m_set_item_slot", bind_method(
        env,
        server_player_cls,
        server_player_cls,
        "setItemSlot",
        "(Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;)V",
    ));
    let _ = m_set_item_slot; // setItemSlot binds lazily per call site (see give_offhand_map)
    Some(Nms {
        map_id_cls,
        m_map_id_new,
        map_patch_cls,
        m_map_patch_new,
        map_packet_cls,
        m_map_packet_new,
        m_send,
        f_connection,
        item_stack_cls,
        m_item_stack_new,
        m_item_stack_set,
        items_cls,
        f_filled_map,
        data_components_cls,
        f_map_id,
        f_offhand,
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

/// Build a ClientboundMapItemDataPacket for one page patch. Returns a LOCAL
/// ref the caller must hand to [`send_packet`] on the same thread.
///
/// # Safety
/// Main thread only.
pub unsafe fn map_page_packet(
    env: Env,
    map_id: i32,
    patch: Option<(u8, u8, u8, u8, &[u8])>,
) -> Option<jobject> {
    let n = nms()?;
    let jmid = env.new_object(n.map_id_cls, n.m_map_id_new, &[ji(map_id)])?;
    let jpatch = match patch {
        Some((x, y, w, h, bytes)) => {
            let jarr = env.new_byte_array(bytes.len() as i32)?;
            let jb: Vec<jvmti_bindings::jni::jbyte> = bytes.iter().map(|b| *b as i8).collect();
            env.set_byte_array_region(jarr, 0, bytes.len() as i32, &jb);
            if env.exception_check() {
                env.exception_clear();
                return None;
            }
            env.new_object(
                n.map_patch_cls,
                n.m_map_patch_new,
                &[
                    ji(x as i32),
                    ji(y as i32),
                    ji(w as i32),
                    ji(h as i32),
                    jv(jarr),
                ],
            )?
        }
        None => std::ptr::null_mut(),
    };
    let args = [
        jv(jmid),
        jvmti_bindings::jni::jvalue { b: 3 }, // scale byte (cosmetic)
        jz(true),                             // locked
        jv(std::ptr::null_mut()),             // no decorations
        jv(jpatch),
    ];
    let pkt = env.new_object(n.map_packet_cls, n.m_map_packet_new, &args);
    if env.exception_check() {
        env.exception_describe();
        env.exception_clear();
        return None;
    }
    jobj(pkt.unwrap_or(std::ptr::null_mut()))
}

/// Send any packet to one player (raw NMS connection.send).
///
/// # Safety
/// Main thread only; both refs must be live local refs.
pub unsafe fn send_packet(env: Env, player: jobject, packet: jobject) -> bool {
    let Some(n) = nms() else { return false };
    let Some(handle) = get_handle(env, player) else {
        clear_if_pending(env);
        return false;
    };
    let Some(conn) = jobj(env.get_object_field(handle, n.f_connection)) else {
        clear_if_pending(env);
        return false;
    };
    env.call_void_method(conn, n.m_send, &[jv(packet)]);
    if env.exception_check() {
        env.exception_describe();
        env.exception_clear();
        return false;
    }
    STREAM_PACKETS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    true
}

/// Put (or replace) the offhand item with a FILLED_MAP bound to `map_id`.
///
/// # Safety
/// Main thread only.
pub unsafe fn give_offhand_map(env: Env, player: jobject, map_id: i32) -> bool {
    let Some(n) = nms() else { return false };
    let Some(filled) = jobj(env.get_static_object_field(n.items_cls, n.f_filled_map)) else {
        clear_if_pending(env);
        return false;
    };
    // Items.FILLED_MAP is an Item (ItemLike) — construct ItemStack(item).
    let Some(stack) = jobj(
        env.new_object(n.item_stack_cls, n.m_item_stack_new, &[jv(filled)])
            .unwrap_or(std::ptr::null_mut()),
    ) else {
        clear_if_pending(env);
        return false;
    };
    let Some(mid) = jobj(
        env.new_object(n.map_id_cls, n.m_map_id_new, &[ji(map_id)])
            .unwrap_or(std::ptr::null_mut()),
    ) else {
        clear_if_pending(env);
        return false;
    };
    let Some(map_id_comp) = jobj(env.get_static_object_field(n.data_components_cls, n.f_map_id))
    else {
        clear_if_pending(env);
        return false;
    };
    // stack = stack.set(MAP_ID, mid)
    let set_args = [jv(map_id_comp), jv(mid)];
    let _ = env.call_object_method(stack, n.m_item_stack_set, &set_args);
    if env.exception_check() {
        env.exception_describe();
        env.exception_clear();
        return false;
    }
    let Some(offhand) = jobj(env.get_static_object_field(n.item_stack_cls, n.f_offhand)) else {
        clear_if_pending(env);
        return false;
    };
    // setItemSlot resolves through the concrete player handle class (Entity).
    let Some(handle) = get_handle(env, player) else {
        clear_if_pending(env);
        return false;
    };
    let hcls = env.get_object_class(handle);
    let probe = env.get_method_id(
        hcls,
        "setItemSlot",
        "(Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;)V",
    );
    env.exception_clear();
    let hg = env.new_global_ref(hcls);
    let mid_set = probe.or_else(|| {
        env.exception_clear();
        env.get_method_id(
            hg as jobject,
            "setItemSlot",
            "(Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;)V",
        )
    });
    let Some(mid_set) = mid_set else {
        clear_if_pending(env);
        return false;
    };
    env.call_void_method(handle, mid_set, &[jv(offhand), jv(stack)]);
    if env.exception_check() {
        env.exception_describe();
        env.exception_clear();
        return false;
    }
    STREAM_PAGE_SWAPS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    true
}

/// Remember the CraftServer classloader global for the NMS loader path
/// (called from resolve(), main thread).
pub(crate) fn stash_loader(env: Env, loader_local: jobject) {
    let g = env.new_global_ref(loader_local) as usize;
    *LOADER.lock().unwrap() = g;
}
