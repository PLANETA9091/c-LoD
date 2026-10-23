#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// Vanilla Fog UBO (assets/minecraft/shaders/include/fog.glsl, layout
// verbatim). The earlier "post chains receive no Fog UBO" note was WRONG:
// RenderSystem.bindDefaultUniforms sets "Fog" on every pass and PostPass
// (PostPass.java:150, 26.3) calls it for each post pass; LevelRenderer sets
// the terrain fog BEFORE applyPostEffects runs. Live-verified reasoning in
// the 26.3 decompile, v4.4.2.
layout(std140) uniform Fog {
    vec4 FogColor;
    float FogEnvironmentalStart;
    float FogEnvironmentalEnd;
    float FogRenderDistanceStart;
    float FogRenderDistanceEnd;
    float FogSkyEnd;
    float FogCloudsEnd;
};

// c-LoD atmosphere: a distance veil over real terrain that hides the
// render-distance edge. Pure depth-driven (no view rotation needed), so it
// works on every frame, before the LOD chain ever sees the sun.
//
// Depth thresholds are uniforms rather than reconstructed distances: the
// device->eye mapping differs between renderpearl backends, and a monotonic
// smoothstep over device depth is backend-independent and fully tunable.
// The veil color tracks the day cycle through FogColor (same source as the
// LOD pass — see lod_terrain.fsh for why GameTime is wrong in 26.3).

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
    // night, so the edge never glows against a dark sky. FogColor is the
    // live world fog (sun-correct clock in 26.3); the all-zero fallback
    // covers a hypothetically unbound UBO.
    float fogLum = dot(FogColor.rgb, vec3(0.299, 0.587, 0.114));
    float dayF;
    if (all(equal(FogColor, vec4(0.0)))) {
        float sunY = sin(fract(GameTime) * 6.28318530718);
        dayF = smoothstep(-0.14, 0.22, sunY);
    } else {
        dayF = smoothstep(0.09, 0.30, fogLum);
    }
    vec3 veilCol = VeilColor.rgb * mix(0.42, 1.0, dayF);
    veilCol = mix(veilCol * vec3(0.85, 0.9, 1.12), veilCol, dayF);

    float veil = smoothstep(AtmoParams.x, AtmoParams.y, eyeZ) * AtmoParams.z;
    fragColor = vec4(mix(scene.rgb, veilCol, clamp(veil, 0.0, 1.0)), scene.a);
}
