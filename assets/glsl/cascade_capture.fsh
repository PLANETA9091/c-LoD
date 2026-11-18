#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD v6.59 THE CASCADE — atlas capture (see src/cascade.rs for the full
// protocol). Three modes via StreamInfo.w:
//   3.0  BEACON (1x1 pass): probe the beacon slot at every ring's Z offset
//        and every rank; the decoded byte (w*17: high nibble = low nibble =
//        the window index) broadcasts the ACTIVE window to the atlas passes.
//   0.0  HI ATLAS (2048x256, 256 px per ring): decode the 6-slot height word
//        (4 height bits + 2 tag dots) of the fragment's ring cell.
//   1.0  LO ATLAS: decode the 4-slot color word; the height twin's rank
//        vouches for the cell identity (the color word carries no tag — it
//        streams with the height word at the same slot, the v6.36 pattern).
//
// GEOMETRY (must match src/cascade.rs exactly):
//   anchor   = floor(CameraBlockPos.xz / 128) * 128      (ANCHOR_GRID)
//   slot ctr = anchor + (slot + 0.5) * 2.0                (SCAFFOLD_PITCH)
//   sub      = 0.5                                        (SCAFFOLD_SUB)
//   zoff     = ringIdx * 4.0                              (ZSTRIDE)
//   HI word  : 6 dots, Z = (s - 2.5) * sub + zoff, X = (qx - 1.5) * sub,
//              dot s carries word bits (14 - 2s)..(15 - 2s) as its X value;
//              bits 8..15 = height byte, bits 4..7 = window tag.
//   LO word  : the ring-0 4-dot geometry + zoff.
//   window w = (cx/64) + (cz/64)*4 — the sixteen 64x64 windows partition
//              the 256x256 ring grid; a word is accepted only when its tag
//              equals the fragment's own window (the stateless lap gate).
//
// The rank walk mirrors capture.fsh (the capture cannot know the height
// before the word decodes; the first honest strobe wins, a ROW-COMPLETE
// tier stops the walk). The merge mirrors v6.23's honesty with ONE ring
// simplification: a complete decode REPLACES even a marked cell — the tag
// gate is the honesty proof for rings (ring words never carry resets).

uniform sampler2D InSampler;      // minecraft:main color (dust hue strobe)
uniform sampler2D InDepthSampler; // minecraft:main depth (the v6.25 gate)
uniform sampler2D PrevSampler;    // the twin atlas (persistent, ping-pong)
uniform sampler2D RotSampler;     // clod_rot_a (camera + fog channel)
uniform sampler2D RwinSampler;    // clod_rwin (the beacon's window byte)
uniform sampler2D HeightRefSampler; // the ring h twin (the LO cross-check)

// x = ring-0 disk origin x, y = origin z, z = HI plane y (legacy), w = LO
layout(std140) uniform ClodGrid {
    vec4 GridInfo;
};

// x = cell size, y = sub quantum, z = device depth near, w = mode
// (3.0 beacon, 0.0 HI atlas, 1.0 LO atlas)
layout(std140) uniform ClodStream {
    vec4 StreamInfo;
};

// The rank ladders (must match src/stream.rs / capture.fsh exactly).
layout(std140) uniform ClodTiers {
    vec4 TiersLo;
    vec4 TiersHi;
    vec4 TiersRs;
};

float tierY(vec4 ladders, int r) {
    return r == 0 ? ladders.x : r == 1 ? ladders.y : r == 2 ? ladders.z : ladders.w;
}

// stream.rs::tier_rank mirrored (the v6.23 honesty gate).
int surface_rank(float surface) {
    float h = surface + 2.0;
    for (int r = 0; r < 4; ++r) {
        if (tierY(TiersLo, r) > h) {
            return r;
        }
    }
    return 3;
}

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;

