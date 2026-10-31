package org.bukkit.event.block;

import org.bukkit.block.Block;
import org.bukkit.event.Event;

public class BlockBurnEvent extends Event {
    private final Block block;

    public BlockBurnEvent(Block block) {
        this.block = block;
    }

    public Block getBlock() {
        return block;
    }
}
