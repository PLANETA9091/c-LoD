package clod;

import org.bukkit.plugin.java.JavaPlugin;

/**
 * c-LoD CI HOST (v6.4.6): a minimal Bukkit plugin whose only job is to be
 * ENABLED.
 *
 * Why it exists: ClodEdits.tryInit registers the block-event listener with
 * the FIRST enabled plugin it finds (PluginManager.registerEvents needs a
 * live Plugin instance, and dispatch gates on the host's isEnabled()). On
 * a bare CI server no plugin exists, tryInit returned 0 forever, the
 * channel stayed EV_DEFINING and the whole session ran with
 * `edits ~0p/0c/0e` — both visual legs were carried by the rescan wave
 * alone while the v6.3 event push (the actual product feature) never
 * fired once. This jar ships into dist/plugins/ so the listener
 * registers on the first engine tick after the loader stash fills.
 *
 * Built in CI against the same paper.jar the server runs:
 *   javac -cp paper.jar -d hostout scripts/java/host/ClodHost.java
 *   jar cf dist/plugins/clodhost.jar -C hostout plugin.yml -C hostout clod
 *
 * It must stay minimal: no listeners of its own (they could steal or
 * double-feed block events), no config, nothing but a lifecycle log line.
 */
public final class ClodHost extends JavaPlugin {

    @Override
    public void onEnable() {
        getLogger().info("c-LoD host plugin up — ClodEdits can register now");
    }
}
