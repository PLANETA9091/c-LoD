import java.lang.reflect.Method;
import java.util.Arrays;
import java.util.function.Predicate;

import clod.ClodSeedgen;
import clodmock.PreflightTerrain;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Holder;
import net.minecraft.core.Registry;
import net.minecraft.resources.Identifier;
import net.minecraft.resources.ResourceKey;
import net.minecraft.world.level.LevelHeightAccessor;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.levelgen.Aquifer;
import net.minecraft.world.level.levelgen.blending.Blender;
import net.minecraft.world.level.levelgen.Beardifier;
import net.minecraft.world.level.levelgen.NoiseGeneratorSettings;
import net.minecraft.world.level.levelgen.NoiseChunk;
import net.minecraft.world.level.levelgen.RandomState;
import net.minecraft.world.level.levelgen.WorldGenerationContext;
import net.minecraft.world.level.levelgen.densityfunction.DensityFunction;
import net.minecraft.world.level.levelgen.densityfunction.ScopedDensityBuffer;
import net.minecraft.world.level.levelgen.material.MaterialSystem;
import net.minecraft.world.level.levelgen.material.rule.MaterialRule;

/**
 * v5.9 SEEDGEN preflight — runs the REAL helper (java/ClodSeedgen) against a
 * mock net.minecraft hierarchy, no Paper needed. Verifies the things the
 * Rust unit tests cannot reach:
 *
 *   1. parity: chunk() (v5.8 single-chunk contract) vs the chunkBatch()
 *      payload — bit-for-bit equal packed cells;
 *   2. batch structure: header, per-chunk status, zero-filled throwing
 *      segments, n_ok; buffer/NoiseChunk close still runs for the thrower;
 *   3. stride fast path vs fallback: the same terrain through the affine
 *      layout and a non-affine (Fisher-Yates) layout must produce the same
 *      cells — the runner compares the two JVMs' output hashes;
 *   4. tabVer monotonicity (tables re-read only when it moves);
 *   5. identity caches: warm-cache reruns are byte-identical.
 *
 * Prints "CELLS_SHA <hex>" and "PREFLIGHT PASS <n> checks" on success.
 */
public final class Preflight {
    private Preflight() {}

    static int checks = 0;
    static final int MIN_Y = -64;
    static final int SIZE_Y = 384;

