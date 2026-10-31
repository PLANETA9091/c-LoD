package net.minecraft.world.level.levelgen;

import java.util.function.Predicate;

import net.minecraft.world.level.block.state.BlockState;

/** Mock Heightmap with the nested Types enum — the REAL predicate shapes:
 *  MOTION_BLOCKING counts non-air, NO_LEAVES additionally excludes leaves,
 *  WORLD_SURFACE counts non-air. */
public final class Heightmap {

    private Heightmap() {}

    public enum Types {
        WORLD_SURFACE(s -> !isAir(s)),
        MOTION_BLOCKING(s -> !isAir(s)),
        MOTION_BLOCKING_NO_LEAVES(s -> !isAir(s) && !isLeaves(s));

        public final Predicate<BlockState> opaque;

        Types(Predicate<BlockState> opaque) {
            this.opaque = opaque;
        }

        public Predicate<BlockState> isOpaque() {
            return opaque;
        }

        private static boolean isAir(BlockState s) {
            return s != null && "air".equals(s.block.name);
        }

        private static boolean isLeaves(BlockState s) {
            return s != null && s.block.name.contains("leaves");
        }
    }
}
