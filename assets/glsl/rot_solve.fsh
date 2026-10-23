#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD pass 2 (v4.4.2): decode the camera channel into the persistent
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
// decodes.
//
// v4.4.2 ADAPTIVE GLIDE: while the payload flows frame to frame, the fresh
// bytes are forwarded BIT-EXACT (no float round trip, zero lag). After an
// occlusion gap the held basis is stale by however far the camera turned
// meanwhile; forwarding it verbatim makes the LOD horizon SNAP the moment
// any group column sees sky again (the residual pan flicker). When the
// fresh right vector differs from the held one by more than ~3 degrees,
// blend 35%/frame instead: the horizon glides back over ~5 frames, then
// hands over to the exact path once the angle drops under the threshold.
// The 3-degree gate keeps every continuous tracking frame (up to ~180 deg/s
// pans) on the exact path — the glide only covers genuine gaps.
//
// Confidence jumps to 1 on the first valid row; while the column is fully
// occluded the previous payload is kept and confidence decays at 0.99/frame.
// Since v4.4.1 confidence is diagnostics + the never-solved init gate only —
// it no longer fades the LOD.

uniform sampler2D ScanSampler; // clod:clod_scan (640x1 slice report)
uniform sampler2D InSampler;   // minecraft:main (payload rows)
uniform sampler2D PrevSampler; // clod:clod_rot_a (1x5 persistent)

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const int SLICES = 5120;
const float GLIDE_DOT = 0.9986; // cos(~3 deg): below this the held basis is stale
const float GLIDE_K = 0.35;

// (hi,lo) big-endian 16-bit byte pair from RGBA8 (stored n/255), with the
// (c+1)/2 * 65535 signed remap used by the camera channel.
float c16(float hi, float lo) {
    return (hi * 65280.0 + lo * 255.0) / 65535.0 * 2.0 - 1.0;
}

ivec2 enc16(float c) {
    float q = floor(clamp((c + 1.0) * 0.5, 0.0, 1.0) * 65535.0 + 0.5);
    return ivec2(int(floor(q / 256.0)), int(mod(q, 256.0)));
}

float enc8s(float c) {
    return floor(clamp((c + 1.0) * 0.5, 0.0, 1.0) * 255.0 + 0.5);
}

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
    vec4 pc = texelFetch(PrevSampler, ivec2(4, 0), 0);

    if (j == 4) {
        // Confidence pixel: 1.0 while the channel is alive, slow decay when
        // the column is occluded (facing a wall of terrain / GUI overlay).
        fragColor = vec4(best >= 0 ? min(pc.r + 0.08, 1.0) : pc.r * 0.99,
                         0.0, 0.0, 0.0);
        return;
    }

    if (best < 0) {
        fragColor = prev; // keep the last good basis
        return;
    }

    // Payload bytes x2..x17 (16 bytes, see assets/glsl/sky.fsh):
    //   b[0..5]  right xyz (16-bit each)   b[6..11] up xyz (16-bit each)
    //   b[12..13] fov 16-bit               b[14] tx  b[15] ty (8-bit signed)
    float b[16];
    for (int i = 0; i < 16; ++i) {
        b[i] = texelFetch(InSampler, ivec2(2 + i, best), 0).a * 255.0;
    }

    vec4 p0 = texelFetch(PrevSampler, ivec2(0, 0), 0);
    vec4 p1 = texelFetch(PrevSampler, ivec2(1, 0), 0);
    vec4 p2 = texelFetch(PrevSampler, ivec2(2, 0), 0);
    vec4 p3 = texelFetch(PrevSampler, ivec2(3, 0), 0);

    // Adaptive glide (see header): bit-exact unless the held basis is stale.
    vec3 Rp = vec3(c16(p0.x, p0.y), c16(p0.z, p0.w), c16(p1.x, p1.y));
    vec3 Rn = vec3(c16(b[0] / 255.0, b[1] / 255.0), c16(b[2] / 255.0, b[3] / 255.0),
                   c16(b[4] / 255.0, b[5] / 255.0));
    float k = 1.0;
    if (pc.r > 0.02) {
        float lp = length(Rp);
        float ln = length(Rn);
        if (lp > 0.5 && ln > 0.5 && dot(Rp, Rn) / (lp * ln) < GLIDE_DOT) {
            k = GLIDE_K;
        }
    }

    if (k >= 1.0) {
        if (j == 0) {
            fragColor = vec4(b[0], b[1], b[2], b[3]) / 255.0;
        } else if (j == 1) {
            fragColor = vec4(b[4], b[5], b[6], b[7]) / 255.0;
        } else if (j == 2) {
            fragColor = vec4(b[8], b[9], b[10], b[11]) / 255.0;
        } else {
            fragColor = vec4(b[12], b[13], b[14], b[15]) / 255.0;
        }
        return;
    }

    // Glide path: blend the pair in float space, re-orthonormalize, re-encode.
    vec3 Rb = normalize(mix(Rp, Rn, k));
    vec3 Um = mix(vec3(c16(p1.z, p1.w), c16(p2.x, p2.y), c16(p2.z, p2.w)),
                  vec3(c16(b[6] / 255.0, b[7] / 255.0), c16(b[8] / 255.0, b[9] / 255.0),
                       c16(b[10] / 255.0, b[11] / 255.0)), k);
    vec3 Ub = normalize(Um - Rb * dot(Rb, Um));

    // fov (16-bit, marker 65535 = invalid) and the 8-bit signed eye wobble.
    float fovP = p3.x * 65280.0 + p3.y * 255.0;
    float fovN = b[12] * 65280.0 + b[13] * 255.0;
    float fov;
    if (fovP > 65534.5 || fovN > 65534.5) {
        fov = fovN > 65534.5 ? 70.0 : 5.0 + fovN / 65535.0 * 115.0;
    } else {
        fov = mix(5.0 + fovP / 65535.0 * 115.0, 5.0 + fovN / 65535.0 * 115.0, k);
    }
    float tx = mix(p3.z * 2.0 - 1.0, b[14] / 255.0 * 2.0 - 1.0, k);
    float ty = mix(p3.w * 2.0 - 1.0, b[15] / 255.0 * 2.0 - 1.0, k);

    ivec2 e;
    if (j == 0) {
        e = enc16(Rb.x);
        fragColor = vec4(float(e.x), float(e.y), 0.0, 0.0) / 255.0;
        e = enc16(Rb.y);
        fragColor.z = float(e.x) / 255.0;
        fragColor.w = float(e.y) / 255.0;
    } else if (j == 1) {
        e = enc16(Rb.z);
        fragColor = vec4(float(e.x), float(e.y), 0.0, 0.0) / 255.0;
        e = enc16(Ub.x);
        fragColor.z = float(e.x) / 255.0;
        fragColor.w = float(e.y) / 255.0;
    } else if (j == 2) {
        e = enc16(Ub.y);
        fragColor = vec4(float(e.x), float(e.y), 0.0, 0.0) / 255.0;
        e = enc16(Ub.z);
        fragColor.z = float(e.x) / 255.0;
        fragColor.w = float(e.y) / 255.0;
    } else {
        float fovq = floor(clamp((fov - 5.0) / 115.0, 0.0, 1.0) * 65534.0 + 0.5);
        fragColor = vec4(floor(fovq / 256.0), mod(fovq, 256.0), enc8s(tx), enc8s(ty)) / 255.0;
    }
}
