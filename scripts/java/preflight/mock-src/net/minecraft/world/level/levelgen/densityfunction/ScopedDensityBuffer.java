package net.minecraft.world.level.levelgen.densityfunction;
public class ScopedDensityBuffer {
    public static int closeCount = 0;
    private final DensityVolume v;
    ScopedDensityBuffer(DensityVolume v) { this.v = v; }
    public float get(int idx) { return (float) v.at(idx); }
    public void close() { closeCount++; }
}
