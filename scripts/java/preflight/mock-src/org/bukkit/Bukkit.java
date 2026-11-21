package org.bukkit;

import org.bukkit.command.PluginCommand;
import org.bukkit.plugin.PluginManager;

// Mock of org.bukkit.Bukkit (static surface used by ClodEdits).
public final class Bukkit {
    private static PluginManager pm;
    private static PluginCommand clodpush;

    private Bukkit() {}

    public static void mock(PluginManager manager) {
        pm = manager;
    }

    public static PluginManager getPluginManager() {
        if (pm == null) {
            throw new IllegalStateException("server not started");
        }
        return pm;
    }

    // v6.59.15f: the /clodpush executor registration surface.
    public static void mockCommand(PluginCommand command) {
        clodpush = command;
    }

    public static PluginCommand getPluginCommand(String name) {
        return name.equals("clodpush") ? clodpush : null;
    }
}
