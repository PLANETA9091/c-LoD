#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD pass 6: composite the LOD frame back into minecraft:main.

uniform sampler2D InSampler;

layout(std140) uniform BlitConfig {
    vec4 ColorModulate;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

void main() {
    // v4.12.0: the march output (clod:clod_hist_b) carries the TAA history
    // flag in alpha (1 = march-written pixel, 0 = live vanilla content).
    // minecraft:main must stay opaque — force alpha back to 1 here.
    fragColor = vec4(texture(InSampler, texCoord).rgb, 1.0) * ColorModulate;
}
