package net.minecraft.world.level.biome;

import java.util.List;

/** Mock BiomeGenerationSettings — features() is List<HolderSet> like the
 *  real one: index = decoration step, element = the step's HolderSet. */
public class BiomeGenerationSettings {
    public static final BiomeGenerationSettings GLOBAL = new BiomeGenerationSettings();

    private final Object[] steps;

    public BiomeGenerationSettings() {
        // enough entries for any mock step ordinal; the harness fills them
        this.steps = new Object[16];
    }

    public void setStep(int ordinal, Object holderSet) {
        steps[ordinal] = holderSet;
    }

    public List<Object> features() {
        return java.util.Arrays.asList(steps);
    }
}
