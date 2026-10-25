#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD pass 2 (v4.1 + day channel + v4.12 TAA fingerprint): decode the
// camera basis from the channel scan, solve the day-light factor and emit the
// TAA camera fingerprint. The rot targets are 4x1:
//
//   pixel 0: camera in the EXACT v4.0 encoding consumed by clod:post/lod_terrain
//            R = az / (2*pi),  G = (el + pi/2) / pi,  B = confidence,  A = fov byte.
//   pixel 1: DAY-LIGHT factor in R — the vanilla lightmap stand-in (the max
//            over the 640 per-column sky probes through the celestial curve).
//   pixel 2: TAA fingerprint of THIS frame — rgb = 3-byte hash of the solved
//            camera (az 12b, el 12b, fov 8b, dayF 8b, camera xyz 0.25-block
//            steps mod 64), a = jitter phase counter (0..7)/255. DH's
//            frameIndexMod8 stand-in: advances every frame, the march jitters
//            only while it is > 0 (`if (uFrameMod8 > 0)` — frame 0 identity).
//   pixel 3: PREVIOUS frame's fingerprint verbatim (Prev.pixel 2) — the march
//            accumulates only while pixel 2 == pixel 3.
//
// DH 3.3.4 (terrain/gl/vert.vert) jitters LOD vertices over 8 sub-pixel
// offsets and leaves blending to the client temporal anti-aliaser; the march
// blends through the persistent clod_hist pair instead (posteffect-native
// replacement, no ghosting — a camera/time move changes the fingerprint).

uniform sampler2D ScanSampler; // clod:clod_scan (640x1 slice report)
uniform sampler2D InSampler;   // minecraft:main (payload rows)
uniform sampler2D PrevSampler; // clod:clod_rot_a (persistent)

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const int SLICES = 640;
const float PI = 3.14159265358979;

// Shared solve so the camera pixel and the fingerprint pixel cannot drift
// apart (the fingerprint of a static camera must be byte-identical or the
// TAA accumulator would never engage).
vec4 solved_camera(vec4 prev) {
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
        return vec4(az / 6.28318530718, (el + PI * 0.5) / PI, 1.0, fovq / 254.0);
    }
    // no sky this frame: hold the previous solution, confidence decays
    return vec4(prev.r, prev.g, prev.b * 0.99, prev.a);
}

float day_factor(vec4 prev) {
    float probe = 0.0;
    for (int j = 0; j < SLICES; ++j) {
        probe = max(probe, texelFetch(ScanSampler, ivec2(j, 0), 0).a);
    }
    float dayF = smoothstep(0.08, 0.72, probe);
    return probe > 0.001 ? dayF : prev.r;
}

uint hash_u(uint x) {
    x ^= x >> 16;
    x *= 2246822519u;
    x ^= x >> 13;
    x *= 3266489917u;
    x ^= x >> 16;
    return x;
}

vec4 fingerprint(vec4 cam, float dayF, float prevPhase) {
    uint azq = uint(round(cam.r * 4095.0));
    uint elq = uint(round(cam.g * 4095.0));
    uint fovq = uint(round(cam.a * 255.0));
    uint dayq = uint(round(clamp(dayF, 0.0, 1.0) * 255.0));
    // 26.3 Globals: CameraBlockPos = floor(camPos), CameraOffset = floor - camPos.
    vec3 camPos = vec3(CameraBlockPos) - CameraOffset;
    // 0.25-block steps wrap every 64 blocks: a straight walk repeats the
    // fingerprint once per 64 blocks and ghosts for a single frame.
    uint cx = uint(mod(floor(camPos.x * 4.0), 256.0));
    uint cz = uint(mod(floor(camPos.z * 4.0), 256.0));
    uint cy = uint(mod(floor(camPos.y * 4.0), 256.0));

    uint h = azq * 73856093u;
    h ^= elq * 19349663u;
    h ^= fovq * 83492791u;
    h ^= (cx << 8 | cy) * 2654435761u;
    h ^= (cz << 8 | dayq) * 40503u;
    h = hash_u(h);

    vec3 fp = vec3(
        float(h & 255u),
        float((h >> 8) & 255u),
        float((h >> 16) & 255u)
    ) / 255.0;
    float phase = mod(prevPhase + 1.0, 8.0) / 255.0;
    return vec4(fp, phase);
}

void main() {
    int px = int(gl_FragCoord.x);
    vec4 prev = texelFetch(PrevSampler, ivec2(0), 0);

    if (px == 1) {
        fragColor = vec4(day_factor(prev), 0.0, 0.0, 1.0);
        return;
    }
    if (px == 2) {
        vec4 cam = solved_camera(prev);
        float dayF = day_factor(prev);
        float prevPhase = floor(texelFetch(PrevSampler, ivec2(2, 0), 0).a * 255.0 + 0.5);
        fragColor = fingerprint(cam, dayF, prevPhase);
        return;
    }
    if (px == 3) {
        // previous frame's fingerprint verbatim
        fragColor = texelFetch(PrevSampler, ivec2(2, 0), 0);
        return;
    }

    fragColor = solved_camera(prev);
}
