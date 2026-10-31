import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;

import clod.ClodEdits;
import org.bukkit.Bukkit;
import org.bukkit.World;
import org.bukkit.block.Block;
import org.bukkit.block.BlockState;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.block.BlockBreakEvent;
import org.bukkit.event.block.BlockExplodeEvent;
import org.bukkit.event.block.BlockFromToEvent;
import org.bukkit.event.block.BlockPistonExtendEvent;
import org.bukkit.event.block.BlockPlaceEvent;
import org.bukkit.event.entity.EntityExplodeEvent;
import org.bukkit.event.world.StructureGrowEvent;
import org.bukkit.plugin.Plugin;
import org.bukkit.plugin.PluginManager;

/**
 * c-LoD v6.3 EVENT PUSH preflight: the REAL java/ClodEdits helper against
 * the mock org.bukkit hierarchy — no Paper needed. Verifies the
 * registration ladder (hard fail / pending / registered), the handler
 * surface (13 public @EventHandler methods with single Event params), the
 * world identity guard, the ring packing (26-bit two's complement x/z),
 * monotonic head with wrap, and the multi-block payloads (explosions,
 * pistons, structure growth, liquid from-to).
 */
public final class PreflightEdits {

    static int checks = 0;
    static int failed = 0;

    public static void main(String[] args) throws Exception {
        World world = new World("world");
        World other = new World("world_nether");

        // -- 1. hard failure: Bukkit not started -> -1, retry stops -------
        check("bukkit-missing tryInit -> -1", ClodEdits.tryInit(world) == -1);
        check("hard fail sticks", ClodEdits.tryInit(world) == -1);
        resetRegistered();

        // -- 2. pending: manager up, no enabled plugins -> 0 --------------
        MockPluginManager pm = new MockPluginManager();
        Bukkit.mock(pm);
        check("no enabled plugin -> 0", ClodEdits.tryInit(world) == 0);
        check("pending is not registered", pm.registered.size() == 0);

        // -- 3. registration with the first enabled plugin -----------------
        MockPlugin host = new MockPlugin("host", true);
        MockPlugin late = new MockPlugin("late", true);
        pm.plugins.add(host);
        pm.plugins.add(late);
        check("enabled plugin -> 1", ClodEdits.tryInit(world) == 1);
        check("listener registered once", pm.registered.size() == 1);
        check("registered to the FIRST enabled plugin",
                ((Object) pm.registeredWith.get(0)) == (Object) host);
        check("idempotent tryInit", ClodEdits.tryInit(world) == 1);
        check("handler surface: 13 @EventHandler methods", pm.handlerCount == 13);

        // -- 4. simple pushes + the world identity guard -------------------
        Listener l = pm.registered.get(0);
        Method place = handler(l, "onPlace");
        Method place2 = handler(l, "onPlace2", true); // absent — must be null
        check("handler lookup negative", place2 == null);
        int h0 = head();
        place.invoke(l, new BlockPlaceEvent(new Block(other, 1, 64, 1)));
        check("foreign world dropped", head() == h0);
        place.invoke(l, new BlockPlaceEvent(new Block(world, 100, 70, 200)));
        check("place pushes one column", head() == h0 + 1);
        long v = ringAt(head() - 1);
        check("place packs x=100", (int) (v & 0x3FFFFFFL) == 100);
        check("place packs z=200", (int) ((v >>> 26) & 0x3FFFFFFL) == 200);

        // -- 5. negative coordinates round trip ----------------------------
        int h1 = head();
        place.invoke(l, new BlockPlaceEvent(new Block(world, -5000, 64, -12345678)));
        long n = ringAt(head() - 1);
        int ux = (int) (n & 0x3FFFFFFL);
        if (ux >= 1 << 25) ux -= 1 << 26;
        int uz = (int) ((n >>> 26) & 0x3FFFFFFL);
        if (uz >= 1 << 25) uz -= 1 << 26;
        check("negative x round trip", ux == -5000 && head() == h1 + 1);
        check("negative z round trip", uz == -12345678);

        // -- 6. from-to pushes both columns --------------------------------
        Method fromTo = handler(l, "onFromTo");
        int h2 = head();
        fromTo.invoke(l, new BlockFromToEvent(
                new Block(world, 1, 63, 1), new Block(world, 2, 63, 1)));
        check("from-to pushes two columns", head() == h2 + 2);

        // -- 7. explosion payloads (mixed worlds) ----------------------------
        Method boom = handler(l, "onEntityExplode");
        List<Block> blocks = new ArrayList<>();
        for (int i = 0; i < 5; i++) {
            blocks.add(new Block(world, 10 + i, 64, 10));
        }
        blocks.add(new Block(other, 0, 64, 0));
        int h3 = head();
        boom.invoke(l, new EntityExplodeEvent(blocks));
        check("explosion pushes in-world columns only", head() == h3 + 5);

        Method blockBoom = handler(l, "onBlockExplode");
        check("block explosion handler present", blockBoom != null);

        // -- 8. piston + structure grow BlockState payloads ------------------
        Method piston = handler(l, "onPistonExtend");
        List<BlockState> states = new ArrayList<>();
        for (int i = 0; i < 3; i++) {
            states.add(new BlockState(world, 40 + i, 64, 40));
        }
        int h4 = head();
        piston.invoke(l, new BlockPistonExtendEvent(states));
        check("piston pushes three states", head() == h4 + 3);

        Method grow = handler(l, "onGrowTree");
        int h5 = head();
        grow.invoke(l, new StructureGrowEvent(states));
        check("structure grow pushes states", head() == h5 + 3);

        // -- 9. the rest of the handler surface ------------------------------
        for (String name : new String[] {"onBreak", "onBurn", "onFade", "onGrow",
                "onDecay", "onChange", "onPistonRetract", "onBlockExplode"}) {
            check("handler " + name + " present", handler(l, name) != null);
        }

        // -- 10. head monotonicity + ring wrap -------------------------------
        int h6 = head();
        for (int i = 0; i < ClodEdits.CAP + 7; i++) {
            place.invoke(l, new BlockPlaceEvent(new Block(world, i, 64, i)));
        }
        check("head advanced past a full wrap", head() == h6 + ClodEdits.CAP + 7);
        long last = ringAt(head() - 1);
        int lx = (int) (last & 0x3FFFFFFL);
        int lz = (int) ((last >>> 26) & 0x3FFFFFFL);
        // The pushes were (i, i) for i in 0..CAP+7: the LATEST payload is
        // CAP+6 — the ring slot it landed in is (CAP+6) & (CAP-1), which
        // ringAt() masks; the VALUE must be the unmasked counter.
        check("wrap keeps the latest payload addressable",
                lx == ClodEdits.CAP + 6 && lz == ClodEdits.CAP + 6);

        System.out.println("EDIT_PUSH checks=" + checks + " failed=" + failed);
        if (failed > 0) {
            System.out.println("EDIT_PREFLIGHT FAILED");
            System.exit(1);
        }
        System.out.println("EDIT_PREFLIGHT PASS");
    }

