#!/usr/bin/env bash
# Validates the c-LoD post-effect shaders with glslangValidator.
#
# The shaders are written for Minecraft 26.3 (shaderc-based compilation):
#   - `#include <minecraft:...>` is resolved by the game, so we replace the
#     known includes with exact stubs (Globals for post chains; Fog /
#     DynamicTransforms / Projection for the sky camera-channel override —
#     layouts extracted from client.jar). Shaderc accepts only #include,
#     the legacy #moj_import directive no longer exists in 26.3.
#   - Custom UBOs come from the chain JSON at runtime; the harness appends
#     declaration stubs collected from assets/post_effect/*.json.
#   - Post shaders get the screenquad texCoord varying; the sky override
#     (sky.vsh/sky.fsh) is compiled in its own vertex/fragment stages.
set -euo pipefail
cd "$(dirname "$0")/.."

command -v glslangValidator >/dev/null || {
  echo "glslangValidator not found" >&2
  exit 1
}

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

python3 - "$WORK" <<'PYEOF'
import json, re, sys
from pathlib import Path

work = Path(sys.argv[1])

GLOBALS_STUB = """layout(std140) uniform Globals {
    ivec3 CameraBlockPos;
    float GlintAlpha;
    vec3 CameraOffset;
    float GameTime;
    vec2 ScreenSize;
    int MenuBlurRadius;
    int UseRgss;
};"""

# Vanilla 26.3 core-shader include headers (extracted from client.jar) — the
# sky camera-channel override pulls these instead of the post-chain Globals.
FOG_STUB = """layout(std140) uniform Fog {
    vec4 FogColor;
    float FogEnvironmentalStart;
    float FogEnvironmentalEnd;
    float FogRenderDistanceStart;
    float FogRenderDistanceEnd;
    float FogSkyEnd;
    float FogCloudsEnd;
};
float linear_fog_value(float vertexDistance, float fogStart, float fogEnd) {
    if (vertexDistance <= fogStart) { return 0.0; }
    else if (vertexDistance >= fogEnd) { return 1.0; }
    return (vertexDistance - fogStart) / (fogEnd - fogStart);
}
float total_fog_value(float sphericalVertexDistance, float cylindricalVertexDistance, float environmentalStart, float environmantalEnd, float renderDistanceStart, float renderDistanceEnd) {
    return max(linear_fog_value(sphericalVertexDistance, environmentalStart, environmantalEnd), linear_fog_value(cylindricalVertexDistance, renderDistanceStart, renderDistanceEnd));
}
vec4 apply_fog(vec4 inColor, float sphericalVertexDistance, float cylindricalVertexDistance, float environmentalStart, float environmantalEnd, float renderDistanceStart, float renderDistanceEnd, vec4 fogColor) {
    float fogValue = total_fog_value(sphericalVertexDistance, cylindricalVertexDistance, environmentalStart, environmantalEnd, renderDistanceStart, renderDistanceEnd);
    return vec4(mix(inColor.rgb, fogColor.rgb, fogValue * fogColor.a), inColor.a);
}
float fog_spherical_distance(vec3 pos) { return length(pos); }
float fog_cylindrical_distance(vec3 pos) { return max(length(pos.xz), abs(pos.y)); }"""

DYNAMIC_TRANSFORMS_STUB = """layout(std140) uniform DynamicTransforms {
    mat4 ModelViewMat;
    mat4 TextureMat;
    vec4 ColorModulator;
    vec3 ModelOffset;
};"""

PROJECTION_STUB = """layout(std140) uniform Projection {
    mat4 ProjMat;
};"""

INCLUDES = {
    "#include <minecraft:globals.glsl>": GLOBALS_STUB,
    "#include <minecraft:fog.glsl>": FOG_STUB,
    "#include <minecraft:dynamictransforms.glsl>": DYNAMIC_TRANSFORMS_STUB,
    "#include <minecraft:projection.glsl>": PROJECTION_STUB,
}

# Vanilla 26.3 include headers (extracted from client.jar) — the sky override
# pulls them in instead of the post-chain Globals UBO.

