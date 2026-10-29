#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD v5.2.3 pass 5b: DUST CONCEALMENT (eye-side).
//
// The capture passes earlier in this chain decoded the sky-grid words out
// of minecraft:main — they NEED the bright villager-green carrier. The
// eye does not: the user requirement is that the carrier (and the dotted
// sheet it forms when seen edge-on from above/below) is invisible from
// every vantage, including while the camera moves. This pass runs after
// the LAST capture pass and before the march: it finds carrier-green
// pixels and fills them with the average of the non-green pixels around
// them (a 5x5 local inpaint).
//
// Why this is safe for the stream:
//   * the decode already happened this frame — capture passes read
//     minecraft:main BEFORE this pass; this pass writes to a side target
//     (clod_conceal_swap) that only the march consumes, so minecraft:main
//     itself is never modified and there is no feedback loop into the
//     next frame's strobe;
//   * the depth buffer is not touched (color-only pass), so the march's
//     sky-pixel depth rule and the capture's in-front-of-geometry rule
//     behave exactly as before;
//   * the mask is same-frame and stateless — full camera motion, fov
//     changes and teleporting cannot desync it from the dust.
//
// Mask thresholds are WIDER than the capture strobe (0.45 / +0.10) so the
// soft sprite fringe blends away too. False positives on greenish terrain
// are harmless: the fill comes from the same neighborhood, so a grass
// pixel is replaced by nearby grass colors and nothing visible changes.
// The sky camera-channel alpha (columns 0..15) is passed through
// untouched: only the RGB channels are repainted.

uniform sampler2D InSampler; // minecraft:main (dust still present)

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

// Villager-sparkle carrier: G dominant over BOTH R and B. Looser than the
// capture strobe to also catch the blended sprite edges.
bool is_dust(vec3 c) {
    return c.g > 0.32 && c.g > c.r + 0.06 && c.g > c.b + 0.06;
}

void main() {
    ivec2 sz = ivec2(max(ScreenSize, vec2(1.0)));
    ivec2 px = ivec2(clamp(texCoord, 0.0, 1.0) * vec2(sz));
    px = clamp(px, ivec2(0), sz - 1);

    vec4 c0 = texelFetch(InSampler, px, 0);
    if (!is_dust(c0.rgb)) {
        fragColor = c0;
        return;
    }

    // Local background estimate: mean of the non-dust samples in 5x5.
    vec3 sum = vec3(0.0);
    int n = 0;
    for (int dy = -2; dy <= 2; ++dy) {
        for (int dx = -2; dx <= 2; ++dx) {
            if (dx == 0 && dy == 0) {
                continue;
            }
            ivec2 q = clamp(px + ivec2(dx, dy), ivec2(0), sz - 1);
            vec4 s = texelFetch(InSampler, q, 0);
            if (!is_dust(s.rgb)) {
                sum += s.rgb;
                ++n;
            }
        }
    }
    float a = c0.a;
    fragColor = vec4(n > 0 ? sum / float(n) : c0.rgb, a);
}
