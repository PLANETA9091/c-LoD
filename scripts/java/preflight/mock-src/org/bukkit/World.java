package org.bukkit;

// Mock of org.bukkit.World — identity instance per world (CraftWorld is
// canonical per world on the real server).
public final class World {
    private final String name;

    public World(String name) {
        this.name = name;
    }

    public String name() {
        return name;
    }
}
