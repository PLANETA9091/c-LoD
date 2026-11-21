package org.bukkit.command;

// Mock of org.bukkit.command.PluginCommand (v6.59.15f /clodpush path).
public final class PluginCommand extends Command {
    private CommandExecutor executor;

    PluginCommand(String name) {
        super(name);
    }

    public void setExecutor(CommandExecutor executor) {
        this.executor = executor;
    }

    public CommandExecutor getExecutor() {
        return executor;
    }
}
