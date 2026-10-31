package net.minecraft.world.level;

import net.minecraft.core.BlockPos;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.levelgen.Heightmap;

/** Mock WorldGenLevel — exactly the surface the mock FeaturePlacer drives:
 *  setBlock / getBlockState / getHeight(Types,x,z) / getMinY. The REAL
 *  helper's dynamic proxy answers these — this interface is what makes the
 *  proxy assignable. */
public interface WorldGenLevel {
    boolean setBlock(BlockPos pos, BlockState state, int flags);

    BlockState getBlockState(BlockPos pos);

    int getHeight(Heightmap.Types type, int x, int z);

    int getMinY();
}
