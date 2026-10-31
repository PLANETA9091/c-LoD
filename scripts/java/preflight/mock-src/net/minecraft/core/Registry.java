package net.minecraft.core;
public class Registry {
    public net.minecraft.resources.Identifier getKey(Object o) {
        return new net.minecraft.resources.Identifier("minecraft:" + ((net.minecraft.world.level.block.Block) o).name);
    }
}
