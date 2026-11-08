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
// v6.18.1 SECOND STRIP (run #286 = 42466249389): the parity-world vantages
// flank the frame with the v6.15 mountains — the leftmost 16 columns are
// TERRAIN at every row, no intact payload row exists, the channel decays
// below the 0.25 gate and the whole downstream (capture, march, the v6.18
// un-fog) goes blind exactly on the pairs the gate measures (01-10 unmoved,
// the white fog band still in our frames). The SAME 16 bytes now also ride
// the fixed columns 632..647 (the CI gate frame is 1280x720; both legs
// shoot 1280x720): the top-center of those vantages is SKY in every frame
// of the tour, so the channel arms at the shot pose and stays armed through
// the settle. Same layout, same checksums: byte k lives at x=k (left strip)
// and x=632+k (center strip); the scanner (col_scan) validates both and
// reports which one won in the slice's alpha. No new UBOs (the vanilla sky
// program binds no Globals — a Globals include here would kill the program,
// the run #232 failure class), no RGB changes (alpha of the scene target is
// never displayed). GUI overdraw (hotbar/bossbar rows) fails the checksum
// on its rows only — the topmost-intact-row scan self-heals, as before.
//
// RGB stays 100% vanilla — alpha of the scene target is never displayed.
// Downstream writes (terrain alpha = 1.0, clouds, particles, weather, GUI)
// fail the magic/checksum checks; the post chain scans the column top-down
// for the topmost INTACT row, so transient overdraw self-heals within a
// frame for as long as any sky row is visible in either strip.
void main() {
    vec4 color = apply_fog(ColorModulator, sphericalVertexDistance,
        cylindricalVertexDistance, 0.0, FogSkyEnd, FogSkyEnd, FogSkyEnd, FogColor);

    ivec2 px = ivec2(gl_FragCoord.xy);
    // v6.18.1: the payload rides TWO strips — the v4.2 left edge (x 0..15)
    // and the fixed center columns (x 632..647). Byte k lives at x=k and
    // x=632+k; the scanner validates both independently.
    if (px.x < 16 || (px.x >= 632 && px.x < 648)) {
        int k = px.x < 16 ? px.x : px.x - 632;
        if (k < 8) {
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

            int b = k == 0 ? 199
                  : k == 1 ? 99
                  : k == 2 ? azh
                  : k == 3 ? azl
                  : k == 4 ? elh
                  : k == 5 ? ell
                  : k == 6 ? fovq
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

            int b = k == 8  ? fogR
                  : k == 9  ? fogG
                  : k == 10 ? fogB
                  : k == 11 ? fogA
                  : k == 12 ? stq
                  : k == 13 ? enq
                  : k == 14 ? rdq
                  : chk2;
            color.a = float(b) / 255.0;
        }
    }

    fragColor = color;
}
