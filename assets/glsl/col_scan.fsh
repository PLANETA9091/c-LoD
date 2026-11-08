#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD pass 1 (v4.2): scan the camera-channel columns.
//
// The sky program writes TWO independent payload groups into the alpha of
// the leftmost 16 columns (see assets/glsl/sky.fsh):
//   group A (x0..x7)  — camera basis, magic 0xC7 0x63, checksum additive;
//   group B (x8..x15) — vanilla fog state, checksum (bytes + 61) mod 256.
//
// v6.18.1 SECOND STRIP: the same 16 bytes also ride the fixed columns
// 632..647 (sky.fsh) — the parity-world vantages flank the left edge with
// terrain (run #286: no intact row in the left strip, the whole channel
// blind on the gate's pairs). A row is intact when EITHER strip decodes;
// the LEFT strip wins ties (v4.2 semantics unchanged when it is alive) and
// the slice's alpha reports the winner (0 = left base 0, 1 = center base
// 632) so the solver (rot_solve) reads the bytes from the right base.
//
// A row is "intact" for a group when its 8 alpha bytes decode to a sane
// payload with a matching checksum. The two groups are validated INDEPENDENTLY:
// clouds overwriting x8..x15 must not kill the camera, and terrain occluding
// x0..x7 must not kill the fog readback.
//
// Output target is 640x2. Pixel (j, 0) scans rows [j*4, j*4+4) for group A
// and reports the TOPMOST intact row (GL y grows upward: topmost = largest
// y). Pixel (j, 1) does the same for group B. The 2x2 solver (clod:post/
// rot_solve) picks the best slice per row. Cost: 2 * 640 pixels * 4 rows *
// 16 fetches — trivial.

uniform sampler2D InSampler; // minecraft:main

// v5.0.3: SamplerInfo never existed as a client UBO — it stayed zeroed
// (the whole capture wrote cell (0,0), the march divided by zero).
// The screen size rides the STANDARD Globals block (ScreenSize), the
// target size is compile-time known.

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const int ROWS_PER_SLICE = 4;
// v6.18.1: the center strip base (sky.fsh writes byte k at x=k and x=632+k;
// the CI gate frame is 1280x720 — both legs shoot 1280x720).
const int CENTER_BASE = 632;

float alpha_at(int x, int y) {
    return texelFetch(InSampler, ivec2(x, y), 0).a * 255.0;
}

// Group A at strip base: magic + sane fov + additive checksum (v4.1 semantics).
bool row_valid_camera_at(int base, int y) {
    float a0 = alpha_at(base, y);
    float a1 = alpha_at(base + 1, y);
    if (abs(a0 - 199.0) > 0.5) return false;  // 0xC7
    if (abs(a1 - 99.0) > 0.5) return false;   // 0x63
    float a6 = alpha_at(base + 6, y);
    if (a6 > 254.5) return false;             // 255 = opaque alpha (terrain)
    float s = a0 + a1 + alpha_at(base + 2, y) + alpha_at(base + 3, y)
            + alpha_at(base + 4, y) + alpha_at(base + 5, y) + a6;
    return abs(mod(s, 256.0) - alpha_at(base + 7, y)) < 1.0;
}

// Group B at strip base: additive checksum with seed 61 over the 7 fog bytes.
bool row_valid_fog_at(int base, int y) {
    float s = alpha_at(base + 8, y) + alpha_at(base + 9, y) + alpha_at(base + 10, y)
            + alpha_at(base + 11, y) + alpha_at(base + 12, y) + alpha_at(base + 13, y)
            + alpha_at(base + 14, y) + 61.0;
    return abs(mod(s, 256.0) - alpha_at(base + 15, y)) < 1.0;
}

// v4.2 left-strip semantics: left wins ties, the alpha reports the winner.
bool row_valid_camera(int y, out float strip) {
    if (row_valid_camera_at(0, y)) { strip = 0.0; return true; }
    if (row_valid_camera_at(CENTER_BASE, y)) { strip = 1.0; return true; }
    return false;
}

bool row_valid_fog(int y, out float strip) {
    if (row_valid_fog_at(0, y)) { strip = 0.0; return true; }
    if (row_valid_fog_at(CENTER_BASE, y)) { strip = 1.0; return true; }
    return false;
}

void main() {
    int slice = int(gl_FragCoord.x);
    int group = int(gl_FragCoord.y); // 0 = camera, 1 = fog
    int rows = int(ScreenSize.y);
    int best = -1;
    float bestStrip = 0.0;
    for (int r = 0; r < ROWS_PER_SLICE; ++r) {
        int y = slice * ROWS_PER_SLICE + r;
        if (y >= rows) break;
        float strip;
        bool ok = group == 0 ? row_valid_camera(y, strip) : row_valid_fog(y, strip);
        if (ok) {
            best = y; // ascending: last valid = topmost within the slice
            bestStrip = strip;
        }
    }
    if (best < 0) {
        fragColor = vec4(0.0, 0.0, 0.0, 0.0);
    } else {
        // .a = the winning strip base (0 = left x+k, 1 = center 632+k).
        fragColor = vec4(1.0, float(best / 256) / 255.0, mod(float(best), 256.0) / 255.0, bestStrip);
    }
}
