package org.bukkit.event.entity;

import org.bukkit.block.Block;
import org.bukkit.event.Event;

// Endermen, falling blocks landing/breaking, withers, boats — a common
// source of silent surface changes.
public class EntityChangeBlockEvent extends Event {
    private final Block block;

    public EntityChangeBlockEvent(Block block) {
        this.block = block;
    }

    public Block getBlock() {
        return block;
    }
}