// The cascade word probe: `slots` Z rows x 4 X subs, every dot shifted
// +zoff along Z. Strobe + fog + mirror arms are capture.fsh's verbatim.
void probe_cword(vec2 slotCtr, float planeY, float zoff, int slots, float sub,
                 float depthNear, out uint word, out bool anyHit, out bool allSlots,
                 out float seen) {
    word = 0u;
    anyHit = false;
    allSlots = true;
    seen = 0.0;

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

    vec2 texel = 1.0 / ScreenSize;
    float rowComplete = 0.0;
    for (int qz = 0; qz < slots; ++qz) {
        bool rowHit = false;
        for (int qx = 0; qx < 4; ++qx) {
            vec3 P = vec3(
                slotCtr.x + (float(qx) - 1.5) * sub,
                planeY,
                slotCtr.y + (float(qz) - (float(slots) - 1.0) * 0.5) * sub + zoff
            );
            vec3 d = P - cam;
            float z = dot(d, F);
            if (z < 2.0) {
                continue;
            }
            vec2 ndc = vec2(
                dot(d, R) / (z * tanH * aspect),
                dot(d, U) / (z * tanH)
            );
            if (abs(ndc.x) > 1.0 || abs(ndc.y) > 1.0) {
                continue;
            }
            vec2 sc = ndc * 0.5 + 0.5;
            seen += 1.0; // the projection landed in the frustum
            vec2 scrPx = vec2(ScreenSize);
            ivec2 basePx = ivec2(clamp(sc * scrPx, vec2(0.0), scrPx - 1.0));
            float ringGB = 0.0;
            float ringLum = 0.0;
            for (int d2 = 0; d2 < 4; ++d2) {
                ivec2 o = d2 == 0 ? ivec2(3, 3)
                        : d2 == 1 ? ivec2(-3, 3)
                        : d2 == 2 ? ivec2(3, -3)
                        : ivec2(-3, -3);
                ivec2 q = clamp(basePx + o, ivec2(0), ivec2(scrPx) - 1);
                vec3 s = texelFetch(InSampler, q, 0).rgb;
                ringGB += s.g - s.b;
                ringLum += s.r + s.g + s.b;
            }
            ringGB *= 0.25;
            ringLum *= 0.25 / 3.0;
            float zScene = depthNear / max(texture(InDepthSampler, sc).r, 1.0e-6);
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
                bool delta = (raw.g - raw.b) - ringGB > 0.06;
                bool absolute = delta && pc.g > 0.40 && pc.g > pc.r + 0.08 && pc.g > pc.b + 0.08;
                bool relative = delta && raw.g > raw.r + 0.02;
                float lum = (raw.r + raw.g + raw.b) * (1.0 / 3.0);
                bool mirror = depthNear > 0.0
                    && (ringLum - lum) > 0.06
                    && abs(raw.g - raw.b) <= 0.08
                    && abs(raw.g - raw.r) <= 0.08
                    && zScene >= z;
                if (absolute || relative || mirror) {
                    dust = true;
                    break;
                }
            }
            if (!dust) {
                continue;
            }
            word |= uint(qx) << (14 - 2 * qz);
            rowHit = true;
            anyHit = true;
        }
        if (rowHit) {
            rowComplete += 1.0;
        }
    }
    allSlots = rowComplete >= float(slots);
}

// The anchor + slot world position for a ring cell fragment.
vec2 slot_center(vec2 pxCell, out int ringIdx, out float wExpected) {
    ringIdx = int(floor(pxCell.x / 256.0));
    float cx = mod(pxCell.x, 256.0);
    float cz = pxCell.y;
    wExpected = floor(cx / 64.0) + floor(cz / 64.0) * 4.0;
    vec2 slot = vec2(mod(cx, 64.0), mod(cz, 64.0));
    vec2 anchor = floor(vec2(CameraBlockPos) / 128.0) * 128.0;
    return anchor + (slot + 0.5) * 2.0;
}

