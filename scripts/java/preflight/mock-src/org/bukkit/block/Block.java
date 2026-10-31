package org.bukkit.block;

import org.bukkit.World;

// Mock of org.bukkit.block.Block.
public final class Block {
    private final World world;
    private final int x;
    private final int y;
    private final int z;

    public Block(World world, int x, int y, int z) {
        this.world = world;
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public World getWorld() {
        return world;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    public int getZ() {
        return z;
    }
}
