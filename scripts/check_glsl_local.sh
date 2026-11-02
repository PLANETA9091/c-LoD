#!/usr/bin/env bash
# GLSL validation for c-LoD post shaders with glslangValidator (same GLSL
# front-end family as Minecraft 26.3's shaderc).
#
# The game resolves `#include <minecraft:...>`; here the known includes are
# replaced with exact stubs (Globals: ScreenSize/CameraBlockPos/CameraOffset;
# the chain's custom UBOs from assets/post_effect/*.json).
set -euo pipefail
cd "$(dirname "$0")/.."

if ! command -v glslangValidator >/dev/null; then
    GLSLANG=/home/z/tools/glslang/usr/bin/glslangValidator
else
    GLSLANG=glslangValidator
fi
command -v "$GLSLANG" >/dev/null 2>&1 || "$GLSLANG" --version >/dev/null || {
    echo "glslangValidator not found" >&2
    exit 1
}

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

for fsh in assets/glsl/lod_terrain.fsh assets/glsl/capture.fsh assets/glsl/col_scan.fsh assets/glsl/rot_solve.fsh assets/glsl/atmosphere.fsh; do
    name=$(basename "$fsh" .fsh)
    python3 - "$fsh" "$WORK/$name.frag" <<'PYEOF'
import re
import sys
src = open(sys.argv[1]).read()
# Globals stub (the game-provided minecraft:globals.glsl) ...
globals = '''uniform vec2 ScreenSize;
uniform vec4 CameraBlockPos;
uniform vec3 CameraOffset;
uniform float GameTime;
'''
# ... plus chain UBOs, but ONLY the ones the shader does not declare itself
# (glslang rejects duplicate block names within one interface).
blocks = {
    'ClodGrid':   ('GridInfo',  'layout(std140) uniform ClodGrid { vec4 GridInfo; };\n'),
    'ClodAtlas':  ('AtlasInfo', 'layout(std140) uniform ClodAtlas { vec4 AtlasInfo; };\n'),
    'ClodParams': ('LodParams', 'layout(std140) uniform ClodParams { vec4 LodParams; };\n'),
    'ClodFog':    ('FogParams', 'layout(std140) uniform ClodFog { vec4 FogParams; };\n'),
    'ClodColors': ('VeilColor', 'layout(std140) uniform ClodColors { vec4 ColLow; vec4 ColHigh; vec4 ColWater; vec4 VeilColor; };\n'),
    'ClodStream': ('StreamInfo','layout(std140) uniform ClodStream { vec4 StreamInfo; };\n'),
}
# skip a stub block if the shader already declares its block name OR any
# of its member names (glslang rejects duplicate globals)
stubs = globals + ''.join(b for n, (m, b) in blocks.items()
                          if n not in src and not re.search(r'\b' + m + r'\b', src))
open(sys.argv[2], 'w').write(src.replace('#include <minecraft:globals.glsl>', stubs))
PYEOF
    if "$GLSLANG" -S frag "$WORK/$name.frag" 2>"$WORK/$name.err"; then
        echo "OK   $fsh"
    else
        echo "FAIL $fsh"
        cat "$WORK/$name.err"
        exit 1
    fi
done
