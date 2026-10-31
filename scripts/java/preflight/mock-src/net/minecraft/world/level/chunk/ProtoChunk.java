package net.minecraft.world.level.chunk;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.world.level.LevelHeightAccessor;
import net.minecraft.world.level.levelgen.blending.BlendingData;
public class ProtoChunk extends ChunkAccess {
    public final ChunkPos pos;
    public ProtoChunk(ChunkPos pos, UpgradeData up, LevelHeightAccessor lha,
            PalettedContainerFactory pcf, BlendingData bd) {
        this.pos = pos;
    }
}
