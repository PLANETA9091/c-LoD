package org.bukkit.event.block;

import org.bukkit.block.Block;
import org.bukkit.event.Event;

public class LeavesDecayEvent extends Event {
    private final Block block;

    public LeavesDecayEvent(Block block) {
        this.block = block;
    }

    public Block getBlock() {
        return block;
    }
}
