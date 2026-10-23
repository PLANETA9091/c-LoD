#!/usr/bin/env bash
# Voxy reference client: real Voxy 0.2.20-beta (Fabric 0.20.1 + fabric-api +
# sodium 0.9.3-beta.1) on the same bench (seed 4819696040575474753, user
# camera, port 25565). Used ONLY to produce the A/B comparison screenshots
# against the c-LoD posteffect LOD (run_voxy_ab.sh drives it).
# Logs to /tmp/voxy_client.log, DISPLAY :99.
set -euo pipefail
C="$HOME/mc-test/client"
JAVA="${CLOD_JAVA:-$HOME/mc-test/jdk25/bin/java}"

pkill -f "KnotClient" 2>/dev/null || true
pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
pkill Xvfb 2>/dev/null || true
sleep 1

Xvfb :99 -screen 0 1280x720x24 +extension GLX +render -noreset > /tmp/xvfb.log 2>&1 &
XVFB=$!
sleep 2

export DISPLAY=:99
export LIBGL_ALWAYS_SOFTWARE=1
# Voxy 0.2.20 ships #version 460 core shaders (hiz.comp, AsyncNodeManager
# pipeline) — llvmpipe with a 4.5 override rejects them at compile time
# (live-crash 2026-10-23). Request 4.6: llvmpipe creates the highest context
# it can; when its caps allow 4.6 the GLSL 460 compiles fine.
export MESA_GL_VERSION_OVERRIDE=4.6
export __GLX_VENDOR_LIBRARY_NAME=mesa
export GLX_VENDOR_LIBRARY_NAME=mesa
export LD_LIBRARY_PATH="$HOME/mc-test/shim${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

cd "$C"
mkdir -p screenshots mods

# mods/ carries voxy-0.2.20-beta.jar + fabric-api + sodium 0.9.3-beta.1
# (voxy depends: fabric-api >=0.91.1, sodium =0.9.2 | >=0.9.3- <=0.9.3).
ls mods/voxy-*.jar >/dev/null 2>&1 || { echo "voxy jar missing in mods/" >&2; exit 1; }
ls mods/fabric-api-*.jar >/dev/null 2>&1 || { echo "fabric-api jar missing in mods/" >&2; exit 1; }
ls mods/sodium-*.jar >/dev/null 2>&1 || { echo "sodium jar missing in mods/" >&2; exit 1; }

OPT="$C/options.txt"
touch "$OPT"; chmod u+w "$OPT"
# Same visual base as the c-LoD runs (RD 6 = identical real-chunk ring).
grep -q "^preferredGraphicsBackend:" "$OPT" && sed -i 's/^preferredGraphicsBackend:.*/preferredGraphicsBackend:opengl/' "$OPT" || echo "preferredGraphicsBackend:opengl" >> "$OPT"
grep -q "^pauseOnLostFocus:" "$OPT" && sed -i 's/^pauseOnLostFocus:.*/pauseOnLostFocus:false/' "$OPT" || echo "pauseOnLostFocus:false" >> "$OPT"
grep -q "^renderDistance:" "$OPT" && sed -i 's/^renderDistance:.*/renderDistance:4/' "$OPT" || echo "renderDistance:4" >> "$OPT"
grep -q "^fullscreen:" "$OPT" && sed -i 's/^fullscreen:.*/fullscreen:false/' "$OPT" || echo "fullscreen:false" >> "$OPT"
grep -q "^graphicsMode:" "$OPT" && sed -i 's/^graphicsMode:.*/graphicsMode:0/' "$OPT" || echo "graphicsMode:0" >> "$OPT"
grep -q "^mipmapLevels:" "$OPT" && sed -i 's/^mipmapLevels:.*/mipmapLevels:0/' "$OPT" || echo "mipmapLevels:0" >> "$OPT"
grep -q "^entityShadows:" "$OPT" && sed -i 's/^entityShadows:.*/entityShadows:false/' "$OPT" || echo "entityShadows:false" >> "$OPT"
grep -q "^onboardAccessibility:" "$OPT" && sed -i 's/^onboardAccessibility:.*/onboardAccessibility:false/' "$OPT" || echo "onboardAccessibility:false" >> "$OPT"
grep -q "^tutorialStep:" "$OPT" && sed -i 's/^tutorialStep:.*/tutorialStep:none/' "$OPT" || echo "tutorialStep:none" >> "$OPT"
grep -q "^biomeBlendRadius:" "$OPT" && sed -i 's/^biomeBlendRadius:.*/biomeBlendRadius:0/' "$OPT" || echo "biomeBlendRadius:0" >> "$OPT"
grep -q "^simulationDistance:" "$OPT" && sed -i 's/^simulationDistance:.*/simulationDistance:6/' "$OPT" || echo "simulationDistance:6" >> "$OPT"

