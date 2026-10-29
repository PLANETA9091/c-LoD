#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD v5.1 SKY-GRID BIT-STREAM capture pass.
//
// The server streams LOD cells as villager-sparkle particles (the only
// texture the 26.3 software rasterizers draw — dust quads die with
// degenerate UVs, verified live on GL llvmpipe AND Vulkan lavapipe).
// The HEIGHT word rides the high layer (Y=120), the COLOR word the low
// layer (Y=80): the carrier has one hue, so the capture pass tells words
// apart by plane height. One 16-bit word = 4 particles per layer:
// particle slot s (Z axis, msb-first) carries bits (14-2s)..(15-2s); its X
// sub-position inside the cell IS the 2-bit value, its Z sub-position IS
// the slot. Sub-cell quantum = cell/4, centered.
//
// This pass decodes ONE word back into its persistent 256x256 target (1 px
// = 1 cell). For the pixel's cell it re-projects all 16 sub-particle
// positions onto the screen (probe camera) and strobes every projection
// against the bright villager-green hue, then OR-merges the decoded word
// into the previous value. Anything not strobed keeps its old value; the
// stream self-heals on the next lap.
//
// v5.3: the old sky-depth rejection (dep < 1e-6) is GONE. 26.3 reversed-Z
// reads the bare sky as depth 0 and the carrier (translucent sheet) never
// writes depth, so the old gate silently blinded the decoder to every dot
// against the sky — the most visible ones, and the whole HI plane from
// ground views. The hue test alone is precise (sky is blue: g - b < 0),
// sky-background dots now decode AND get erased by the dust-mask pass.
// v5.3: the fresh THIS-frame word also rides the .b channel (never
// OR-merged) — dust_mask.fsh projects exactly those live sub-positions
// to build the eraser mask.

uniform sampler2D InSampler;      // minecraft:main color (dust hue strobe)
uniform sampler2D InDepthSampler; // minecraft:main depth (in-front-of-geometry test)
uniform sampler2D PrevSampler;    // previous plane target (persistent)
uniform sampler2D RotSampler;     // clod_rot_a (camera + fog channel)

