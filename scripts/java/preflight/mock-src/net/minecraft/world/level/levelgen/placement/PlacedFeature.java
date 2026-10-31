package net.minecraft.world.level.levelgen.placement;

import net.minecraft.core.BlockPos;
import net.minecraft.util.RandomSource;

/** Mock PlacedFeature — feature payload + tree count for the mock placer. */
public class PlacedFeature {
    public final String name;
    public final int count;

    public PlacedFeature(String name, int count) {
        this.name = name;
        this.count = count;
    }
}
