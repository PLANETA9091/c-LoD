package org.bukkit.event.world;

import java.util.List;

import org.bukkit.block.BlockState;
import org.bukkit.event.Event;

public class StructureGrowEvent extends Event {
    private final List<BlockState> blocks;

    public StructureGrowEvent(List<BlockState> blocks) {
        this.blocks = blocks;
    }

    public List<BlockState> getBlocks() {
        return blocks;
    }
}
