package org.bukkit.event;

// Mock of org.bukkit.event.EventPriority (the real annotation members the
// SimplePluginManager scan reads).
public enum EventPriority {
    LOWEST, LOW, NORMAL, HIGH, HIGHEST, MONITOR
}
