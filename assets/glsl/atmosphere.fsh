#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// NOTE: post pipelines cannot declare the vanilla Fog UBO — PostChain builds
// the bind group layout from inputs + SamplerInfo + JSON uniforms only
// (BindGroupLayouts.FOG is world-pipeline-only), and an undeclared uniform
// fails the whole pack ("Unable to find shader defined uniform (Fog)"). The
// day grade below samples the frame's own sky instead.

// c-LoD atmosphere: a distance veil over real terrain that hides the
// render-distance edge. Pure depth-driven (no view rotation needed), so it
// works on every frame, before the LOD chain ever sees the sun.
//
// Depth thresholds are uniforms rather than reconstructed distances: the
// device->eye mapping differs between renderpearl backends, and a monotonic
// smoothstep over device depth is backend-independent and fully tunable.
// The veil color tracks the day cycle through the frame's own sky (see the
// dayGrade function — Globals.GameTime is wrong in 26.3, see lod_terrain).

uniform sampler2D InSampler;      // minecraft:main color
uniform sampler2D InDepthSampler; // minecraft:main depth

layout(std140) uniform SamplerInfo {
    vec2 OutSize;
    vec2 InSize;
};

// x = veil start (blocks), y = veil end (blocks), z = strength, w = unused
layout(std140) uniform ClodAtmo {
    vec4 AtmoParams;
};

// 26.3 device-depth conversion: d = near/eyeZ (infinite far), so eye-space
// BLOCK distance = DepthNear / max(d, 1e-9). The sky and the sun/moon quads
// sit at EXACTLY 0.0 and must never be veiled — veiling them starves the
// sun-tracker in the LOD chain (live-debugged on a real 26.3 client).
layout(std140) uniform ClodDepth {
    vec4 DepthParams; // x = depth near (blocks), yzw = unused
};

layout(std140) uniform ClodAtmoColor {
    vec4 VeilColor;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float SKY_DEPTH = 1.0e-6;

// Day-grade window (matches rot_solve's DAY_LO/DAY_HI so both chains grade
// in sync): night sky ~0.03-0.06 luminance, day >= 0.45.
const float DAY_LO = 0.09;
const float DAY_HI = 0.30;

// Day factor from the frame's own sky. Six fixed screen points; keep only
// SKY ones (device depth 0.0) and take the MINIMUM luminance — the min
// kills stars and the moon disk (only 1-2 of the points can ever land on
// the moon, and a daytime sun point loses the min against the dark-blue
// rest). No sky at all (wall view): fall back to the min over the samples
// themselves (terrain is dark at night, bright by day).
float dayGrade(sampler2D col, sampler2D dep) {
    vec2 pts[6] = vec2[6](
        vec2(0.04, 0.04), vec2(0.50, 0.04), vec2(0.96, 0.04),
        vec2(0.04, 0.96), vec2(0.96, 0.96), vec2(0.50, 0.50)
    );
    float minSky = 1.0e9;
    float minAll = 1.0e9;
    bool anySky = false;
    for (int i = 0; i < 6; ++i) {
        vec3 c = texture(col, pts[i]).rgb;
        float l = dot(c, vec3(0.299, 0.587, 0.114));
        minAll = min(minAll, l);
        if (texture(dep, pts[i]).r <= SKY_DEPTH) {
            anySky = true;
            minSky = min(minSky, l);
        }
    }
    float l = anySky ? minSky : minAll;
    return smoothstep(DAY_LO, DAY_HI, l);
}

void main() {
    vec4 scene = texture(InSampler, texCoord);
    float depth = texture(InDepthSampler, texCoord).r;

    // Sky AND celestial bodies stay vanilla: they sit at device depth 0.0
    // (infinite far), while ANY real geometry reads positive depth.
    if (depth < SKY_DEPTH || depth <= 0.0) {
        fragColor = scene;
        return;
    }

    // Device depth -> eye-space blocks, then a plain distance veil.
    float eyeZ = DepthParams.x / max(depth, 1.0e-9);

    // Day-graded veil: bright and neutral by day, darker moonlit blue at
    // night, so the edge never glows against a dark sky. The grade comes
    // from the frame's own sky (see dayGrade) — clock-correct even after
    // beds//time set, which the old Globals.GameTime estimate was not.
    float dayF = dayGrade(InSampler, InDepthSampler);
    vec3 veilCol = VeilColor.rgb * mix(0.42, 1.0, dayF);
    veilCol = mix(veilCol * vec3(0.85, 0.9, 1.12), veilCol, dayF);

    float veil = smoothstep(AtmoParams.x, AtmoParams.y, eyeZ) * AtmoParams.z;
    fragColor = vec4(mix(scene.rgb, veilCol, clamp(veil, 0.0, 1.0)), scene.a);
}
