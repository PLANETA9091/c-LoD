package clod;

import java.util.List;

import org.bukkit.Bukkit;
import org.bukkit.World;
import org.bukkit.block.Block;
import org.bukkit.block.BlockState;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.block.BlockBreakEvent;
import org.bukkit.event.block.BlockBurnEvent;
import org.bukkit.event.block.BlockExplodeEvent;
import org.bukkit.event.block.BlockFadeEvent;
import org.bukkit.event.block.BlockFromToEvent;
import org.bukkit.event.block.BlockGrowEvent;
import org.bukkit.event.block.BlockPistonExtendEvent;
import org.bukkit.event.block.BlockPistonRetractEvent;
import org.bukkit.event.block.BlockPlaceEvent;
import org.bukkit.event.block.LeavesDecayEvent;
import org.bukkit.event.entity.EntityChangeBlockEvent;
import org.bukkit.event.entity.EntityExplodeEvent;
import org.bukkit.event.world.StructureGrowEvent;
import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;
import org.bukkit.command.PluginCommand;
import org.bukkit.plugin.Plugin;
import org.bukkit.plugin.PluginManager;

/**
 * c-LoD EVENT PUSH (v6.3): a Bukkit event listener that feeds every block
 * change inside the streamed world into a lock-free ring the native engine
 * drains on the NEXT engine tick. Detection drops from the rescan wave's
 * `rescan_ticks` (up to a minute at slow tick cadences) to at most one
 * engine tick — the edit lands in the dirty phase with its reset electron
 * on the same tick it drained, so the LOD shows the change in the very
 * next stream batch.
 *
 * WHY A LISTENER AND NOT NMS HOOKS: this is the STABLE plugin API surface
 * (PluginManager.registerEvents has not moved since 1.0), reached from a
 * runtime-defined class exactly like the seedgen helper — defineClass over
 * JNI, bytecode compiled from the JDK only (the mock org.bukkit tree on the
 * sourcepath carries the API shapes; the real classes take over at runtime
 * because the names match). No bytecode instrumentation, no NMS internals,
 * nothing to break across 26.3.x.
 *
 * HOST PLUGIN: registerEvents needs a live Plugin instance. The helper uses
 * the FIRST ENABLED plugin from the manager (events are dispatched through
 * the event's HandlerList — the host only answers the isEnabled() gate at
 * dispatch). No enabled plugin yet -> tryInit returns 0 and the engine
 * retries every tick (a plugin can come up late). Any hard failure (Bukkit
 * missing, registration threw) -> -1, the channel is dead and the v6.2
 * rescan wave remains the sole detector — belt and braces, never a
 * regression.
 *
 * WORLD GUARD: tryInit receives the streaming world; handlers drop edits
 * from any other world by identity (CraftWorld instances are canonical).
 *
 * RING (single-producer single-consumer, both on the server main thread —
 * block events fire there and the engine tick runs there): the producer is
 * the event handler (push = one array write + head++), the consumer is the
 * native side (reads head, copies the new longs out, advances its tail).
 * head is a monotonic count (wraps at 2^31 — consumers use u32 arithmetic);
 * the slot is head & (CAP-1). Packing: bits 0-25 = x, bits 26-51 = z (26-bit
 * two's complement each). The consumer skips ahead if it ever falls more
 * than CAP behind (drops are counted Rust-side; the wave re-covers).
 *
 * COVERAGE: place/break/burn/fade/grow(+form)/leaves-decay/liquid
 * from-to/entity-change-block/both explosions/both pistons/structure
 * growth. Sub-classed events ride along for free (a handler registered on
 * BlockPlaceEvent also fires for BlockMultiPlaceEvent; BlockFormEvent
 * extends BlockGrowEvent). Every push triggers a FULL column resample —
 * same-height material swaps are caught too (the color words are compared
 * as well). Anything events miss is caught by the rescan wave.
 */
public final class ClodEdits implements Listener, CommandExecutor {

    /** Ring capacity (power of two). ~8k pushes absorb worst-case floods. */
    public static final int CAP = 8192;

    static final long[] RING = new long[CAP];

    /** Monotonic push count (wraps; consumers use u32 arithmetic). */
    static volatile int head;

    /** 0 = pending, 1 = registered, -1 = hard failure (stop retrying). */
    static volatile int registered;

    /** The streaming world (identity guard); null until tryInit wins. */
    static volatile World world;

    static final ClodEdits INSTANCE = new ClodEdits();

    private ClodEdits() {}

