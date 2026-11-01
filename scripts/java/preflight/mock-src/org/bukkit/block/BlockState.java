package org.bukkit.block;

import org.bukkit.World;

// Mock of org.bukkit.block.BlockState — SHAPE-EXACT with the real API:
// BlockState is an INTERFACE in Bukkit and carries getX/getY/getZ/getWorld
// (verified against the Paper javadoc). The concrete test double lives in
// MockBlockState (preflight-only).
public interface BlockState {
    World getWorld();

    int getX();

    int getY();

    int getZ();
}
