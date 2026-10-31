//! v6.0 NATIVE DUST WIRE — the packet side of the transport rewrite.
//!
//! The legacy path paid the full Java tax for EVERY particle: two JNI
//! crossings (getHandle + connection field), one NMS record constructor,
//! a vanilla Connection.send dispatch and a Netty write per particle.
//! The wire path amortizes all of it:
//!
//!   - ONE ClientboundLevelParticlesPacket object per UNIQUE spawn
//!     position, built once and cached as a global ref. Spawn positions
//!     live on the cell lattice (cell origins are integer/half block
//!     values, sub-cell offsets are quarter multiples, the two plane Ys
//!     are exact), so scaling by 16 maps every distinct position to a
//!     distinct exact i64 key — no epsilon, no collisions;
//!   - the per-tick batch rides ONE ClientboundBundlePacket per player
//!     (single frame, single encode, single Netty write) instead of one
//!     packet per particle — see `jni::bundle_send`;
//!   - per-player ArrayLists are reused across ticks (clear() per tick).
//!
//! The cached packets are byte-identical to the legacy ones (same
//! constructor, same arguments — `build_particle_packet` is shared), so
//! the client decodes exactly the same packets; only the envelope and
//! the server-side cost changed. The steady state after the first lap
//! is ~100% cache hits: zero Java objects allocated per particle.

use crate::jni;
use jvmti_bindings::jni::jobject;
use jvmti_bindings::prelude::JniEnv;
use std::collections::HashMap;

type Env<'a> = &'a JniEnv;

/// Position quantum for the cache key (see module docs).
const QUANT: f64 = 16.0;

/// Cache cap in packets. A fully painted 512-block cell-2 disk holds
/// 256x256 cells x 8 spawns = 524k positions; the cap keeps the global-ref
/// table bounded (~a few MB) while age-based eviction keeps the hot set
/// (recently streamed cells) resident.
const CACHE_CAP: usize = 24576;

/// When the map is full, entries untouched for this many accesses are
/// dropped first; if none is that cold, the oldest quarter is force-evicted.
const EVICT_AGE: u64 = 100_000;

/// Exact i64 key of a spawn position (module docs: why 16 is exact).
pub(crate) fn quantize(x: f64, y: f64, z: f64) -> (i64, i64, i64) {
    ((x * QUANT) as i64, (y * QUANT) as i64, (z * QUANT) as i64)
}

struct Entry {
    /// Global ref handle (main-thread-only JNI object).
    pkt: usize,
    /// Cache clock at the last hit (age-based eviction).
    used: u64,
}

/// The shared packet cache (one instance per engine, main-thread-only).
#[derive(Default)]
pub struct PacketCache {
    map: HashMap<(i64, i64, i64), Entry>,
    clock: u64,
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
}

