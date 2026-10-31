package clod;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.util.Arrays;
import java.util.IdentityHashMap;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Predicate;

/**
 * c-LoD SEEDGEN HELPER (v5.9): the whole per-chunk noise walk runs INSIDE the
 * JVM. v5.8 crossed the JNI boundary once per chunk; v5.9 crosses it once per
 * BATCH of chunks (chunkBatch) and strips the remaining per-chunk fat:
 *
 *   - intern tables are no longer read back per chunk — the payload header
 *     carries a monotonically increasing tabVer, the native side re-reads
 *     matTable/bioTable only when it changes (a table read used to cost one
 *     JNI call PER STRING, 60-230 crossings per chunk);
 *   - the per-cell material/biome interning (getBlock -> getKey -> toString
 *     -> map lookup, plus getFluidState/isEmpty) is cached in per-worker
 *     IdentityHashMaps. Vanilla BlockStates and biome Holders are canonical
 *     singletons, so identity equality is exact and the packed result is
 *     bit-identical to recomputing it;
 *   - the y-walk uses affine index/height stepping: indexUnchecked and
 *     blockY are verified LINEAR on three points of EVERY column before the
 *     fast path is taken (any real volume layout is affine; a non-affine
 *     layout automatically falls back to the v5.8 per-step calls). Two of
 *     the five MethodHandle invocations per y-step disappear.
 *
 * The recipe itself is unchanged — the v5.7 iterateNoiseColumn transcription
 * is executed call-for-call in the same order, so outputs stay bit-for-bit
 * identical to the v5.7/v5.8 paths (guarded by the preflight parity harness,
 * scripts/java/preflight).
 *
 * Compiled against the JDK ONLY (javac --release 17, no Minecraft jars):
 * every Minecraft type is reached through MethodHandles resolved in init()
 * from the objects the JNI side passes in. The mojang-mapped names below are
 * the exact ones proven live by the v5.7 Rust resolve() on paper-26.3.
 *
 * Threading: init() runs once on the server main thread; chunk()/chunkBatch()
 * run on the clod-seedgen-N daemon workers. Shared state is immutable-after-
 * init (volatile fields) or concurrent (intern tables); per-call state is
 * local; the identity caches are per-worker ThreadLocals.
 *
 * Output contract (packed long per LOD cell, mirrored by src/seedgen.rs):
 *   bit 63    : valid (1 = terrain hit, 0 = void column -> long is 0)
 *   bits 0-15 : blockY of the surface + 32768
 *   bit 16    : liquid (top block has a non-empty fluid state)
 *   bits 17-31: material ordinal (matTable)
 *   bits 32-46: biome ordinal   (bioTable)
 *
 * chunkBatch payload (long[]):
 *   [0] = number of chunks requested (k)
 *   [1] = segments that completed OK (k minus the ones that threw)
 *   [2] = tabVer snapshot (tables re-read by native only when it moves)
 *   [3] = 0 (spare)
 *   then k segments of (1 + n*n) longs each (dx-major, dz-minor cells):
 *     seg[0] = 1 (chunk OK) | 0 (chunk threw; the rest of the segment is 0)
 *     seg[1..] = packed cells
 */
public final class ClodSeedgen {
    private ClodSeedgen() {}

    // --- method handles (resolved once; volatile so workers see init()) ---
    private static volatile MethodHandle ncNew, ncCachingSamplers, ncVolume,
            ncAquifer, ncClose, dvNew, dvIndex, dvBlockY, setGet, sampleVolume,
            bufGet, bufClose, aqSubstance, topMaterial, protoNew, chunkPosNew,
            blockPosNew, stFluidState, fsEmpty, stBlock, regGetKey,
            holderUnwrapKey, rkIdentifier, biomeGet;
    private static volatile Predicate<Object> pred;
    private static volatile Object randomState, settings, rule, surface, ctx,
            fluidPicker, blender, defaultBlock, finalDensity, upgradeData,
            level, factory, biomeFn, blockRegistry, bman;
    private static volatile int minY, sizeY;
    private static volatile boolean ready;

