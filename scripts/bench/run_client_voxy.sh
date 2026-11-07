#!/usr/bin/env bash
# Launch the Fabric 26.3 client (Fabric API + Voxy 0.2.20-beta) under Xvfb and
# auto-join the REFERENCE server. This is the Voxy-visual-reference leg: the
# client carries NO c-LoD pack — Voxy's own renderer (octree quads + SSAO +
# vanilla fog) is the ground truth the c-LoD marcher is judged against.
# Usage: run_client_voxy.sh   — logs to /tmp/voxy_client.log, DISPLAY :99
set -euo pipefail
C="$HOME/mc-test/client"
JAVA="${CLOD_JAVA:-$HOME/tools/jdk25/bin/java}"

pkill -f "KnotClient" 2>/dev/null || true
pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
pkill Xvfb 2>/dev/null || true
sleep 1

Xvfb :99 -screen 0 1280x720x24 +extension GLX +render -noreset > /tmp/xvfb.log 2>&1 &
XVFB=$!
sleep 2

export DISPLAY=:99
export LIBGL_ALWAYS_SOFTWARE=1
# Voxy needs GL4.6 (MDIC + compute HiZ/SSAO). llvmpipe reports 4.5 core in
# older Mesa; the override asks for 4.6 and the client log prints the real
# string — if a feature is missing Voxy logs its own fatal and we see it.
export MESA_GL_VERSION_OVERRIDE=4.6
# llvmpipe caps its GLSL compiler at 4.50 while the GL version string says 4.6
# (Mesa 25.0.7) — Voxy's #version 460 shaders then fail to compile. GLSL
# 4.50 -> 4.60 adds no language features we use; force the compiler up.
export MESA_GLSL_VERSION_OVERRIDE=460
export __GLX_VENDOR_LIBRARY_NAME=mesa
export GLX_VENDOR_LIBRARY_NAME=mesa
# same sRGB-less GLX shim as the vanilla leg (renderpearl SDL sRGB request)
export LD_LIBRARY_PATH="$HOME/mc-test/shim${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

cd "$C"
mkdir -p screenshots

OPT="$C/options.txt"
touch "$OPT"; chmod u+w "$OPT"
grep -q "^preferredGraphicsBackend:" "$OPT" && sed -i 's/^preferredGraphicsBackend:.*/preferredGraphicsBackend:opengl/' "$OPT" || echo "preferredGraphicsBackend:opengl" >> "$OPT"
grep -q "^pauseOnLostFocus:" "$OPT" && sed -i 's/^pauseOnLostFocus:.*/pauseOnLostFocus:false/' "$OPT" || echo "pauseOnLostFocus:false" >> "$OPT"
grep -q "^renderDistance:" "$OPT" && sed -i 's/^renderDistance:.*/renderDistance:8/' "$OPT" || echo "renderDistance:8" >> "$OPT"
grep -q "^fullscreen:" "$OPT" && sed -i 's/^fullscreen:.*/fullscreen:false/' "$OPT" || echo "fullscreen:false" >> "$OPT"
grep -q "^graphicsMode:" "$OPT" && sed -i 's/^graphicsMode:.*/graphicsMode:0/' "$OPT" || echo "graphicsMode:0" >> "$OPT"
grep -q "^mipmapLevels:" "$OPT" && sed -i 's/^mipmapLevels:.*/mipmapLevels:4/' "$OPT" || echo "mipmapLevels:4" >> "$OPT"
# v6.17 PARITY: the c-LoD leg pins particles:2 (ALL — the v6.10.0 lesson:
# MINIMAL culls non-forced particles and censors the bursts). Pin the SAME
# value here so the reference client spawns vanilla particles under the
# identical rules — a missing pin is a hidden asymmetry the gate would
# measure as renderer diff.
grep -q "^particles:" "$OPT" && sed -i 's/^particles:.*/particles:2/' "$OPT" || echo "particles:2" >> "$OPT"
grep -q "^entityShadows:" "$OPT" && sed -i 's/^entityShadows:.*/entityShadows:false/' "$OPT" || echo "entityShadows:false" >> "$OPT"
grep -q "^onboardAccessibility:" "$OPT" && sed -i 's/^onboardAccessibility:.*/onboardAccessibility:false/' "$OPT" || echo "onboardAccessibility:false" >> "$OPT"
grep -q "^tutorialStep:" "$OPT" && sed -i 's/^tutorialStep:.*/tutorialStep:none/' "$OPT" || echo "tutorialStep:none" >> "$OPT"
grep -q "^biomeBlendRadius:" "$OPT" && sed -i 's/^biomeBlendRadius:.*/biomeBlendRadius:0/' "$OPT" || echo "biomeBlendRadius:0" >> "$OPT"
grep -q "^simulationDistance:" "$OPT" && sed -i 's/^simulationDistance:.*/simulationDistance:4/' "$OPT" || echo "simulationDistance:4" >> "$OPT"
grep -q "^renderClouds:" "$OPT" && sed -i 's/^renderClouds:.*/renderClouds:false/' "$OPT" || echo "renderClouds:false" >> "$OPT"

