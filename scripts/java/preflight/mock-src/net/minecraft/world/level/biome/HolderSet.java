package net.minecraft.world.level.biome;

import java.util.Iterator;

/** Mock HolderSet — Iterable<Holder> like the real interface. */
public class HolderSet implements Iterable<Object> {
    private final Object[] holders;

    public HolderSet(Object... holders) {
        this.holders = holders;
    }

    @Override
    public Iterator<Object> iterator() {
        return java.util.List.of(holders).iterator();
    }
}
