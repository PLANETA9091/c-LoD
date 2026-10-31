package org.bukkit.plugin;

import org.bukkit.event.Listener;

// Mock of org.bukkit.plugin.PluginManager. The preflight mock records the
// listener and its @EventHandler methods (the real SimplePluginManager
// scans public single-Event-param methods and registers executors).
public interface PluginManager {
    Plugin[] getPlugins();

    void registerEvents(Listener listener, Plugin plugin);
}
