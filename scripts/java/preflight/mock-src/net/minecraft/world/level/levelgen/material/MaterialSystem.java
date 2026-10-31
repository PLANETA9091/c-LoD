package net.minecraft.world.level.levelgen.material;
import java.util.Optional;
import java.util.function.Function;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.levelgen.RandomState;
import net.minecraft.world.level.levelgen.WorldGenerationContext;
import net.minecraft.world.level.levelgen.densityfunction.DensitySamplerSet;
import net.minecraft.world.level.levelgen.material.rule.MaterialRule;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.material.FluidState;

/** Mock surface rules: deterministic by pos; throws for POISON_CHUNK so the
 *  batch status-0 path is exercised. */
public class MaterialSystem {
    public static final int POISON_CHUNK = 99;
    public static final BlockState SNOW = new BlockState(new Block("snow_block"), FluidState.EMPTY);
    public static final BlockState SAND = new BlockState(new Block("sand"), FluidState.EMPTY);
    public static final BlockState GRASS = new BlockState(new Block("grass_block"), FluidState.EMPTY);

    public Optional<BlockState> topMaterial(MaterialRule rule, RandomState rs,
            WorldGenerationContext ctx, Function<BlockPos, ?> biomeFn, ChunkAccess chunk,
            DensitySamplerSet set, BlockPos pos, boolean steep) {
        if ((pos.x >> 4) == POISON_CHUNK || (pos.z >> 4) == POISON_CHUNK) {
            throw new IllegalStateException("poison chunk surface");
        }
        if (pos.y > 140) {
            return Optional.of(SNOW);
        }
        if (pos.y <= 63) {
            return Optional.empty(); // water/stone from the walk stays
        }
        return ((pos.x + pos.z) % 3 == 0) ? Optional.of(SAND) : Optional.of(GRASS);
    }
}
