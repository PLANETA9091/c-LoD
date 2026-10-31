package net.minecraft.world.level.levelgen;

import net.minecraft.util.RandomSource;

/** Mock XoroshiroRandomSource — ctor(long) like the real one. */
public class XoroshiroRandomSource extends RandomSource {
    public XoroshiroRandomSource(long seed) {
        super(seed);
    }
}