impl PacketCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// The (immutable, shared) particle packet for this spawn position —
    /// built once, then reused by every player and every lap. The returned
    /// ref is a GLOBAL ref owned by the cache (valid for the process
    /// lifetime; do not delete). Main thread only.
    pub fn packet_for(&mut self, env: Env, x: f64, y: f64, z: f64) -> Option<jobject> {
        self.clock += 1;
        let key = quantize(x, y, z);
        if let Some(e) = self.map.get_mut(&key) {
            e.used = self.clock;
            self.hits += 1;
            return Some(e.pkt as jobject);
        }
        self.misses += 1;
        // # Safety: main thread only (the engine contract), NMS resolved.
        let local = unsafe { jni::build_particle_packet(env, x, y, z) }?;
        // # Safety: same thread; local is a valid local ref.
        let g = unsafe { jni::globalize(env, local) };
        self.map.insert(
            key,
            Entry {
                pkt: g,
                used: self.clock,
            },
        );
        if self.map.len() >= CACHE_CAP {
            self.evict(env, EVICT_AGE);
        }
        Some(g as jobject)
    }

    /// Diagnostics line for stream_diag.
    pub fn stat(&self) -> String {
        format!(
            "cache {}/{} h{} m{} e{}",
            self.map.len(),
            CACHE_CAP,
            self.hits,
            self.misses,
            self.evictions
        )
    }

    /// Age-based eviction: drop everything untouched for `age` accesses;
    /// when nothing is that cold, force-drop the oldest quarter. `on_free`
    /// receives each evicted global-ref handle (the caller releases it via
    /// jni::free_global — split out so tests can drive eviction without a
    /// JVM).
    pub(crate) fn evict(&mut self, env: Env, age: u64) {
        self.evict_with(age, |h| unsafe { jni::free_global(env, h) });
    }

    pub(crate) fn evict_with(&mut self, age: u64, mut on_free: impl FnMut(usize)) {
        let cutoff = self.clock.saturating_sub(age);
        let mut stale: Vec<(u64, (i64, i64, i64))> = self
            .map
            .iter()
            .filter(|(_, e)| e.used <= cutoff)
            .map(|(k, e)| (e.used, *k))
            .collect();
        if stale.is_empty() {
            // Nothing cold enough — force-drop the oldest quarter.
            let mut order: Vec<(u64, (i64, i64, i64))> =
                self.map.iter().map(|(k, e)| (e.used, *k)).collect();
            let cut = order.len() / 4;
            order.sort_unstable();
            stale = order.into_iter().take(cut.max(1)).collect();
        } else {
            // Oldest-first: maximal headroom for the hot set.
            stale.sort_unstable();
        }
        for (_, k) in stale {
            if let Some(e) = self.map.remove(&k) {
                on_free(e.pkt);
                self.evictions += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cell-2 lattice: origins are even integers, sub offsets are
    /// {-1.5,-0.5,0.5,1.5} * 0.5 = quarter positions. All exact in f64 and
    /// pairwise distinct after *16.
    #[test]
    fn quant_is_exact_and_collision_free() {
        let sub = 0.5f64;
        let mut keys = std::collections::HashSet::new();
        for cz in 0..128u32 {
            for cx in 0..128u32 {
                let cxw = cx as f64 * 2.0 + 1.0; // cell center
                let czw = cz as f64 * 2.0 + 1.0;
                for slot in 0..4usize {
                    for bits in 0..4u64 {
                        let dx = (bits as f64 - 1.5) * sub;
                        let dz = (slot as f64 - 1.5) * sub;
                        let k = quantize(cxw + dx, 120.0, czw + dz);
                        assert!(keys.insert(k), "cache key collision at {cx}/{cz}");
                    }
                }
                // HI/LO planes never collide either.
                assert_ne!(quantize(cxw, 80.0, czw), quantize(cxw, 120.0, czw));
            }
        }
    }

    /// Half-block origins (player-anchored disks land on .0/.5) stay exact.
    #[test]
    fn quant_handles_half_blocks() {
        let a = quantize(-930.5 + 0.25, 80.0, 796.0 - 0.75);
        let b = quantize(-930.5 - 0.25, 80.0, 796.0 - 0.75);
        assert_ne!(a, b);
        // Round-trip: the key of an already-quantized value is stable.
        assert_eq!(
            quantize(a.0 as f64 / 16.0, a.1 as f64 / 16.0, a.2 as f64 / 16.0),
            a
        );
    }

    /// Eviction: cold entries go first, hot ones survive; a full cache of
    /// hot entries force-drops the oldest quarter. No JVM involved.
    #[test]
    fn evict_prefers_cold_and_bounds_size() {
        let mut c = PacketCache::new();
        let mut freed = Vec::new();
        // 4 inserts, all touched at their insert clock.
        for i in 0..4i64 {
            c.clock += 1;
            c.map.insert(
                (i, 0, 0),
                Entry {
                    pkt: i as usize,
                    used: c.clock,
                },
            );
        }
        // Re-touch entry 0 (hot), leave 1..3 cold.
        c.clock += 1;
        c.map.get_mut(&(0, 0, 0)).unwrap().used = c.clock;
        c.evict_with(1, |h| freed.push(h));
        assert_eq!(freed, vec![1, 2, 3], "cold entries evicted oldest-first");
        assert!(c.map.contains_key(&(0, 0, 0)));
        assert_eq!(c.evictions, 3);

        // Nothing cold: force-drop the oldest quarter.
        freed.clear();
        for i in 10..14i64 {
            c.clock += 1;
            c.map.insert(
                (i, 0, 0),
                Entry {
                    pkt: i as usize,
                    used: c.clock,
                },
            );
        }
        c.evict_with(1_000_000, |h| freed.push(h));
        assert_eq!(freed.len(), 1, "oldest quarter (1 of 5) force-evicted");
        assert_eq!(c.evictions, 4);
    }
}
