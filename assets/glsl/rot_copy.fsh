#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD pass: ping-pong copy for the persistent post-effect targets
// (clod_rot_b -> clod_rot_a and, since v4.12.0, clod_fp_b -> clod_fp_a and
// clod_hist_b -> clod_hist_a) so the next frame seeds from the newest solve
// without ever reading and writing the same target in one pass.
//
// PER-PIXEL copy — the pass is target-agnostic (2x1 rot, 1x1 fp, full-screen
// hist). It used to fetch texel (0,0) for BOTH fragments, which overwrote the
// day pixel with the camera azimuth every frame: the LOD was lit with a
// constant bogus dayF (stuck at its daylight look through dusk and night)
// while the camera channel kept working.

uniform sampler2D InSampler;

layout(std140) uniform SamplerInfo {
    vec2 OutSize;
    vec2 InSize;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

void main() {
    fragColor = texelFetch(InSampler, ivec2(gl_FragCoord.xy), 0);
}
