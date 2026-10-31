package net.minecraft.world.level.material;
public class FluidState {
    public static final FluidState EMPTY = new FluidState(true);
    public static final FluidState WATER = new FluidState(false);
    private final boolean empty;
    private FluidState(boolean empty) { this.empty = empty; }
    public boolean isEmpty() { return empty; }
}
