#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD pass 2 (v4.2): decode both payload groups into the persistent
// 2x2 rotation target (clod_rot_b, copied to clod_rot_a by clod:post/rot_copy):
//
//   pixel (0,0) = camera   (R = az/2pi, G = (el+pi/2)/pi, B = conf, A = fov)
//   pixel (1,0) = fogColor (R,G,B,A exactly as the sky shader saw the UBO)
//   pixel (0,1) = fogRange (R = envStart/8/255, G = envEnd/8/255,
//                           B = renderEnd/16/255, A = 1 once decoded)
//   pixel (1,1) = unused (0)
//
// Camera: the payload is measured, not fitted — confidence jumps to 1 on
// the first valid row and the basis is frame-exact. When the column is
// fully occluded the previous estimate decays at 0.99/frame (~3 s).
// Fog: no decay semantics needed — values persist unchanged until a new
// valid group B row arrives (the fog state is constant between changes).

uniform sampler2D ScanSampler; // clod:clod_scan (640x2 slice reports)
uniform sampler2D InSampler;   // minecraft:main (payload rows)
uniform sampler2D PrevSampler; // clod:clod_rot_a (persistent 2x2)

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const int SLICES = 640;
const float PI = 3.14159265358979;

int best_row(int group) {
    int best = -1;
    for (int j = 0; j < SLICES; ++j) {
        vec4 s = texelFetch(ScanSampler, ivec2(j, group), 0);
        if (s.r > 0.5) {
            int y = int(s.g * 255.0 + 0.5) * 256 + int(s.b * 255.0 + 0.5);
            if (y > best) {
                best = y;
            }
        }
    }
    return best;
}

void main() {
    ivec2 px = ivec2(gl_FragCoord.xy);

    if (px == ivec2(0, 0)) {
        // Camera pixel (v4.1 semantics).
        vec4 prev = texelFetch(PrevSampler, ivec2(0, 0), 0);
        vec4 rot = vec4(prev.r, prev.g, prev.b * 0.99, prev.a);
        int best = best_row(0);
        if (best >= 0) {
            float a[6];
            for (int i = 2; i < 8; ++i) {
                a[i - 2] = texelFetch(InSampler, ivec2(i, best), 0).a * 255.0;
            }
            float azq = a[0] * 256.0 + a[1];
            float elq = a[2] * 256.0 + a[3];
            float fovq = a[4];
            float az = azq / 65535.0 * 6.28318530718;
            float el = elq / 65535.0 * PI - PI * 0.5;
            rot = vec4(az / 6.28318530718, (el + PI * 0.5) / PI, 1.0, fovq / 254.0);
        }
        fragColor = rot;
    } else if (px == ivec2(1, 0)) {
        // Fog color pixel: persist the previous decode until a valid row.
        vec4 prev = texelFetch(PrevSampler, ivec2(1, 0), 0);
        int best = best_row(1);
        if (best >= 0) {
            vec4 f;
            for (int i = 8; i < 12; ++i) {
                f[i - 8] = texelFetch(InSampler, ivec2(i, best), 0).a;
            }
            fragColor = f;
        } else {
            fragColor = prev;
        }
    } else if (px == ivec2(0, 1)) {
        // Fog range pixel: bytes are (start/8, end/8, renderEnd/16).
        vec4 prev = texelFetch(PrevSampler, ivec2(0, 1), 0);
        int best = best_row(1);
        if (best >= 0) {
            float st = texelFetch(InSampler, ivec2(12, best), 0).a * 255.0;
            float en = texelFetch(InSampler, ivec2(13, best), 0).a * 255.0;
            float rd = texelFetch(InSampler, ivec2(14, best), 0).a * 255.0;
            fragColor = vec4(st / 255.0, en / 255.0, rd / 255.0, 1.0);
        } else {
            fragColor = prev;
        }
    } else {
        fragColor = vec4(0.0);
    }
}
