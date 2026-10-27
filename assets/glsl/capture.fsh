#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD v5.0 SKY-GRID BIT-STREAM capture pass.
//
// The server streams LOD cells as dust particles on two sky planes over the
// disk (HI Y=260: 8-bit height word; LO Y=272: 8-bit color index word). One
// 16-bit word = 4 particles: particle slot s (Z axis, msb-first) carries
// bits (14-2s)..(15-2s) of the word; its X sub-position inside the cell IS
// the 2-bit value, its Z sub-position IS the slot. Sub-cell quantum =
// cell/4, centered.
//
// This pass decodes ONE plane back into its persistent 256x256 target
// (1 pixel = 1 cell). For the pixel's cell it re-projects all 16 sub-cell
// particle positions onto the screen (camera from the clod_rot_a channel),
// strobes every projection against the live depth buffer — a dust particle
// renders on the OPAQUE layer and writes depth, so a particle that is
// actually visible leaves a depth spike at its screen position; sky pixels
// read 0.0 in 26.3 device depth — and OR-merges the decoded word into the
// previous value. Anything that never made it through the strobe keeps its
// old value; the stream self-heals on the next lap (512 ticks).

uniform sampler2D PrevSampler;   // previous plane target (persistent)
uniform sampler2D InDepthSampler; // minecraft:main depth (device depth)
uniform sampler2D RotSampler;    // clod_rot_a (camera + fog channel)

layout(std140) uniform SamplerInfo {
    vec2 OutSize; // this target (256 x 256): 1 px = 1 cell
    vec2 InSize;  // screen size (projection viewport)
};

// x = origin_x, y = origin_z, z = plane HI y, w = plane LO y
layout(std140) uniform ClodGrid {
    vec4 GridInfo;
};

// x = cell size, y = sub quantum (cell/4), z = device depth near, w = plane (0 = HI, 1 = LO)
layout(std140) uniform ClodStream {
    vec4 StreamInfo;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;

void main() {
    ivec2 px = ivec2(clamp(texCoord, 0.0, 1.0) * OutSize);
    px = clamp(px, ivec2(0), ivec2(OutSize) - 1);

    float planeY = StreamInfo.w < 0.5 ? GridInfo.z : GridInfo.w;

    // Camera from the persistent rotation target (v4.1 semantics).
    vec4 rot = texelFetch(RotSampler, ivec2(0, 0), 0);
    float az = rot.r * 2.0 * PI;
    float el = rot.g * PI - PI * 0.5;
    float fov = 5.0 + rot.a * 115.0; // inverse of (fov-5)/115*254

    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));

    float tanH = tan(radians(fov) * 0.5);
    float aspect = InSize.x / max(InSize.y, 1.0);
    vec3 cam = vec3(CameraBlockPos) + CameraOffset;

    // World center of this pixel's cell.
    float cell = StreamInfo.x;
    float sub = StreamInfo.y;
    vec2 cellWorld = GridInfo.xy + (vec2(px) + 0.5) * cell;

    float near = StreamInfo.z;
    uint word = 0u; // 8 data bits in the HIGH byte (slots msb-first)
    bool anyHit = false;

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
            float dep = texture(InDepthSampler, sc).r;
            // 26.3 device depth: d = near/eyeZ, sky == 0. A dust particle is
            // OPAQUE: when visible it OWNS the depth sample at its screen
            // position (a terrain silhouette in front of it pushes depth
            // nearer, the sky behind it leaves exactly 0).
            if (dep < 1.0e-6 || dep >= 0.99999) {
                continue; // sky — nothing rendered there
            }
            float vz = near / dep;
            if (vz < z - 1.5 || vz > z + 0.6) {
                continue; // depth does not match this particle
            }
            word |= uint(qx) << (14 - 2 * qz);
            anyHit = true;
        }
    }

    vec4 prev = texelFetch(PrevSampler, px, 0);
    uint prevByte = uint(prev.r * 255.0 + 0.5);
    uint outByte = anyHit ? (prevByte | (word >> 8)) : prevByte;
    float mark = anyHit ? 1.0 : prev.a;
    fragColor = vec4(float(outByte) / 255.0, anyHit ? 1.0 : 0.0, 0.0, mark);
}