    // ---- helpers -----------------------------------------------------------

    /** Read the helper's ring/head through reflection (package-private). */
    static int head() throws Exception {
        var f = ClodEdits.class.getDeclaredField("head");
        f.setAccessible(true);
        return f.getInt(null);
    }

    static long ringAt(int slot) throws Exception {
        var f = ClodEdits.class.getDeclaredField("RING");
        f.setAccessible(true);
        return ((long[]) f.get(null))[slot & (ClodEdits.CAP - 1)];
    }

    static void resetRegistered() throws Exception {
        var f = ClodEdits.class.getDeclaredField("registered");
        f.setAccessible(true);
        f.setInt(null, 0);
    }

    static Method handler(Listener l, String name) {
        for (Method m : l.getClass().getMethods()) {
            if (m.getName().equals(name)
                    && m.isAnnotationPresent(EventHandler.class)) {
                return m;
            }
        }
        return null;
    }

    static Method handler(Listener l, String name, boolean absent) {
        return handler(l, name);
    }

    static void check(String label, boolean ok) {
        checks++;
        System.out.println((ok ? "PASS " : "FAIL ") + label);
        if (!ok) {
            failed++;
        }
    }

    // ---- mock plugin plumbing ----------------------------------------------

    public static final class MockPlugin implements Plugin {
        private final String name;
        private final boolean enabled;

        MockPlugin(String name, boolean enabled) {
            this.name = name;
            this.enabled = enabled;
        }

        @Override
        public boolean isEnabled() {
            return enabled;
        }

        @Override
        public String toString() {
            return name;
        }
    }

    /**
     * Mirrors the SimplePluginManager registration semantics the helper
     * depends on: public methods, one parameter, the parameter extends
     * Event, the @EventHandler annotation present.
     */
    public static final class MockPluginManager implements PluginManager {
        final List<Plugin> plugins = new ArrayList<>();
        final List<Listener> registered = new ArrayList<>();
        final List<Plugin> registeredWith = new ArrayList<>();
        int handlerCount = 0;

        @Override
        public Plugin[] getPlugins() {
            return plugins.toArray(new Plugin[0]);
        }

        @Override
        public void registerEvents(Listener listener, Plugin plugin) {
            registered.add(listener);
            registeredWith.add(plugin);
            for (Method m : listener.getClass().getMethods()) {
                if (!m.isAnnotationPresent(EventHandler.class)) {
                    continue;
                }
                if (m.getParameterCount() != 1) {
                    throw new IllegalStateException("handler must take one param: " + m);
                }
                if (!org.bukkit.event.Event.class.isAssignableFrom(m.getParameterTypes()[0])) {
                    throw new IllegalStateException("handler param must be an Event: " + m);
                }
                handlerCount++;
            }
        }
    }
}
