package org.bukkit.event.block;

import org.bukkit.block.Block;
import org.bukkit.event.Event;

// Mock (real: extends BlockEvent — one handler on BlockPlaceEvent also
// catches BlockMultiPlaceEvent, which extends it).
public class BlockPlaceEvent extends Event {
    private final Block block;

    public BlockPlaceEvent(Block block) {
        this.block = block;
    }

    public Block getBlock() {
        return block;
    }
}
