package net.minecraft.util;

/** Mock RandomSource — java.util.Random with the real constructor shapes. */
public class RandomSource extends java.util.Random {
    public RandomSource() {
        super(0L);
    }

    public RandomSource(long seed) {
        super(seed);
    }
}
