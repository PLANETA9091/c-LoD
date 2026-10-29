#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD v5.3 DUST ERASER — the mask builder.
//
// The eye must never see the transport. The capture passes read the raw
// frame (dust included) BEFORE this pass; this pass marks every screen
// pixel a live particle covers, and the final lod_terrain pass repaints
// those pixels from their surroundings. The displayed frame is dust-free
// from EVERY vantage while the capture still decodes at full rate.
//
// Which dots are live RIGHT NOW? A cell's word decoded THIS frame
// (buffer .g flag) means at least one of its sub-particles passed the
// hue strobe on the current frame — the dots are rendered and bright.
// The fresh word (.b, written by capture this frame) carries the exact
// live sub-positions: 4 slots x 2-bit X offset. For the pixel's view ray
// we intersect both particle planes, walk the 3x3 cell ring around the
// crossing, project each live sub-particle and mark the pixel when it
// falls inside the dot's screen disc (size ~ world-size / (2 z tanH)).
//
// Geometry MUST match src/stream.rs::word_spawns and capture.fsh exactly:
// spawn = cell CENTER + ((qx - 1.5) * sub, 0, (slot - 1.5) * sub).

uniform sampler2D HeightSampler; // clod_h_b — fresh HI words (this frame)
uniform sampler2D ColBufSampler; // clod_c_b — fresh LO words (this frame)
uniform sampler2D RotSampler;    // clod_rot_a (camera channel)

// x = origin_x, y = origin_z, z = HIGH layer y, w = LOW layer y
layout(std140) uniform ClodGrid {
    vec4 GridInfo;
};

// x = cell size, y = sub quantum (cell/4), z = device depth near, w = unused
layout(std140) uniform ClodStream {
    vec4 StreamInfo;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;
// Rendered world diameter of the happy_villager quad, calibrated live on
// the bench (1-2 px dots at ~180 blocks, fov 70, 720p => ~1.0 blocks) plus
// a small safety margin so the eraser never under-covers.
const float DOT_WORLD = 1.3;

void main() {
    vec4 rot = texelFetch(RotSampler, ivec2(0, 0), 0);
    float az = rot.r * 2.0 * PI;
    float el = rot.g * PI - PI * 0.5;
    float fov = 5.0 + rot.a * 115.0;
    if (rot.b < 0.05) {
        az = 180.0 * PI / 180.0;
        el = -5.0 * PI / 180.0;
        fov = 70.0;
    }

    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));

    float tanH = tan(radians(fov) * 0.5);
    float aspect = ScreenSize.x / max(ScreenSize.y, 1.0);
    vec3 cam = vec3(CameraBlockPos) + CameraOffset;

    vec2 px = texCoord * ScreenSize;
    vec2 ndc = texCoord * 2.0 - 1.0;
    vec3 dir = normalize(F + R * (ndc.x * tanH * aspect) + U * (ndc.y * tanH));

    float cell = StreamInfo.x;
    float sub = StreamInfo.y;

    float marked = 0.0;
    float radius = 4.0;

    // Two planes: HI word on the high layer, LO word on the low layer.
    for (int sel = 0; sel < 2; ++sel) {
        float planeY = sel == 0 ? GridInfo.z : GridInfo.w;
        if (abs(dir.y) < 1.0e-4) {
            continue;
        }
        float t = (planeY - cam.y) / dir.y;
        if (t < 2.0 || t > 4096.0) {
            continue; // plane behind the camera or absurdly far
        }
        vec2 X = cam.xz + dir.xz * t;
        vec2 cellF = (X - GridInfo.xy) / cell;
        ivec2 base = ivec2(floor(cellF));
        for (int dz = -1; dz <= 1; ++dz) {
            for (int dx = -1; dx <= 1; ++dx) {
                ivec2 c = base + ivec2(dx, dz);
                if (any(lessThan(c, ivec2(0))) || any(greaterThanEqual(c, ivec2(256)))) {
                    continue;
                }
                vec4 b = sel == 0 ? texelFetch(HeightSampler, c, 0)
                                  : texelFetch(ColBufSampler, c, 0);
                if (b.g < 0.5) {
                    continue; // word not strobed this frame: no live dots
                }
                uint byte = uint(floor(b.b * 255.0 + 0.5));
                vec2 center = GridInfo.xy + (vec2(c) + 0.5) * cell;
                for (int s = 0; s < 4; ++s) {
                    uint qx = (byte >> uint(6 - 2 * s)) & 3u;
                    vec3 P = vec3(
                        center.x + (float(qx) - 1.5) * sub,
                        planeY,
                        center.y + (float(s) - 1.5) * sub
                    );
                    vec3 d = P - cam;
                    float z = dot(d, F);
                    if (z < 2.0) {
                        continue;
                    }
                    vec2 ndcP = vec2(
                        dot(d, R) / (z * tanH * aspect),
                        dot(d, U) / (z * tanH)
                    );
                    if (abs(ndcP.x) > 1.05 || abs(ndcP.y) > 1.05) {
                        continue;
                    }
                    vec2 Q = (ndcP * 0.5 + 0.5) * ScreenSize;
                    float r = clamp(
                        DOT_WORLD * 0.5 / (z * tanH) * ScreenSize.y * 0.5 + 1.5,
                        1.5,
                        48.0
                    );
                    if (distance(px, Q) <= r) {
                        marked = 1.0;
                        radius = r;
                        break;
                    }
                }
                if (marked > 0.5) {
                    break;
                }
            }
            if (marked > 0.5) {
                break;
            }
        }
        if (marked > 0.5) {
            break;
        }
    }

    // r = mask flag, g = disc radius in px / 64 for the eraser kernel.
    fragColor = vec4(marked, clamp(radius / 64.0, 0.0, 1.0), 0.0, 1.0);
}
