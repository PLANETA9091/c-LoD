package org.bukkit;

import org.bukkit.plugin.PluginManager;

// Mock of org.bukkit.Bukkit (static surface used by ClodEdits).
public final class Bukkit {
    private static PluginManager pm;

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
}
