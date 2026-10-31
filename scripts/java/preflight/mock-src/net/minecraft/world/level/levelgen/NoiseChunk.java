package net.minecraft.world.level.levelgen;
import net.minecraft.world.level.levelgen.densityfunction.DensitySamplerSet;
import net.minecraft.world.level.levelgen.blending.Blender;
import net.minecraft.world.level.levelgen.densityfunction.DensityVolume;
public class NoiseChunk {
    public static int closeCount = 0;
    private final DensityVolume vol;
    public NoiseChunk(RandomState rs, Beardifier beard, NoiseGeneratorSettings st,
            Aquifer.FluidPicker fp, Blender bl, DensityVolume vol) {
        this.vol = vol;
    }
    public DensitySamplerSet cachingSamplers() { return new DensitySamplerSet(); }
    public DensityVolume volume() { return vol; }
    public Aquifer aquifer() { return new Aquifer(); }
    public void close() { closeCount++; }
}