    // --- intern tables (grown by workers; JNI reads the snapshots) ---
    public static volatile String[] matTable = new String[0];
    public static volatile String[] bioTable = new String[0];
    /** Bumped on every table rebuild; the batch payload header snapshots it
     *  so the native side can cache both tables until it moves. */
    public static volatile int tabVer = 0;
    private static final Map<String, Integer> MAT = new ConcurrentHashMap<>();
    private static final Map<String, Integer> BIO = new ConcurrentHashMap<>();
    private static final AtomicInteger MAT_N = new AtomicInteger();
    private static final AtomicInteger BIO_N = new AtomicInteger();

    // --- v5.9 per-worker identity caches (canonical singletons in vanilla;
    // identity equality is exact; per-worker so no synchronization) ---
    private static final ThreadLocal<IdentityHashMap<Object, Long>> ST_CACHE =
            ThreadLocal.withInitial(IdentityHashMap::new);
    private static final ThreadLocal<IdentityHashMap<Object, Integer>> BIO_CACHE =
            ThreadLocal.withInitial(IdentityHashMap::new);
    private static final int CACHE_CAP = 4096;

    // Telemetry for the preflight harness: how many columns took the affine
    // stride fast path vs the verified fallback.
    public static final java.util.concurrent.atomic.AtomicLong FAST_COLUMNS =
            new java.util.concurrent.atomic.AtomicLong();
    public static final java.util.concurrent.atomic.AtomicLong SLOW_COLUMNS =
            new java.util.concurrent.atomic.AtomicLong();

    // Resolve helpers: full access to Paper's unnamed-module classes.
    private static MethodHandle ctor(Class<?> t, MethodType mt) throws Throwable {
        try {
            return MethodHandles.privateLookupIn(t, MethodHandles.lookup())
                    .findConstructor(t, mt);
        } catch (IllegalAccessException e) {
            return MethodHandles.lookup().in(t).findConstructor(t, mt);
        }
    }

    private static MethodHandle vh(Class<?> t, String name, MethodType mt)
            throws Throwable {
        try {
            return MethodHandles.privateLookupIn(t, MethodHandles.lookup())
                    .findVirtual(t, name, mt);
        } catch (IllegalAccessException e) {
            return MethodHandles.lookup().in(t).findVirtual(t, name, mt);
        }
    }

    /** Adapt a resolved handle to (Object…, primitives kept)Object so the
     *  hot loop can invoke it without boxing. Reference widening is exact
     *  for asType; primitives pass through unchanged. */
    private static MethodHandle asObj(MethodHandle h) {
        MethodType t = h.type().changeReturnType(Object.class);
        Class<?>[] ps = new Class<?>[t.parameterCount()];
        for (int i = 0; i < ps.length; i++) {
            Class<?> p = t.parameterType(i);
            ps[i] = p.isPrimitive() ? p : Object.class;
        }
        return h.asType(MethodType.methodType(Object.class, ps));
    }