def load_chain(p: Path):
    # Templates contain __TOKEN__ placeholders (substituted by packgen);
    # replace them with 0 for validation.
    txt = re.sub(r"__[A-Z0-9_]+__", "0", p.read_text())
    return json.loads(txt)

ubos = {}
for chain in Path("assets/post_effect").glob("*.json"):
    j = load_chain(chain)
    for p in j.get("passes", []):
        for name, uniforms in (p.get("uniforms") or {}).items():
            ubos[name] = [(u["name"], u["type"]) for u in uniforms]

for fsh in sorted(list(Path("assets/glsl").glob("*.fsh")) + list(Path("assets/glsl").glob("*.vsh"))):
    src = fsh.read_text()
    for inc, stub in INCLUDES.items():
        src = src.replace(inc, stub)
    # Shader templates carry __TOKEN__ placeholders too (e.g. __DEBUG__);
    # substitute 0 exactly like packgen does for validation.
    src = re.sub(r"__[A-Z0-9_]+__", "0", src)
    # Legacy directive (pre-26.3): reject loudly if it sneaks back in.
    if "#moj_import" in src:
        print(f"FAIL {fsh.name}: #moj_import is not supported by the 26.3 shaderc pipeline")
        sys.exit(1)

    # Harness-only: binding layout qualifier needs 420pack under GL semantics.
    src = src.replace(
        "#version 330",
        "#version 330\n#extension GL_ARB_shading_language_420pack : enable",
        1,
    )

    # glslang -G (SPIR-V) demands explicit bindings; Minecraft assigns them
    # at runtime, so the harness auto-numbers UBO blocks.
    seen = {}
    def add_binding(m):
        name = m.group(1)
        if name not in seen:
            seen[name] = len(seen) + 1
        return f"layout(std140, binding={seen[name]}) uniform {name}"
    src = re.sub(r"layout\(std140\) uniform (\w+)", add_binding, src)

    declared = set(re.findall(r"layout\(std140, binding=\d+\) uniform (\w+)", src))
    used = set(re.findall(r"uniform\s+(\w+)\s*\{", src))
    extra = []
    for name in sorted(used - declared):
        if name not in ubos:
            print(f"FAIL {fsh.name}: UBO {name} neither declared nor in chain JSONs")
            sys.exit(1)
        binding = len(seen) + 1
        seen[name] = binding
        lines = [f"layout(std140, binding={binding}) uniform {name} {{"]
        for fname, ftype in ubos[name]:
            lines.append(f"    {ftype} {fname};")
        lines.append("};")
        extra.append("\n".join(lines))

    # texCoord is the varying produced by vanilla core/screenquad.vsh — only
    # post shaders need it; the sky override has its own varyings (and the
    # vsh runs its own stage).
    if not re.search(r"layout\(location = 0\) in", src):
        extra.append("layout(location = 0) in vec2 texCoord;")
    ext = "vert" if fsh.suffix == ".vsh" else "frag"
    (work / (fsh.stem + ".comp." + ext)).write_text(src + "\n" + "\n".join(extra) + "\n")

# ---------------------------------------------------------------------------
# v4.3.4: the vanilla core-shader overrides (assets/mc/) are validated against
# the REAL 26.3 include headers, extracted from the committed client.jar —
# these files replace vanilla sources at runtime, so any drift from the true
# interface (UBO layouts, varyings, helper functions) would kill the whole
# terrain pipeline on the client. Every pipeline variant the game compiles is
# checked: multidraw, per-section UBO, alpha-cutout, and the OIT stages.
# ---------------------------------------------------------------------------
inc_root = work / "inc_root"
JAR = Path("analysis/mc263/client.jar")
if not JAR.exists():
    print(f"FAIL mc overrides: {JAR} missing (committed reference jar)")
    sys.exit(1)
import subprocess
subprocess.run(
    ["unzip", "-o", "-q", str(JAR), "assets/minecraft/shaders/include/*",
     "-d", str(inc_root)],
    check=True,
)
inc_dir = inc_root / "assets" / "minecraft" / "shaders" / "include"

