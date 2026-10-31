package org.bukkit.event.block;

import org.bukkit.block.Block;
import org.bukkit.event.Event;

// Real BlockFormEvent (snow/concrete forms) extends BlockGrowEvent, so one
// handler catches both — same trick as BlockPlaceEvent/BlockMultiPlaceEvent.
public class BlockGrowEvent extends Event {
    private final Block block;

    public BlockGrowEvent(Block block) {
        this.block = block;
    }

    public Block getBlock() {
        return block;
    }
}