    /** One-time resolve. Throws on any failure (the JNI side falls back to
     *  the v5.7 per-column path). All object args are global refs. */
    @SuppressWarnings({"unchecked", "rawtypes"})
    public static synchronized void init(
            Object lvl, Object rs, Object st, Object rl, Object surf,
            Object cx, Object pd, Object fp, Object bl, Object db,
            Object fd, Object ud, Object fac, Object bfn, Object reg,
            Object bm, int minBlockY, int heightY) throws Throwable {
        Class<?> nc = Class.forName("net.minecraft.world.level.levelgen.NoiseChunk");
        Class<?> dv = Class.forName(
                "net.minecraft.world.level.levelgen.densityfunction.DensityVolume");
        Class<?> dss = Class.forName(
                "net.minecraft.world.level.levelgen.densityfunction.DensitySamplerSet");
        Class<?> bound = Class.forName(
                "net.minecraft.world.level.levelgen.densityfunction.DensitySampler$Bound");
        Class<?> sdb = Class.forName(
                "net.minecraft.world.level.levelgen.densityfunction.ScopedDensityBuffer");
        Class<?> df = Class.forName(
                "net.minecraft.world.level.levelgen.densityfunction.DensityFunction");
        Class<?> aq = Class.forName("net.minecraft.world.level.levelgen.Aquifer");
        Class<?> fpq = Class.forName(
                "net.minecraft.world.level.levelgen.Aquifer$FluidPicker");
        Class<?> beard = Class.forName("net.minecraft.world.level.levelgen.Beardifier");
        Class<?> ngs = Class.forName(
                "net.minecraft.world.level.levelgen.NoiseGeneratorSettings");
        Class<?> rsc = Class.forName("net.minecraft.world.level.levelgen.RandomState");
        Class<?> blc = Class.forName(
                "net.minecraft.world.level.levelgen.blending.Blender");
        Class<?> ms = Class.forName(
                "net.minecraft.world.level.levelgen.material.MaterialSystem");
        Class<?> mr = Class.forName(
                "net.minecraft.world.level.levelgen.material.rule.MaterialRule");
        Class<?> wgc = Class.forName(
                "net.minecraft.world.level.levelgen.WorldGenerationContext");
        Class<?> fn = Class.forName("java.util.function.Function");
        Class<?> ca = Class.forName("net.minecraft.world.level.chunk.ChunkAccess");
        Class<?> bp = Class.forName("net.minecraft.core.BlockPos");
        Class<?> proto = Class.forName("net.minecraft.world.level.chunk.ProtoChunk");
        Class<?> cpos = Class.forName("net.minecraft.world.level.ChunkPos");
        Class<?> up = Class.forName("net.minecraft.world.level.chunk.UpgradeData");
        Class<?> lha = Class.forName("net.minecraft.world.level.LevelHeightAccessor");
        Class<?> pcf = Class.forName(
                "net.minecraft.world.level.chunk.PalettedContainerFactory");
        Class<?> bdata = Class.forName(
                "net.minecraft.world.level.levelgen.blending.BlendingData");
        Class<?> bs = Class.forName(
                "net.minecraft.world.level.block.state.BlockState");
        Class<?> fs = Class.forName("net.minecraft.world.level.material.FluidState");
        Class<?> blk = Class.forName("net.minecraft.world.level.block.Block");
        Class<?> ident = Class.forName("net.minecraft.resources.Identifier");
        Class<?> regc = Class.forName("net.minecraft.core.Registry");
        Class<?> holder = Class.forName("net.minecraft.core.Holder");
        Class<?> rk = Class.forName("net.minecraft.resources.ResourceKey");
        Class<?> bmc = Class.forName(
                "net.minecraft.world.level.biome.BiomeManager");

        ncNew = asObj(ctor(nc, MethodType.methodType(void.class, rsc, beard,
                ngs, fpq, blc, dv)));
        dvNew = asObj(ctor(dv, MethodType.methodType(void.class, int.class,
                int.class, int.class, int.class, int.class, int.class)));
        protoNew = asObj(ctor(proto, MethodType.methodType(void.class, cpos,
                up, lha, pcf, bdata)));
        chunkPosNew = asObj(ctor(cpos, MethodType.methodType(void.class,
                int.class, int.class)));
        blockPosNew = asObj(ctor(bp, MethodType.methodType(void.class,
                int.class, int.class, int.class)));

        ncCachingSamplers = asObj(vh(nc, "cachingSamplers", MethodType.methodType(dss)));
        ncVolume = asObj(vh(nc, "volume", MethodType.methodType(dv)));
        ncAquifer = asObj(vh(nc, "aquifer", MethodType.methodType(aq)));
        ncClose = vh(nc, "close", MethodType.methodType(void.class))
                .asType(MethodType.methodType(void.class, Object.class));
        dvIndex = vh(dv, "indexUnchecked", MethodType.methodType(int.class,
                int.class, int.class, int.class)).asType(MethodType.methodType(
                int.class, Object.class, int.class, int.class, int.class));
        dvBlockY = vh(dv, "blockY", MethodType.methodType(int.class, int.class))
                .asType(MethodType.methodType(int.class, Object.class, int.class));
        setGet = asObj(vh(dss, "get", MethodType.methodType(bound, df)));
        sampleVolume = asObj(vh(bound, "sampleVolume", MethodType.methodType(sdb, dv)));
        bufGet = vh(sdb, "get", MethodType.methodType(float.class, int.class))
                .asType(MethodType.methodType(float.class, Object.class, int.class));
        bufClose = vh(sdb, "close", MethodType.methodType(void.class))
                .asType(MethodType.methodType(void.class, Object.class));
        aqSubstance = asObj(vh(aq, "computeSubstance", MethodType.methodType(
                bs, int.class, int.class, int.class, double.class)));
        topMaterial = asObj(vh(ms, "topMaterial", MethodType.methodType(
                Optional.class, mr, rsc, wgc, fn, ca, dss, bp, boolean.class)));
        stFluidState = asObj(vh(bs, "getFluidState", MethodType.methodType(fs)));
        fsEmpty = vh(fs, "isEmpty", MethodType.methodType(boolean.class))
                .asType(MethodType.methodType(boolean.class, Object.class));
        stBlock = asObj(vh(bs, "getBlock", MethodType.methodType(blk)));
        regGetKey = asObj(vh(regc, "getKey", MethodType.methodType(ident, Object.class)));
        holderUnwrapKey = asObj(vh(holder, "unwrapKey", MethodType.methodType(Optional.class)));
        rkIdentifier = asObj(vh(rk, "identifier", MethodType.methodType(ident)));
        biomeGet = asObj(vh(bmc, "getBiome", MethodType.methodType(holder, bp)));

        pred = (Predicate) pd;
        randomState = rs;
        settings = st;
        rule = rl;
        surface = surf;
        ctx = cx;
        fluidPicker = fp;
        blender = bl;
        defaultBlock = db;
        finalDensity = fd;
        upgradeData = ud;
        level = lvl;
        factory = fac;
        biomeFn = bfn;
        blockRegistry = reg;
        bman = bm;
        minY = minBlockY;
        sizeY = heightY;
        ready = true;
    }

