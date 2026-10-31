package org.bukkit.plugin;

// Mock of org.bukkit.plugin.Plugin (preflight only — the real interface is
// resolved at runtime on Paper).
public interface Plugin {
    boolean isEnabled();
}
