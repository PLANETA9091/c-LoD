#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD v5 MAP CARRIER — the vanilla core/text fragment shader with one
// addition: fragments of the parked map quad (vClodMap == 1) skip the world
// fog so the scrape pass reads EXACT palette bytes. The OIT accumulation
// transform (when the frame routes text through order-independent
// transparency) is preserved — alpha is 1.0 on map quads, so the composite
// reproduces the texel exactly.

#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
#include <minecraft:fog.glsl>
#endif

#include <minecraft:dynamictransforms.glsl>
#include <minecraft:oit.glsl>

uniform sampler2D Sampler0;

#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
layout(location = 0) in float sphericalVertexDistance;
layout(location = 1) in float cylindricalVertexDistance;
#endif

layout(location = 2) in vec4 vertexColor;
layout(location = 3) in vec2 texCoord0;
#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
layout(location = 4) in float vClodMap;
#endif

#ifndef OIT_ALPHA_ONLY
layout(location = 0) out vec4 fragColor;
#endif

vec4 calculateFinalColor(vec4 color) {
    #ifdef OIT_ACCUMULATE
    color = sampleColorForAccumulation(color);
    #endif

    #if !defined(IS_SEE_THROUGH) && !defined(IS_GUI)

    #ifdef OIT_ACCUMULATE
    vec4 fogColor = vec4(FogColor.rgb * color.a, FogColor.a);
    #else
    vec4 fogColor = FogColor;
    #endif

    color = apply_fog(color, sphericalVertexDistance, cylindricalVertexDistance, FogEnvironmentalStart, FogEnvironmentalEnd, FogRenderDistanceStart, FogRenderDistanceEnd, fogColor);
    #endif

    return color;
}

void main() {
    #ifdef IS_GRAYSCALE
    vec4 texColor = texture(Sampler0, texCoord0).rrrr;
    #else
    vec4 texColor = texture(Sampler0, texCoord0);
    #endif

    vec4 color = texColor * vertexColor * ColorModulator;

    if (color.a < 0.1) {
        discard;
    }

    #ifdef OIT_ALPHA_ONLY
    executeAlphaOnlyPhase(gl_FragCoord.z, color.a);
    #else
    #if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
    if (vClodMap > 0.5) {
        // TEMPORARY BENCH PROBE: map fragments go solid magenta.
        #ifdef OIT_ACCUMULATE
        color = sampleColorForAccumulation(vec4(1.0, 0.0, 1.0, 1.0));
        fragColor = color;
        #else
        fragColor = vec4(1.0, 0.0, 1.0, 1.0);
        #endif
        return;
    }
    #endif
    fragColor = calculateFinalColor(color);
    #endif
}
