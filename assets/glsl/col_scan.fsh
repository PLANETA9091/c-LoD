#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD pass 1 (v4.3): scan the camera-channel column.
//
// The sky program writes the BOBBED view basis into the ALPHA of the
// leftmost 19 columns (see assets/glsl/sky.fsh). Everything else in the
// frame writes different alpha: terrain is 1.0, translucent passes blend,
// the GUI comes later. A row is "intact" when its 19 alpha bytes decode to
// valid magic, a sane fov pair and a matching additive checksum.
//
// This pass covers the screen column in slices: output pixel j scans rows
// [j*4, j*4+4) and reports the TOPMOST intact row in its slice (GL y grows
// upward, so topmost = largest y). The 1x5 solver (clod:post/rot_solve)
// then picks the best slice. Cost: 640 pixels * 4 rows * 19 fetches —
// trivial, and no sun is required.

uniform sampler2D InSampler; // minecraft:main

layout(std140) uniform SamplerInfo {
    vec2 OutSize;
    vec2 InSize;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const int COLS = 19;
const int ROWS_PER_SLICE = 4;

bool row_valid(int y) {
    float a[19];
    for (int i = 0; i < COLS; ++i) {
        a[i] = texelFetch(InSampler, ivec2(i, y), 0).a * 255.0;
    }
    if (abs(a[0] - 199.0) > 0.5) return false;        // 0xC7
    if (abs(a[1] - 99.0) > 0.5) return false;         // 0x63
    if (a[14] > 254.5 && a[15] > 254.5) return false; // fov 65535 = invalid
    float s = 0.0;
    for (int i = 0; i < 18; ++i) {
        s += a[i];
    }
    return abs(mod(s, 256.0) - a[18]) < 1.0;
}

void main() {
    int slice = int(gl_FragCoord.x);
    int rows = int(InSize.y);
    int best = -1;
    for (int r = 0; r < ROWS_PER_SLICE; ++r) {
        int y = slice * ROWS_PER_SLICE + r;
        if (y >= rows) break;
        if (row_valid(y)) {
            best = y; // ascending: last valid = topmost within the slice
        }
    }
    if (best < 0) {
        fragColor = vec4(0.0, 0.0, 0.0, 0.0);
    } else {
        fragColor = vec4(1.0, float(best / 256) / 255.0, mod(float(best), 256.0) / 255.0, 0.0);
    }
}
