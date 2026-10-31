package org.bukkit.event.block;

import java.util.List;

import org.bukkit.block.BlockState;
import org.bukkit.event.Event;

public class BlockPistonRetractEvent extends Event {
    private final List<BlockState> blocks;

    public BlockPistonRetractEvent(List<BlockState> blocks) {
        this.blocks = blocks;
    }

    public List<BlockState> getBlocks() {
        return blocks;
    }
}
