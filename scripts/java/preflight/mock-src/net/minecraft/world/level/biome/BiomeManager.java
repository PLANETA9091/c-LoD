package net.minecraft.world.level.biome;
import net.minecraft.core.Holder;
import net.minecraft.core.BlockPos;
public class BiomeManager {
    public Holder getBiome(BlockPos p) {
        int r = Math.floorDiv(p.x, 200);
        String b = switch (Math.floorMod(r, 3)) {
            case 0 -> "minecraft:plains";
            case 1 -> "minecraft:desert";
            default -> "minecraft:snowy_plains";
        };
        return new Holder(b);
    }
}