// v5.0.3: SamplerInfo never existed as a client UBO — it stayed zeroed
// (the whole capture wrote cell (0,0), the march divided by zero).
// The screen size rides the STANDARD Globals block (ScreenSize), the
// target size is compile-time known.

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
    ivec2 px = ivec2(clamp(texCoord, 0.0, 1.0) * vec2(256.0));
    px = clamp(px, ivec2(0), ivec2(vec2(256.0)) - 1);

    // Probe camera: the LIVE camera from the sky camera-channel (clod_rot_a,
    // written by the sky override every frame): R = az/2pi,
    // G = (el+pi/2)/pi, B = conf, A = (fov-5)/115*254. The v5.0.0 hard-coded
    // bench pose (yaw=180, pitch=-5, fov=70) decoded NOTHING the moment the
    // player turned — the projections missed every particle.
    vec4 rotP = texelFetch(RotSampler, ivec2(0, 0), 0);
    if (rotP.b < 0.25) {
        // v5.3.1 BLIND-CAMERA GATE: the camera channel is measured from sky
        // pixels and its confidence decays once no sky is in view. Decoding
        // with a stale pose would OR random hue noise into the persistent
        // words and stamp garbage into the eraser mask. Pass the previous
        // state through instead; the stream self-heals on the next sky view
        // (confidence jumps back to 1 on the first valid row, frame-exact).
        // The server-side spawn gate (stream.rs spawn_visible) stops feeding
        // dots into sky-blind frustums, so there is nothing to decode here
        // anyway — this gate keeps the buffers honest while conf decays.
        fragColor = texelFetch(PrevSampler, px, 0);
        return;
    }
    float az = rotP.r * 2.0 * PI;
    float el = rotP.g * PI - PI * 0.5;
    float fov = 5.0 + rotP.a * 115.0;

    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));

    float tanH = tan(radians(fov) * 0.5);
    float aspect = ScreenSize.x / max(ScreenSize.y, 1.0);
    vec3 cam = vec3(CameraBlockPos) + CameraOffset;

    // v5.3 UN-FOG: the vanilla particle renderer fogs the carrier exactly
    // like terrain, so dots beyond the fog start fail the hue strobe (the
    // green channel sinks toward the blue fog color — measured live:
    // (112,149,139) instead of villager green). The fog state rides the
    // same camera channel (group B, identical to lod_terrain.fsh); the
    // loop un-fogs every strobe sample with the dot's own distance.
    vec4 fogCol = texelFetch(RotSampler, ivec2(1, 0), 0);
    vec4 fogRng = texelFetch(RotSampler, ivec2(0, 1), 0);
    bool fogLive = fogRng.a > 0.5 && fogCol.a > 0.004;
    float fStart = fogLive ? fogRng.r * 255.0 * 8.0 : 96.0;
    float fEnd = fogLive ? fogRng.g * 255.0 * 8.0 : 320.0;
    float vanRD = fogRng.b * 255.0 * 16.0;
    vec3 fogTint = fogLive ? fogCol.rgb : vec3(0.75, 0.85, 1.0);
    float fogOpacity = fogLive ? fogCol.a : 1.0;
    float invEndFogDelta = 1.0 / max(fEnd - fStart, 1.0);
    float startDelta = -fStart * invEndFogDelta;
    float endDistance = max(fogLive ? vanRD : 128.0, 320.0) * 1.7320508;
    float maxFog = clamp(endDistance * invEndFogDelta + startDelta, 0.0, 1.0);

    float cell = StreamInfo.x;
    float sub = StreamInfo.y;
    float near = StreamInfo.z;
    vec2 cellWorld = GridInfo.xy + (vec2(px) + 0.5) * cell;

    uint word = 0u;
    bool anyHit = false;

    // v5.1: one plane per pass — the HI word decodes on the high layer,
    // the LO word on the low layer (single carrier hue).
    float planeY = StreamInfo.w < 0.5 ? GridInfo.z : GridInfo.w;

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
            // STROBE v5.3.1: 5-tap cross (center + 4 axis taps at 0.75
            // texel) on the UN-FOGGED color. Tolerates +-0.75 px of camera
            // jitter / angular motion between the client's particle
            // projection and the rot-channel reconstruction (bilinear smear
            // widens effective coverage to ~1.5 px); first passing tap wins.
            // The dot's own distance z drives the same linear fog the
            // renderer applied; (1-f) is clamped so deep-fog noise never
            // explodes.
            vec2 texel = 1.0 / ScreenSize;
            float fog = clamp(z * invEndFogDelta + startDelta, 0.0, maxFog) * fogOpacity;
            bool dust = false;
            for (int t = 0; t < 5; ++t) {
                vec2 off = t == 0 ? vec2(0.0)
                    : t == 1 ? vec2(0.75 * texel.x, 0.0)
                    : t == 2 ? vec2(-0.75 * texel.x, 0.0)
                    : t == 3 ? vec2(0.0, 0.75 * texel.y)
                    : vec2(0.0, -0.75 * texel.y);
                vec3 raw = texture(InSampler, sc + off).rgb;
                vec3 pc = (raw - fogTint * fog) / max(1.0 - fog, 0.15);
                if (pc.g > 0.40 && pc.g > pc.r + 0.08 && pc.g > pc.b + 0.08) {
                    dust = true;
                    break;
                }
            }
            if (!dust) {
                continue;
            }
            word |= uint(qx) << (14 - 2 * qz);
            anyHit = true;
        }
    }

    vec4 prev = texelFetch(PrevSampler, px, 0);
    uint prevByte = uint(prev.r * 255.0 + 0.5);
    uint outByte = anyHit ? (prevByte | (word >> 8)) : prevByte;
    float mark = anyHit ? 1.0 : prev.a;
    // .b = the FRESH word (this frame, not OR-merged) for the eraser mask.
    fragColor = vec4(
        float(outByte) / 255.0,
        anyHit ? 1.0 : 0.0,
        float(word >> 8) / 255.0,
        mark
    );
}
