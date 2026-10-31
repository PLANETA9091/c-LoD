package net.minecraft.world.level.levelgen.densityfunction;
public class DensitySampler {
    public static class Bound {
        public ScopedDensityBuffer sampleVolume(DensityVolume v) {
            return new ScopedDensityBuffer(v);
        }
    }
}
