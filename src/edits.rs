//! v6.3 EVENT PUSH: the Java-side Bukkit listener (`java/ClodEdits.java`)
//! feeds every block change of the streamed world into a lock-free ring;
//! the engine drains it on the NEXT engine tick and re-samples those
//! columns immediately — detection drops from the rescan wave's
//! `rescan_ticks` to at most one engine tick, and the edit rides the dirty
//! phase (with its v6.2 reset electron) in the very same stream batch.
//!
//! Contract with `ClodEdits` (mirrored by the preflight):
//!   - `static int tryInit(Object world)` — 1 = registered with the first
//!     enabled plugin, 0 = no plugin enabled yet (retry next tick),
//!     -1 = hard failure (stop; the rescan wave stays the detector);
//!   - `static volatile int head` — monotonic push count (u32 arithmetic,
//!     wraps at 2^31);
//!   - `static final long[] RING` — CAP slots; bits 0-25 = x, bits 26-51 =
//!     z (26-bit two's complement each).
//!
//! Producer (event handlers) and consumer (engine tick) both run on the
//! server main thread — no concurrency, the ring is a plain array.
//! Registration needs at least one ENABLED Bukkit plugin (the host only
//! answers the isEnabled() gate at dispatch); a bare server without any
//! plugin keeps the wave as the sole detector. Any other failure degrades
//! the same way — the channel can never break the transport.

use crate::engine::State;
use crate::jni::{clear_if_pending, ji, jobj, jv};
use jvmti_bindings::jni::{jfieldID, jmethodID, jobject};
use jvmti_bindings::prelude::JniEnv;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

type Env<'a> = &'a JniEnv;

/// Ring capacity — must match `ClodEdits.CAP` (power of two).
pub const RING_CAP: usize = 8192;

/// Channel lifecycle: 0 = class not defined yet, 1 = defined, still
/// hunting a host plugin, 2 = registered (draining), 3 = failed (wave-only).
pub const EV_DEFINE_PENDING: u8 = 0;
pub const EV_DEFINING: u8 = 1;
pub const EV_LIVE: u8 = 2;
pub const EV_FAILED: u8 = 3;

/// One defined helper class + the consumer's cursor into its ring. The
/// JVM handles are stored as usize (the State style: raw JNI handles are
/// not Send; the content is main-thread-only).
pub struct EditChannel {
    cls: usize, // global ref
    m_try_init: usize,
    f_head: usize,
    f_ring: usize,
    tail: u32,
}

/// Engine-side event-push state (main-thread content behind a Mutex to
/// match the State style; counters are atomics for the diag line).
pub struct Edits {
    pub channel: Mutex<Option<EditChannel>>,
    pub state: AtomicU8,
    /// Raw ring entries drained since boot.
    pub pushes: AtomicU64,
    /// Unique in-disk columns pushed since boot.
    pub cells: AtomicU64,
    /// Of those, how many changed the encoded words (real edits).
    pub edits: AtomicU64,
    /// Entries skipped because the consumer fell more than CAP behind.
    pub drops: AtomicU64,
}

impl Edits {
    pub fn new() -> Self {
        Self {
            channel: Mutex::new(None),
            state: AtomicU8::new(EV_DEFINE_PENDING),
            pushes: AtomicU64::new(0),
            cells: AtomicU64::new(0),
            edits: AtomicU64::new(0),
            drops: AtomicU64::new(0),
        }
    }

    pub fn state(&self) -> u8 {
        self.state.load(Ordering::Relaxed)
    }
}

/// Unpack one ring entry: bits 0-25 = x, bits 26-51 = z (26-bit two's
/// complement each). Mirrors `ClodEdits.push` bit-for-bit.
pub fn unpack(v: i64) -> (i32, i32) {
    let v = v as u64;
    let x = (v & 0x3FF_FFFF) as u32;
    let z = ((v >> 26) & 0x3FF_FFFF) as u32;
    (sign26(x), sign26(z))
}

fn sign26(v: u32) -> i32 {
    if v >= 1 << 25 {
        v as i32 - (1 << 26)
    } else {
        v as i32
    }
}

/// The two ring segments covering `count` entries starting at `tail`
/// (slot = offset & (CAP-1)): the tail-to-end run and the wrapped head run.
pub fn ring_segments(tail: u32, count: u32) -> ((usize, usize), (usize, usize)) {
    let start = (tail & (RING_CAP as u32 - 1)) as usize;
    let first = (count as usize).min(RING_CAP - start);
    ((start, first), (0, count as usize - first))
}

