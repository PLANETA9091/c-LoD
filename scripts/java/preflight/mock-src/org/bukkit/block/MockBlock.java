package org.bukkit.block;

import org.bukkit.World;

// Concrete preflight double for the org.bukkit.block.Block mock interface.
// The real runtime classes (CraftBlock) are resolved on Paper; this one only
// serves the preflight suite, which constructs blocks by (world, x, y, z).
public final class MockBlock implements Block {
    private final World world;
    private final int x;
    private final int y;
    private final int z;

    public MockBlock(World world, int x, int y, int z) {
        this.world = world;
        this.x = x;
        this.y = y;
        this.z = z;
    }

    @Override
    public World getWorld() {
        return world;
    }

    @Override
    public int getX() {
        return x;
    }

    @Override
    public int getY() {
        return y;
    }

    @Override
    public int getZ() {
        return z;
    }
}
