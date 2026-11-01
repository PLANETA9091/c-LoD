package org.bukkit.block;

import org.bukkit.World;

// Mock of org.bukkit.block.Block — SHAPE-EXACT with the real API: Block is
// an INTERFACE in Bukkit (v6.5.1 lesson: run 41293241318 died with
// "IncompatibleClassChangeError: Found interface org.bukkit.block.Block,
// but class was expected" because the mock declared it a final class, so
// javac emitted invokevirtual and the JVM refused the dispatch at runtime).
// The concrete test double lives in MockBlock (preflight-only).
public interface Block {
    World getWorld();

    int getX();

    int getY();

    int getZ();
}