def resolve_includes(src: str, stack: list) -> str:
    out = []
    for line in src.splitlines():
        m = re.match(r"\s*#include <minecraft:([A-Za-z0-9_.]+)>\s*$", line)
        if m:
            name = m.group(1)
            if name in stack:
                print(f"FAIL mc overrides: include cycle at {name}")
                sys.exit(1)
            p = inc_dir / name
            if not p.exists():
                print(f"FAIL mc overrides: vanilla include {name} not in jar")
                sys.exit(1)
            out.append(resolve_includes(p.read_text(), stack + [name]))
        else:
            out.append(line)
    return "\n".join(out)

# stage -> (file, defines) matrix. "" = the plain per-section-UBO variant.
MC_VARIANTS = {
    "terrain.vsh": ["MULTIDRAW_TERRAIN", ""],
    "terrain.fsh": ["MULTIDRAW_TERRAIN", "OIT_ALPHA_ONLY",
                    "OIT OIT_ACCUMULATE OIT_COEFF_COUNT=8 OIT_WAVELET_RANK=3"],
    "block.vsh": [""],
    "block.fsh": ["", "OIT OIT_ACCUMULATE OIT_COEFF_COUNT=8 OIT_WAVELET_RANK=3"],
}

for fname, variants in MC_VARIANTS.items():
    fsh = Path("assets/mc") / fname
    raw = fsh.read_text()
    if "#moj_import" in raw:
        print(f"FAIL {fsh}: #moj_import is not supported by the 26.3 pipeline")
        sys.exit(1)
    for vi, defines in enumerate(variants):
        src = resolve_includes(raw, [])
        src = re.sub(r"__[A-Z0-9_]+__", "0", src)
        src = src.replace(
            "#version 330",
            "#version 330\n#extension GL_ARB_shading_language_420pack : enable",
            1,
        )
        seen = {}
        def add_binding(m):
            name = m.group(1)
            if name not in seen:
                seen[name] = len(seen) + 1
            return f"layout(std140, binding={seen[name]}) uniform {name}"
        src = re.sub(r"layout\(std140\) uniform (\w+)", add_binding, src)
        dflags = " ".join(f"-D{d}" for d in defines.split()) if defines else ""
        tag = defines.replace(" ", "_") if defines else "plain"
        ext = "vert" if fsh.suffix == ".vsh" else "frag"
        out = work / f"mc__{fsh.name}__{tag}.comp.{ext}"
        out.write_text(src + "\n")
        # stash the define flags for the validation loop
        (work / f"mc__{fsh.name}__{tag}.defines").write_text(dflags)
PYEOF

status=0
for comp in "$WORK"/*.comp.frag "$WORK"/*.comp.vert; do
    [ -e "$comp" ] || continue
    base=$(basename "$comp")
    stage=frag
    case "$comp" in
        *.vert) stage=vert ;;
    esac
    # mc override variants carry their pipeline defines next to the source.
    dflags=""
    if [ -f "$WORK/${base%.comp.*}.defines" ]; then
        dflags=$(cat "$WORK/${base%.comp.*}.defines")
        label="assets/mc/${base#mc__}"
        label="${label%.comp.*}"
        label="${label/__/ [}"
        label="$label]"
    else
        label="assets/glsl/${base%.comp.*}"
    fi
    if glslangValidator -S "$stage" -G $dflags -o /dev/null "$comp" > "$WORK/$base.log" 2>&1; then
        echo "OK   $label"
    else
        echo "FAIL $label"
        cat "$WORK/$base.log"
        status=1
    fi
done

# Chain JSONs must be structurally valid (templates with __TOKEN__ placeholders).
python3 - <<'PYEOF'
import json, re, sys
from pathlib import Path
ok = True
for chain in sorted(Path("assets/post_effect").glob("*.json")):
    try:
        txt = re.sub(r"__[A-Z0-9_]+__", "0", chain.read_text())
        j = json.loads(txt)
        assert "passes" in j and "targets" in j
        for p in j["passes"]:
            assert "vertex_shader" in p and "fragment_shader" in p
            assert "output" in p and "inputs" in p
        print(f"OK   {chain}")
    except Exception as e:
        print(f"FAIL {chain}: {e}")
        ok = False
sys.exit(0 if ok else 1)
PYEOF

[ "$status" -eq 0 ] && echo "all shaders valid"
exit "$status"
