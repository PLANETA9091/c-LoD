package net.minecraft.world.level.block.state;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.material.FluidState;
public class BlockState {
    public final Block block;
    public final FluidState fluid;
    public BlockState(Block block, FluidState fluid) { this.block = block; this.fluid = fluid; }
    public FluidState getFluidState() { return fluid; }
    public Block getBlock() { return block; }
}
