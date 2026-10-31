package net.minecraft.world.level.levelgen;

import net.minecraft.util.RandomSource;

/** Mock WorldgenRandom — the REAL setDecorationSeed/setFeatureSeed recipes
 *  (transcribed from paper-26.3 bytecode) so seeding is deterministic and
 *  shape-accurate. */
public class WorldgenRandom extends RandomSource {
    public WorldgenRandom(RandomSource wrapped) {
        super(wrapped.nextLong());
    }

    public long setDecorationSeed(long levelSeed, int step, int index) {
        setSeed(levelSeed);
        long a = nextLong() | 1L;
        long b = nextLong() | 1L;
        long mixed = ((long) step * a + (long) index * b) ^ levelSeed;
        setSeed(mixed);
        return mixed;
    }

    public void setFeatureSeed(long decoratedSeed, int index, int step) {
        setSeed(decoratedSeed + (long) index + (long) step * 10000L);
    }
}
