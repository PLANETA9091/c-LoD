#version 330
#extension GL_ARB_separate_shader_objects : require

#include <minecraft:fog.glsl>
#include <minecraft:dynamictransforms.glsl>
#include <minecraft:projection.glsl>

layout(location = 0) in float sphericalVertexDistance;
layout(location = 1) in float cylindricalVertexDistance;

layout(location = 0) out vec4 fragColor;

// c-LoD camera channel (v4.2) — the sky program is the only vanilla stage
// that sees the true camera basis (ModelViewMat), the projection (ProjMat)
// AND the full Fog UBO (fog.glsl): FogColor, FogEnvironmentalStart/End and
// the render-distance fog pair. v4.2 packs BOTH into the alpha of the
// leftmost 16 columns of every sky fragment as two independent groups with
// separate checksums (losing one must not kill the other):
//
//   group A (x0..x7) — camera, encoding unchanged since v4.1:
//     x0 = 0xC7, x1 = 0x63          magic
//     x2,x3 = azimuth               big-endian, az/2pi * 65535
//     x4,x5 = elevation             big-endian, (el+pi/2)/pi * 65535
//     x6 = fov byte                 (fovDeg - 5) / 115 * 254  (255 = invalid)
//     x7 = checksum A               (x0..x6 additive) mod 256
//
//   group B (x8..x15) — vanilla fog, NEW in v4.2 (the Voxy look):
//     x8..x10 = FogColor.rgb        final dimension x biome x timeline color
//     x11     = FogColor.a          fog opacity (Voxy: fogColour.a)
//     x12     = envStart / 8        FogEnvironmentalStart  (<= 2040 blocks)
//     x13     = envEnd / 8          FogEnvironmentalEnd
//     x14     = renderEnd / 16      FogRenderDistanceEnd (render distance proxy)
//     x15     = checksum B          (x8..x14 + 61) mod 256
//
// RGB stays 100% vanilla — alpha of the scene target is never displayed.
// Downstream writes (terrain alpha = 1.0, clouds, particles, weather, GUI)
// fail the magic/checksum checks; the post chain scans the column top-down
// for the topmost INTACT row, so transient overdraw self-heals within a
// frame for as long as any sky row is visible in the leftmost 16 columns.
void main() {
    vec4 color = apply_fog(ColorModulator, sphericalVertexDistance,
        cylindricalVertexDistance, 0.0, FogSkyEnd, FogSkyEnd, FogSkyEnd, FogColor);

    ivec2 px = ivec2(gl_FragCoord.xy);
    if (px.x < 16) {
        if (px.x < 8) {
            // Group A: camera basis (v4.1 encoding, byte-for-byte stable).
            vec3 fwd = transpose(mat3(ModelViewMat)) * vec3(0.0, 0.0, -1.0);
            float az = atan(fwd.z, fwd.x);                          // (-pi, pi]
            float el = asin(clamp(fwd.y, -1.0, 1.0));               // [-pi/2, pi/2]
            float fovDeg = degrees(atan(1.0 / ProjMat[1][1])) * 2.0;

            float a16 = floor(fract(az / 6.28318530718) * 65535.0 + 0.5);
            float e16 = floor(clamp((el + 1.57079632679) / 3.14159265359, 0.0, 1.0) * 65535.0 + 0.5);
            int azh = int(floor(a16 / 256.0));
            int azl = int(mod(a16, 256.0));
            int elh = int(floor(e16 / 256.0));
            int ell = int(mod(e16, 256.0));
            int fovq = int(clamp((fovDeg - 5.0) / 115.0, 0.0, 1.0) * 254.0 + 0.5);
            int chk = (199 + 99 + azh + azl + elh + ell + fovq) & 255;

            int b = px.x == 0 ? 199
                  : px.x == 1 ? 99
                  : px.x == 2 ? azh
                  : px.x == 3 ? azl
                  : px.x == 4 ? elh
                  : px.x == 5 ? ell
                  : px.x == 6 ? fovq
                  : chk;
            color.a = float(b) / 255.0;
        } else {
            // Group B: the vanilla fog state (v4.2). Scaled bytes; the
            // decoder reconstructs blocks = byte * scale. envEnd < envStart
            // or a zero fog color would be garbage — checksum still guards.
            float fr = clamp(FogColor.r, 0.0, 1.0);
            float fg = clamp(FogColor.g, 0.0, 1.0);
            float fb = clamp(FogColor.b, 0.0, 1.0);
            float fa = clamp(FogColor.a, 0.0, 1.0);
            int fogR = int(floor(fr * 255.0 + 0.5));
            int fogG = int(floor(fg * 255.0 + 0.5));
            int fogB = int(floor(fb * 255.0 + 0.5));
            int fogA = int(floor(fa * 255.0 + 0.5));
            int stq  = int(floor(clamp(FogEnvironmentalStart / 8.0, 0.0, 255.0) + 0.5));
            int enq  = int(floor(clamp(FogEnvironmentalEnd / 8.0, 0.0, 255.0) + 0.5));
            int rdq  = int(floor(clamp(FogRenderDistanceEnd / 16.0, 0.0, 255.0) + 0.5));
            int chk2 = (fogR + fogG + fogB + fogA + stq + enq + rdq + 61) & 255;

            int b = px.x == 8  ? fogR
                  : px.x == 9  ? fogG
                  : px.x == 10 ? fogB
                  : px.x == 11 ? fogA
                  : px.x == 12 ? stq
                  : px.x == 13 ? enq
                  : px.x == 14 ? rdq
                  : chk2;
            color.a = float(b) / 255.0;
        }
    }

    fragColor = color;
}
