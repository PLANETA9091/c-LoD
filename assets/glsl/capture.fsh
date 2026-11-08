#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD v6.2 SKY-GRID BIT-STREAM capture pass.
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
// against the bright villager-green hue, then merges the decoded word into
// the previous value. Anything not strobed keeps its old value; the stream
// self-heals on the next lap.
//
// v6.2 EDIT PROTOCOL: the OR-merge can never LOSE bits, so a changed word
// would corrupt forever. Every edit therefore ships one RESET electron on
// a third plane (Y=100, between the data planes — mode 2 of this shader
// writes the clod_reset flag). A flagged cell REPLACES its word the first
// time the fresh decode completes (all four slots strobed); partial
// decodes keep the legacy OR self-heal, and once replaced, further ORs of
// the same word are idempotent for the rest of the electron's lifetime.
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
uniform sampler2D InDepthSampler; // minecraft:main depth (unused since v5.3, kept for the chain schema)
uniform sampler2D PrevSampler;    // previous plane target (persistent)
uniform sampler2D RotSampler;     // clod_rot_a (camera + fog channel)
uniform sampler2D ResetSampler;   // clod_reset (v6.2 edit-reset flags, this frame)

// v5.0.3: SamplerInfo never existed as a client UBO — it stayed zeroed
// (the whole capture wrote cell (0,0), the march divided by zero).
// The screen size rides the STANDARD Globals block (ScreenSize), the
// target size is compile-time known.

// x = origin_x, y = origin_z, z = HIGH layer y, w = LOW layer y
layout(std140) uniform ClodGrid {
    vec4 GridInfo;
};

// x = cell size, y = sub quantum (cell/4), z = device depth near (data
// modes) or the RESET plane Y (mode 2), w = word (0 = HI, 1 = LO, 2 = reset)
layout(std140) uniform ClodStream {
    vec4 StreamInfo;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;

// One cell word probe: re-project the cell's 16 sub-particle positions at
// `planeY`, strobe each against the carrier hue and OR the decoded 2-bit
// values into the word. anyHit = at least one sub-position strobed this
// frame; allSlots = every Z slot strobed (a COMPLETE word — the v6.2
// replace gate). The strobe is the v5.3.1 5-tap cross on the UN-FOGGED
// color; geometry must match stream.rs::word_spawns exactly.
void probe_word(vec2 cellWorld, float planeY, float sub, out uint word, out bool anyHit, out bool allSlots) {
    word = 0u;
    anyHit = false;
    allSlots = true;

    vec4 rotP = texelFetch(RotSampler, ivec2(0, 0), 0);
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

    bool slot0 = false;
    bool slot1 = false;
    bool slot2 = false;
    bool slot3 = false;

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
                allSlots = false;
                continue; // behind / too close
            }
            vec2 ndc = vec2(
                dot(d, R) / (z * tanH * aspect),
                dot(d, U) / (z * tanH)
            );
            if (abs(ndc.x) > 1.0 || abs(ndc.y) > 1.0) {
                allSlots = false;
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
            // v6.5 FOG-INVARIANT RING: the mean (g-b) of the 4 diagonal
            // texels ~3 px out. Fog is a per-depth lerp toward one color, so
            // it scales the dot and its backdrop IDENTICALLY — the (g-b)
            // contrast between them survives as contrast * (1-f) while every
            // absolute hue threshold dies (measured on CI run 41260665463:
            // day-lit core (0.46, 0.54, 0.70) — b-dominant, the sky-blue fog
            // wins the absolute gate even after the un-fog when group B is
            // missing). Sky ring (g-b) ~ -0.35, fogged carrier ~ +0.4*(1-f):
            // the margin 0.06 holds to f ~ 0.85, far past the decode band.
            // The mean damps a neighboring dot landing in one diagonal
            // (sub-quantum spacing ~3.5 px at 75 m: one polluted tap of
            // four costs a quarter of its contrast).
            vec2 scrPx = vec2(ScreenSize);
            ivec2 basePx = ivec2(clamp(sc * scrPx, vec2(0.0), scrPx - 1.0));
            float ringGB = 0.0;
            for (int d = 0; d < 4; ++d) {
                ivec2 o = d == 0 ? ivec2(3, 3)
                        : d == 1 ? ivec2(-3, 3)
                        : d == 2 ? ivec2(3, -3)
                        : ivec2(-3, -3);
                ivec2 q = clamp(basePx + o, ivec2(0), ivec2(scrPx) - 1);
                vec3 s = texelFetch(InSampler, q, 0).rgb;
                ringGB += s.g - s.b;
            }
            ringGB *= 0.25;
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
                // v6.5 two arms: the v5.3.1 absolute hue test on the
                // un-fogged color (precise when the group B fog payload is
                // alive) OR the fog-invariant local contrast (immune to a
                // dead payload and to the cylindrical-vs-forward fog model
                // mismatch). Neutrals fail the second arm on g > r; sky
                // fails on contrast ~ 0 (the ring IS the backdrop).
                // v6.20.0 UNIVERSAL LOCAL-CONTRAST GATE (#305 forensics):
                // BOTH arms now require the fog-invariant ring delta. The
                // absolute arm alone fired on every grass/foliage pixel a
                // probe landed on — vanilla grass reads g ~ 0.49 and passes
                // g > 0.40 && g > r + 0.08 && g > b + 0.08 — a standing
                // false-dot field the OR-merge locked into the persistent
                // words; the march then painted the garbage bytes as the
                // #305 far-band towers (black lattice-0 cells, wrong
                // heights, the purple = the 2x2 color average of red fill
                // next to false water). The §5b lanes stayed clean only
                // because the synthetic palette's green (g = 0.40) misses
                // the absolute gate by 0.005. The ring delta is the dot's
                // real signature, fog-invariant (fog scales the dot and
                // the ring identically): +0.7 over sky, +0.13 over grass,
                // +0.4 over stone, +0.83 over water; a backdrop false
                // positive reads 0.
                bool delta = (raw.g - raw.b) - ringGB > 0.06;
                bool absolute = delta && pc.g > 0.40 && pc.g > pc.r + 0.08 && pc.g > pc.b + 0.08;
                bool relative = delta && raw.g > raw.r + 0.02;
                if (absolute || relative) {
                    dust = true;
                    break;
                }
            }
            if (!dust) {
                allSlots = false;
                continue;
            }
            word |= uint(qx) << (14 - 2 * qz);
            anyHit = true;
            if (qz == 0) slot0 = true;
            else if (qz == 1) slot1 = true;
            else if (qz == 2) slot2 = true;
            else slot3 = true;
        }
    }
    // v6.20.0 ROW-COMPLETE = the word is complete: every slot's dot was
    // found in THIS frame (each found dot contributes its 2-bit column, so
    // a row without its dot decodes as column 0 = zero bits — garbage the
    // old allSlots could never catch: it ANDed the 16-probe grid where 12
    // probes are structurally empty and was therefore always false, which
    // also silently killed the v6.2 fresh-replace edit path). The old mark
    // = anyHit stamped a PARTIAL first decode as decoded forever — the OR
    // cannot heal rows whose dots are swallowed by terrain above the
    // stream planes (LAYER_Y = 80/120 vs the §7b world's 190 m summits),
    // so those cells painted garbage bytes for the whole session. The mark
    // now flips only on a row-complete frame.
    allSlots = slot0 && slot1 && slot2 && slot3;
}

