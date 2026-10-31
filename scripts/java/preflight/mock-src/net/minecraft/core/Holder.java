package net.minecraft.core;
import java.util.Optional;
public class Holder {
    private final String key;
    public Holder(String key) { this.key = key; }
    public Optional<net.minecraft.resources.ResourceKey> unwrapKey() {
        return Optional.of(new net.minecraft.resources.ResourceKey(key));
    }
}
