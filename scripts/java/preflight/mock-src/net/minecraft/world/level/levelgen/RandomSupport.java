package net.minecraft.world.level.levelgen;

import java.util.concurrent.atomic.AtomicLong;

/** Mock RandomSupport — generateUniqueSeed (process-unique, like vanilla). */
public final class RandomSupport {
    private static final AtomicLong UID = new AtomicLong(8682522807148012L);

    private RandomSupport() {}

    public static long generateUniqueSeed() {
        return UID.incrementAndGet();
    }
}