void main() {
    ivec2 px = ivec2(clamp(texCoord, 0.0, 1.0) * vec2(256.0));
    px = clamp(px, ivec2(0), ivec2(vec2(256.0)) - 1);

    float cell = StreamInfo.x;
    float sub = StreamInfo.y;
    vec2 cellWorld = GridInfo.xy + (vec2(px) + 0.5) * cell;

    // Probe camera gate: the LIVE camera from the sky camera-channel
    // (clod_rot_a, written by the sky override every frame): R = az/2pi,
    // G = (el+pi/2)/pi, B = conf, A = (fov-5)/115*254. The v5.0.0
    // hard-coded bench pose (yaw=180, pitch=-5, fov=70) decoded NOTHING
    // the moment the player turned — the projections missed every particle.
    vec4 rotP = texelFetch(RotSampler, ivec2(0, 0), 0);
    // ---- v6.2 mode 2: the edit-reset plane --------------------------------
    // Probe the cell's 16 sub-positions at the reset Y (StreamInfo.z for
    // this pass) and flag the cell when ANY of them strobes. A reset
    // electron is a COMMAND, not data: one dot suffices and partial
    // strobes mean the same thing — unlike data words, nothing here may
    // OR-accumulate. A blind camera flags nothing (the data captures
    // decode nothing either).
    if (StreamInfo.w >= 1.5) {
        if (rotP.b < 0.25) {
            fragColor = vec4(0.0, 0.0, 0.0, 1.0);
            return;
        }
        uint w;
        bool hit;
        bool all;
        probe_word(cellWorld, StreamInfo.z, sub, w, hit, all);
        fragColor = vec4(hit ? 1.0 : 0.0, 0.0, 0.0, 1.0);
        return;
    }
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

    // v5.1: one plane per pass — the HI word decodes on the high layer,
    // the LO word on the low layer (single carrier hue).
    float planeY = StreamInfo.w < 0.5 ? GridInfo.z : GridInfo.w;

    uint word;
    bool anyHit;
    bool allSlots;
    probe_word(cellWorld, planeY, sub, word, anyHit, allSlots);

    vec4 prev = texelFetch(PrevSampler, px, 0);
    uint prevByte = uint(prev.r * 255.0 + 0.5);
    bool reset = texelFetch(ResetSampler, px, 0).r > 0.5;
    uint fresh = word >> 8;
    uint outByte = prevByte;
    if (anyHit) {
        // v6.2 EDIT PROTOCOL: after a reset electron, a COMPLETE fresh
        // decode REPLACES the OR-merged word (an edit must be able to
        // lose bits — a refresh cannot, that is what OR is for). Partial
        // decodes keep the legacy OR self-heal.
        outByte = (reset && allSlots) ? fresh : (prevByte | fresh);
    }
    float mark = allSlots ? 1.0 : prev.a;
    // .b = the FRESH word (this frame, not merged) for the eraser mask.
    fragColor = vec4(
        float(outByte) / 255.0,
        anyHit ? 1.0 : 0.0,
        float(fresh) / 255.0,
        mark
    );
}
