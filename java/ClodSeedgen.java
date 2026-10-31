package clod;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.InvocationHandler;
import java.lang.reflect.Method;
import java.lang.reflect.Proxy;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.IdentityHashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Predicate;
import java.util.function.Supplier;
import java.util.function.ToIntFunction;

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
 * v6.1 SEEDGEN TREES: after the 256-column walk the helper runs the REAL
 * vanilla feature pipeline for the VEGETAL_DECORATION step — the exact
 * addVanillaDecorations recipe lifted from paper-26.3 bytecode:
 *
 *   decoratedSeed = WorldgenRandom.setDecorationSeed(seed, originX, originZ)
 *   per placed feature (sorted global index in the step's FeatureSorter
 *   data): setFeatureSeed(decoratedSeed, idx, stepOrdinal), then
 *   FeaturePlacer.placeWithBiomeCheck(feature, random, chunkOrigin)
 *
 * The pipeline writes through a dynamic Proxy that implements the full
 * WorldGenLevel interface set (pure JDK — interfaces resolved by name):
 * setBlock records motion-blocking writes into live per-block tops (the
 * three Heightmap.Types predicates are the REAL vanilla ones), and every
 * other query (getBlockState/getFluidState/getHeight/getBiome/isStateAt...)
 * answers from the walked terrain model. Because each feature is freshly
 * seeded per chunk and the biome filter runs on the REAL BiomeManager,
 * running only our own chunk's biomes is bit-equivalent to vanilla's 3x3
 * palette union (a neighbor-only biome's features all fail the position
 * biome filter inside our chunk). Tree tops/leaves now land in the packed
 * cells exactly as they will when the chunk really generates.
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
 *   bits 0-15 : blockY of the surface + 32768 (with v6.1: max of terrain
 *               walk and tree/feature writes)
 *   bit 16    : liquid (top block has a non-empty fluid state)
 *   bits 17-31: material ordinal (matTable) — leaves/logs when a tree tops
 *               the column
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

    // --- v6.1 feature surfaces (resolved once; any miss -> features off) --
    private static volatile boolean feats;
    private static volatile Object gen, worldSeed, cgCls;
    private static volatile MethodHandle featSupplier, wgrNew, xrsNew,
            uniqueSeed, setDecoSeed, setFeatSeed, placerNew, placeWBC,
            holderValue, biomeGenSettings, genSettingsFeatures, posAsLong,
            vecGetX, vecGetY, vecGetZ;
    private static volatile Predicate<Object> predMB, predNL, predWS;
    private static volatile Object mbType, mblType, wsType, airState;
    private static volatile Class<?>[] proxyIfaces;
    private static volatile ClassLoader proxyLoader;
    private static volatile List<?> stepDataList;
    private static volatile Method stepFeatM, stepIdxM;
    private static volatile int stepOrd;

    /** Chunks whose feature run completed (threw or not), and chunk-level
     *  feature failures (whole-run fallback to terrain-only). Read by the
     *  JNI side after each batch for the stream diagnostics. */
    public static volatile long featChunks = 0;
    public static volatile long featFails = 0;

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
            Object bm, Object generator, long seed, int minBlockY,
            int heightY, int features) throws Throwable {
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
        gen = generator;
        worldSeed = seed;
        feats = features != 0;
        ready = true;
        resolveFeatures(generator);
    }

    public static boolean isReady() {
        return ready;
    }

    /** v6.1: true when the feature pipeline resolved (trees enabled). */
    public static boolean featReady() {
        return feats;
    }

    /** v6.1: resolve the feature pipeline. EVERY failure path lands on
     *  feats=false — the terrain-only walk (v5.7/v5.9 output) stays, trees
     *  are the upgrade, never the dependency. */
    @SuppressWarnings("unchecked")
    private static void resolveFeatures(Object generator) {
        if (generator == null || !feats) {
            feats = false;
            return;
        }
        try {
            Class<?> wgl = Class.forName("net.minecraft.world.level.WorldGenLevel");
            Class<?> cg = Class.forName(
                    "net.minecraft.world.level.chunk.ChunkGenerator");
            Class<?> fpl = Class.forName(
                    "net.minecraft.world.level.levelgen.placement.FeaturePlacer");
            Class<?> pfCls = Class.forName(
                    "net.minecraft.world.level.levelgen.placement.PlacedFeature");
            Class<?> rsCls = Class.forName("net.minecraft.util.RandomSource");
            Class<?> wgrCls = Class.forName(
                    "net.minecraft.world.level.levelgen.WorldgenRandom");
            Class<?> xrsCls = Class.forName(
                    "net.minecraft.world.level.levelgen.XoroshiroRandomSource");
            Class<?> rsup = Class.forName(
                    "net.minecraft.world.level.levelgen.RandomSupport");
            Class<?> deco = Class.forName(
                    "net.minecraft.world.level.levelgen.GenerationStep$Decoration");
            Class<?> hm = Class.forName(
                    "net.minecraft.world.level.levelgen.Heightmap$Types");
            Class<?> holder = Class.forName("net.minecraft.core.Holder");
            Class<?> biome = Class.forName("net.minecraft.world.level.biome.Biome");
            Class<?> bgs = Class.forName(
                    "net.minecraft.world.level.biome.BiomeGenerationSettings");
            Class<?> bpc = Class.forName("net.minecraft.core.BlockPos");
            Class<?> v3i = Class.forName("net.minecraft.core.Vec3i");
            Class<?> blkc = Class.forName(
                    "net.minecraft.world.level.block.state.BlockState");

            // the per-step feature index lives in a private generator field
            featSupplier = MethodHandles.privateLookupIn(cg,
                    MethodHandles.lookup())
                    .findGetter(cg, "featuresPerStep", Supplier.class)
                    .asType(MethodType.methodType(Object.class, Object.class));
            Class<?> stepDataCls = Class.forName(
                    "net.minecraft.world.level.biome.FeatureSorter$StepFeatureData");
            stepFeatM = stepDataCls.getMethod("features");
            stepIdxM = stepDataCls.getMethod("indexMapping");

            // step ordinal by name (enum order is a reordering hazard)
            Object[] steps = (Object[]) deco.getMethod("values").invoke(null);
            for (int i = 0; i < steps.length; i++) {
                if ("VEGETAL_DECORATION".equals(((Enum<?>) steps[i]).name())) {
                    stepOrd = i;
                }
            }
            stepDataList = (List<Object>) ((Supplier<Object>) featSupplier
                    .invoke(generator)).get();

            wgrNew = asObj(ctor(wgrCls, MethodType.methodType(void.class, rsCls)));
            xrsNew = asObj(ctor(xrsCls,
                    MethodType.methodType(void.class, long.class)));
            uniqueSeed = MethodHandles.privateLookupIn(rsup, MethodHandles
                    .lookup()).findStatic(rsup, "generateUniqueSeed",
                    MethodType.methodType(long.class));
            setDecoSeed = vh(wgrCls, "setDecorationSeed", MethodType
                    .methodType(long.class, long.class, int.class, int.class))
                    .asType(MethodType.methodType(long.class, Object.class,
                            Object.class, int.class, int.class));
            setFeatSeed = vh(wgrCls, "setFeatureSeed", MethodType
                    .methodType(void.class, long.class, int.class, int.class))
                    .asType(MethodType.methodType(void.class, Object.class,
                            long.class, int.class, int.class));
            placerNew = asObj(ctor(fpl, MethodType.methodType(void.class, wgl, cg)));
            placeWBC = vh(fpl, "placeWithBiomeCheck", MethodType.methodType(
                    boolean.class, pfCls, rsCls, bpc))
                    .asType(MethodType.methodType(boolean.class, Object.class,
                            Object.class, Object.class, Object.class));
            holderValue = vh(holder, "value", MethodType.methodType(Object.class))
                    .asType(MethodType.methodType(Object.class, Object.class));
            biomeGenSettings = asObj(vh(biome, "getGenerationSettings",
                    MethodType.methodType(bgs)));
            genSettingsFeatures = asObj(vh(bgs, "features",
                    MethodType.methodType(List.class)));

            // the REAL vanilla heightmap predicates (identity-dispatched)
            predMB = typePred(hm, "MOTION_BLOCKING");
            mbType = hm.getField("MOTION_BLOCKING").get(null);
            predNL = typePred(hm, "MOTION_BLOCKING_NO_LEAVES");
            mblType = hm.getField("MOTION_BLOCKING_NO_LEAVES").get(null);
            predWS = typePred(hm, "WORLD_SURFACE");
            wsType = hm.getField("WORLD_SURFACE").get(null);

            airState = vh(Class.forName(
                            "net.minecraft.world.level.block.Block"),
                    "defaultBlockState", MethodType.methodType(blkc))
                    .asType(MethodType.methodType(Object.class, Object.class))
                    .invoke(Class.forName("net.minecraft.world.level.block.Blocks")
                            .getField("AIR").get(null));

            posAsLong = vh(bpc, "asLong", MethodType.methodType(long.class))
                    .asType(MethodType.methodType(long.class, Object.class));
            vecGetX = vh(v3i, "getX", MethodType.methodType(int.class))
                    .asType(MethodType.methodType(int.class, Object.class));
            vecGetY = vh(v3i, "getY", MethodType.methodType(int.class))
                    .asType(MethodType.methodType(int.class, Object.class));
            vecGetZ = vh(v3i, "getZ", MethodType.methodType(int.class))
                    .asType(MethodType.methodType(int.class, Object.class));

            // proxy interfaces: the full WorldGenLevel closure (missing
            // optional patch interfaces are skipped)
            String[] names = {
                    "net.minecraft.world.level.WorldGenLevel",
                    "net.minecraft.world.level.ServerLevelAccessor",
                    "net.minecraft.world.level.LevelAccessor",
                    "net.minecraft.world.level.CommonLevelAccessor",
                    "net.minecraft.world.level.LevelReader",
                    "net.minecraft.world.level.BlockAndLightGetter",
                    "net.minecraft.world.level.BlockGetter",
                    "net.minecraft.world.level.CollisionGetter",
                    "net.minecraft.world.level.SignalGetter",
                    "net.minecraft.world.level.biome.BiomeResolver",
                    "ca.spottedleaf.moonrise.patches.chunk_system.level"
                            + ".ChunkSystemLevelReader",
                    "net.minecraft.world.level.LevelSimulatedRW",
                    "net.minecraft.world.level.LevelSimulatedReader",
                    "net.minecraft.world.level.LevelWriter",
                    "net.minecraft.world.level.ScheduledTickAccess",
                    "net.minecraft.world.level.LevelHeightAccessor",
            };
            ArrayList<Class<?>> ifs = new ArrayList<>();
            for (String n : names) {
                try {
                    ifs.add(Class.forName(n));
                } catch (Throwable ignored) {
                    // optional interface (Paper patches move names)
                }
            }
            proxyIfaces = ifs.toArray(new Class<?>[0]);
            proxyLoader = wgl.getClassLoader();
        } catch (Throwable t) {
            feats = false; // terrain-only output, exactly the v5.9 contract
        }
    }

    private static Predicate<Object> typePred(Class<?> hm, String cst)
            throws Throwable {
        Object t = hm.getField(cst).get(null);
        return (Predicate<Object>) t.getClass()
                .getMethod("isOpaque").invoke(t);
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
            // v6.1: per-column snapshots feed the feature-run terrain model;
            // biome Holders gathered identity-distinct for the feature list.
            int[] wtop = new int[n * n];
            Object[] wst = new Object[n * n];
            boolean[] wliq = new boolean[n * n];
            ArrayList<Object> holders = new ArrayList<>(8);
            int o = off + 1;
            int ci = 0;
            for (int dx = 0; dx < 16; dx += cell) {
                for (int dz = 0; dz < 16; dz += cell) {
                    out[o++] = walkColumn(vol, buf, aquifer, set, proto, p, db,
                            stc, bioc, wx0 + dx, wz0 + dz, dx, dz, wtop, wst,
                            wliq, ci, holders);
                    ci++;
                }
            }
            if (feats) {
                runFeatures(out, off + 1, n, cell, wx0, wz0, proto, stc,
                        wtop, wst, holders);
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
            int wx, int wz, int dx, int dz, int[] wtop, Object[] wst,
            boolean[] wliq, int ci, ArrayList<Object> holders)
            throws Throwable {
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
        Object bh = biomeGet.invoke(bman, bp);
        int bo = bioOrdOf(bh, bioc);
        if (bh != null && !holders.contains(bh) && holders.size() < 16) {
            holders.add(bh);
        }
        wtop[ci] = hit;
        wst[ci] = top;
        wliq[ci] = (minfo & 1L) != 0L;
        long l = ((long) (hit + 32768)) & 0xFFFFL;
        if ((minfo & 1L) != 0L) {
            l |= 1L << 16;
        }
        l |= (minfo >>> 1) << 17;
        l |= (long) bo << 32;
        l |= 1L << 63;
        return l;
    }

    // ------------------------------------------------------------------
    // v6.1 SEEDGEN TREES: the real vanilla VEGETAL_DECORATION pass over a
    // proxy WorldGenLevel backed by the walked terrain model.
    // ------------------------------------------------------------------

    /** The per-chunk feature pass: exact addVanillaDecorations seeding and
     *  ordering (sorted global feature index inside the step), one fresh
     *  WorldgenRandom, the real FeaturePlacer + biome filter. Any throw —
     *  whole pass or single feature — never kills the terrain output. */
    private static void runFeatures(long[] out, int o0, int n, int cell,
            int wx0, int wz0, Object proto, IdentityHashMap<Object, Long> stc,
            int[] wtop, Object[] wst,
            ArrayList<Object> holders) {
        try {
            GenModel model = new GenModel(wx0, wz0, cell, n, proto, wtop, wst);
            Object proxy = Proxy.newProxyInstance(proxyLoader, proxyIfaces,
                    model);
            Object placer = placerNew.invoke(proxy, gen);
            Object wgr = wgrNew.invoke(xrsNew.invoke(
                    (long) uniqueSeed.invoke()));
            model.wgr = wgr;
            Object origin = blockPosNew.invoke(wx0, minY, wz0);
            long deco = (long) setDecoSeed.invoke(wgr, worldSeed, wx0, wz0);

            // gather the step's placed features for OUR chunk's biomes and
            // run them in vanilla order (ascending global index; each one
            // freshly seeded — a skipped/failed feature cannot shift others)
            Object stepData = ((List<?>) stepDataList).get(stepOrd);
            List<?> featsOfStep = (List<?>) stepFeatM.invoke(stepData);
            ToIntFunction<Object> idxOf = (ToIntFunction<Object>)
                    stepIdxM.invoke(stepData);
            int[] idxs = new int[64];
            int cnt = 0;
            for (Object h : holders) {
                Object gs = biomeGenSettings.invoke(holderValue.invoke(h));
                List<?> fs = (List<?>) genSettingsFeatures.invoke(gs);
                if (stepOrd >= fs.size()) {
                    continue;
                }
                for (Object pf : (Iterable<?>) fs.get(stepOrd)) {
                    int gi = idxOf.applyAsInt(holderValue.invoke(pf));
                    if (cnt == idxs.length) {
                        idxs = Arrays.copyOf(idxs, cnt * 2);
                    }
                    idxs[cnt++] = gi;
                }
            }
            if (cnt == 0) {
                featChunks++;
                return;
            }
            Arrays.sort(idxs, 0, cnt);
            int prev = -1;
            for (int i = 0; i < cnt; i++) {
                int gi = idxs[i];
                if (gi == prev) {
                    continue;
                }
                prev = gi;
                Object pf = featsOfStep.get(gi);
                setFeatSeed.invoke(wgr, deco, gi, stepOrd);
                try {
                    placeWBC.invoke(placer, pf, wgr, origin);
                } catch (Throwable ignored) {
                    // one bad feature never stops the chunk (JNI-side
                    // clear_if_pending guards the boundary)
                }
            }
            // patch the packed cells: feature tops win over terrain tops
            for (int cix = 0; cix < n * n; cix++) {
                int bi = ((cix % n) * cell) * 16 + ((cix / n) * cell);
                int wt = wtop[cix];
                if (wt == Integer.MIN_VALUE || model.topMB[bi] <= wt) {
                    continue;
                }
                long old = out[o0 + cix];
                if (old == 0L) {
                    continue;
                }
                long minfo = matInfo(model.topSt[bi], stc);
                long l = ((long) (model.topMB[bi] + 32768)) & 0xFFFFL;
                if ((minfo & 1L) != 0L) {
                    l |= 1L << 16; // terrain liquid survives tree tops
                }
                l |= (minfo >>> 1) << 17;
                l |= old & (0x7FFFL << 32); // biome bits
                l |= old & (1L << 63);      // valid bit
                out[o0 + cix] = l;
            }
            featChunks++;
        } catch (Throwable ignored) {
            featFails++; // terrain-only cells stay exactly v5.9-correct
        }
    }

    /** The dynamic WorldGenLevel: full interface set, vanilla semantics for
     *  everything the feature pipeline asks, terrain model for the rest. */
    private static final class GenModel implements InvocationHandler {
        final int wx0, wz0, cell, n;
        final Object proto;
        final int[] topMB, topNL, topWS;
        final Object[] topSt;
        Object wgr;
        int topChanged = 0;
        private final HashMap<Long, Object> writes = new HashMap<>();

        GenModel(int wx0, int wz0, int cell, int n, Object proto,
                int[] wtop, Object[] wst) {
            this.wx0 = wx0;
            this.wz0 = wz0;
            this.cell = cell;
            this.n = n;
            this.proto = proto;
            this.topMB = new int[256];
            this.topNL = new int[256];
            this.topWS = new int[256];
            this.topSt = new Object[256];
            // nearest-sampled-column terrain tops for all 256 block columns
            for (int bx = 0; bx < 16; bx++) {
                int sbx = (bx / cell) * cell;
                for (int bz = 0; bz < 16; bz++) {
                    int sbz = (bz / cell) * cell;
                    int ci = (sbx / cell) * n + (sbz / cell);
                    int bi = bz * 16 + bx;
                    topMB[bi] = wtop[ci];
                    topNL[bi] = wtop[ci];
                    topWS[bi] = wtop[ci];
                    topSt[bi] = wst[ci];
                }
            }
        }

        private int blockIdx(Object pos) throws Throwable {
            int x = (int) vecGetX.invoke(pos) - wx0;
            int z = (int) vecGetZ.invoke(pos) - wz0;
            if (x < 0 || x >= 16 || z < 0 || z >= 16) {
                return -1;
            }
            return z * 16 + x;
        }

        private void record(Object pos, Object st) throws Throwable {
            int bi = blockIdx(pos);
            if (bi < 0) {
                return;
            }
            int y = (int) vecGetY.invoke(pos);
            if (y < minY || y >= minY + sizeY) {
                return;
            }
            if (predMB.test(st) && y > topMB[bi]) {
                topMB[bi] = y;
                topSt[bi] = st;
                topChanged++;
            }
            if (predNL.test(st) && y > topNL[bi]) {
                topNL[bi] = y;
            }
            if (predWS.test(st) && y > topWS[bi]) {
                topWS[bi] = y;
            }
            writes.put((long) posAsLong.invoke(pos), st);
        }

        private Object state(Object pos) throws Throwable {
            Object w = writes.get((long) posAsLong.invoke(pos));
            if (w != null) {
                return w;
            }
            int bi = blockIdx(pos);
            if (bi < 0) {
                return airState;
            }
            int y = (int) vecGetY.invoke(pos);
            if (y > topMB[bi]) {
                return airState;
            }
            if (y == topMB[bi] && topSt[bi] != null) {
                return topSt[bi];
            }
            return defaultBlock;
        }

        private int heightAt(Object type, int wx, int wz) {
            if (wx < wx0 || wz < wz0 || wx >= wx0 + 16 || wz >= wz0 + 16) {
                return minY;
            }
            int bi = (wz - wz0) * 16 + (wx - wx0);
            int t;
            if (type == mbType) {
                t = topMB[bi];
            } else if (type == mblType) {
                t = topNL[bi];
            } else if (type == wsType) {
                t = topWS[bi];
            } else {
                t = topMB[bi];
            }
            return t >= minY ? t + 1 : minY;
        }

        @Override
        @SuppressWarnings("rawtypes")
        public Object invoke(Object proxy, Method m, Object[] args)
                throws Throwable {
            switch (m.getName()) {
                case "setBlock": // (pos, state, flags[, flags2]) — record
                    record(args[0], args[1]);
                    return Boolean.TRUE;
                case "removeBlock":
                case "destroyBlock":
                    if (args != null && args.length >= 1) {
                        record(args[0], airState);
                    }
                    return Boolean.TRUE;
                case "getBlockState":
                    return state(args[0]);
                case "getBlockStateIfLoaded":
                    return state(args[0]);
                case "getFluidState": {
                    Object st = state(args[0]);
                    return stFluidState.invoke(st);
                }
                case "isStateAtPosition":
                    return ((Predicate) args[1]).test(state(args[0]));
                case "isFluidAtPosition":
                    return ((Predicate) args[1])
                            .test(stFluidState.invoke(state(args[0])));
                case "getBlockEntity":
                    return args != null && args.length >= 2
                            ? Optional.empty() : null;
                case "getHeight":
                    return args != null && args.length == 3
                            ? heightAt(args[0], (Integer) args[1],
                                    (Integer) args[2])
                            : (Object) sizeY;
                case "getHeightmapPos":
                    return blockPosNew.invoke((int) vecGetX.invoke(args[1]),
                            heightAt(args[0], (int) vecGetX.invoke(args[1]),
                                    (int) vecGetZ.invoke(args[1])),
                            (int) vecGetZ.invoke(args[1]));
                case "getMinY":
                    return minY;
                case "getMaxY":
                    return minY + sizeY;
                case "getSectionsCount":
                    return sizeY >> 4;
                case "getMinSectionY":
                    return minY >> 4;
                case "getMaxSectionY":
                    return ((minY + sizeY) - 1) >> 4;
                case "isOutsideBuildHeight": {
                    int y = args[0] instanceof Integer
                            ? (Integer) args[0] : (int) vecGetY.invoke(args[0]);
                    return y < minY || y >= minY + sizeY;
                }
                case "isInsideBuildHeight": {
                    int y = args[0] instanceof Integer
                            ? (Integer) args[0] : (int) vecGetY.invoke(args[0]);
                    return !(y < minY || y >= minY + sizeY);
                }
                case "getSectionIndex":
                    return ((Integer) args[0] >> 4) - (minY >> 4);
                case "getSectionIndexFromSectionY":
                    return ((Integer) args[0]) - (minY >> 4);
                case "getSectionYFromSectionIndex":
                    return ((Integer) args[0]) + (minY >> 4);
                case "getBiome":
                    return biomeGet.invoke(bman, args[0]);
                case "getSeed":
                    return worldSeed;
                case "getLevel":
                case "getMinecraftWorld":
                    return level;
                case "getChunk":
                    return proto;
                case "hasChunkAt":
                    return Boolean.TRUE;
                case "ensureCanWrite":
                    return Boolean.TRUE;
                case "getRandom":
                    return wgr; // features otherwise take their random per place
                case "getBlockTicks":
                case "getFluidTicks":
                case "createTick":
                case "scheduleTick":
                case "addFreshEntity":
                case "addFreshEntityWithPassengers":
                case "setCurrentlyGenerating":
                case "getCurrentDifficultyAt":
                    return null;
                case "hashCode":
                    return System.identityHashCode(proxy);
                case "equals":
                    return proxy == args[0];
                case "toString":
                    return "clod.GenLevel";
                default:
                    Class<?> r = m.getReturnType();
                    if (r == boolean.class) {
                        return Boolean.FALSE;
                    }
                    if (r == int.class) {
                        return 0;
                    }
                    if (r == long.class) {
                        return 0L;
                    }
                    if (r == float.class) {
                        return 0f;
                    }
                    if (r == double.class) {
                        return 0d;
                    }
                    return null;
            }
        }
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

    /** Biome ordinal for one biome Holder, caching the Holder->ordinal half
     *  (getBiome itself stays per-cell — it is position-dependent). */
    private static int bioOrdOf(Object h,
            IdentityHashMap<Object, Integer> bioc) throws Throwable {
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
