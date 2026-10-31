package net.minecraft.world.level.biome;

/** Mock Biome — getGenerationSettings() chain for the feature gather. */
public class Biome {
    public static final Biome GLOBAL = new Biome();

    public BiomeGenerationSettings getGenerationSettings() {
        return BiomeGenerationSettings.GLOBAL;
    }
}
