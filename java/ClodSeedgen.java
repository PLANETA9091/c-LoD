package clod;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Predicate;

/**
 * c-LoD SEEDGEN HELPER (v5.8): the whole per-chunk noise walk runs INSIDE the
 * JVM, so the JNI boundary is crossed ONCE per chunk instead of ~1500 times
 * per column. The recipe is the v5.7 iterateNoiseColumn transcription,
 * byte-for-byte — same calls, same order, same outputs.
 *
 * Compiled against the JDK ONLY (javac --release 17, no Minecraft jars):
 * every Minecraft type is reached through MethodHandles resolved in init()
 * from the objects the JNI side passes in. The mojang-mapped names below are
 * the exact ones proven live by the v5.7 Rust resolve() on paper-26.3.
 *
 * Threading: init() runs once on the server main thread; chunk() runs on the
 * clod-seedgen-N daemon workers. Shared state is immutable-after-init
 * (volatile fields) or concurrent (intern tables); per-call state is local.
 *
 * Output contract (packed long per LOD cell, mirrored by src/seedgen.rs):
 *   bit 63    : valid (1 = terrain hit, 0 = void column -> long is 0)
 *   bits 0-15 : blockY of the surface + 32768
 *   bit 16    : liquid (top block has a non-empty fluid state)
 *   bits 17-31: material ordinal (matTable)
 *   bits 32-46: biome ordinal   (bioTable)
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
    private static final Map<String, Integer> MAT = new ConcurrentHashMap<>();
    private static final Map<String, Integer> BIO = new ConcurrentHashMap<>();
    private static final AtomicInteger MAT_N = new AtomicInteger();
    private static final AtomicInteger BIO_N = new AtomicInteger();

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
     * One LOD chunk, one JNI call. Returns packed longs in dx-major,
     * dz-minor order with cell-size stepping: index
     * (dx/cell)*(16/cell)+(dz/cell).
     */
    public static long[] chunk(int cx, int cz, int cell) throws Throwable {
        int n = 16 / cell;
        long[] out = new long[n * n];
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
            int o = 0;
            for (int dx = 0; dx < 16; dx += cell) {
                for (int dz = 0; dz < 16; dz += cell) {
                    int wx = wx0 + dx;
                    int wz = wz0 + dz;
                    int hit = Integer.MIN_VALUE;
                    Object top = db;
                    for (int i = sizeY - 1; i >= 0; --i) {
                        int idx = (int) dvIndex.invoke(vol, dx, i, dz);
                        float d = (float) bufGet.invoke(buf, idx);
                        int y = (int) dvBlockY.invoke(vol, i);
                        Object s = aqSubstance.invoke(aquifer, wx, y, wz, (double) d);
                        if (s == null) {
                            s = db;
                        }
                        if (p.test(s)) {
                            hit = y;
                            top = s;
                            break;
                        }
                    }
                    if (hit == Integer.MIN_VALUE) {
                        out[o++] = 0L; // void column: valid bit clear
                        continue;
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
                    boolean liquid = false;
                    Object fsO = stFluidState.invoke(top);
                    if (fsO != null) {
                        liquid = !(boolean) fsEmpty.invoke(fsO);
                    }
                    String mat = "";
                    Object b = stBlock.invoke(top);
                    if (b != null) {
                        Object id = regGetKey.invoke(blockRegistry, b);
                        if (id != null) {
                            mat = id.toString();
                        }
                    }
                    String bio = "";
                    Object h = biomeGet.invoke(bman, bp);
                    if (h != null) {
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
                    }
                    out[o++] = pack(hit, liquid, matOrd(mat), bioOrd(bio));
                }
            }
        } finally {
            if (buf != null) {
                bufClose.invoke(buf);
            }
            ncClose.invoke(nc);
        }
        return out;
    }

    private static long pack(int y, boolean liquid, int mat, int bio) {
        long l = ((long) (y + 32768)) & 0xFFFFL;
        if (liquid) {
            l |= 1L << 16;
        }
        l |= (long) mat << 17;
        l |= (long) bio << 32;
        l |= 1L << 63;
        return l;
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
    }
}
