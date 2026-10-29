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
//
// v5.3.2 ZERO-RESIDUAL: the projection above only covers dots whose word
// decoded this frame. A strobe miss (rendered sprite offset beyond the
// 5-tap cross, or hue sunk below threshold at the fog edge) leaves a live
// green pixel with NO mask — the measured 19 residual px/frame. Two closes:
// (1) the mask now merges BOTH capture rounds' fresh words (the first
// round's word used to die in the h_a/c_a ping-pong twin); (2) a COLOR NET
// marks every screen pixel inside the spawn decode band whose raw color
// carries the bright carrier hue — a strobe miss cannot survive it. Outside
// the band no live dots exist by construction (stream.rs spawn gate), so
// grass/leaves far away keep their pixels; inside it a false positive only
// repaints a texel from its own surroundings (self-repairing).

uniform sampler2D HeightSampler;  // clod_h_b — fresh HI words (this frame)
uniform sampler2D ColBufSampler;  // clod_c_b — fresh LO words (this frame)
uniform sampler2D Height2Sampler; // clod_h_a — round-1 ping-pong twin (v5.3.2)
uniform sampler2D ColBuf2Sampler; // clod_c_a — round-1 ping-pong twin (v5.3.2)
uniform sampler2D InSampler;      // minecraft:main — per-pixel color net (v5.3.2)
uniform sampler2D RotSampler;     // clod_rot_a (camera channel)

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
    // v5.3.2a: per-plane crossing distances for the color-net band gate.
    // A HI dot seen slightly upward crosses the LO plane a few meters from
    // the eye (the LO plane rides 2 m above it) — the NEAREST crossing says
    // nothing about which plane the dot lives on, so the gate must accept
    // EITHER plane inside the spawn decode band.
    float tHi = 1.0e9;
    float tLo = 1.0e9;

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
        t = clamp(t, 2.0, 4096.0);
        if (sel == 0) {
            tHi = min(tHi, t);
        } else {
            tLo = min(tLo, t);
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
                vec4 b2 = sel == 0 ? texelFetch(Height2Sampler, c, 0)
                                   : texelFetch(ColBuf2Sampler, c, 0);
                if (b.g < 0.5 && b2.g < 0.5) {
                    continue; // word not strobed this frame: no live dots
                }
                // v5.3.2: merge both rounds' fresh bytes (they are identical
                // for a steady frame; OR covers pipeline edge cases).
                uint byte = 0u;
                if (b.g >= 0.5) {
                    byte |= uint(floor(b.b * 255.0 + 0.5));
                }
                if (b2.g >= 0.5) {
                    byte |= uint(floor(b2.b * 255.0 + 0.5));
                }
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

    // v5.3.2 COLOR NET — the strobe-miss safety net (see header). Same hue
    // gate as the capture strobe's raw branch.
    // v5.3.2b TWO TIERS: in-band dots get the roomy 3px disc; OUT-OF-BAND
    // carrier hue still gets a 2px disc. Out-of-band green exists: stale
    // dots from a pre-teleport camera keep rendering (and rising) for their
    // whole 3-5 s lifetime against the NEW view — undecodable, unprojectable,
    // but plainly visible (measured live: 2-61 px/frame right after hops).
    // The fill is self-repairing, so the wider vegetation false-positive
    // surface costs nothing visible.
    bool bandOK = (tHi >= 24.0 && tHi <= 128.0) || (tLo >= 24.0 && tLo <= 128.0);
    if (marked < 0.5) {
        vec3 raw = texture(InSampler, texCoord).rgb;
        if (raw.g > 0.45 && raw.g > raw.r + 0.08 && raw.g > raw.b + 0.08) {
            marked = 1.0;
            radius = bandOK ? 3.0 : 2.0;
        }
    }

    // r = mask flag, g = disc radius in px / 64 for the eraser kernel.
    fragColor = vec4(marked, clamp(radius / 64.0, 0.0, 1.0), 0.0, 1.0);
}