# Voxy config preseed (config/voxy.json, LOWER_CASE_WITH_UNDERSCORES):
# sectionRenderDistance default 16 sections x 64 blocks = 1024 blocks of LOD.
# 8 sections = 512 blocks — EXACTLY the c-LoD atlas window (±512), and halves
# the meshed-geometry footprint (live-OOM 2026-10-23: client hit 2.3GB RSS on
# the 4GB bench box next to the 1.2GB server and the OOM-killer took it).
CFG="$C/config/voxy-config.json"
mkdir -p "$C/config"
if [ ! -f "$CFG" ]; then
  cat > "$CFG" <<'EOF'
{
  "enabled": true,
  "enable_rendering": true,
  "ingest_enabled": true,
  "section_render_distance": 16.0,
  "service_threads": 2,
  "sub_division_size": 64
}
EOF
fi

# Fabric classpath = loader + mixin + asm, in front of the vanilla cp.
FAB_CP="$(ls "$C"/libraries/net/fabricmc/fabric-loader/*/fabric-loader-*.jar):$(ls "$C"/libraries/net/fabricmc/sponge-mixin/*/sponge-mixin-*.jar):$(ls "$C"/libraries/org/ow2/asm/asm/*/asm-*.jar):$(ls "$C"/libraries/org/ow2/asm/asm-analysis/*/asm-analysis-*.jar):$(ls "$C"/libraries/org/ow2/asm/asm-commons/*/asm-commons-*.jar):$(ls "$C"/libraries/org/ow2/asm/asm-tree/*/asm-tree-*.jar):$(ls "$C"/libraries/org/ow2/asm/asm-util/*/asm-util-*.jar)"

# Wipe the Mesa on-disk shader cache: OOM-killed clients leave stale locks /
# half-written entries and the next client's render thread SILENTLY freezes on
# the cache lock for minutes (live 21:34-21:39: log ends at the first UBO
# resize, then 2 min of nothing, then a clean disconnect).
# Fresh voxy DB: every join creates a NEW world-engine id, so the old DB
# data is orphaned; the accumulated stale pyramid data feeds garbage sizes
# into the AsyncNodeManager scratch-resize loop -> a giant llvmpipe memset
# that safepoint-stalls the whole JVM (live 21:34+: jcmd attach = connection
# reset, log ends at "Resizing scratch").
rm -rf "$C/.voxy/saves" 2>/dev/null || true
rm -rf "$HOME/.cache/mesa_shader_cache" "$HOME/.cache/mesa_shader_cache.db" 2>/dev/null || true

rm -f /tmp/voxy_client.log

export LP_NUM_THREADS=1
# -Xmx1024M -XX:MaxDirectMemorySize=128M: vanilla ran at 900M; Voxy adds a 512MB direct geometry buffer +
# GL copies (llvmpipe keeps them in system RAM) — 640M heap keeps the whole
# client ~1.6-1.8GB RSS so server+client fit the 4GB box (live-OOM above).
nohup "$JAVA" -Xmx1024M -XX:MaxDirectMemorySize=128M \
  -Djava.library.path="$C/natives" \
  -Dorg.lwjgl.librarypath="$C/natives" \
  -Dorg.lwjgl.opengl.libname=libGL.so.1 \
  -cp "$FAB_CP:$(cat $C/cp.txt)" \
  net.fabricmc.loader.impl.launch.knot.KnotClient \
  --username Stepan \
  --version 26.3 \
  --gameDir "$C" \
  --assetsDir "$C/assets" \
  --assetIndex 34 \
  --accessToken 0 \
  --userType legacy \
  --versionType release \
  --width 640 --height 360 \
  --quickPlayMultiplayer "127.0.0.1:${CLOD_VOXY_PORT:-25565}" \
  > /tmp/voxy_client.log 2>&1 &
CLI=$!
echo "voxy client pid $CLI (xvfb $XVFB)"
