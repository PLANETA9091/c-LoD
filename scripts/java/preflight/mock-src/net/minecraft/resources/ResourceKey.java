package net.minecraft.resources;
public class ResourceKey {
    private final String key;
    public ResourceKey(String key) { this.key = key; }
    public net.minecraft.resources.Identifier identifier() {
        return new net.minecraft.resources.Identifier(key);
    }
}