    /**
     * Register the listener with the first enabled plugin. Returns 1 when
     * registered, 0 while no plugin is enabled yet (retry next tick), -1 on
     * a hard failure (never retry — the rescan wave stays the detector).
     *
     * @param worldHandle the streaming world (org.bukkit.World, passed as
     *     Object so the native side can hand over its global ref directly)
     */
    public static int tryInit(Object worldHandle) {
        if (registered != 0) {
            return registered;
        }
        try {
            World w = (World) worldHandle;
            PluginManager pm = Bukkit.getPluginManager();
            for (Plugin p : pm.getPlugins()) {
                if (p.isEnabled()) {
                    world = w;
                    pm.registerEvents(INSTANCE, p);
                    // v6.59.15f THE CONSOLE FILL PATH: a console /fill fires
                    // no Bukkit block events (the vanilla command writes
                    // blocks directly), so the CI marker stayed a stale
                    // 'water' cell forever (rgb 63,118,228 h62 in the
                    // 44820069718/44889619819 dumps while the real world =
                    // a red_concrete column). Register the /clodpush
                    // executor here — the CI calls it right after the fill.
                    PluginCommand pc = Bukkit.getPluginCommand("clodpush");
                    if (pc != null) {
                        pc.setExecutor(INSTANCE);
                    }
                    registered = 1;
                    return 1;
                }
            }
            return 0;
        } catch (Throwable t) {
            registered = -1;
            return -1;
        }
    }

    // ---- the 13 event handlers ---------------------------------------------

    @EventHandler
    public void onPlace(BlockPlaceEvent e) {
        pushBlock(e.getBlock());
    }

    @EventHandler
    public void onBreak(BlockBreakEvent e) {
        pushBlock(e.getBlock());
    }

    @EventHandler
    public void onBurn(BlockBurnEvent e) {
        pushBlock(e.getBlock());
    }

    @EventHandler
    public void onFade(BlockFadeEvent e) {
        pushBlock(e.getBlock());
    }

    @EventHandler
    public void onGrow(BlockGrowEvent e) {
        pushBlock(e.getBlock());
    }

    @EventHandler
    public void onDecay(LeavesDecayEvent e) {
        pushBlock(e.getBlock());
    }

    @EventHandler
    public void onChange(EntityChangeBlockEvent e) {
        pushBlock(e.getBlock());
    }

    @EventHandler
    public void onFromTo(BlockFromToEvent e) {
        pushBlock(e.getBlock());
        pushBlock(e.getToBlock());
    }

    @EventHandler
    public void onBlockExplode(BlockExplodeEvent e) {
        pushBlocks(e.blockList());
    }

    @EventHandler
    public void onEntityExplode(EntityExplodeEvent e) {
        pushBlocks(e.blockList());
    }

    @EventHandler
    public void onPistonExtend(BlockPistonExtendEvent e) {
        pushStates(e.getBlocks());
    }

    @EventHandler
    public void onPistonRetract(BlockPistonRetractEvent e) {
        pushStates(e.getBlocks());
    }

    @EventHandler
    public void onGrowTree(StructureGrowEvent e) {
        pushStates(e.getBlocks());
    }

    // ---- the push path -------------------------------------------------------

    static void pushBlock(Block b) {
        if (world != null && b.getWorld() == world) {
            push(b.getX(), b.getZ());
        }
    }

    static void pushBlocks(List<Block> blocks) {
        for (int i = 0; i < blocks.size(); i++) {
            pushBlock(blocks.get(i));
        }
    }

    static void pushStates(List<BlockState> states) {
        for (int i = 0; i < states.size(); i++) {
            BlockState s = states.get(i);
            if (world != null && s.getWorld() == world) {
                push(s.getX(), s.getZ());
            }
        }
    }

    /** bits 0-25 = x, bits 26-51 = z (26-bit two's complement each). */
    static void push(int x, int z) {
        if (registered != 1) {
            return;
        }
        final int h = head;
        RING[h & (CAP - 1)] = ((long) x & 0x3FFFFFFL) | (((long) z & 0x3FFFFFFL) << 26);
        head = h + 1;
    }

    /**
     * v6.59.15f /clodpush x z [radius=4] — the console-fill bridge. The
     * vanilla /fill (and /setblock) from the console bypasses every Bukkit
     * block event, so an edit the CI makes by command never reached this
     * listener and the affected cells streamed their pre-edit height/color
     * forever. Pushes every BLOCK coordinate of the square into the same
     * ring the 13 event handlers use (the native side converts to cells,
     * marks them dirty and re-strokes with the reset electron — the full
     * edit protocol, byte-identical to a player-placed block).
     */
    @Override
    public boolean onCommand(CommandSender sender, Command cmd, String label, String[] args) {
        if (args.length < 2) {
            return false;
        }
        try {
            int x = Integer.parseInt(args[0]);
            int z = Integer.parseInt(args[1]);
            int r = args.length > 2 ? Integer.parseInt(args[2]) : 4;
            if (r < 0) {
                r = 0;
            }
            if (r > 32) {
                r = 32;
            }
            if (world == null) {
                return false;
            }
            for (int dx = -r; dx <= r; dx++) {
                for (int dz = -r; dz <= r; dz++) {
                    push(x + dx, z + dz);
                }
            }
            return true;
        } catch (Throwable t) {
            return false;
        }
    }
}