    public static boolean isReady() {
        return ready;
    }

    /**
     * One LOD chunk (v5.8 contract kept for the preflight parity harness and
     * the single-chunk JNI path). Returns packed longs in dx-major, dz-minor
     * order with cell-size stepping: index (dx/cell)*(16/cell)+(dz/cell).
     */
    public static long[] chunk(int cx, int cz, int cell) throws Throwable {
        int n = 16 / cell;
        long[] seg = new long[1 + n * n];
        chunkInto(seg, 0, cx, cz, cell, n); // throws on failure
        long[] out = new long[n * n];
        System.arraycopy(seg, 1, out, 0, n * n);
        return out;
    }

    /**
     * v5.9: one JNI call per BATCH of chunks. coords = [cx0,cz0,cx1,cz1,...].
     * A chunk that throws marks only its own segment (status 0, zero-filled)
     * and the batch continues — the native side falls back to the legacy
     * per-column path for exactly that chunk.
     */
    public static long[] chunkBatch(int[] coords, int cell) throws Throwable {
        int k = coords.length / 2;
        int n = 16 / cell;
        int seg = 1 + n * n;
        long[] out = new long[4 + k * seg];
        out[0] = k;
        int ok = 0;
        for (int i = 0; i < k; i++) {
            int off = 4 + i * seg;
            try {
                chunkInto(out, off, coords[2 * i], coords[2 * i + 1], cell, n);
                ok++;
            } catch (Throwable t) {
                Arrays.fill(out, off, off + seg, 0L);
            }
        }
        out[1] = ok;
        out[2] = tabVer;
        return out;
    }

