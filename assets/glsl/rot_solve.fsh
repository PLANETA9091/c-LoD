#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD pass 2 (v4.1 + day channel): decode the camera basis from the channel
// scan and solve the day-light factor.
//
// The rot targets are 2x1: pixel x=0 carries the camera in the EXACT v4.0
// encoding consumed by clod:post/lod_terrain:
//
//   R = az / (2*pi),  G = (el + pi/2) / pi,  B = confidence,  A = fov byte.
//
// Pixel x=1 carries the DAY-LIGHT factor in R: the vanilla lightmap stand-in.
// DH lights distant LODs through the vanilla lightmap at (skyLight 15, block
// light 0), which follows the celestial angle — not the horizon glow. The
// col_scan slices already probe each screen column for its darkest true-sky
// pixel (depth == 0); this pixel takes the MAX over the 640 probes (the most
// open view of the sky wins — columns blocked by walls just report 0) and
// maps it through the lightmap-ish curve. Noon ≈ 1.0, dusk dims smoothly,
// night floors at the moonlight level inside lod_terrain.

uniform sampler2D ScanSampler; // clod:clod_scan (640x1 slice report)
uniform sampler2D InSampler;   // minecraft:main (payload rows)
uniform sampler2D PrevSampler; // clod:clod_rot_a (persistent)

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const int SLICES = 640;
const float PI = 3.14159265358979;

void main() {
    if (gl_FragCoord.x > 0.5) {
        // Day-light pixel: max over the 640 per-column sky probes, mapped
        // through the celestial curve. Measured on the bench camera: noon
        // zenith-band ≈ 0.74, dusk ≈ 0.40, night ≈ 0.027 — the curve puts
        // dusk at dayF ≈ 0.5 (lightMul ≈ 0.61, the vanilla dusk lightmap
        // level), night at 0 (moonlight floor), noon at 1. When no sky is
        // visible at all (walls everywhere) the previous frame's value holds
        // so a short glance at the ground doesn't strobe the LOD.
        float probe = 0.0;
        for (int j = 0; j < SLICES; ++j) {
            probe = max(probe, texelFetch(ScanSampler, ivec2(j, 0), 0).a);
        }
        float dayF = smoothstep(0.08, 0.72, probe);
        float prevDay = texelFetch(PrevSampler, ivec2(1, 0), 0).r;
        fragColor = vec4(probe > 0.001 ? dayF : prevDay, 0.0, 0.0, 1.0);
        return;
    }

    vec4 prev = texelFetch(PrevSampler, ivec2(0), 0);
    vec4 rot = vec4(prev.r, prev.g, prev.b * 0.99, prev.a);

    int best = -1;
    for (int j = 0; j < SLICES; ++j) {
        vec4 s = texelFetch(ScanSampler, ivec2(j, 0), 0);
        if (s.r > 0.5) {
            int y = int(s.g * 255.0 + 0.5) * 256 + int(s.b * 255.0 + 0.5);
            if (y > best) {
                best = y;
            }
        }
    }

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
}
