package org.bukkit.command;

// Mock of org.bukkit.command.CommandExecutor (v6.59.15f /clodpush path).
public interface CommandExecutor {
    boolean onCommand(CommandSender sender, Command cmd, String label, String[] args);
}
