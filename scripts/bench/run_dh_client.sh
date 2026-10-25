#!/usr/bin/env bash
# DH reference client: real Distant Horizons 3.3.4-26.3 (Fabric 0.20.1) on the
# same bench (seed 4819696040575474753, user camera). Used ONLY to produce
# reference screenshots for the DH-exact calibration of the posteffect LOD.
# Logs to /tmp/dh_client.log, DISPLAY :99.
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
export MESA_GL_VERSION_OVERRIDE=4.5
export __GLX_VENDOR_LIBRARY_NAME=mesa
export GLX_VENDOR_LIBRARY_NAME=mesa
export LD_LIBRARY_PATH="$HOME/mc-test/shim${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

cd "$C"
mkdir -p screenshots mods

# mods/ already carries DistantHorizons-3.3.4-26.3.jar + fabric-api.jar
ls mods/*.jar >/dev/null || { echo "mods/ empty — place DH + fabric-api jars" >&2; exit 1; }

OPT="$C/options.txt"
touch "$OPT"; chmod u+w "$OPT"
# Same visual base as the c-LoD runs (RD 6 = identical real-chunk ring).
grep -q "^preferredGraphicsBackend:" "$OPT" && sed -i 's/^preferredGraphicsBackend:.*/preferredGraphicsBackend:opengl/' "$OPT" || echo "preferredGraphicsBackend:opengl" >> "$OPT"
grep -q "^pauseOnLostFocus:" "$OPT" && sed -i 's/^pauseOnLostFocus:.*/pauseOnLostFocus:false/' "$OPT" || echo "pauseOnLostFocus:false" >> "$OPT"
grep -q "^renderDistance:" "$OPT" && sed -i 's/^renderDistance:.*/renderDistance:6/' "$OPT" || echo "renderDistance:6" >> "$OPT"
grep -q "^fullscreen:" "$OPT" && sed -i 's/^fullscreen:.*/fullscreen:false/' "$OPT" || echo "fullscreen:false" >> "$OPT"
grep -q "^graphicsMode:" "$OPT" && sed -i 's/^graphicsMode:.*/graphicsMode:0/' "$OPT" || echo "graphicsMode:0" >> "$OPT"
grep -q "^mipmapLevels:" "$OPT" && sed -i 's/^mipmapLevels:.*/mipmapLevels:0/' "$OPT" || echo "mipmapLevels:0" >> "$OPT"
grep -q "^entityShadows:" "$OPT" && sed -i 's/^entityShadows:.*/entityShadows:false/' "$OPT" || echo "entityShadows:false" >> "$OPT"
grep -q "^onboardAccessibility:" "$OPT" && sed -i 's/^onboardAccessibility:.*/onboardAccessibility:false/' "$OPT" || echo "onboardAccessibility:false" >> "$OPT"
grep -q "^tutorialStep:" "$OPT" && sed -i 's/^tutorialStep:.*/tutorialStep:none/' "$OPT" || echo "tutorialStep:none" >> "$OPT"
grep -q "^biomeBlendRadius:" "$OPT" && sed -i 's/^biomeBlendRadius:.*/biomeBlendRadius:0/' "$OPT" || echo "biomeBlendRadius:0" >> "$OPT"
grep -q "^simulationDistance:" "$OPT" && sed -i 's/^simulationDistance:.*/simulationDistance:6/' "$OPT" || echo "simulationDistance:6" >> "$OPT"

# Fabric classpath = loader + mixin + asm, in front of the vanilla cp.
FAB_CP="$(ls "$C"/libraries/net/fabricmc/fabric-loader/*/fabric-loader-*.jar):$(ls "$C"/libraries/net/fabricmc/sponge-mixin/*/sponge-mixin-*.jar):$(ls "$C"/libraries/org/ow2/asm/asm/*/asm-*.jar):$(ls "$C"/libraries/org/ow2/asm/asm-analysis/*/asm-analysis-*.jar):$(ls "$C"/libraries/org/ow2/asm/asm-commons/*/asm-commons-*.jar):$(ls "$C"/libraries/org/ow2/asm/asm-tree/*/asm-tree-*.jar):$(ls "$C"/libraries/org/ow2/asm/asm-util/*/asm-util-*.jar)"

export LP_NUM_THREADS=2
nohup "$JAVA" -Xmx700M \
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
  --quickPlayMultiplayer "127.0.0.1:${CLOD_DH_PORT:-25565}" \
  > /tmp/dh_client.log 2>&1 &
CLI=$!
echo "dh client pid $CLI (xvfb $XVFB)"