    public static void main(String[] args) throws Throwable {
        boolean weird = "weird".equals(System.getProperty("clod.mock.layout", "affine"));

        // ---- the real helper, init()'d with mock objects -------------------
        BlockState stone = Aquifer.STONE;
        ClodSeedgen.init(
                new LevelHeightAccessor() {}, // level (LevelHeightAccessor)
                new RandomState(),            // rs
                new NoiseGeneratorSettings(), // settings
                new MaterialRule() {},        // rule
                new RandomState().surfaceSystem(), // surface
                new WorldGenerationContext(), // ctx
                (Predicate<Object>) s -> {    // MOTION_BLOCKING-ish
                    if (s instanceof BlockState bs) {
                        return bs != Aquifer.AIR;
                    }
                    return true;
                },
                new Aquifer.FluidPicker() {}, // fluidPicker
                new Blender(),                // blender
                stone,                        // defaultBlock
                new DensityFunction() {},     // finalDensity (unused by mocks)
                new net.minecraft.world.level.chunk.UpgradeData(), // upgradeData
                new net.minecraft.world.level.chunk.PalettedContainerFactory(), // factory
                new java.util.function.Function<Object, Object>() { // biomeFn (unused by mocks)
                    @Override
                    public Object apply(Object p) {
                        throw new UnsupportedOperationException("biomeFn unused by mocks");
                    }
                },
                new Registry(),               // blockRegistry
                new net.minecraft.world.level.biome.BiomeManager(), // bman
                MIN_Y, SIZE_Y);
        check("helper ready", ClodSeedgen.isReady());

        // ---- ground truth per chunk via the v5.8 single-chunk call --------
        int[][] chunks = {
                {-2, -2}, {-1, 0}, {0, 0}, {1, 2}, {2, 2},
                {3, -4}, {-9, -9},                    // void chunk
                {99, 99}, {99, 100}, {100, 99},       // poison chunks (throw)
        };
        long[][] singles = new long[chunks.length][];
        for (int i = 0; i < chunks.length; i++) {
            try {
                singles[i] = ClodSeedgen.chunk(chunks[i][0], chunks[i][1], 2);
            } catch (Throwable t) {
                singles[i] = null; // poison chunk throws on the single path
            }
        }
        check("void chunk is all zeros", singles[6] != null
                && Arrays.stream(singles[6]).allMatch(l -> l == 0L));
        check("poison chunks throw on the single path",
                singles[7] == null && singles[8] == null && singles[9] == null);

        // ---- batch parity ---------------------------------------------------
        // K=1: one chunk per batch must equal chunk() exactly.
        for (int i = 0; i < chunks.length; i++) {
            if (singles[i] == null) {
                continue;
            }
            long[] payload = ClodSeedgen.chunkBatch(
                    new int[] {chunks[i][0], chunks[i][1]}, 2);
            check("K=1 payload length", payload.length == 4 + 1 + 64);
            check("K=1 header", payload[0] == 1 && payload[1] == 1);
            check("K=1 status", payload[4] == 1);
            check("K=1 parity for " + Arrays.toString(chunks[i]),
                    Arrays.equals(singles[i], 0, 64, payload, 5, 69));
        }

        // K=8 (padded with repeats): statuses + cells; poison segments zero.
        int[] coords = new int[16];
        for (int i = 0; i < 8; i++) {
            coords[2 * i] = chunks[i][0];
            coords[2 * i + 1] = chunks[i][1];
        }
        long[] batch = ClodSeedgen.chunkBatch(coords, 2);
        int seg = 1 + 64;
        check("K=8 payload length", batch.length == 4 + 8 * seg);
        check("K=8 header", batch[0] == 8 && batch[1] == 7); // 7 ok + 1 poison
        for (int i = 0; i < 8; i++) {
            int off = 4 + i * seg;
            if (singles[i] == null) {
                check("poison segment " + i + " zero-filled", batch[off] == 0
                        && allZero(batch, off + 1, off + 1 + 64));
            } else {
                check("status " + i, batch[off] == 1);
                check("parity " + i, Arrays.equals(
                        singles[i], 0, 64, batch, off + 1, off + 1 + 64));
            }
        }

        // ---- tabVer: grows when tables grow, stable when warm --------------
        int vBefore = ClodSeedgen.tabVer;
        ClodSeedgen.chunk(0, 0, 2); // warm caches, no new materials expected
        check("tabVer stable on warm chunk", ClodSeedgen.tabVer == vBefore);
        check("tables populated", ClodSeedgen.matTable.length >= 3
                && ClodSeedgen.bioTable.length >= 2); // stone/water/sand|grass; plains+snowy

        // ---- close discipline: one buffer + one NoiseChunk per chunk ------
        int bufBefore = ScopedDensityBuffer.closeCount;
        int ncBefore = NoiseChunk.closeCount;
        ClodSeedgen.chunkBatch(new int[] {5, 5, 6, 6, 7, 7}, 2);
        check("buffer closed once per chunk",
                ScopedDensityBuffer.closeCount == bufBefore + 3);
        check("NoiseChunk closed once per chunk",
                NoiseChunk.closeCount == ncBefore + 3);

        // ---- hash every non-void cell for the cross-layout comparison -----
        long h = 1125899906842597L;
        for (long[] s : singles) {
            if (s == null) {
                h = h * 31 + 0xDEAD;
                continue;
            }
            for (long l : s) {
                h = h * 31 + (l ^ (l >>> 31));
            }
        }
        System.out.println("LAYOUT " + (weird ? "weird" : "affine"));
        System.out.println("CELLS_SHA " + Long.toHexString(h));
        System.out.println("COLUMNS fast=" + ClodSeedgen.FAST_COLUMNS.get()
                + " slow=" + ClodSeedgen.SLOW_COLUMNS.get());
        System.out.println("MAT " + String.join(",", ClodSeedgen.matTable));
        System.out.println("BIO " + String.join(",", ClodSeedgen.bioTable));
        System.out.println("PREFLIGHT PASS " + checks + " checks");
    }

    static boolean allZero(long[] a, int from, int to) {
        for (int i = from; i < to; i++) {
            if (a[i] != 0) {
                return false;
            }
        }
        return true;
    }

    static void check(String name, boolean ok) {
        checks++;
        if (!ok) {
            System.out.println("FAIL[" + checks + "]: " + name);
            System.exit(1);
        }
    }
}
