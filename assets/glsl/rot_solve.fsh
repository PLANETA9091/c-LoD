#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD pass 2 (v4.3): decode the camera channel into the persistent
// rotation target.
//
// Picks the topmost intact payload row across all col_scan slices and packs
// the raw sky.fsh payload bytes into clod_rot_b (1x5, RGBA8), which the
// ping-pong copy promotes to clod_rot_a for clod:post/lod_terrain:
//
//   px0 = (Rx_hi, Rx_lo, Ry_hi, Ry_lo)   bobbed world-space right
//   px1 = (Rz_hi, Rz_lo, Ux_hi, Ux_lo)
//   px2 = (Uy_hi, Uy_lo, Uz_hi, Uz_lo)   bobbed world-space up
//   px3 = (fov_hi, fov_lo, tx, ty)       fov 16-bit, eye wobble 8-bit
//   px4 = (conf, 0, 0, 0)
//
// The payload is stored ENCODED (not decoded to floats): the persistent
// target is RGBA8 and 16-bit basis components must survive the round trip
// bit-exact or the LOD horizon would swim by half a pixel. The LOD pass
// decodes. Confidence jumps straight to 1 on the first valid row (no
// blending, no warm-up: blending would make the LOD horizon swim behind the
// true one); when the column is fully occluded the previous payload is kept
// and confidence decays at 0.99/frame (~3 s), so the LOD fades instead of
// popping.

uniform sampler2D ScanSampler; // clod:clod_scan (640x1 slice report)
uniform sampler2D InSampler;   // minecraft:main (payload rows)
uniform sampler2D PrevSampler; // clod:clod_rot_a (1x5 persistent)

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const int SLICES = 640;

void main() {
    int j = int(gl_FragCoord.x); // 0..4, one output pixel per fragment

    int best = -1;
    for (int s = 0; s < SLICES; ++s) {
        vec4 sc = texelFetch(ScanSampler, ivec2(s, 0), 0);
        if (sc.r > 0.5) {
            int y = int(sc.g * 255.0 + 0.5) * 256 + int(sc.b * 255.0 + 0.5);
            if (y > best) {
                best = y;
            }
        }
    }

    vec4 prev = texelFetch(PrevSampler, ivec2(j, 0), 0);

    if (j == 4) {
        // Confidence pixel: 1.0 while the channel is alive, slow decay when
        // the column is occluded (facing a wall of terrain / GUI overlay).
        fragColor = vec4(best >= 0 ? 1.0 : prev.r * 0.99, 0.0, 0.0, 0.0);
        return;
    }

    if (best < 0) {
        fragColor = prev; // keep the last good basis, fade handles the rest
        return;
    }

    // Payload bytes x2..x17 (16 bytes, see assets/glsl/sky.fsh):
    //   b[0..5]  right xyz (16-bit each)   b[6..11] up xyz (16-bit each)
    //   b[12..13] fov 16-bit               b[14] tx  b[15] ty (8-bit signed)
    float b[16];
    for (int i = 0; i < 16; ++i) {
        b[i] = texelFetch(InSampler, ivec2(2 + i, best), 0).a * 255.0;
    }

    if (j == 0) {
        fragColor = vec4(b[0], b[1], b[2], b[3]) / 255.0;
    } else if (j == 1) {
        fragColor = vec4(b[4], b[5], b[6], b[7]) / 255.0;
    } else if (j == 2) {
        fragColor = vec4(b[8], b[9], b[10], b[11]) / 255.0;
    } else {
        fragColor = vec4(b[12], b[13], b[14], b[15]) / 255.0;
    }
}
