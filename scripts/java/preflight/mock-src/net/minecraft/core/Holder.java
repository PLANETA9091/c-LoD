package net.minecraft.core;

import java.util.Optional;

/** Mock Holder — value() added for the v6.1 feature gather (nullable). */
public class Holder {
    private final String key;
    private final Object value;

    public Holder(String key) {
        this(key, null);
    }

    public Holder(String key, Object value) {
        this.key = key;
        this.value = value;
    }

    public Optional<net.minecraft.resources.ResourceKey> unwrapKey() {
        return Optional.of(new net.minecraft.resources.ResourceKey(key));
    }

    public Object value() {
        return value;
    }
}
