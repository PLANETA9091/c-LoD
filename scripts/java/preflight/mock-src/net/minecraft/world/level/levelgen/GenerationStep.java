package net.minecraft.world.level.levelgen;

/** Mock GenerationStep with the nested Decoration enum (order scrambled on
 *  purpose — the helper must find VEGETAL_DECORATION BY NAME). */
public final class GenerationStep {

    private GenerationStep() {}

    public enum Decoration {
        TOP_LAYER_MODIFICATION,
        SURFACE_STRUCTURES,
        VEGETAL_DECORATION;

        public static final int VEGETAL_ORDINAL = VEGETAL_DECORATION.ordinal();
    }
}
