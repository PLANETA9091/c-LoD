//! SEEDGEN (v5.7): off-thread generation of chunks the server never made.
//!
//! Voxy's seedgen lesson, applied to c-LoD: the chunk SYSTEM (tickets,
//! worker pool, TPS coupling) is the slow part of filling the LOD disk —
//! not the generation math. Vanilla 26.3 exposes everything needed to
//! produce a terrain column WITHOUT a chunk in the world. The recipe below
//! is transcribed from `NoiseBasedChunkGenerator.iterateNoiseColumn`
//! bytecode (paper-26.3, mojang-mapped):
//!
//! ```text
//! vol = new DensityVolume(1, ns.height(), 1, x, ns.minY(), z)
//! nc  = new NoiseChunk(randomState, null, settings, fluidPicker,
//!                      Blender.empty(), vol)          // Beardifier=null is
//!                                                      // what vanilla passes
//! buf = nc.cachingSamplers().get(finalDensity).sampleVolume(nc.volume())
//! for i in (0..vol.sizeY()).rev():
//!     d  = buf.get(vol.indexUnchecked(0, i, 0))
//!     y  = vol.blockY(i)
//!     st = aquifer.computeSubstance(x, y, z, d) ?? settings.defaultBlock()
//!     if MOTION_BLOCKING.test(st) -> surface found at y
//! buf.close(); nc.close()                             // pooled buffers!
//! ```
//!
//! `getBaseHeight()` stops there (raw stone/air/water). The surface MATERIAL
//! (grass/sand/snow/badlands bands) comes from the surface-rule tree:
//!
//! ```text
//! top = MaterialSystem.topMaterial(rule, rs, ctx, biomeFn, chunk,
//!                                  samplers, pos, steep=false)
//! ```
//!
//! with a throwaway `ProtoChunk` per chunk: `topMaterial` unconditionally
//! feeds the chunk to the steepness gradient (`getSurfaceGradientX/Z` read
//! `ChunkAccess.getHeight`, which creates an empty heightmap on demand) —
//! an empty ProtoChunk yields gradient 0, i.e. flat-surface rules. Cliff
//! stone is lost on generated cells; the real-chunk upgrade path restores
//! it (cells stream as `provisional` and are re-sampled from the live
//! chunk the moment the server generates it — trees, carvers, structures
//! all arrive with that upgrade).
//!
//! The `Function<BlockPos, Holder<Biome>>` that `topMaterial` needs is
//! built at boot WITHOUT any helper class:
//! `MethodHandles.lookup().unreflect(BiomeManager#getBiome).bindTo(mgr)`
//! then `MethodHandleProxies.asInterfaceInstance(Function.class, mh)`.
//!
//! Threading: workers are OUR daemon threads attached to the JVM; one
//! NoiseChunk covers a whole 16x16 chunk volume (one `sampleVolume`, its
//! columns all read the shared buffer). No tickets, no chunk-system
//! contention, zero TPS impact — and, unlike the ticket frontier, it fills
//! the disk pre-join by construction.
//!
//! v5.8: the ~1500 JNI transitions per column (indexUnchecked, buf get,
//! blockY, computeSubstance, predicate — per Y step) are gone. The whole
//! per-chunk walk now runs INSIDE the JVM behind `java/ClodSeedgen.java`,
//! a JDK-only helper class embedded as bytecode and defined at bootstrap
//! via ClassLoader.defineClass; the JNI boundary is crossed once per chunk
//! (and twice per intern-table read). The recipe inside is byte-for-byte
//! the transcription above, so outputs are identical; this side only
//! unpacks the packed longs, filters by the disk window and paints.
//!
//! v5.9: the boundary is crossed once per BATCH of chunks (chunkBatch,
//! `seedgen_batch` chunks per call — a chunk that throws marks only its own
//! segment and falls back to the legacy walk), the intern tables ride a
//! monotonic tabVer in the payload header (they used to be re-read one JNI
//! call PER STRING every chunk — 60-230 crossings, an order of magnitude
//! more than the walk itself), and the Java side steps the y-walk on
//! numerically-verified affine index/height strides (2 of 5 MethodHandle
//! calls per y-step gone; any non-affine layout falls back automatically).
//! Worker paint takes the stream lock once per batch instead of twice per
//! cell, and the auto worker cap grew 4 → 8.
//!
//! v6.1 SEEDGEN TREES: after the walk the helper replays the exact vanilla
//! `addVanillaDecorations` recipe for the VEGETAL_DECORATION step —
//! `setDecorationSeed(seed, originX, originZ)` (chunk-origin block coords,
//! transcribed from paper-26.3 bytecode), per placed feature
//! `setFeatureSeed(decorated, globalIdx, step)` in ascending
//! FeatureSorter-index order, then the REAL `FeaturePlacer.placeWithBiomeCheck`
//! against a dynamic-proxy WorldGenLevel backed by the walked terrain.
//! Every feature is freshly seeded per chunk, so running only our own
//! biomes is bit-equivalent to vanilla's 3x3 palette union; tree tops and
//! leaf materials land in the same packed cells (max of walk top and
//! feature top). Any throw degrades that chunk to the terrain-only output —
//! the v5.9 cells, proven bit-identical by the preflight; `native_trees`
//! turns the pass off entirely.

use crate::jni::{bind_method, clear_if_pending, jd, ji, jl, jobj, jstr, jv, jz, LOADER};
use crate::{engine, log_info, palette};
use jvmti_bindings::jni::{jfieldID, jmethodID, jobject};
use jvmti_bindings::prelude::JniEnv;
use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};

type Env<'a> = &'a JniEnv;

