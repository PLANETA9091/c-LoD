package net.minecraft.world.level.chunk;

import java.util.function.Supplier;

/**
 * Mock ChunkGenerator — carries the private featuresPerStep field the
 * helper digs out with privateLookupIn(...).findGetter.
 */
public class ChunkGenerator {
    @SuppressWarnings("unused")
    private final Supplier<?> featuresPerStep;

    public ChunkGenerator(Supplier<?> featuresPerStep) {
        this.featuresPerStep = featuresPerStep;
    }
}