void main() {
    float mode = StreamInfo.w;

    // ---- mode 3: the beacon (1x1 pass) -------------------------------------
    if (mode >= 2.5) {
        // The beacon slot's world position (ring-independent: zoff 0 slot
        // position + the per-ring Z shifts are probed in the loop below).
        vec2 anchor = floor(vec2(CameraBlockPos) / 128.0) * 128.0;
        vec2 ctr = anchor + vec2(63.5, 63.5) * 2.0;
        vec4 rotP = texelFetch(RotSampler, ivec2(0, 0), 0);
        uint byte = 0u;
        bool got = false;
        float bestSeen = 0.0;
        for (int k = 0; k < 6 && !got; ++k) {
            for (int r = 0; r < 4 && !got; ++r) {
                uint w;
                bool hit;
                bool all;
                float sn;
                probe_cword(ctr, tierY(TiersHi, r), float(k) * 4.0, 4, StreamInfo.y,
                            0.0, w, hit, all, sn);
                bestSeen = max(bestSeen, sn);
                if (hit) {
                    byte = w >> 8;
                    got = true;
                }
            }
        }
        // The checksum: byte = w*17 (high nibble = low nibble = w).
        float wOut = 0.0;
        float conf = 0.0;
        if (got) {
            float hi_n = floor(float(byte) / 16.0);
            float lo_n = mod(float(byte), 16.0);
            if (hi_n == lo_n && hi_n < 16.0) {
                wOut = hi_n;
                conf = 1.0;
            }
        }
        // v6.59.8b: b carries the projection-aliveness fraction (the max
        // on-screen probe count / 16) — 0 = every projection missed the
        // screen (a camera/anchor mismatch), 1 = the dots were projected
        // and still produced no honest word (the detector or the dots).
        fragColor = vec4(wOut / 16.0, conf, bestSeen / 16.0, 1.0);
        return;
    }

    // ---- atlas modes: 2048x256 target, 256 px per ring ----------------------
    vec2 tsz = vec2(2048.0, 256.0);
    ivec2 px = ivec2(clamp(texCoord, 0.0, 1.0) * tsz);
    px = clamp(px, ivec2(0), ivec2(tsz) - ivec2(1));

    int ringIdx;
    float wExp;
    vec2 ctr = slot_center(vec2(px), ringIdx, wExp);
    float zoff = float(ringIdx) * 4.0;
    vec4 prev = texelFetch(PrevSampler, px, 0);

    // The beacon's early-out: when the broadcast is confident and names a
    // DIFFERENT window, this cell cannot decode this lap — 15/16 of the
    // atlas keeps its value with one fetch (the llvmpipe budget).
    vec4 rwin = texelFetch(RwinSampler, ivec2(0, 0), 0);
    bool rwinLive = rwin.g > 0.5;
    if (rwinLive && floor(rwin.r * 16.0 + 0.5) != wExp) {
        fragColor = prev;
        return;
    }

    vec4 rotP = texelFetch(RotSampler, ivec2(0, 0), 0);
    if (rotP.b < 0.25) {
        fragColor = prev; // blind camera: the v5.3.1 passthrough
        return;
    }

    bool hiPass = mode < 0.5;
    vec4 ladders = hiPass ? TiersHi : TiersLo;
    int slots = hiPass ? 6 : 4;

    uint stash = 0u;
    bool stashed = false;
    bool stashAll = false;
    for (int r = 0; r < 4; ++r) {
        uint w;
        bool hit;
        bool all;
        float sn;
        probe_cword(ctr, tierY(ladders, r), zoff, slots, StreamInfo.y,
                    StreamInfo.z, w, hit, all, sn);
        if (!hit) {
            continue;
        }
        uint fresh = w >> 8;
        bool honest;
        if (hiPass) {
            // The word's own tag MUST name this fragment's window (the
            // stateless lap gate). The v6.23 rank equality is DROPPED for
            // rings (run #508 forensics): the engine paints the word on the
            // slot's LOCAL rank plane (the eye-band geometry, 28..112 m),
            // which has no reason to equal the far cell's height rank — the
            // rank gate rejected every rank-1/2/3 cell forever and the atlas
            // stayed empty. The rings' merge is safe without it: a complete
            // decode replaces the cell outright, partials never set the
            // mark, so no ratchet class can poison a marked byte.
            float tag = floor(float((w >> 4) & 0xFu) + 0.5);
            honest = tag == wExp;
        } else {
            // The color word carries no tag: the height twin vouches (same
            // slot, same cell, streamed together — the v6.36 pattern). The
            // rank equality drops with the HI gate (same forensics).
            vec4 hpx = texelFetch(HeightRefSampler, px, 0);
            honest = hpx.a >= 0.5;
        }
        if (!honest) {
            continue;
        }
        if (all) {
            stash = w;
            stashed = true;
            stashAll = true;
            break;
        }
        if (!stashed) {
            stash = w;
            stashed = true;
        }
    }

    uint word = stash;
    bool anyHit = stashed;
    bool allSlots = stashAll;

    // ---- the merge (v6.23 honesty, ring simplification) ---------------------
    uint prevByte = uint(prev.r * 255.0 + 0.5);
    uint fresh = word >> 8;
    bool marked = prev.a > 0.5;
    uint outByte = prevByte;
    if (allSlots) {
        // A complete decode is tag-verified (HI) or height-twin-vouched (LO)
        // — it replaces everything, marked or not: ring words never carry
        // resets, the complete decode IS the reset for rings.
        outByte = fresh;
    } else if (anyHit && !marked) {
        outByte = prevByte | fresh;
    }
    float mark = allSlots ? 1.0 : prev.a;
    float eraseFlag = (allSlots || prev.a > 0.5) ? 1.0 : 0.0;
    fragColor = vec4(
        float(outByte) / 255.0,
        eraseFlag,
        float(fresh) / 255.0,
        mark
    );
}