/// One engine-tick step: define the helper once, register it when a host
/// plugin appears, drain the ring when live. All main thread.
pub fn step(state: &Arc<State>, env: Env) {
    if !state.cfg.native_edits {
        return;
    }
    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        return; // no streaming world yet
    }
    let world = unsafe { as_obj(world_handle) };
    let mut ch_slot = state.edits.channel.lock().unwrap();
    match state.edits.state() {
        EV_DEFINE_PENDING => {
            // Define once per process; a failure is permanent (wave-only).
            if let Some(ch) = define(env) {
                *ch_slot = Some(ch);
                state.edits.state.store(EV_DEFINING, Ordering::Relaxed);
                crate::log_info("edit events: helper defined, hunting a host plugin");
            } else {
                state.edits.state.store(EV_FAILED, Ordering::Relaxed);
                crate::log_info(
                    "edit events: helper failed to define — rescan wave stays the detector",
                );
            }
        }
        EV_DEFINING => {
            let Some(ch) = ch_slot.as_ref() else {
                return;
            };
            // 1 = registered, 0 = no enabled plugin yet, -1 = dead.
            let r = env.call_static_int_method(
                ch.cls as jobject,
                ch.m_try_init as jmethodID,
                &[jv(world)],
            );
            clear_if_pending(env);
            match r {
                1 => {
                    state.edits.state.store(EV_LIVE, Ordering::Relaxed);
                    crate::log_info(
                        "edit events: listener registered — edits push straight into the dirty phase",
                    );
                }
                -1 => {
                    state.edits.state.store(EV_FAILED, Ordering::Relaxed);
                    crate::log_info(
                        "edit events: no host plugin / registration threw — rescan wave stays the detector",
                    );
                }
                _ => {}
            }
        }
        EV_LIVE => {
            let Some(ch) = ch_slot.as_mut() else {
                return;
            };
            unsafe { drain(state, env, ch) };
        }
        _ => {}
    }
}

/// Define `clod.ClodEdits` through the stashed app loader (the seedgen
/// pattern: ClassLoader.defineClass bypasses the protected modifier).
fn define(env: Env) -> Option<EditChannel> {
    const HELPER: &[u8] = include_bytes!("../java/ClodEdits.class");
    let loader = crate::seedgen::loader_obj();
    if loader.is_null() {
        eprintln!("[c-LoD] edits helper: loader stash empty");
        return None;
    }
    let class_cls = cplug_sdk::classes::find_class("java/lang/Class")?.as_jclass();
    let m_define = env.get_method_id(
        class_cls,
        "defineClass",
        "(Ljava/lang/String;[BII)Ljava/lang/Class;",
    )?;
    let name = env.new_string_utf("clod.ClodEdits")?;
    let arr = env.new_byte_array(HELPER.len() as i32)?;
    let bytes: &[i8] =
        unsafe { std::slice::from_raw_parts(HELPER.as_ptr() as *const i8, HELPER.len()) };
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
    let m_try_init = env.get_static_method_id(cls, "tryInit", "(Ljava/lang/Object;)I")?;
    let f_head = env.get_static_field_id(cls, "head", "I")?;
    let f_ring = env.get_static_field_id(cls, "RING", "[J")?;
    Some(EditChannel {
        cls: cls as usize,
        m_try_init: m_try_init as usize,
        f_head: f_head as usize,
        f_ring: f_ring as usize,
        tail: 0,
    })
}