/// Local bail macro (jni.rs gstep twin; kept local — no cross-module macros).
macro_rules! sgstep {
    ($name:expr, $e:expr) => {
        match $e {
            Some(v) => v,
            None => {
                eprintln!("[c-LoD] seedgen resolve FAILED at: {}", $name);
                return None;
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Pure job queue (unit-tested without a JVM)
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct SeedQueue {
    pending: Mutex<VecDeque<(i32, i32)>>,
    inflight: Mutex<HashSet<(i32, i32)>>,
    cv: Condvar,
    running: AtomicBool,
}

impl SeedQueue {
    pub fn new() -> Self {
        Self {
            running: AtomicBool::new(true),
            ..Default::default()
        }
    }

    /// Enqueue a chunk for native generation. Returns false when the chunk
    /// is already queued or being generated (dedup) or the queue is closed.
    pub fn push(&self, cx: i32, cz: i32) -> bool {
        if !self.running.load(Ordering::SeqCst) {
            return false;
        }
        let mut inflight = self.inflight.lock().unwrap();
        if !inflight.insert((cx, cz)) {
            return false;
        }
        self.pending.lock().unwrap().push_back((cx, cz));
        self.cv.notify_one();
        true
    }

    /// Block for the first job, then drain up to `max` more without
    /// waiting (v5.9: one JNI batch call amortized over the drain).
    /// Some(non-empty) on jobs, None only when closed.
    pub fn pop_batch(&self, max: usize, timeout_ms: u64) -> Option<Vec<(i32, i32)>> {
        let max = max.max(1);
        let mut pending = self.pending.lock().unwrap();
        loop {
            let mut out = Vec::with_capacity(max.min(16));
            while out.len() < max {
                match pending.pop_front() {
                    Some(job) => out.push(job),
                    None => break,
                }
            }
            if !out.is_empty() {
                return Some(out);
            }
            if !self.running.load(Ordering::SeqCst) {
                return None;
            }
            let (guard, _t) = self
                .cv
                .wait_timeout(pending, std::time::Duration::from_millis(timeout_ms))
                .unwrap();
            pending = guard;
        }
    }

    /// Mark a job done (removes the dedup entry; failed chunks re-queue on
    /// the next sweep pass).
    pub fn done(&self, cx: i32, cz: i32) {
        self.inflight.lock().unwrap().remove(&(cx, cz));
    }

    pub fn inflight_len(&self) -> usize {
        self.inflight.lock().unwrap().len()
    }

    /// Shutdown hook (kept for tests and a future module-stop path).
    #[allow(dead_code)]
    pub fn close(&self) {
        self.running.store(false, Ordering::SeqCst);
        self.cv.notify_all();
    }
}

// ---------------------------------------------------------------------------
// JNI surface (resolved once on the main thread; global refs everywhere)
// ---------------------------------------------------------------------------

/// Raw JNI handles: global refs + method ids are JVM-process-global and
/// safe to use from any ATTACHED thread (workers attach as daemons).
unsafe impl Send for SeedIds {}
unsafe impl Sync for SeedIds {}

#[derive(Clone, Copy)]
struct SeedIds {
    // live objects (global refs)
    level: jobject,          // ServerLevel (also the LevelHeightAccessor)
    random_state: jobject,   // ServerChunkCache.randomState()
    settings: jobject,       // NoiseGeneratorSettings (unwrapped Holder)
    rule: jobject,           // MaterialRule (unwrapped Holder)
    surface: jobject,        // MaterialSystem (RandomState.surfaceSystem())
    ctx: jobject,            // WorldGenerationContext.of(level)
    pred: jobject,           // Heightmap.Types.MOTION_BLOCKING.isOpaque()
    fluid_picker: jobject,   // NoiseBasedChunkGenerator.globalFluidPicker.get()
    blender: jobject,        // Blender.empty()
    default_block: jobject,  // settings.defaultBlock()
    final_density: jobject,  // settings.noiseRouter().finalDensity()
    upgrade_data: jobject,   // UpgradeData.EMPTY
    factory: jobject,        // level.palettedContainerFactory()
    bman: jobject,           // level.getBiomeManager()
    biome_fn: jobject,       // Function<BlockPos,Holder<Biome>> via proxies
    block_registry: jobject, // BuiltInRegistries.BLOCK
    // classes (global refs)
    noise_chunk_cls: jobject,
    density_volume_cls: jobject,
    proto_chunk_cls: jobject,
    chunk_pos_cls: jobject,
    block_pos_cls: jobject,
    // methods
    nc_new: jmethodID,
    nc_caching_samplers: jmethodID,
    nc_volume: jmethodID,
    nc_aquifer: jmethodID,
    nc_close: jmethodID,
    dv_new: jmethodID,
    dv_block_y: jmethodID,
    dv_index: jmethodID,
    set_get: jmethodID,
    bound_sample: jmethodID,
    buf_get: jmethodID,
    buf_close: jmethodID,
    aq_substance: jmethodID,
    pred_test: jmethodID,
    top_material: jmethodID,
    proto_new: jmethodID,
    pos_new: jmethodID,
    opt_present: jmethodID,
    opt_get: jmethodID,
    st_fluid_state: jmethodID,
    fs_empty: jmethodID,
    st_block: jmethodID,
    reg_get_key: jmethodID,
    id_to_string: jmethodID,
    biome_get: jmethodID,
    holder_unwrap_key: jmethodID,
    rk_identifier: jmethodID,
    // v5.9 Java-side chunk helper (all null when unavailable — the legacy
    // per-column path stays as the fallback).
    helper_cls: jobject,
    helper_batch: jmethodID,
    f_mat_table: jfieldID,
    f_bio_table: jfieldID,
    // v6.1 feature-run counters (static longs in the helper class).
    f_feat_chunks: jfieldID,
    f_feat_fails: jfieldID,
}

/// Boot-time NoiseSettings scalars (clamped to the level once).
#[derive(Clone, Copy)]
struct NoiseDims {
    min_block_y: i32,
    size_y: i32,
}

pub struct Seedgen {
    q: Arc<SeedQueue>,
    ids: OnceLock<SeedIds>,
    dims: OnceLock<NoiseDims>,
    spawned: AtomicBool,
    /// Resolve attempted once (success or failure — no re-resolve spam).
    tried: AtomicBool,
    /// Cells filled by native generation since boot.
    pub cells: AtomicU64,
    /// Chunks generated natively since boot.
    pub chunks: AtomicU64,
    /// Failed chunk generations (re-queued on the next sweep pass).
    pub fails: AtomicU64,
    /// Worker thread count.
    pub workers: AtomicU64,
    /// Java-side chunk helper armed (one JNI call per chunk, v5.8).
    pub helper: AtomicBool,
    /// Helper chunks that threw at runtime; the path disables itself after
    /// repeated failures and the v5.7 per-column path takes over.
    pub helper_fails: AtomicU64,
    /// v6.1: chunks whose VEGETAL_DECORATION pass completed (trees/leaves
    /// patched into the cells) — mirrored from the helper's static counters.
    pub tree_chunks: AtomicU64,
    /// v6.1: chunks whose whole feature pass threw (terrain-only output).
    pub tree_fails: AtomicU64,
}

impl Seedgen {
    pub fn new() -> Self {
        Self {
            q: Arc::new(SeedQueue::new()),
            ids: OnceLock::new(),
            dims: OnceLock::new(),
            spawned: AtomicBool::new(false),
            tried: AtomicBool::new(false),
            cells: AtomicU64::new(0),
            chunks: AtomicU64::new(0),
            fails: AtomicU64::new(0),
            workers: AtomicU64::new(0),
            helper: AtomicBool::new(false),
            helper_fails: AtomicU64::new(0),
            tree_chunks: AtomicU64::new(0),
            tree_fails: AtomicU64::new(0),
        }
    }

    pub fn ready(&self) -> bool {
        self.ids.get().is_some()
    }

    /// True once a resolve attempt was made (success or failure).
    pub fn tried(&self) -> bool {
        self.tried.load(Ordering::SeqCst)
    }

    pub fn push(&self, cx: i32, cz: i32) -> bool {
        if !self.ready() {
            return false;
        }
        self.q.push(cx, cz)
    }

    pub fn inflight_len(&self) -> usize {
        self.q.inflight_len()
    }
}

/// True once a bootstrap attempt happened in this process (any state).
/// The lazy arm polls this before calling bootstrap — a plain atomic read
/// per unloaded cell, no locking in the hot scan path.
pub fn tried(state: &Arc<engine::State>) -> bool {
    let sg = state.seedgen.lock().unwrap().clone();
    sg.map(|s| s.tried()).unwrap_or(false)
}

/// Resolve the whole JNI surface + spawn worker threads. Must run on the
/// main thread (JNI-safe); no-op after the first attempt.
pub fn bootstrap(state: &Arc<engine::State>, env: Env, world: jobject) {
    {
        let mut g = state.seedgen.lock().unwrap();
        if g.is_none() {
            *g = Some(Arc::new(Seedgen::new()));
        }
    }
    let sg = match state.seedgen.lock().unwrap().clone() {
        Some(sg) => sg,
        None => return,
    };
    if sg.tried.swap(true, Ordering::SeqCst) {
        return; // one attempt per process
    }
    if sg.ready() {
        return;
    }
    let Some((ids, dims)) = (unsafe { resolve(env, world, state.cfg.native_trees) }) else {
        log_info("seedgen: resolve failed — native gen OFF (ticket frontier stays)");
        return;
    };
    let helper_ok = !ids.helper_cls.is_null();
    sg.helper.store(helper_ok, Ordering::SeqCst);
    let _ = sg.ids.set(ids);
    let _ = sg.dims.set(dims);
    let workers = if state.cfg.native_threads > 0 {
        state.cfg.native_threads as usize
    } else {
        std::thread::available_parallelism()
            .map(|n| n.get().saturating_sub(1))
            .unwrap_or(1)
            .clamp(1, 8)
    };
    let batch = state.cfg.seedgen_batch.clamp(1, 64);
    if sg
        .spawned
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        == Ok(false)
    {
        for i in 0..workers {
            let st = Arc::clone(state);
            let sg2 = Arc::clone(&sg);
            std::thread::Builder::new()
                .name(format!("clod-seedgen-{i}"))
                .spawn(move || worker_loop(&st, &sg2))
                .ok();
        }
        sg.workers.store(workers as u64, Ordering::Relaxed);
    }
    log_info(&format!(
        "seedgen: ARMED — {workers} native workers, batch {batch} chunks/call \
         (noise+surface per column, no tickets){}{}",
        if helper_ok {
            ", helper: one JNI call per batch"
        } else {
            ", helper OFF: per-column JNI (v5.7 path)"
        },
        if helper_ok && state.cfg.native_trees {
            ", trees on"
        } else {
            ""
        }
    ));
}

/// Enqueue one chunk for native generation. False = seedgen not armed
/// (caller falls back to the ticket frontier).
pub fn enqueue(state: &Arc<engine::State>, cx: i32, cz: i32) -> bool {
    let sg = state.seedgen.lock().unwrap().clone();
    match sg {
        Some(sg) => sg.push(cx, cz),
        None => false,
    }
}

fn worker_loop(state: &Arc<engine::State>, sg: &Arc<Seedgen>) {
    let Some(env) = attach_daemon() else {
        eprintln!("[c-LoD] seedgen worker: JVM attach failed");
        return;
    };
    let Some(ids) = sg.ids.get().cloned() else {
        return;
    };
    let Some(dims) = sg.dims.get().cloned() else {
        return;
    };
    let batch = state.cfg.seedgen_batch.clamp(1, 64) as usize;
    // Intern-table snapshots cached by the batch payload's tabVer: a table
    // re-read costs one JNI call per string, so it happens only when the
    // helper actually grew a table (a new material/biome was interned).
    let mut tables = Tables {
        ver: i64::MIN,
        mats: Vec::new(),
        bios: Vec::new(),
    };
    // Loop until the queue closes (pop_batch returns None only on shutdown).
    while let Some(jobs) = sg.q.pop_batch(batch, 1000) {
        let helper_on = sg.helper.load(Ordering::SeqCst);
        let results: Vec<Result<usize, String>> = if helper_on {
            unsafe { gen_batch(state, &env, &ids, &mut tables, &jobs) }
        } else {
            jobs.iter()
                .map(|_| Err("helper disabled".to_string()))
                .collect()
        };
        if helper_on {
            // v6.1: mirror the helper's feature-run counters (2 static reads
            // per batch; the diag is rate-limited so this stays free).
            let tc = env.get_static_long_field(ids.helper_cls, ids.f_feat_chunks);
            let tf = env.get_static_long_field(ids.helper_cls, ids.f_feat_fails);
            sg.tree_chunks.store(tc as u64, Ordering::Relaxed);
            sg.tree_fails.store(tf as u64, Ordering::Relaxed);
        }
        let mut disable_now = false;
        for (i, (cx, cz)) in jobs.iter().enumerate() {
            let (cx, cz) = (*cx, *cz);
            match &results[i] {
                Ok(n) => {
                    sg.chunks.fetch_add(1, Ordering::Relaxed);
                    sg.cells.fetch_add(*n as u64, Ordering::Relaxed);
                }
                Err(e) => {
                    // Only helper-path failures count toward the disable
                    // threshold; once the helper is off, every batch goes
                    // straight to the legacy walk without counting.
                    if helper_on {
                        let f = sg.helper_fails.fetch_add(1, Ordering::SeqCst);
                        if f == 15 {
                            sg.helper.store(false, Ordering::SeqCst);
                            disable_now = true;
                        } else if f < 15 {
                            eprintln!(
                                "[c-LoD] seedgen batch chunk ({cx},{cz}) threw: {e} — legacy fallback"
                            );
                        }
                    }
                    clear_if_pending(&env);
                    // The legacy per-column walk owns this chunk (it may
                    // still fail for the same vanilla reason — the sweep
                    // re-queues it either way).
                    match unsafe { gen_chunk_legacy(state, &env, &ids, dims, cx, cz) } {
                        Ok(n) => {
                            sg.chunks.fetch_add(1, Ordering::Relaxed);
                            sg.cells.fetch_add(n as u64, Ordering::Relaxed);
                        }
                        Err(e2) => {
                            sg.fails.fetch_add(1, Ordering::Relaxed);
                            eprintln!("[c-LoD] seedgen chunk ({cx},{cz}) FAILED: {e2}");
                            clear_if_pending(&env);
                        }
                    }
                }
            }
        }
        if disable_now {
            eprintln!(
                "[c-LoD] seedgen helper: DISABLED after repeated throws — the v5.7 per-column path stays"
            );
        }
        for (cx, cz) in &jobs {
            sg.q.done(*cx, *cz);
        }
    }
}

/// Attach the current thread to the JVM as a daemon and keep it attached for
/// the thread's lifetime (daemon threads never block VM shutdown).
fn attach_daemon() -> Option<JniEnv> {
    unsafe {
        let raw_vm = cplug_sdk::vm();
        let vm = raw_vm as *mut jvmti_bindings::jni::JavaVM;
        if vm.is_null() || (*vm).is_null() {
            return None;
        }
        let mut env_ptr: *mut jvmti_bindings::jni::JNIEnv = std::ptr::null_mut();
        let rc = ((**vm).GetEnv)(
            vm,
            &mut env_ptr as *mut *mut _ as *mut *mut std::ffi::c_void,
            jvmti_bindings::jni::JNI_VERSION_1_6,
        );
        if rc == jvmti_bindings::jni::JNI_OK && !env_ptr.is_null() {
            return Some(JniEnv::from_raw(env_ptr));
        }
        if rc != jvmti_bindings::jni::JNI_EDETACHED {
            return None;
        }
        let rc = ((**vm).AttachCurrentThreadAsDaemon)(
            vm,
            &mut env_ptr as *mut *mut _ as *mut *mut std::ffi::c_void,
            std::ptr::null_mut(),
        );
        if rc != jvmti_bindings::jni::JNI_OK || env_ptr.is_null() {
            return None;
        }
        Some(JniEnv::from_raw(env_ptr))
    }
}

// ---------------------------------------------------------------------------
// Resolution (main thread, once)
// ---------------------------------------------------------------------------

/// The stashed app loader as a jobject (Class.forName's third arg).
fn loader_obj() -> jobject {
    *LOADER.lock().unwrap() as jobject
}

/// Load a class through the stashed app loader (jni.rs Class.forName path).
fn load_cls(env: Env, dotted: &str) -> Option<jobject> {
    use cplug_sdk::classes as sdkcls;
    let loader = loader_obj();
    if loader.is_null() {
        eprintln!("[c-LoD] seedgen resolve FAILED at: loader stash empty");
        return None;
    }
    let class_cls = sdkcls::find_class("java/lang/Class")?.as_jclass();
    let m_for_name = env.get_static_method_id(
        class_cls,
        "forName",
        "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
    )?;
    let jn = env.new_string_utf(dotted)?;
    let k = env.call_static_object_method(class_cls, m_for_name, &[jv(jn), jz(true), jv(loader)]);
    clear_if_pending(env);
    jobj(k).map(|c| env.new_global_ref(c) as jobject)
}

unsafe fn resolve(env: Env, world_craft: jobject, feats: bool) -> Option<(SeedIds, NoiseDims)> {
    // Bukkit facade -> ServerLevel (global; workers will use it as the
    // LevelHeightAccessor).
    let craft_cls = env.get_object_class(world_craft);
    let m_get_handle = sgstep!(
        "m_get_handle",
        bind_method(
            env,
            craft_cls,
            craft_cls,
            "getHandle",
            "()Lnet/minecraft/server/level/ServerLevel;"
        )
    );
    let h = env.call_object_method(world_craft, m_get_handle, &[]);
    clear_if_pending(env);
    let level = sgstep!("level handle", jobj(h));
    let level = env.new_global_ref(level) as jobject;

    // ServerChunkCache: getGenerator() + randomState().
    let cache_cls = sgstep!(
        "server_chunk_cache_cls",
        load_cls(env, "net.minecraft.server.level.ServerChunkCache")
    );
    let level_cls = env.get_object_class(level);
    let m_get_cc = sgstep!(
        "m_getChunkSource",
        bind_method(
            env,
            level_cls,
            level_cls,
            "getChunkSource",
            "()Lnet/minecraft/server/level/ServerChunkCache;"
        )
    );
    let cc_o = env.call_object_method(level, m_get_cc, &[]);
    clear_if_pending(env);
    let cc = sgstep!("chunk cache", jobj(cc_o));
    let m_get_generator = sgstep!(
        "m_getGenerator",
        bind_method(
            env,
            cache_cls,
            cache_cls,
            "getGenerator",
            "()Lnet/minecraft/world/level/chunk/ChunkGenerator;"
        )
    );
    let m_random_state = sgstep!(
        "m_randomState",
        bind_method(
            env,
            cache_cls,
            cache_cls,
            "randomState",
            "()Lnet/minecraft/world/level/levelgen/RandomState;"
        )
    );
    let gen_o = env.call_object_method(cc, m_get_generator, &[]);
    clear_if_pending(env);
    let gen = sgstep!("generator", jobj(gen_o));
    let rs_o = env.call_object_method(cc, m_random_state, &[]);
    clear_if_pending(env);
    let rs = sgstep!("random state", jobj(rs_o));
    let gen = env.new_global_ref(gen) as jobject;
    let rs = env.new_global_ref(rs) as jobject;
    // v6.1: the world seed — the feature pipeline derives per-chunk
    // decoration seeds from it (setDecorationSeed(seed, originX, originZ)).
    let m_get_seed = sgstep!(
        "m_getSeed",
        bind_method(env, level_cls, level_cls, "getSeed", "()J")
    );
    let world_seed = env.call_long_method(level, m_get_seed, &[]);
    clear_if_pending(env);

    // NoiseBasedChunkGenerator: generatorSettings().value().
    let gen_cls = sgstep!(
        "noise_gen_cls",
        load_cls(
            env,
            "net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator"
        )
    );
    let m_gen_settings = sgstep!(
        "m_generatorSettings",
        bind_method(
            env,
            gen_cls,
            gen_cls,
            "generatorSettings",
            "()Lnet/minecraft/core/Holder;"
        )
    );
    let holder_o = env.call_object_method(gen, m_gen_settings, &[]);
    clear_if_pending(env);
    let holder = sgstep!("settings holder", jobj(holder_o));
    let holder_cls = env.get_object_class(holder);
    let m_holder_value = sgstep!(
        "m_holder_value",
        bind_method(env, holder_cls, holder_cls, "value", "()Ljava/lang/Object;")
    );
    let settings_o = env.call_object_method(holder, m_holder_value, &[]);
    clear_if_pending(env);
    let settings = sgstep!("settings", jobj(settings_o));
    let settings = env.new_global_ref(settings) as jobject;
    let settings_cls = env.get_object_class(settings);

    // materialRule().value() — the surface-rule tree.
    let m_material_rule = sgstep!(
        "m_materialRule",
        bind_method(
            env,
            settings_cls,
            settings_cls,
            "materialRule",
            "()Lnet/minecraft/core/Holder;"
        )
    );
    let rule_holder_o = env.call_object_method(settings, m_material_rule, &[]);
    clear_if_pending(env);
    let rule_holder = sgstep!("rule holder", jobj(rule_holder_o));
    let rule_holder_cls = env.get_object_class(rule_holder);
    let m_rule_value = sgstep!(
        "m_rule_value",
        bind_method(
            env,
            rule_holder_cls,
            rule_holder_cls,
            "value",
            "()Ljava/lang/Object;"
        )
    );
    let rule_o = env.call_object_method(rule_holder, m_rule_value, &[]);
    clear_if_pending(env);
    let rule = sgstep!("material rule", jobj(rule_o));
    let rule = env.new_global_ref(rule) as jobject;
    let m_default_block = sgstep!(
        "m_defaultBlock",
        bind_method(
            env,
            settings_cls,
            settings_cls,
            "defaultBlock",
            "()Lnet/minecraft/world/level/block/state/BlockState;"
        )
    );
    let db_o = env.call_object_method(settings, m_default_block, &[]);
    clear_if_pending(env);
    let default_block = sgstep!("default block", jobj(db_o));
    let default_block = env.new_global_ref(default_block) as jobject;

    // noiseRouter().finalDensity() — the one function we sample.
    let m_noise_router = sgstep!(
        "m_noiseRouter",
        bind_method(
            env,
            settings_cls,
            settings_cls,
            "noiseRouter",
            "()Lnet/minecraft/world/level/levelgen/NoiseRouter;"
        )
    );
    let router_o = env.call_object_method(settings, m_noise_router, &[]);
    clear_if_pending(env);
    let router = sgstep!("noise router", jobj(router_o));
    let router_cls = sgstep!(
        "router_cls",
        load_cls(env, "net.minecraft.world.level.levelgen.NoiseRouter")
    );
    let m_final_density = sgstep!(
        "m_finalDensity",
        bind_method(
            env,
            router_cls,
            router_cls,
            "finalDensity",
            "()Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;"
        )
    );
    let fd_o = env.call_object_method(router, m_final_density, &[]);
    clear_if_pending(env);
    let final_density = sgstep!("final density", jobj(fd_o));
    let final_density = env.new_global_ref(final_density) as jobject;

    // NoiseSettings clamped to the level -> the volume scalars.
    let m_ns = sgstep!(
        "m_noiseSettings",
        bind_method(
            env,
            settings_cls,
            settings_cls,
            "noiseSettings",
            "()Lnet/minecraft/world/level/levelgen/NoiseSettings;"
        )
    );
    let ns_o = env.call_object_method(settings, m_ns, &[]);
    clear_if_pending(env);
    let ns = sgstep!("noise settings", jobj(ns_o));
    let ns_cls = sgstep!(
        "ns_cls",
        load_cls(env, "net.minecraft.world.level.levelgen.NoiseSettings")
    );
    let m_clamp = sgstep!(
        "m_clampToHeightAccessor",
        bind_method(
            env,
            ns_cls,
            ns_cls,
            "clampToHeightAccessor",
            "(Lnet/minecraft/world/level/LevelHeightAccessor;)Lnet/minecraft/world/level/levelgen/NoiseSettings;"
        )
    );
    let ns_c_o = env.call_object_method(ns, m_clamp, &[jv(level)]);
    clear_if_pending(env);
    let ns_c = sgstep!("clamped ns", jobj(ns_c_o));
    let ns_c_cls = env.get_object_class(ns_c);
    let m_min_y = sgstep!(
        "m_minY",
        bind_method(env, ns_c_cls, ns_c_cls, "minY", "()I")
    );
    let m_height = sgstep!(
        "m_height",
        bind_method(env, ns_c_cls, ns_c_cls, "height", "()I")
    );
    let min_block_y = env.call_int_method(ns_c, m_min_y, &[]);
    let size_y = env.call_int_method(ns_c, m_height, &[]);
    clear_if_pending(env);
    if size_y <= 0 {
        eprintln!("[c-LoD] seedgen: noise height {size_y} <= 0");
        return None;
    }
    let dims = NoiseDims {
        min_block_y,
        size_y,
    };

    // MaterialSystem (surface rules) + WorldGenerationContext.of(level).
    let rs_cls = env.get_object_class(rs);
    let m_surface_system = sgstep!(
        "m_surfaceSystem",
        bind_method(
            env,
            rs_cls,
            rs_cls,
            "surfaceSystem",
            "()Lnet/minecraft/world/level/levelgen/material/MaterialSystem;"
        )
    );
    let surf_o = env.call_object_method(rs, m_surface_system, &[]);
    clear_if_pending(env);
    let surface = sgstep!("material system", jobj(surf_o));
    let surface = env.new_global_ref(surface) as jobject;
    let surface_cls = env.get_object_class(surface);
    let wgctx_cls = sgstep!(
        "wgctx_cls",
        load_cls(
            env,
            "net.minecraft.world.level.levelgen.WorldGenerationContext"
        )
    );
    // NOT WorldGenerationContext.of(accessor): that stores level=null and
    // Paper's OptionallyFlatBedrockCondition calls .level() while compiling
    // the surface rule (NPE on every column). The 3-arg ctor keeps the Level.
    let m_wg_new = sgstep!(
        "m_wg_new",
        env.get_method_id(
            wgctx_cls,
            "<init>",
            "(Lnet/minecraft/world/level/chunk/ChunkGenerator;Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/Level;)V"
        )
    );
    let ctx_o = match env.new_object(wgctx_cls, m_wg_new, &[jv(gen), jv(level), jv(level)]) {
        Some(c) => c,
        None => {
            eprintln!("[c-LoD] seedgen resolve FAILED at: WorldGenerationContext");
            return None;
        }
    };
    let ctx = env.new_global_ref(ctx_o) as jobject;

    // MOTION_BLOCKING predicate (the heightmap semantics Bukkit samples).
    let types_cls = sgstep!(
        "heightmap_types_cls",
        load_cls(env, "net.minecraft.world.level.levelgen.Heightmap$Types")
    );
    let f_mb = sgstep!(
        "f_motion_blocking",
        env.get_static_field_id(
            types_cls,
            "MOTION_BLOCKING",
            "Lnet/minecraft/world/level/levelgen/Heightmap$Types;"
        )
    );
    let mb_o = env.get_static_object_field(types_cls, f_mb);
    let mb = sgstep!("motion blocking type", jobj(mb_o));
    let mb_cls = env.get_object_class(mb);
    let m_is_opaque = sgstep!(
        "m_isOpaque",
        bind_method(
            env,
            mb_cls,
            mb_cls,
            "isOpaque",
            "()Ljava/util/function/Predicate;"
        )
    );
    let pred_o = env.call_object_method(mb, m_is_opaque, &[]);
    clear_if_pending(env);
    let pred = sgstep!("predicate", jobj(pred_o));
    let pred = env.new_global_ref(pred) as jobject;
    let pred_cls = env.get_object_class(pred);
    let m_pred_test = sgstep!(
        "m_pred_test",
        bind_method(env, pred_cls, pred_cls, "test", "(Ljava/lang/Object;)Z")
    );

    // Fluid picker: private field globalFluidPicker -> Supplier.get().
    let m_fluid_picker = sgstep!(
        "f_globalFluidPicker",
        env.get_field_id(
            gen_cls,
            "globalFluidPicker",
            "Ljava/util/function/Supplier;"
        )
    );
    let sup_o = env.get_object_field(gen, m_fluid_picker);
    clear_if_pending(env);
    let sup = sgstep!("fluid picker supplier", jobj(sup_o));
    let sup_cls = env.get_object_class(sup);
    let m_sup_get = sgstep!(
        "m_sup_get",
        bind_method(env, sup_cls, sup_cls, "get", "()Ljava/lang/Object;")
    );
    let fp_o = env.call_object_method(sup, m_sup_get, &[]);
    clear_if_pending(env);
    let fluid_picker = sgstep!("fluid picker", jobj(fp_o));
    let fluid_picker = env.new_global_ref(fluid_picker) as jobject;

    // Blender.empty().
    let blender_cls = sgstep!(
        "blender_cls",
        load_cls(env, "net.minecraft.world.level.levelgen.blending.Blender")
    );
    let m_blend_empty = sgstep!(
        "m_blender_empty",
        env.get_static_method_id(
            blender_cls,
            "empty",
            "()Lnet/minecraft/world/level/levelgen/blending/Blender;"
        )
    );
    let bl_o = env.call_static_object_method(blender_cls, m_blend_empty, &[]);
    clear_if_pending(env);
    let blender = sgstep!("blender", jobj(bl_o));
    let blender = env.new_global_ref(blender) as jobject;

    // Biome manager (direct calls) + the Function for topMaterial.
    let m_get_bman = sgstep!(
        "m_getBiomeManager",
        bind_method(
            env,
            level_cls,
            level_cls,
            "getBiomeManager",
            "()Lnet/minecraft/world/level/biome/BiomeManager;"
        )
    );
    let bman_o = env.call_object_method(level, m_get_bman, &[]);
    clear_if_pending(env);
    let bman = sgstep!("biome manager", jobj(bman_o));
    let bman = env.new_global_ref(bman) as jobject;
    let bman_cls = env.get_object_class(bman);
    let block_pos_cls = sgstep!(
        "block_pos_cls",
        load_cls(env, "net.minecraft.core.BlockPos")
    );
    let m_biome_get = sgstep!(
        "m_biome_getBiome",
        bind_method(
            env,
            bman_cls,
            bman_cls,
            "getBiome",
            "(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/Holder;"
        )
    );
    let biome_fn = unsafe { build_biome_function(env, bman, bman_cls, block_pos_cls)? };

    // Block registry (material names).
    let registries_cls = sgstep!(
        "builtin_registries_cls",
        load_cls(env, "net.minecraft.core.registries.BuiltInRegistries")
    );
    let f_block = sgstep!(
        "f_block_registry",
        env.get_static_field_id(
            registries_cls,
            "BLOCK",
            "Lnet/minecraft/core/DefaultedRegistry;"
        )
    );
    let reg_o = env.get_static_object_field(registries_cls, f_block);
    clear_if_pending(env);
    let block_registry = sgstep!("block registry", jobj(reg_o));
    let block_registry = env.new_global_ref(block_registry) as jobject;
    let reg_cls = env.get_object_class(block_registry);
    let m_reg_get_key = sgstep!(
        "m_getKey",
        bind_method(
            env,
            reg_cls,
            reg_cls,
            "getKey",
            "(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;"
        )
    );
    let identifier_cls = sgstep!(
        "identifier_cls",
        load_cls(env, "net.minecraft.resources.Identifier")
    );
    let m_id_to_string = sgstep!(
        "m_id_toString",
        bind_method(
            env,
            identifier_cls,
            identifier_cls,
            "toString",
            "()Ljava/lang/String;"
        )
    );

    // Optional + Holder.unwrapKey + ResourceKey.identifier (biome names).
    let optional_cls = sgstep!("optional_cls", load_cls(env, "java.util.Optional"));
    let m_opt_present = sgstep!(
        "m_opt_isPresent",
        bind_method(env, optional_cls, optional_cls, "isPresent", "()Z")
    );
    let m_opt_get = sgstep!(
        "m_opt_get",
        bind_method(
            env,
            optional_cls,
            optional_cls,
            "get",
            "()Ljava/lang/Object;"
        )
    );
    let holder_cls = sgstep!("holder_cls", load_cls(env, "net.minecraft.core.Holder"));
    let m_unwrap = sgstep!(
        "m_unwrapKey",
        bind_method(
            env,
            holder_cls,
            holder_cls,
            "unwrapKey",
            "()Ljava/util/Optional;"
        )
    );
    let rk_cls = sgstep!(
        "resource_key_cls",
        load_cls(env, "net.minecraft.resources.ResourceKey")
    );
    let m_rk_id = sgstep!(
        "m_rk_identifier",
        bind_method(
            env,
            rk_cls,
            rk_cls,
            "identifier",
            "()Lnet/minecraft/resources/Identifier;"
        )
    );

    // ProtoChunk (surface-gradient stub; heightmap auto-created on demand).
    let proto_chunk_cls = sgstep!(
        "proto_chunk_cls",
        load_cls(env, "net.minecraft.world.level.chunk.ProtoChunk")
    );
    let upgrade_data_cls = sgstep!(
        "upgrade_data_cls",
        load_cls(env, "net.minecraft.world.level.chunk.UpgradeData")
    );
    let f_up_empty = sgstep!(
        "f_upgrade_empty",
        env.get_static_field_id(
            upgrade_data_cls,
            "EMPTY",
            "Lnet/minecraft/world/level/chunk/UpgradeData;"
        )
    );
    let up_o = env.get_static_object_field(upgrade_data_cls, f_up_empty);
    clear_if_pending(env);
    let upgrade_data = sgstep!("upgrade data empty", jobj(up_o));
    let upgrade_data = env.new_global_ref(upgrade_data) as jobject;
    let m_pcf = sgstep!(
        "m_palettedContainerFactory",
        bind_method(
            env,
            level_cls,
            level_cls,
            "palettedContainerFactory",
            "()Lnet/minecraft/world/level/chunk/PalettedContainerFactory;"
        )
    );
    let pcf_o = env.call_object_method(level, m_pcf, &[]);
    clear_if_pending(env);
    let factory = sgstep!("paletted container factory", jobj(pcf_o));
    let factory = env.new_global_ref(factory) as jobject;
    let chunk_pos_cls = sgstep!(
        "chunk_pos_cls",
        load_cls(env, "net.minecraft.world.level.ChunkPos")
    );
    let m_proto_new = sgstep!(
        "m_proto_new",
        env.get_method_id(
            proto_chunk_cls,
            "<init>",
            "(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/chunk/UpgradeData;Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/chunk/PalettedContainerFactory;Lnet/minecraft/world/level/levelgen/blending/BlendingData;)V"
        )
    );
    let m_pos_new = sgstep!(
        "m_pos_new",
        env.get_method_id(chunk_pos_cls, "<init>", "(II)V")
    );

    // NoiseChunk + DensityVolume + the sampler chain.
    let noise_chunk_cls = sgstep!(
        "noise_chunk_cls",
        load_cls(env, "net.minecraft.world.level.levelgen.NoiseChunk")
    );
    let density_volume_cls = sgstep!(
        "density_volume_cls",
        load_cls(
            env,
            "net.minecraft.world.level.levelgen.densityfunction.DensityVolume"
        )
    );
    let m_nc_new = sgstep!(
        "m_nc_new",
        env.get_method_id(
            noise_chunk_cls,
            "<init>",
            "(Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/world/level/levelgen/Beardifier;Lnet/minecraft/world/level/levelgen/NoiseGeneratorSettings;Lnet/minecraft/world/level/levelgen/Aquifer$FluidPicker;Lnet/minecraft/world/level/levelgen/blending/Blender;Lnet/minecraft/world/level/levelgen/densityfunction/DensityVolume;)V"
        )
    );
    let m_dv_new = sgstep!(
        "m_dv_new",
        env.get_method_id(density_volume_cls, "<init>", "(IIIIII)V")
    );
    let m_nc_cs = sgstep!(
        "m_cachingSamplers",
        bind_method(
            env,
            noise_chunk_cls,
            noise_chunk_cls,
            "cachingSamplers",
            "()Lnet/minecraft/world/level/levelgen/densityfunction/DensitySamplerSet;"
        )
    );
    let m_nc_vol = sgstep!(
        "m_nc_volume",
        bind_method(
            env,
            noise_chunk_cls,
            noise_chunk_cls,
            "volume",
            "()Lnet/minecraft/world/level/levelgen/densityfunction/DensityVolume;"
        )
    );
    let m_nc_aq = sgstep!(
        "m_nc_aquifer",
        bind_method(
            env,
            noise_chunk_cls,
            noise_chunk_cls,
            "aquifer",
            "()Lnet/minecraft/world/level/levelgen/Aquifer;"
        )
    );
    let m_nc_close = sgstep!(
        "m_nc_close",
        bind_method(env, noise_chunk_cls, noise_chunk_cls, "close", "()V")
    );
    let dss_cls = sgstep!(
        "dss_cls",
        load_cls(
            env,
            "net.minecraft.world.level.levelgen.densityfunction.DensitySamplerSet"
        )
    );
    let m_set_get = sgstep!(
        "m_set_get",
        bind_method(
            env,
            dss_cls,
            dss_cls,
            "get",
            "(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;)Lnet/minecraft/world/level/levelgen/densityfunction/DensitySampler$Bound;"
        )
    );
    let bound_cls = sgstep!(
        "bound_cls",
        load_cls(
            env,
            "net.minecraft.world.level.levelgen.densityfunction.DensitySampler$Bound"
        )
    );
    let m_bound_sample = sgstep!(
        "m_sampleVolume",
        bind_method(
            env,
            bound_cls,
            bound_cls,
            "sampleVolume",
            "(Lnet/minecraft/world/level/levelgen/densityfunction/DensityVolume;)Lnet/minecraft/world/level/levelgen/densityfunction/ScopedDensityBuffer;"
        )
    );
    let sdb_cls = sgstep!(
        "scoped_buf_cls",
        load_cls(
            env,
            "net.minecraft.world.level.levelgen.densityfunction.ScopedDensityBuffer"
        )
    );
    let m_buf_get = sgstep!(
        "m_buf_get",
        bind_method(env, sdb_cls, sdb_cls, "get", "(I)F")
    );
    let m_buf_close = sgstep!(
        "m_buf_close",
        bind_method(env, sdb_cls, sdb_cls, "close", "()V")
    );
    let m_dv_block_y = sgstep!(
        "m_dv_blockY",
        bind_method(
            env,
            density_volume_cls,
            density_volume_cls,
            "blockY",
            "(I)I"
        )
    );
    let m_dv_index = sgstep!(
        "m_dv_indexUnchecked",
        bind_method(
            env,
            density_volume_cls,
            density_volume_cls,
            "indexUnchecked",
            "(III)I"
        )
    );
    let aquifer_cls = sgstep!(
        "aquifer_cls",
        load_cls(env, "net.minecraft.world.level.levelgen.Aquifer")
    );
    let m_aq_sub = sgstep!(
        "m_computeSubstance",
        bind_method(
            env,
            aquifer_cls,
            aquifer_cls,
            "computeSubstance",
            "(IIID)Lnet/minecraft/world/level/block/state/BlockState;"
        )
    );

    // MaterialSystem.topMaterial(...).
    let m_top = sgstep!(
        "m_topMaterial",
        bind_method(
            env,
            surface_cls,
            surface_cls,
            "topMaterial",
            "(Lnet/minecraft/world/level/levelgen/material/rule/MaterialRule;Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/world/level/levelgen/WorldGenerationContext;Ljava/util/function/Function;Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/levelgen/densityfunction/DensitySamplerSet;Lnet/minecraft/core/BlockPos;Z)Ljava/util/Optional;"
        )
    );

    // BlockState accessors (resolved on the default_block concrete class).
    let db_cls = env.get_object_class(default_block);
    let m_st_fluid = sgstep!(
        "m_getFluidState",
        bind_method(
            env,
            db_cls,
            db_cls,
            "getFluidState",
            "()Lnet/minecraft/world/level/material/FluidState;"
        )
    );
    let m_st_block = sgstep!(
        "m_getBlock",
        bind_method(
            env,
            db_cls,
            db_cls,
            "getBlock",
            "()Lnet/minecraft/world/level/block/Block;"
        )
    );
    let fs_o = env.call_object_method(default_block, m_st_fluid, &[]);
    clear_if_pending(env);
    let fs = sgstep!("fluid state probe", jobj(fs_o));
    let fs_cls = env.get_object_class(fs);
    let m_fs_empty = sgstep!(
        "m_fs_isEmpty",
        bind_method(env, fs_cls, fs_cls, "isEmpty", "()Z")
    );
    env.delete_local_ref(fs);

    // v5.8: the Java-side chunk helper — the whole noise walk behind ONE JNI
    // call per chunk. Failure here degrades to the v5.7 per-column path,
    // never to seedgen OFF.
    let (helper_cls, _m_helper_init, m_helper_batch, f_mat, f_bio, f_fchunk, f_ffail) =
        define_helper(
            env,
            level,
            rs,
            settings,
            rule,
            surface,
            ctx,
            pred,
            fluid_picker,
            blender,
            default_block,
            final_density,
            upgrade_data,
            factory,
            biome_fn,
            block_registry,
            bman,
            gen,
            world_seed,
            feats,
            min_block_y,
            size_y,
        )
        .unwrap_or((
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ));
    if helper_cls.is_null() {
        eprintln!("[c-LoD] seedgen helper: unavailable — per-column v5.7 path stays");
    }

    let ids = SeedIds {
        level,
        random_state: rs,
        settings,
        rule,
        surface,
        ctx,
        pred,
        fluid_picker,
        blender,
        default_block,
        final_density,
        upgrade_data,
        factory,
        bman,
        biome_fn,
        block_registry,
        noise_chunk_cls,
        density_volume_cls,
        proto_chunk_cls,
        chunk_pos_cls,
        block_pos_cls,
        nc_new: m_nc_new,
        nc_caching_samplers: m_nc_cs,
        nc_volume: m_nc_vol,
        nc_aquifer: m_nc_aq,
        nc_close: m_nc_close,
        dv_new: m_dv_new,
        dv_block_y: m_dv_block_y,
        dv_index: m_dv_index,
        set_get: m_set_get,
        bound_sample: m_bound_sample,
        buf_get: m_buf_get,
        buf_close: m_buf_close,
        aq_substance: m_aq_sub,
        pred_test: m_pred_test,
        top_material: m_top,
        proto_new: m_proto_new,
        pos_new: m_pos_new,
        opt_present: m_opt_present,
        opt_get: m_opt_get,
        st_fluid_state: m_st_fluid,
        fs_empty: m_fs_empty,
        st_block: m_st_block,
        reg_get_key: m_reg_get_key,
        id_to_string: m_id_to_string,
        biome_get: m_biome_get,
        holder_unwrap_key: m_unwrap,
        rk_identifier: m_rk_id,
        helper_cls,
        helper_batch: m_helper_batch,
        f_mat_table: f_mat,
        f_bio_table: f_bio,
        f_feat_chunks: f_fchunk,
        f_feat_fails: f_ffail,
    };
    Some((ids, dims))
}

/// The `Function<BlockPos, Holder<Biome>>` for topMaterial's biomeGetter —
/// pure JDK reflection plumbing: MethodHandles.lookup().unreflect(method)
/// .bindTo(bman) -> MethodHandleProxies.asInterfaceInstance(Function, mh).
unsafe fn build_biome_function(
    env: Env,
    bman: jobject,
    bman_cls: jobject,
    block_pos_cls: jobject,
) -> Option<jobject> {
    let class_cls = cplug_sdk::classes::find_class("java/lang/Class")?.as_jclass();
    let m_get_method = sgstep!(
        "m_reflect_getMethod",
        env.get_method_id(
            class_cls,
            "getMethod",
            "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;"
        )
    );
    let name = env.new_string_utf("getBiome")?;
    let arr = env.new_object_array(1, class_cls, block_pos_cls)?;
    env.set_object_array_element(arr, 0, block_pos_cls);
    let method_o = env.call_object_method(bman_cls, m_get_method, &[jv(name), jv(arr as jobject)]);
    clear_if_pending(env);
    let method = sgstep!("getBiome Method", jobj(method_o));
    let mh_cls = sgstep!(
        "methodhandles_cls",
        load_cls(env, "java.lang.invoke.MethodHandles")
    );
    let m_lookup = sgstep!(
        "m_mh_lookup",
        env.get_static_method_id(
            mh_cls,
            "lookup",
            "()Ljava/lang/invoke/MethodHandles$Lookup;"
        )
    );
    let lookup_o = env.call_static_object_method(mh_cls, m_lookup, &[]);
    clear_if_pending(env);
    let lookup = sgstep!("mh lookup", jobj(lookup_o));
    let lookup_cls = env.get_object_class(lookup);
    let m_unreflect = sgstep!(
        "m_unreflect",
        bind_method(
            env,
            lookup_cls,
            lookup_cls,
            "unreflect",
            "(Ljava/lang/reflect/Method;)Ljava/lang/invoke/MethodHandle;"
        )
    );
    let mh0_o = env.call_object_method(lookup, m_unreflect, &[jv(method)]);
    clear_if_pending(env);
    let mh0 = sgstep!("unreflected mh", jobj(mh0_o));
    let mh_t_cls = sgstep!(
        "methodhandle_cls",
        load_cls(env, "java.lang.invoke.MethodHandle")
    );
    let m_bind_to = sgstep!(
        "m_bindTo",
        bind_method(
            env,
            mh_t_cls,
            mh_t_cls,
            "bindTo",
            "(Ljava/lang/Object;)Ljava/lang/invoke/MethodHandle;"
        )
    );
    let mh1_o = env.call_object_method(mh0, m_bind_to, &[jv(bman)]);
    clear_if_pending(env);
    let mh1 = sgstep!("bound mh", jobj(mh1_o));
    let proxies_cls = sgstep!(
        "proxies_cls",
        load_cls(env, "java.lang.invoke.MethodHandleProxies")
    );
    let function_cls = sgstep!("function_cls", load_cls(env, "java.util.function.Function"));
    let m_as_iface = sgstep!(
        "m_asInterfaceInstance",
        env.get_static_method_id(
            proxies_cls,
            "asInterfaceInstance",
            "(Ljava/lang/Class;Ljava/lang/invoke/MethodHandle;)Ljava/lang/Object;"
        )
    );
    let fn_o = env.call_static_object_method(proxies_cls, m_as_iface, &[jv(function_cls), jv(mh1)]);
    clear_if_pending(env);
    let f = sgstep!("biome function", jobj(fn_o));
    Some(env.new_global_ref(f) as jobject)
}

/// Define + init the Java-side chunk helper (v5.8). The bytecode is compiled
/// from `java/ClodSeedgen.java` (JDK-only source, see scripts/java/build.sh)
/// and embedded in the module; `ClassLoader.defineClass` is reached through
/// JNI, which bypasses the protected modifier. `init` receives the global
/// refs resolved above and resolves every MethodHandle Java-side; any throw
/// degrades to the v5.7 per-column path.
#[allow(clippy::too_many_arguments)]
unsafe fn define_helper(
    env: Env,
    level: jobject,
    rs: jobject,
    settings: jobject,
    rule: jobject,
    surface: jobject,
    ctx: jobject,
    pred: jobject,
    fluid_picker: jobject,
    blender: jobject,
    default_block: jobject,
    final_density: jobject,
    upgrade_data: jobject,
    factory: jobject,
    biome_fn: jobject,
    block_registry: jobject,
    bman: jobject,
    gen: jobject,
    world_seed: i64,
    feats: bool,
    min_block_y: i32,
    size_y: i32,
) -> Option<(
    jobject,
    jmethodID,
    jmethodID,
    jfieldID,
    jfieldID,
    jfieldID,
    jfieldID,
)> {
    const HELPER: &[u8] = include_bytes!("../java/ClodSeedgen.class");
    let loader = loader_obj();
    if loader.is_null() {
        eprintln!("[c-LoD] seedgen helper: loader stash empty");
        return None;
    }
    let cl_cls = load_cls(env, "java.lang.ClassLoader")?;
    let m_define = sgstep!(
        "m_defineClass",
        env.get_method_id(
            cl_cls,
            "defineClass",
            "(Ljava/lang/String;[BII)Ljava/lang/Class;"
        )
    );
    let name = env.new_string_utf("clod.ClodSeedgen")?;
    let arr = env.new_byte_array(HELPER.len() as i32)?;
    let bytes: &[i8] = std::slice::from_raw_parts(HELPER.as_ptr() as *const i8, HELPER.len());
    env.set_byte_array_region(arr, 0, HELPER.len() as i32, bytes);
    clear_if_pending(env);
    let cls_o = env.call_object_method(
        loader,
        m_define,
        &[jv(name), jv(arr as jobject), ji(0), ji(HELPER.len() as i32)],
    );
    clear_if_pending(env);
    let cls = jobj(cls_o)?;
    let cls = env.new_global_ref(cls) as jobject;
    let m_init = sgstep!(
        "m_helper_init",
        env.get_static_method_id(
            cls,
            "init",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;\
             Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;\
             Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;\
             Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;\
             Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;\
             Ljava/lang/Object;Ljava/lang/Object;JIII)V"
        )
    );
    env.call_static_void_method(
        cls,
        m_init,
        &[
            jv(level),
            jv(rs),
            jv(settings),
            jv(rule),
            jv(surface),
            jv(ctx),
            jv(pred),
            jv(fluid_picker),
            jv(blender),
            jv(default_block),
            jv(final_density),
            jv(upgrade_data),
            jv(factory),
            jv(biome_fn),
            jv(block_registry),
            jv(bman),
            jv(gen),
            jl(world_seed),
            ji(min_block_y),
            ji(size_y),
            ji(if feats { 1 } else { 0 }),
        ],
    );
    if env.exception_check() {
        env.exception_describe();
        env.exception_clear();
        eprintln!("[c-LoD] seedgen helper: init threw — per-column path stays");
        return None;
    }
    let m_batch = sgstep!(
        "m_helper_batch",
        env.get_static_method_id(cls, "chunkBatch", "([II)[J")
    );
    let f_mat = sgstep!(
        "f_matTable",
        env.get_static_field_id(cls, "matTable", "[Ljava/lang/String;")
    );
    let f_bio = sgstep!(
        "f_bioTable",
        env.get_static_field_id(cls, "bioTable", "[Ljava/lang/String;")
    );
    let f_fchunk = sgstep!(
        "f_featChunks",
        env.get_static_field_id(cls, "featChunks", "J")
    );
    let f_ffail = sgstep!(
        "f_featFails",
        env.get_static_field_id(cls, "featFails", "J")
    );
    Some((cls, m_init, m_batch, f_mat, f_bio, f_fchunk, f_ffail))
}

// ---------------------------------------------------------------------------
// Generation (worker threads)
// ---------------------------------------------------------------------------

/// Generate one 16x16 chunk's LOD columns with the vanilla noise+surface
/// pipeline (the v5.7 per-column walk). This is the fallback path: the
/// helper batch (gen_batch) owns the hot path and hands single chunks over
/// here when their batch segment threw. Returns the number of cells written.
///
/// # Safety
/// `env` must be attached, ids resolved on the main thread, all stored
/// objects must be global refs. No JVM objects escape this function.
unsafe fn gen_chunk_legacy(
    state: &Arc<engine::State>,
    env: Env,
    ids: &SeedIds,
    dims: NoiseDims,
    cx: i32,
    cz: i32,
) -> Result<usize, String> {
    let cell = state.cfg.cell_size.max(1);
    let wx0 = cx * 16;
    let wz0 = cz * 16;

    // Disk-space mapping (workers write only cells inside the disk; edge
    // chunks degrade gracefully).
    let (origin_x, origin_z, side) = {
        let s = state.stream.lock().unwrap();
        (s.origin_x, s.origin_z, s.side)
    };
    let cell_of = |wx: i32, wz: i32| -> Option<(u32, u32)> {
        let fx = ((wx as f64 - origin_x) / cell as f64).floor() as i32;
        let fz = ((wz as f64 - origin_z) / cell as f64).floor() as i32;
        if fx < 0 || fz < 0 || fx >= side as i32 || fz >= side as i32 {
            return None;
        }
        let bx = origin_x + f64::from(fx) * cell as f64;
        let bz = origin_z + f64::from(fz) * cell as f64;
        if (f64::from(wx) - bx).abs() >= cell as f64 || (f64::from(wz) - bz).abs() >= cell as f64 {
            return None;
        }
        Some((fx as u32, fz as u32))
    };

    // --- one NoiseChunk over the whole chunk volume ---------------------
    let vol = env
        .new_object(
            ids.density_volume_cls,
            ids.dv_new,
            &[
                ji(16),
                ji(dims.size_y),
                ji(16),
                ji(wx0),
                ji(dims.min_block_y),
                ji(wz0),
            ],
        )
        .ok_or("DensityVolume")?;
    let nc = env
        .new_object(
            ids.noise_chunk_cls,
            ids.nc_new,
            &[
                jv(ids.random_state),
                jv(std::ptr::null_mut()), // Beardifier: vanilla passes null here
                jv(ids.settings),
                jv(ids.fluid_picker),
                jv(ids.blender),
                jv(vol),
            ],
        )
        .ok_or("NoiseChunk")?;

    let out = (|| -> Result<usize, String> {
        let aq_o = env.call_object_method(nc, ids.nc_aquifer, &[]);
        let aquifer = jobj(aq_o).ok_or("aquifer")?;
        let set_o = env.call_object_method(nc, ids.nc_caching_samplers, &[]);
        let set = jobj(set_o).ok_or("cachingSamplers")?;
        let bound_o = env.call_object_method(set, ids.set_get, &[jv(ids.final_density)]);
        let bound = jobj(bound_o).ok_or("sampler get")?;
        let vol_o = env.call_object_method(nc, ids.nc_volume, &[]);
        let ncv = jobj(vol_o).ok_or("volume")?;
        let buf_o = env.call_object_method(bound, ids.bound_sample, &[jv(ncv)]);
        let buf = jobj(buf_o).ok_or("sampleVolume")?;

        // Throwaway ProtoChunk for topMaterial's gradient (empty heightmap
        // -> gradient 0 -> flat-surface semantics).
        let pos_o = env
            .new_object(ids.chunk_pos_cls, ids.pos_new, &[ji(cx), ji(cz)])
            .ok_or("ChunkPos")?;
        let proto = env
            .new_object(
                ids.proto_chunk_cls,
                ids.proto_new,
                &[
                    jv(pos_o),
                    jv(ids.upgrade_data),
                    jv(ids.level),
                    jv(ids.factory),
                    jv(std::ptr::null_mut()),
                ],
            )
            .ok_or("ProtoChunk")?;

        let mut written = 0usize;
        // One column per LOD cell over the chunk.
        let mut dx = 0;
        while dx < 16 {
            let mut dz = 0;
            while dz < 16 {
                let wx = wx0 + dx;
                let wz = wz0 + dz;
                if let Some((ccx, ccz)) = cell_of(wx, wz) {
                    let skip = {
                        let st = state.stream.lock().unwrap();
                        st.get(ccx, ccz).map(|c| c.valid).unwrap_or(false)
                    };
                    if !skip {
                        match gen_column(
                            state, env, ids, dims, wx, wz, dx, dz, ccx, ccz, aquifer, buf, ncv,
                            proto, set,
                        ) {
                            Ok(true) => written += 1,
                            Ok(false) => {}
                            Err(e) => {
                                env.call_object_method(buf, ids.buf_close, &[]);
                                env.call_object_method(nc, ids.nc_close, &[]);
                                clear_if_pending(env);
                                return Err(e);
                            }
                        }
                    }
                }
                dz += cell;
            }
            dx += cell;
        }

        env.call_object_method(buf, ids.buf_close, &[]);
        env.call_object_method(nc, ids.nc_close, &[]);
        clear_if_pending(env);
        Ok(written)
    })();

    env.delete_local_ref(vol);
    env.delete_local_ref(nc);
    out
}

/// One LOD column: top-down density walk + surface rule + palette encode.
/// Returns Ok(true) when a cell was written.
#[allow(clippy::too_many_arguments)]
unsafe fn gen_column(
    state: &Arc<engine::State>,
    env: Env,
    ids: &SeedIds,
    dims: NoiseDims,
    wx: i32,
    wz: i32,
    dx: i32,
    dz: i32,
    ccx: u32,
    ccz: u32,
    aquifer: jobject,
    buf: jobject,
    vol: jobject,
    proto: jobject,
    set: jobject,
) -> Result<bool, String> {
    // The y-loop churns local refs: one frame per column.
    if env.push_local_frame(48).is_err() {
        return Err("push_local_frame".into());
    }
    let out = (|| -> Result<bool, String> {
        let mut hit_y: Option<i32> = None;
        let mut top_state = ids.default_block;
        let mut i = dims.size_y - 1;
        while i >= 0 {
            let idx = env.call_int_method(vol, ids.dv_index, &[ji(dx), ji(i), ji(dz)]);
            let d = env.call_float_method(buf, ids.buf_get, &[ji(idx)]);
            let y = env.call_int_method(vol, ids.dv_block_y, &[ji(i)]);
            if env.exception_check() {
                env.exception_clear();
                return Err("y-loop JNI".into());
            }
            let st_o = env.call_object_method(
                aquifer,
                ids.aq_substance,
                &[ji(wx), ji(y), ji(wz), jd(f64::from(d))],
            );
            let st = match jobj(st_o) {
                Some(s) => s,
                None => {
                    if env.exception_check() {
                        env.exception_clear();
                        return Err("computeSubstance".into());
                    }
                    ids.default_block
                }
            };
            let hit = env.call_boolean_method(ids.pred, ids.pred_test, &[jv(st)]);
            if env.exception_check() {
                env.exception_clear();
                return Err("predicate test".into());
            }
            if hit {
                hit_y = Some(y);
                top_state = st;
                break;
            }
            i -= 1;
        }

        // Void column (outside the world border): leave the cell invalid —
        // the march's void semantics own it.
        let Some(top_y) = hit_y else {
            return Ok(false);
        };

        // Surface rule for the top material (grass/sand/snow/badlands...).
        let pos_o = env
            .new_object(ids.block_pos_cls, ids.pos_new, &[ji(wx), ji(top_y), ji(wz)])
            .ok_or("BlockPos")?;
        let top_opt = env.call_object_method(
            ids.surface,
            ids.top_material,
            &[
                jv(ids.rule),
                jv(ids.random_state),
                jv(ids.ctx),
                jv(ids.biome_fn),
                jv(proto),
                jv(set),
                jv(pos_o),
                jz(false), // steep=false: the gradient is empty-chunk-flat anyway
            ],
        );
        if env.exception_check() {
            env.exception_describe();
            env.exception_clear();
            return Err("topMaterial".into());
        }
        let mut st = top_state;
        if let Some(opt) = jobj(top_opt) {
            let present = env.call_boolean_method(opt, ids.opt_present, &[]);
            if present {
                if let Some(b) = jobj(env.call_object_method(opt, ids.opt_get, &[])) {
                    st = b;
                }
            }
            env.delete_local_ref(opt);
        }

        // Liquid?
        let fs_o = env.call_object_method(st, ids.st_fluid_state, &[]);
        let mut liquid = false;
        if let Some(fs) = jobj(fs_o) {
            liquid = !env.call_boolean_method(fs, ids.fs_empty, &[]);
            env.delete_local_ref(fs);
        }
        if env.exception_check() {
            env.exception_clear();
        }

        // Material name: Registries.BLOCK.getKey(block) -> "minecraft:x".
        let material = {
            let blk_o = env.call_object_method(st, ids.st_block, &[]);
            match jobj(blk_o) {
                Some(blk) => {
                    let id_o =
                        env.call_object_method(ids.block_registry, ids.reg_get_key, &[jv(blk)]);
                    let name = jobj(id_o)
                        .and_then(|id| jstr(env, env.call_object_method(id, ids.id_to_string, &[])))
                        .unwrap_or_default();
                    clear_if_pending(env);
                    name.strip_prefix("minecraft:")
                        .unwrap_or(&name)
                        .to_ascii_uppercase()
                }
                None => {
                    clear_if_pending(env);
                    String::new()
                }
            }
        };

        // Biome name: BiomeManager.getBiome(pos) -> Holder -> key -> id.
        let biome = {
            let h_o = env.call_object_method(ids.bman, ids.biome_get, &[jv(pos_o)]);
            match jobj(h_o) {
                Some(h) => {
                    let opt_o = env.call_object_method(h, ids.holder_unwrap_key, &[]);
                    let mut name = String::new();
                    if let Some(opt) = jobj(opt_o) {
                        let present = env.call_boolean_method(opt, ids.opt_present, &[]);
                        if present {
                            if let Some(rk) = jobj(env.call_object_method(opt, ids.opt_get, &[])) {
                                let id_o = env.call_object_method(rk, ids.rk_identifier, &[]);
                                name = jobj(id_o)
                                    .and_then(|id| {
                                        jstr(env, env.call_object_method(id, ids.id_to_string, &[]))
                                    })
                                    .unwrap_or_default();
                                env.delete_local_ref(rk);
                            }
                        }
                        env.delete_local_ref(opt);
                    }
                    clear_if_pending(env);
                    name.strip_prefix("minecraft:")
                        .unwrap_or(&name)
                        .to_ascii_uppercase()
                }
                None => {
                    clear_if_pending(env);
                    String::new()
                }
            }
        };

        let rgb = palette::surface_color(&material, &biome, liquid);
        let cell = crate::stream::Cell {
            height: top_y.clamp(-64, 1023) as i16,
            water: liquid,
            valid: true,
            provisional: true,
            rgb,
        };
        state.stream.lock().unwrap().set_cell(ccx, ccz, cell);
        Ok(true)
    })();
    let _ = env.pop_local_frame(std::ptr::null_mut());
    out
}

/// v5.9 fast path: one JNI call per BATCH of chunks. The Java helper
/// (java/ClodSeedgen.chunkBatch) runs the whole per-chunk noise walk inside
/// the JVM and returns a payload: header [n, n_ok, tabVer, spare] plus one
/// [status, cells…] segment per chunk. This side parses the payload, re-reads
/// the intern tables only when tabVer moved, filters by the disk window and
/// paints — under ONE stream lock for the whole batch. Failed segments come
/// back Err so the caller falls back to the legacy per-column walk for
/// exactly those chunks. The recipe inside the helper is the same
/// iterateNoiseColumn transcription, byte-for-byte — outputs are identical
/// to the legacy path.
///
/// # Safety
/// Same contract as `gen_chunk_legacy`; the helper class must be armed.
unsafe fn gen_batch(
    state: &Arc<engine::State>,
    env: Env,
    ids: &SeedIds,
    tables: &mut Tables,
    jobs: &[(i32, i32)],
) -> Vec<Result<usize, String>> {
    let k = jobs.len();
    let cell = state.cfg.cell_size.max(1);
    let n_side = (16 / cell) as usize;
    if cell > 16 || 16 % cell != 0 {
        return vec![Err("cell_size must divide 16".into()); k];
    }
    if env.push_local_frame(64).is_err() {
        return vec![Err("push_local_frame".into()); k];
    }
    let out = (|| -> Vec<Result<usize, String>> {
        // coords = [cx0,cz0,cx1,cz1,...] for the single batch call.
        let mut coords = Vec::with_capacity(k * 2);
        for (cx, cz) in jobs {
            coords.push(*cx);
            coords.push(*cz);
        }
        let Some(arr) = env.new_int_array(coords.len() as i32) else {
            return vec![Err("coord array".into()); k];
        };
        env.set_int_array_region(arr, 0, coords.len() as i32, &coords);
        let payload_o = env.call_static_object_method(
            ids.helper_cls,
            ids.helper_batch,
            &[jv(arr as jobject), ji(cell)],
        );
        env.delete_local_ref(arr);
        if env.exception_check() {
            env.exception_clear();
            return vec![Err("helper batch threw".into()); k];
        }
        let Some(payload_arr) = jobj(payload_o) else {
            return vec![Err("helper batch null".into()); k];
        };
        let plen = env.get_array_length(payload_arr);
        let seg = 1 + n_side * n_side;
        let want = 4 + k * seg;
        if (plen as usize) < want {
            return vec![Err(format!("helper payload {plen} < {want}")); k];
        }
        let mut payload = vec![0i64; plen.max(0) as usize];
        env.get_long_array_region(payload_arr, 0, plen, &mut payload);
        let parsed = match parse_batch(&payload, k, n_side) {
            Ok(p) => p,
            Err(e) => return vec![Err(e); k],
        };

        // Intern tables: re-read only when the payload's tabVer moved
        // (a table re-read costs one JNI call per string).
        if parsed.tab_ver != tables.ver {
            tables.mats = helper_table(env, ids, ids.f_mat_table);
            tables.bios = helper_table(env, ids, ids.f_bio_table);
            tables.ver = parsed.tab_ver;
        }

        // One stream lock for the whole batch (v5.8 locked twice per cell).
        let mut stream = state.stream.lock().unwrap();
        let (origin_x, origin_z, side) = (stream.origin_x, stream.origin_z, stream.side);
        let cell_of = |wx: i32, wz: i32| -> Option<(u32, u32)> {
            let fx = ((wx as f64 - origin_x) / cell as f64).floor() as i32;
            let fz = ((wz as f64 - origin_z) / cell as f64).floor() as i32;
            if fx < 0 || fz < 0 || fx >= side as i32 || fz >= side as i32 {
                return None;
            }
            let bx = origin_x + f64::from(fx) * cell as f64;
            let bz = origin_z + f64::from(fz) * cell as f64;
            if (f64::from(wx) - bx).abs() >= cell as f64
                || (f64::from(wz) - bz).abs() >= cell as f64
            {
                return None;
            }
            Some((fx as u32, fz as u32))
        };

        let mut results: Vec<Result<usize, String>> = vec![Ok(0); k];
        for (i, (ok, cells)) in parsed.segments.iter().enumerate() {
            if !ok {
                results[i] = Err("helper segment threw".into());
                continue;
            }
            let (cx, cz) = jobs[i];
            let mut written = 0usize;
            for gx in 0..n_side {
                for gz in 0..n_side {
                    let l = cells[gx * n_side + gz];
                    if l == 0 {
                        continue; // void column: valid bit clear
                    }
                    let wx = cx * 16 + gx as i32 * cell;
                    let wz = cz * 16 + gz as i32 * cell;
                    let Some((ccx, ccz)) = cell_of(wx, wz) else {
                        continue;
                    };
                    if stream.get(ccx, ccz).map(|c| c.valid).unwrap_or(false) {
                        continue; // already painted (loaded-chunk sample or upgrade)
                    }
                    let (top_y, liquid, mo, bo) = unpack_cell(l);
                    let material = tables.mats.get(mo).map(String::as_str).unwrap_or("");
                    let biome = tables.bios.get(bo).map(String::as_str).unwrap_or("");
                    let rgb = palette::surface_color(material, biome, liquid);
                    let c = crate::stream::Cell {
                        height: top_y.clamp(-64, 1023) as i16,
                        water: liquid,
                        valid: true,
                        provisional: true,
                        rgb,
                    };
                    stream.set_cell(ccx, ccz, c);
                    written += 1;
                }
            }
            results[i] = Ok(written);
        }
        results
    })();
    let _ = env.pop_local_frame(std::ptr::null_mut());
    out
}

/// Per-worker intern-table snapshots, cached by the batch payload's tabVer.
struct Tables {
    ver: i64,
    mats: Vec<String>,
    bios: Vec<String>,
}

/// Parse the chunkBatch payload: header [n, n_ok, tab_ver, spare] plus k
/// segments of [status, n_side*n_side cells]. Zero-copy: the cell slices
/// borrow the payload. Mirrors java/ClodSeedgen.chunkBatch.
fn parse_batch<'a>(payload: &'a [i64], k: usize, n_side: usize) -> Result<BatchParsed<'a>, String> {
    let seg = 1 + n_side * n_side;
    let want = 4 + k * seg;
    if payload.len() < want {
        return Err(format!("payload {} < {want}", payload.len()));
    }
    if payload[0] != k as i64 {
        return Err(format!(
            "payload header says {} chunks, sent {k}",
            payload[0]
        ));
    }
    let mut segments = Vec::with_capacity(k);
    for i in 0..k {
        let off = 4 + i * seg;
        segments.push((payload[off] == 1, &payload[off + 1..off + seg]));
    }
    Ok(BatchParsed {
        tab_ver: payload[2],
        segments,
    })
}

struct BatchParsed<'a> {
    tab_ver: i64,
    segments: Vec<(bool, &'a [i64])>,
}

/// Read one String[] intern-table snapshot from the helper.
unsafe fn helper_table(env: Env, ids: &SeedIds, field: jfieldID) -> Vec<String> {
    let mut out = Vec::new();
    let arr = env.get_static_object_field(ids.helper_cls, field);
    if let Some(arr) = jobj(arr) {
        let n = env.get_array_length(arr);
        for i in 0..n {
            let s = env.get_object_array_element(arr, i);
            if let Some(name) = jstr(env, s) {
                out.push(name);
            }
            if !s.is_null() {
                env.delete_local_ref(s);
            }
        }
    }
    clear_if_pending(env);
    out
}

/// Bit contract mirrored from `java/ClodSeedgen.java` `pack()`: bits 0-15
/// height+32768, bit 16 liquid, bits 17-31 material ordinal, bits 32-46
/// biome ordinal, bit 63 valid. Only called for nonzero (valid) longs.
fn unpack_cell(l: i64) -> (i32, bool, usize, usize) {
    let y = ((l & 0xFFFF) as u32 as i32) - 32768;
    let liquid = (l & (1 << 16)) != 0;
    let mat = ((l >> 17) & 0x7FFF) as usize;
    let bio = ((l >> 32) & 0x7FFF) as usize;
    (y, liquid, mat, bio)
}

// ---------------------------------------------------------------------------
// Tests (pure logic — no JVM)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_dedups_and_orders() {
        let q = SeedQueue::new();
        assert!(q.push(1, 2));
        assert!(!q.push(1, 2), "duplicate must be rejected");
        assert!(q.push(3, 4));
        assert_eq!(q.inflight_len(), 2);
        let one = |q: &SeedQueue| q.pop_batch(1, 1).and_then(|mut v| v.pop());
        assert_eq!(one(&q), Some((1, 2)));
        assert_eq!(one(&q), Some((3, 4)));
        q.done(1, 2);
        assert_eq!(q.inflight_len(), 1);
        assert!(q.push(1, 2), "completed chunk can re-queue");
    }

    #[test]
    fn queue_close_unblocks() {
        let q = SeedQueue::new();
        q.close();
        assert!(!q.push(5, 5), "closed queue rejects");
        assert_eq!(q.pop_batch(1, 1), None);
    }

    #[test]
    fn queue_pop_waits_then_close_wakes() {
        // pop keeps waiting while the queue runs; a close() from another
        // thread must wake it up with None.
        let q = Arc::new(SeedQueue::new());
        let q2 = Arc::clone(&q);
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(50));
            q2.close();
        });
        let t = std::time::Instant::now();
        assert_eq!(q.pop_batch(1, 5000), None, "closed queue returns None");
        assert!(t.elapsed() >= std::time::Duration::from_millis(40));
    }

    #[test]
    fn queue_pop_returns_job_across_threads() {
        let q = Arc::new(SeedQueue::new());
        let q2 = Arc::clone(&q);
        assert!(q.push(7, 9));
        let job = std::thread::spawn(move || q2.pop_batch(1, 5000).and_then(|mut v| v.pop()))
            .join()
            .unwrap();
        assert_eq!(job, Some((7, 9)));
    }

    /// Mirror of ClodSeedgen.pack() (java/ClodSeedgen.java) — the two sides
    /// of the v5.8 contract must agree bit-for-bit.
    fn java_pack(y: i64, liquid: bool, mat: i64, bio: i64) -> i64 {
        let mut l = (y + 32768) & 0xFFFF;
        if liquid {
            l |= 1 << 16;
        }
        l |= mat << 17;
        l |= bio << 32;
        l |= 1 << 63;
        l
    }

    #[test]
    fn unpack_matches_java_pack() {
        for y in [-64i64, 0, 62, 63, 100, 319, 320] {
            for liquid in [false, true] {
                for (mat, bio) in [(0i64, 0), (1, 2), (7, 9), (32767, 32767)] {
                    let l = java_pack(y, liquid, mat, bio);
                    let (hy, lq, mo, bo) = unpack_cell(l);
                    assert_eq!(hy, y as i32, "height round-trip {y}");
                    assert_eq!(lq, liquid, "liquid round-trip {y}/{liquid}");
                    assert_eq!(mo, mat as usize, "material ordinal round-trip");
                    assert_eq!(bo, bio as usize, "biome ordinal round-trip");
                    assert!(l < 0, "valid bit (sign) must be set");
                }
            }
        }
    }

    #[test]
    fn void_cell_is_zero_and_skipped() {
        // The helper emits 0 for a void column; the native path must treat
        // exactly 0 as void (valid bit clear) and skip it before unpacking.
        assert_eq!(java_pack(64, false, 0, 0) >> 63, -1);
        let void = 0i64;
        assert_eq!(void >> 63, 0, "void long has no valid bit");
        // unpacking a void long is meaningless and never happens; document
        // the sentinel here so the contract stays obvious.
        let (y, _liquid, _m, _b) = unpack_cell(1 << 63);
        assert_eq!(y, -32768);
    }

    #[test]
    fn ordinal_bit_budget() {
        // 15 bits per table: 32767 max ordinal must survive the round trip,
        // bits 47..62 must stay zero so no future field collides.
        let l = java_pack(100, true, 32767, 32767);
        assert_eq!((l >> 47) & 0x7FFF, 0, "bits 47..62 must stay free");
        let (_, _, mo, bo) = unpack_cell(l);
        assert_eq!(mo, 32767);
        assert_eq!(bo, 32767);
    }

    /// Mirror of ClodSeedgen.chunkBatch()'s payload layout: header
    /// [n, n_ok, tabVer, spare] + per-chunk [status, n*n cells].
    fn java_batch_payload(chunks: &[(i64, bool, Vec<i64>)], tab_ver: i64) -> Vec<i64> {
        let k = chunks.len() as i64;
        let seg_cells = chunks.first().map(|c| c.2.len()).unwrap_or(0);
        let mut out = vec![0i64; 4 + chunks.len() * (1 + seg_cells)];
        out[0] = k;
        out[1] = chunks.iter().filter(|c| c.1).count() as i64;
        out[2] = tab_ver;
        for (i, (_, ok, cells)) in chunks.iter().enumerate() {
            let off = 4 + i * (1 + seg_cells);
            out[off] = *ok as i64;
            out[off + 1..off + 1 + seg_cells].copy_from_slice(cells);
        }
        out
    }

    #[test]
    fn batch_payload_round_trip() {
        let cells_ok: Vec<i64> = (0..64).map(java_pack_i).collect();
        let cells_void = vec![0i64; 64];
        let payload = java_batch_payload(
            &[
                (10, true, cells_ok.clone()),
                (11, true, cells_void.clone()),
                (12, true, cells_ok.clone()),
            ],
            7,
        );
        let p = parse_batch(&payload, 3, 8).expect("valid payload parses");
        assert_eq!(p.tab_ver, 7);
        assert_eq!(p.segments.len(), 3);
        assert!(p.segments.iter().all(|(ok, _)| *ok));
        for (gx, gz) in [(0usize, 0usize), (3, 5), (7, 7)] {
            let (y, liquid, mo, bo) = unpack_cell(p.segments[0].1[gx * 8 + gz]);
            let (y0, l0, m0, b0) = unpack_cell(java_pack_i((gx * 8 + gz) as i64));
            assert_eq!((y, liquid, mo, bo), (y0, l0, m0, b0));
        }
        assert!(
            p.segments[1].1.iter().all(|l| *l == 0),
            "void cells stay zero"
        );
    }

    #[test]
    fn batch_failed_segment_flagged() {
        let cells_ok: Vec<i64> = (0..64).map(java_pack_i).collect();
        // Java zeroes a throwing chunk's whole segment (status 0 + zero cells).
        let payload =
            java_batch_payload(&[(1, true, cells_ok.clone()), (2, false, vec![0; 64])], 3);
        let p = parse_batch(&payload, 2, 8).expect("parses");
        assert!(p.segments[0].0, "first chunk OK");
        assert!(!p.segments[1].0, "failed chunk flagged");
        assert!(
            p.segments[1].1.iter().all(|l| *l == 0),
            "failed segment zero-filled"
        );
    }

    #[test]
    fn batch_payload_rejects_corruption() {
        let cells: Vec<i64> = vec![0; 64];
        let good = java_batch_payload(&[(1, true, cells.clone())], 1);
        let mut short = good.clone();
        short.pop();
        assert!(parse_batch(&short, 1, 8).is_err(), "short payload rejected");
        let mut wrong_header = good.clone();
        wrong_header[0] = 5; // says 5 chunks, sent 1
        assert!(
            parse_batch(&wrong_header, 1, 8).is_err(),
            "header mismatch rejected"
        );
    }

    fn java_pack_i(i: i64) -> i64 {
        java_pack(i % 300 - 50, i % 2 == 0, i % 5, i % 7)
    }

    #[test]
    fn queue_pop_batch_drains_up_to_max() {
        let q = SeedQueue::new();
        for i in 0..10 {
            assert!(q.push(i, i));
        }
        let b = q.pop_batch(4, 1).expect("jobs ready");
        assert_eq!(b.len(), 4, "drain respects max");
        let b2 = q.pop_batch(100, 1).expect("jobs ready");
        assert_eq!(b2.len(), 6, "remainder drained in one batch");
        assert_eq!(q.inflight_len(), 10);
        for (cx, cz) in b.iter().chain(b2.iter()) {
            q.done(*cx, *cz);
        }
        assert_eq!(q.inflight_len(), 0);
    }

    #[test]
    fn queue_pop_batch_blocks_then_wakes() {
        let q = Arc::new(SeedQueue::new());
        let q2 = Arc::clone(&q);
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(50));
            q2.push(3, 4);
            q2.push(5, 6);
        });
        let b = q.pop_batch(2, 5000).expect("woken by pushes");
        assert_eq!(b, vec![(3, 4), (5, 6)]);
    }
}
