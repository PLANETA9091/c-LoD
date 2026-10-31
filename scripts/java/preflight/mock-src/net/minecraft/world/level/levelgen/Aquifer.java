package net.minecraft.world.level.levelgen;
import net.minecraft.world.level.block.state.BlockState;
public class Aquifer {
    public interface FluidPicker {}
    public static final BlockState STONE = new BlockState(
            new net.minecraft.world.level.block.Block("stone"), net.minecraft.world.level.material.FluidState.EMPTY);
    public static final BlockState WATER = new BlockState(
            new net.minecraft.world.level.block.Block("water"), net.minecraft.world.level.material.FluidState.WATER);
    public static final BlockState AIR = new BlockState(
            new net.minecraft.world.level.block.Block("air"), net.minecraft.world.level.material.FluidState.EMPTY);
    public BlockState computeSubstance(int x, int y, int z, double d) {
        if ((x >> 4) == clodmock.PreflightTerrain.VOID_CHUNK
                && (z >> 4) == clodmock.PreflightTerrain.VOID_CHUNK) {
            return AIR; // outside the world border: no fluid, no solid
        }
        if (d > 0) {
            return STONE;
        }
        return y <= 62 ? WATER : AIR;
    }
}