/// Copy the new ring entries out and re-sample their columns now. The
/// consumer runs on the main thread — the same thread the handlers push
/// from — so the cursor arithmetic is race-free by construction.
///
/// # Safety
/// Main thread only; `ch.cls` is a live global ref, the ids are resolved.
unsafe fn drain(state: &Arc<State>, env: Env, ch: &mut EditChannel) {
    let cls = ch.cls as jobject;
    let ring = env.get_static_object_field(cls, ch.f_ring as jfieldID);
    let Some(ring) = jobj(ring) else {
        clear_if_pending(env);
        return;
    };
    let ring = ring as jvmti_bindings::jni::jlongArray;
    let head = env.get_static_int_field(cls, ch.f_head as jfieldID) as u32;
    clear_if_pending(env);
    let mut avail = head.wrapping_sub(ch.tail) as usize;
    if avail == 0 {
        return;
    }
    if avail > RING_CAP {
        // The consumer fell a full lap behind (impossible at engine-tick
        // cadences, but never lose the cursor): keep the freshest CAP.
        state
            .edits
            .drops
            .fetch_add((avail - RING_CAP) as u64, Ordering::Relaxed);
        ch.tail = head.wrapping_sub(RING_CAP as u32);
        avail = RING_CAP;
    }
    let ((s1, n1), (s2, n2)) = ring_segments(ch.tail, avail as u32);
    let mut buf = vec![0i64; avail];
    if n1 > 0 {
        env.get_long_array_region(ring, s1 as i32, n1 as i32, &mut buf[..n1]);
        clear_if_pending(env);
    }
    if n2 > 0 {
        env.get_long_array_region(ring, s2 as i32, n2 as i32, &mut buf[n1..]);
        clear_if_pending(env);
    }
    ch.tail = head;
    state
        .edits
        .pushes
        .fetch_add(avail as u64, Ordering::Relaxed);

    // Dedup columns (floods/explosions push many blocks per cell).
    let mut cols: Vec<(i32, i32)> = buf.iter().map(|&v| unpack(v)).collect();
    cols.sort_unstable();
    cols.dedup();
    state
        .edits
        .cells
        .fetch_add(cols.len() as u64, Ordering::Relaxed);

    let world_handle = *state.world_obj.lock().unwrap();
    if world_handle == 0 {
        return;
    }
    let world = as_obj(world_handle);
    for (wx, wz) in cols {
        // Inside the streamed disk? (world_to_cell returns None outside.)
        let cell = {
            let s = state.stream.lock().unwrap();
            crate::engine::world_to_cell(&s, wx, wz)
        };
        let Some((cx, cz)) = cell else {
            continue;
        };
        let before = {
            let s = state.stream.lock().unwrap();
            s.get(cx, cz)
                .map(|c| crate::stream::Streamer::encode_words(&c))
        };
        // The chunk is loaded (the edit just happened here on this very
        // thread); never queue generation from an edit push.
        crate::engine::scan_cell(state, env, world, cx, cz, wx, wz, false);
        let after = {
            let s = state.stream.lock().unwrap();
            s.get(cx, cz)
                .map(|c| crate::stream::Streamer::encode_words(&c))
        };
        if before != after {
            state.edits.edits.fetch_add(1, Ordering::Relaxed);
        }
    }
}

unsafe fn as_obj(handle: usize) -> jobject {
    handle as jobject
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpack_mirrors_java_pack() {
        // ClodEdits.push: ((long) x & 0x3FFFFFF) | (((long) z & 0x3FFFFFF) << 26)
        let pack = |x: i64, z: i64| (x & 0x3FF_FFFF) | ((z & 0x3FF_FFFF) << 26);
        assert_eq!(unpack(pack(0, 0)), (0, 0));
        assert_eq!(unpack(pack(100, 200)), (100, 200));
        assert_eq!(unpack(pack(-5000, 7)), (-5000, 7));
        assert_eq!(unpack(pack(3, -12345678)), (3, -12345678));
        // The 26-bit two's complement edges.
        assert_eq!(unpack(pack(33_554_431, 0)), (33_554_431, 0));
        assert_eq!(unpack(pack(-33_554_432, 0)), (-33_554_432, 0));
        assert_eq!(unpack(pack(0, 33_554_431)), (0, 33_554_431));
        // x=1 sets bit 0; z=1 sets bit 26 — no cross-talk.
        assert_eq!(unpack(pack(1, 0)), (1, 0));
        assert_eq!(unpack(pack(0, 1)), (0, 1));
    }

    #[test]
    fn ring_segments_cover_wrap_and_linear() {
        // Linear run, no wrap.
        let (a, b) = ring_segments(0, 100);
        assert_eq!(a, (0, 100));
        assert_eq!(b, (0, 0));
        // Wrap: tail near the end of the ring.
        let (a, b) = ring_segments(8190, 10);
        assert_eq!(a, (8190, 2));
        assert_eq!(b, (0, 8));
        // Exactly the capacity, starting mid-ring.
        let (a, b) = ring_segments(4096, RING_CAP as u32);
        assert_eq!(a, (4096, 4096));
        assert_eq!(b, (0, 4096));
        // Masked tail (tail is a monotonic count, not a slot).
        let (a, b) = ring_segments(3 * RING_CAP as u32 + 5, 3);
        assert_eq!(a, (5, 3));
        assert_eq!(b, (0, 0));
        // Zero count.
        let (a, b) = ring_segments(123, 0);
        assert_eq!(a, (123, 0));
        assert_eq!(b, (0, 0));
    }
}
