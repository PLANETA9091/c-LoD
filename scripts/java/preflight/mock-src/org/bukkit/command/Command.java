package org.bukkit.command;

// Mock of org.bukkit.command.Command (v6.59.15f /clodpush path).
public abstract class Command {
    private final String name;

    protected Command(String name) {
        this.name = name;
    }

    public String getName() {
        return name;
    }
}
