package org.bukkit.event.entity;

import java.util.List;

import org.bukkit.block.Block;
import org.bukkit.event.Event;

public class EntityExplodeEvent extends Event {
    private final List<Block> blocks;

    public EntityExplodeEvent(List<Block> blocks) {
        this.blocks = blocks;
    }

    public List<Block> blockList() {
        return blocks;
    }
}
