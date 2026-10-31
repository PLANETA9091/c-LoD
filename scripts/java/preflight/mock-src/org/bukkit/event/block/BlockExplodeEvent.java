package org.bukkit.event.block;

import java.util.List;

import org.bukkit.block.Block;
import org.bukkit.event.Event;

public class BlockExplodeEvent extends Event {
    private final List<Block> blocks;

    public BlockExplodeEvent(List<Block> blocks) {
        this.blocks = blocks;
    }

    public List<Block> blockList() {
        return blocks;
    }
}
