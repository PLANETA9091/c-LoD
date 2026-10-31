package net.minecraft.world.level.block;

import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.material.FluidState;

/** Mock Block + the defaultBlockState() the helper resolves. */
public class Block {
    public final String name;

    public Block(String name) {
        this.name = name;
    }

    public BlockState defaultBlockState() {
        return new BlockState(this, FluidState.EMPTY);
    }
}
