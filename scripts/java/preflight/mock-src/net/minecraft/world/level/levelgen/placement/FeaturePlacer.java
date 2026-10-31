package net.minecraft.world.level.levelgen.placement;

import net.minecraft.core.BlockPos;
import net.minecraft.util.RandomSource;
import net.minecraft.world.level.WorldGenLevel;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.ChunkGenerator;
import net.minecraft.world.level.levelgen.Heightmap;
import net.minecraft.world.level.material.FluidState;

/**
 * Mock FeaturePlacer — ctor/param shapes identical to the real class, so
 * the helper's MethodHandles bind; placeWithBiomeCheck plants deterministic
 * mock trees through the REAL proxy surface (setBlock/getHeight), which
 * exercises the GenModel recording end-to-end.
 *
 * Tree shape: trunk of 4 logs from the (NO_LEAVES) heightmap up, then a
 * 3x3x2 leaf cube. The random consumption mirrors the real pipeline's
 * count/in_square/heightmap ordering (two nextInt(16) per attempt).
 */
public class FeaturePlacer {
    public static BlockState LOG =
            new BlockState(new Block("oak_log"), FluidState.EMPTY);
    public static BlockState LEAVES =
            new BlockState(new Block("oak_leaves"), FluidState.EMPTY);

    private final WorldGenLevel level;
    @SuppressWarnings("unused")
    private final ChunkGenerator generator;
    public int treesPlaced = 0;

    public FeaturePlacer(WorldGenLevel level, ChunkGenerator generator) {
        this.level = level;
        this.generator = generator;
    }

    public boolean placeWithBiomeCheck(PlacedFeature feature,
            RandomSource random, BlockPos origin) {
        int minY = level.getMinY();
        for (int i = 0; i < feature.count; i++) {
            // snap to even coords so trunks land on sampled (cell_size=2)
            // columns — the real vanilla placer hits every parity, the mock
            // just wants log-topped cells reachable by the cell patch
            int x = origin.getX() + (random.nextInt(16) & ~1);
            int z = origin.getZ() + (random.nextInt(16) & ~1);
            int y = level.getHeight(
                    Heightmap.Types.MOTION_BLOCKING_NO_LEAVES, x, z);
            if (y <= minY) {
                continue; // void column — like a biome/water filter reject
            }
            for (int t = 0; t < 4; t++) {
                level.setBlock(new BlockPos(x, y + t, z), LOG, 19);
            }
            // leaf ring around the trunk (center left open — the trunk
            // column tops with a log, like a sheared tree). The outer ring
            // uses even offsets so leaves top SAMPLED (even) columns too.
            for (int dx = -1; dx <= 1; dx++) {
                for (int dz = -1; dz <= 1; dz++) {
                    if (dx == 0 && dz == 0) {
                        continue;
                    }
                    for (int dy = 3; dy < 6; dy++) {
                        level.setBlock(new BlockPos(x + dx, y + dy, z + dz),
                                LEAVES, 19);
                    }
                }
            }
            for (int dx = -2; dx <= 2; dx += 2) {
                for (int dz = -2; dz <= 2; dz += 2) {
                    if (dx == 0 && dz == 0) {
                        continue;
                    }
                    for (int dy = 4; dy < 6; dy++) {
                        level.setBlock(new BlockPos(x + dx, y + dy, z + dz),
                                LEAVES, 19);
                    }
                }
            }
            treesPlaced++;
        }
        return true;
    }
}
