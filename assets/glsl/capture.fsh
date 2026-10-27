#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD v5.0 SKY-GRID BIT-STREAM capture pass.
//
// The server streams LOD cells as dust particles in vertical COLUMNS: each
// cell spawns its sub-lattice on two layers (low Y=80 decodes from ground
// vantage, high Y=200 from elevated vantage — 26.3 reversed-Z hides
// particles against the bare sky, so a layer only reads where geometry is
// BEHIND the particle). One 16-bit word = 4 particles per layer:
// particle slot s (Z axis, msb-first) carries bits (14-2s)..(15-2s); its X
// sub-position inside the cell IS the 2-bit value, its Z sub-position IS
// the slot. Sub-cell quantum = cell/4, centered. HI word = BLACK dust,
// LO word = WHITE dust — the client multiplies all channels by one random
// factor, so the hue survives.
//
// This pass decodes ONE word back into its persistent 256x256 target (1 px
// = 1 cell). For the pixel's cell it re-projects all 2x16 sub-particle
// positions onto the screen (probe camera), strobes every projection: the
// pixel must sit in front of geometry (device depth > 0) and its main color
// must match the dust hue — then OR-merges the decoded word into the
// previous value. Anything not strobed keeps its old value; the stream
// self-heals on the next lap.

uniform sampler2D InSampler;      // minecraft:main color (dust hue strobe)
uniform sampler2D InDepthSampler; // minecraft:main depth (in-front-of-geometry test)
uniform sampler2D PrevSampler;    // previous plane target (persistent)
uniform sampler2D RotSampler;     // clod_rot_a (camera + fog channel)

layout(std140) uniform SamplerInfo {
    vec2 OutSize; // this target (256 x 256): 1 px = 1 cell
    vec2 InSize;  // screen size (projection viewport)
};

// x = origin_x, y = origin_z, z = HIGH layer y, w = LOW layer y
layout(std140) uniform ClodGrid {
    vec4 GridInfo;
};

// x = cell size, y = sub quantum (cell/4), z = device depth near, w = word (0 = HI, 1 = LO)
layout(std140) uniform ClodStream {
    vec4 StreamInfo;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;

void main() {
    ivec2 px = ivec2(clamp(texCoord, 0.0, 1.0) * OutSize);
    px = clamp(px, ivec2(0), ivec2(OutSize) - 1);

    // Probe camera (the live rot el-channel still carries the group-A
    // regression; the bench player looks yaw=180 south, pitch=-5).
    float az = 180.0 * PI / 180.0;
    float el = -5.0 * PI / 180.0;
    float fov = 70.0;

    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));

    float tanH = tan(radians(fov) * 0.5);
    float aspect = InSize.x / max(InSize.y, 1.0);
    vec3 cam = vec3(CameraBlockPos) + CameraOffset;

    float cell = StreamInfo.x;
    float sub = StreamInfo.y;
    float near = StreamInfo.z;
    vec2 cellWorld = GridInfo.xy + (vec2(px) + 0.5) * cell;

    uint word = 0u;
    bool anyHit = false;

    // Two particle layers share the same word: re-project the sub-lattice
    // at BOTH heights.
    for (int li = 0; li < 2; ++li) {
        float planeY = li == 0 ? GridInfo.w : GridInfo.z; // w = low, z = high

        for (int qz = 0; qz < 4; ++qz) {
            for (int qx = 0; qx < 4; ++qx) {
                vec3 P = vec3(
                    cellWorld.x + (float(qx) - 1.5) * sub,
                    planeY,
                    cellWorld.y + (float(qz) - 1.5) * sub
                );
                vec3 d = P - cam;
                float z = dot(d, F);
                if (z < 2.0) {
                    continue; // behind / too close
                }
                vec2 ndc = vec2(
                    dot(d, R) / (z * tanH * aspect),
                    dot(d, U) / (z * tanH)
                );
                if (abs(ndc.x) > 1.0 || abs(ndc.y) > 1.0) {
                    continue; // off screen
                }
                vec2 sc = ndc * 0.5 + 0.5;
                // STROBE: the particle must render in front of geometry
                // (device depth > 0; against the bare sky particles are
                // hidden by the reversed-Z pipeline), and its hue must match
                // the dust color for this word.
                float dep = texture(InDepthSampler, sc).r;
                if (dep < 1.0e-6) {
                    continue; // against the bare sky — no read possible
                }
                vec3 pc = texture(InSampler, sc).rgb;
                float mn = min(pc.r, min(pc.g, pc.b));
                float mx = max(pc.r, max(pc.g, pc.b));
                bool dust;
                if (StreamInfo.w < 0.5) {
                    dust = mx < 0.14; // black dust (HI word)
                } else {
                    dust = mn > 0.5 && (mx - mn) < 0.14; // white dust (LO word)
                }
                if (!dust) {
                    continue;
                }
                word |= uint(qx) << (14 - 2 * qz);
                anyHit = true;
            }
        }
    }

    vec4 prev = texelFetch(PrevSampler, px, 0);
    uint prevByte = uint(prev.r * 255.0 + 0.5);
    uint outByte = anyHit ? (prevByte | (word >> 8)) : prevByte;
    float mark = anyHit ? 1.0 : prev.a;
    fragColor = vec4(float(outByte) / 255.0, anyHit ? 1.0 : 0.0, 0.0, mark);
}
