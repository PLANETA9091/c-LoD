package org.bukkit.event.block;

import org.bukkit.block.Block;
import org.bukkit.event.Event;

// Liquid flow: the source AND the destination column tops can both move.
public class BlockFromToEvent extends Event {
    private final Block block;
    private final Block toBlock;

    public BlockFromToEvent(Block block, Block toBlock) {
        this.block = block;
        this.toBlock = toBlock;
    }

    public Block getBlock() {
        return block;
    }

    public Block getToBlock() {
        return toBlock;
    }
}