export LP_NUM_THREADS=1
export MALLOC_ARENA_MAX=2

# Voxy renders its own frame — the log4j config stays INFO (renderer DEBUG
# noise from two render systems would drown the join markers).
LOG4J_CFG="$C/log4j2-voxy.xml"
if [ ! -f "$LOG4J_CFG" ]; then
cat > "$LOG4J_CFG" <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<Configuration status="WARN">
    <Appenders>
        <Console name="Console" target="SYSTEM_OUT">
            <PatternLayout pattern="[%d{HH:mm:ss}] [%t/%level]: %msg%n%throwable"/>
        </Console>
    </Appenders>
    <Loggers>
        <Logger name="me.cortex.voxy" level="DEBUG"/>
        <Root level="INFO">
            <AppenderRef ref="Console"/>
        </Root>
    </Loggers>
</Configuration>
EOF
fi

# Fabric production launch: KnotClient main + the fabric-loader libraries on
# the classpath. Knot runtime-remaps the vanilla game jar to intermediary on
# first boot (cached under .fabric/). Vanilla libraries come from cp.txt
# (setup_client.py); fabric libs from the installer layout.
FABRIC_CP="$C/libraries/net/fabricmc/fabric-loader/0.20.1/fabric-loader-0.20.1.jar"
FABRIC_CP="$FABRIC_CP:$C/libraries/net/fabricmc/sponge-mixin/0.17.5+mixin.0.8.7/sponge-mixin-0.17.5+mixin.0.8.7.jar"
for a in asm asm-analysis asm-commons asm-tree asm-util; do
  FABRIC_CP="$FABRIC_CP:$C/libraries/org/ow2/asm/$a/9.10.1/$a-9.10.1.jar"
done

nohup "$JAVA" -Xmx"${VOXY_CLIENT_XMX:-700M}" -XX:MaxDirectMemorySize="${VOXY_CLIENT_DIRECT:-192M}" -XX:+UseSerialGC \
  -Djava.library.path="$C/natives" \
  -Dorg.lwjgl.librarypath="$C/natives" \
  -Dorg.lwjgl.opengl.libname=libGL.so.1 \
  -Dlog4j.configurationFile="file:$LOG4J_CFG" \
  -cp "$FABRIC_CP:$(cat $C/cp.txt)" \
  net.fabricmc.loader.impl.launch.knot.KnotClient \
  --username Stepan \
  --version fabric-loader-0.20.1-26.3 \
  --gameDir "$C" \
  --assetsDir "$C/assets" \
  --assetIndex 34 \
  --accessToken 0 \
  --userType legacy \
  --versionType release \
  --width 1280 --height 720 \
  --quickPlayMultiplayer "127.0.0.1:25565" \
  > /tmp/voxy_client.log 2>&1 &
CLI=$!
echo "voxy client pid $CLI (xvfb $XVFB)"