    /** Fill one chunk segment: out[off]=status, out[off+1..]=packed cells.
     *  Throws on any failure (buffer close still runs); the caller owns the
     *  status convention. */
    private static void chunkInto(long[] out, int off, int cx, int cz,
            int cell, int n) throws Throwable {
        out[off] = 0L; // pessimistic status; flipped on full success
        int wx0 = cx * 16;
        int wz0 = cz * 16;

        Object vol = dvNew.invoke(16, sizeY, 16, wx0, minY, wz0);
        Object nc = ncNew.invoke(randomState, null, settings, fluidPicker,
                blender, vol);
        Object buf = null;
        try {
            Object set = ncCachingSamplers.invoke(nc);
            Object bound = setGet.invoke(set, finalDensity);
            Object ncv = ncVolume.invoke(nc);
            buf = sampleVolume.invoke(bound, ncv);
            Object aquifer = ncAquifer.invoke(nc);

            Object pos = chunkPosNew.invoke(cx, cz);
            Object proto = protoNew.invoke(pos, upgradeData, level, factory, null);

            Predicate<Object> p = pred;
            Object db = defaultBlock;
            IdentityHashMap<Object, Long> stc = ST_CACHE.get();
            IdentityHashMap<Object, Integer> bioc = BIO_CACHE.get();
            int o = off + 1;
            for (int dx = 0; dx < 16; dx += cell) {
                for (int dz = 0; dz < 16; dz += cell) {
                    out[o++] = walkColumn(vol, buf, aquifer, set, proto, p, db,
                            stc, bioc, wx0 + dx, wz0 + dz, dx, dz);
                }
            }
            out[off] = 1L;
        } finally {
            if (buf != null) {
                bufClose.invoke(buf);
            }
            ncClose.invoke(nc);
        }
    }

    /** One LOD column: top-down density walk + surface rule + interning.
     *  v5.9 stride fast path: indexUnchecked(dx,i,dz) and blockY(i) are
     *  verified affine on three points of THIS column before stepping is
     *  used; otherwise the v5.8 per-step calls run (bit-identical either
     *  way — the verification is numeric, not structural). */
    private static long walkColumn(Object vol, Object buf, Object aquifer,
            Object set, Object proto, Predicate<Object> p, Object db,
            IdentityHashMap<Object, Long> stc,
            IdentityHashMap<Object, Integer> bioc,
            int wx, int wz, int dx, int dz) throws Throwable {
        int iTop = sizeY - 1;
        int idxTop = (int) dvIndex.invoke(vol, dx, iTop, dz);
        int idxMid = (int) dvIndex.invoke(vol, dx, iTop - 1, dz);
        int idxLow = (int) dvIndex.invoke(vol, dx, iTop - 2, dz);
        int yTop = (int) dvBlockY.invoke(vol, iTop);
        int yMid = (int) dvBlockY.invoke(vol, iTop - 1);
        int yLow = (int) dvBlockY.invoke(vol, iTop - 2);
        int di = idxMid - idxTop; // negative: the index grows with i
        int dy = yMid - yTop;     // must be -1 (blockY = minY + i)
        boolean fast = di != 0
                && (idxTop - idxMid) == (idxMid - idxLow)
                && dy == -1
                && (yTop - yMid) == (yMid - yLow);
        if (fast) {
            FAST_COLUMNS.incrementAndGet();
        } else {
            SLOW_COLUMNS.incrementAndGet();
        }

        int hit = Integer.MIN_VALUE;
        Object top = db;
        int idx = idxTop;
        int y = yTop;
        for (int k = 0; k < sizeY; k++) {
            int i = iTop - k;
            float d;
            int cy;
            if (fast) {
                d = (float) bufGet.invoke(buf, idx);
                cy = y;
            } else {
                d = (float) bufGet.invoke(buf,
                        (int) dvIndex.invoke(vol, dx, i, dz));
                cy = (int) dvBlockY.invoke(vol, i);
            }
            Object s = aqSubstance.invoke(aquifer, wx, cy, wz, (double) d);
            if (s == null) {
                s = db;
            }
            if (p.test(s)) {
                hit = cy;
                top = s;
                break;
            }
            idx += di;
            y += dy;
        }
        if (hit == Integer.MIN_VALUE) {
            return 0L; // void column: valid bit clear
        }
        Object bp = blockPosNew.invoke(wx, hit, wz);
        Object topOpt = topMaterial.invoke(surface, rule,
                randomState, ctx, biomeFn, proto, set, bp, false);
        if (topOpt instanceof Optional opt && opt.isPresent()) {
            Object better = opt.get();
            if (better != null) {
                top = better;
            }
        }
        long minfo = matInfo(top, stc);
        int bo = bioOrdCached(bp, bioc);
        long l = ((long) (hit + 32768)) & 0xFFFFL;
        if ((minfo & 1L) != 0L) {
            l |= 1L << 16;
        }
        l |= (minfo >>> 1) << 17;
        l |= (long) bo << 32;
        l |= 1L << 63;
        return l;
    }

