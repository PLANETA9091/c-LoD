package net.minecraft.core;

/** Mock BlockPos — extends Vec3i (like the real one) + asLong packing. */
public class BlockPos extends Vec3i {
    public BlockPos(int x, int y, int z) {
        super(x, y, z);
    }

    public long asLong() {
        return (((long) x & 0x3FFFFFFL) << 38)
                | (((long) z & 0x3FFFFFFL) << 12)
                | ((long) y & 0xFFFL);
    }
}
