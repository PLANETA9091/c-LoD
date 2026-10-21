#version 330

// c-LoD pass 1 (v4.2): scan the camera-channel columns.
//
// The sky program writes TWO independent payload groups into the alpha of
// the leftmost 16 columns (see assets/glsl/sky.fsh):
//   group A (x0..x7)  — camera basis, magic 0xC7 0x63, checksum additive;
//   group B (x8..x15) — vanilla fog state, checksum (bytes + 61) mod 256.
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
// 8 fetches — trivial.

uniform sampler2D InSampler; // minecraft:main

layout(std140) uniform SamplerInfo {
    vec2 OutSize;
    vec2 InSize;
};

in vec2 texCoord;
out vec4 fragColor;

const int ROWS_PER_SLICE = 4;

float alpha_at(int x, int y) {
    return texelFetch(InSampler, ivec2(x, y), 0).a * 255.0;
}

// Group A: magic + sane fov + additive checksum (v4.1 semantics, stable).
bool row_valid_camera(int y) {
    float a0 = alpha_at(0, y);
    float a1 = alpha_at(1, y);
    if (abs(a0 - 199.0) > 0.5) return false;  // 0xC7
    if (abs(a1 - 99.0) > 0.5) return false;   // 0x63
    float a6 = alpha_at(6, y);
    if (a6 > 254.5) return false;             // 255 = opaque alpha (terrain)
    float s = a0 + a1 + alpha_at(2, y) + alpha_at(3, y)
            + alpha_at(4, y) + alpha_at(5, y) + a6;
    return abs(mod(s, 256.0) - alpha_at(7, y)) < 1.0;
}

// Group B: additive checksum with seed 61 over the 7 fog bytes.
bool row_valid_fog(int y) {
    float s = alpha_at(8, y) + alpha_at(9, y) + alpha_at(10, y)
            + alpha_at(11, y) + alpha_at(12, y) + alpha_at(13, y)
            + alpha_at(14, y) + 61.0;
    return abs(mod(s, 256.0) - alpha_at(15, y)) < 1.0;
}

void main() {
    int slice = int(gl_FragCoord.x);
    int group = int(gl_FragCoord.y); // 0 = camera, 1 = fog
    int rows = int(InSize.y);
    int best = -1;
    for (int r = 0; r < ROWS_PER_SLICE; ++r) {
        int y = slice * ROWS_PER_SLICE + r;
        if (y >= rows) break;
        bool ok = group == 0 ? row_valid_camera(y) : row_valid_fog(y);
        if (ok) {
            best = y; // ascending: last valid = topmost within the slice
        }
    }
    if (best < 0) {
        fragColor = vec4(0.0, 0.0, 0.0, 0.0);
    } else {
        fragColor = vec4(1.0, float(best / 256) / 255.0, mod(float(best), 256.0) / 255.0, 0.0);
    }
}