    /** liquid bit + material ordinal for one BlockState, cached by identity.
     *  Same calls, same order as the v5.8 inline code — only repeated work
     *  is removed (states are canonical singletons). */
    private static long matInfo(Object st,
            IdentityHashMap<Object, Long> stc) throws Throwable {
        Long c = stc.get(st);
        if (c != null) {
            return c;
        }
        boolean liquid = false;
        Object fsO = stFluidState.invoke(st);
        if (fsO != null) {
            liquid = !(boolean) fsEmpty.invoke(fsO);
        }
        String mat = "";
        Object b = stBlock.invoke(st);
        if (b != null) {
            Object id = regGetKey.invoke(blockRegistry, b);
            if (id != null) {
                mat = id.toString();
            }
        }
        long v = (liquid ? 1L : 0L) | ((long) matOrd(mat) << 1);
        if (stc.size() < CACHE_CAP) {
            stc.put(st, v);
        }
        return v;
    }

    /** Biome ordinal for one BlockPos, caching the Holder->ordinal half
     *  (getBiome itself stays per-cell — it is position-dependent). */
    private static int bioOrdCached(Object bp,
            IdentityHashMap<Object, Integer> bioc) throws Throwable {
        Object h = biomeGet.invoke(bman, bp);
        if (h == null) {
            return bioOrd("");
        }
        Integer o = bioc.get(h);
        if (o != null) {
            return o;
        }
        String bio = "";
        Object bo = holderUnwrapKey.invoke(h);
        if (bo instanceof Optional bopt && bopt.isPresent()) {
            Object r = bopt.get();
            if (r != null) {
                Object id = rkIdentifier.invoke(r);
                if (id != null) {
                    bio = id.toString();
                }
            }
        }
        int v = bioOrd(bio);
        if (bioc.size() < CACHE_CAP) {
            bioc.put(h, v);
        }
        return v;
    }

    private static int matOrd(String s) {
        return ord(MAT, MAT_N, s, true);
    }

    private static int bioOrd(String s) {
        return ord(BIO, BIO_N, s, false);
    }

    private static int ord(Map<String, Integer> m, AtomicInteger n, String s,
            boolean mat) {
        Integer o = m.get(s);
        if (o != null) {
            return o;
        }
        o = n.getAndIncrement();
        Integer prev = m.putIfAbsent(s, o);
        if (prev != null) {
            return prev;
        }
        rebuild(m, mat);
        return o;
    }

    private static void rebuild(Map<String, Integer> m, boolean mat) {
        String[] t = new String[m.size()];
        for (Map.Entry<String, Integer> e : m.entrySet()) {
            int i = e.getValue();
            if (i >= 0 && i < t.length) {
                t[i] = e.getKey();
            }
        }
        if (mat) {
            matTable = t;
        } else {
            bioTable = t;
        }
        tabVer++;
    }
}
