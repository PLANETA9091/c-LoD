#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD pass 2 (v4.4.4): decode the camera channel into the persistent
// rotation target.
//
// Picks the topmost intact payload row across all col_scan slices and packs
// the raw sky.fsh payload bytes into clod_rot_b (1x5, RGBA8), which the
// ping-pong copy promotes to clod_rot_a for clod:post/lod_terrain:
//
//   px0 = (Rx_hi, Rx_lo, Ry_hi, Ry_lo)   bobbed world-space right
//   px1 = (Rz_hi, Rz_lo, Ux_hi, Ux_lo)
//   px2 = (Uy_hi, Uy_lo, Uz_hi, Uz_lo)   bobbed world-space up
//   px3 = (fov_hi, fov_lo, tx, ty)       fov 16-bit, eye wobble 8-bit
//   px4 = (conf, day, res, gap)          conf 0..1, day = sky grade 0..1,
//                                        res = glide residual 0..1,
//                                        gap = frames since payload, 0..1
//
// The payload is stored ENCODED (not decoded to floats): the persistent
// target is RGBA8 and 16-bit basis components must survive the round trip
// bit-exact or the LOD horizon would swim by half a pixel. The LOD pass
// decodes.
//
// ADAPTIVE GLIDE (v4.4.2, retuned v4.4.4): while the payload flows frame to
// frame, the fresh bytes are forwarded BIT-EXACT (no float round trip, zero
// lag). After an occlusion gap the held basis is stale by however far the
// camera turned meanwhile; forwarding it verbatim makes the LOD horizon SNAP
// the moment any group column sees sky again. When the fresh right vector
// differs from the held one by more than ~3 degrees, blend 60%/frame
// instead: the horizon glides back over ~4 frames, then hands over to the
// exact path once the angle drops under the threshold. The 3-degree gate
// keeps every continuous tracking frame (up to ~180 deg/s pans) on the
// exact path — the glide only covers genuine gaps. v4.4.4 drops the old
// confidence gate on the glide: the never-solved state (zeroed target) is
// already excluded by the basis-length checks, and a long (>13 s) occlusion
// used to fall back to a raw snap.
//
// v4.4.4 GLIDE RESIDUAL (px4.b): a gliding basis is by definition the WRONG
// pose for the current frame — showing it was the residual flicker the
// adaptive glide never removed (the LOD horizon visibly tilted and swept
// for up to 8 frames after every look-down/look-up cycle). The solver now
// measures the angle between the blended basis and the fresh one after the
// blend and publishes it scaled by the fade window (12 deg full wrong ->
// 0 deg exact). lod_terrain holds the LOD back by that residual: the sky
// pixels show the real sky for the first frames of the glide, then the LOD
// materializes as the pose converges. A tilted world is never shown.
//
// v4.4.5 GAP COUNTER + LATCHED GLIDE (px4.a): the 3-degree glide gate had a
// hole — it fired on ANY fresh-vs-held angle above 3 degrees, and during
// continuous tracking that angle is one FRAME of camera motion. A fast pan
// (flicks go well past 180 deg/s) therefore glided on live data: the LOD
// lagged behind the true pose, dimmed by the residual fade and visibly
// "rotated" while turning — the flicker got WORSE after v4.4.4. The gate is
// now latched: it only arms when the gap counter (px4.a) says a gap actually
// happened (or the previous residual is still converging), so tracking
// frames forward the fresh basis BIT-EXACT at any pan speed. The gap counter
// itself (ramps 0.12/frame occluded, decays 0.3/frame alive) is published
// for lod_terrain: while no group column sees sky the held basis is stale by
// however far the camera turned, and the LOD now DISSOLVES into the sky
// instead of showing a tilted world. Glide retune: K 0.6 -> 0.8 (converge in
// ~3 frames), residual fade window 12 -> 5 degrees (the half-faded tail of
// the old 12-degree window still showed a 3-6 degree tilt).
//
// v4.4.6 GAP RETUNE + HISTORY OSCILLOSCOPE: the v4.4.5 gap fade (0.12/frame)
// punished SHORT payload dropouts that are harmless while the camera holds
// still (a cloud pixel crossing a column): the LOD visibly "breathed" every
// few seconds — worse than the artifact it replaced. Constants retuned:
// grace now ~5 frames, dissolve ~16 frames (fog-like), materialize ~7
// frames. px5 = ring-buffer write cursor (slot/255, byte-exact round trip),
// px6..px261 = 256-frame history of (conf, day, res, gap) written every
// frame; lod_terrain's debug HUD (debug_lod=4) draws the trace so a single
// screenshot shows exactly what the camera channel was doing.
//
// Confidence jumps to 1 on the first valid row; while the column is fully
// occluded the previous payload is kept and confidence decays at 0.995/frame
// (v4.4.4, was 0.99 — the glide+fade path now covers gaps up to ~13 s).
// Since v4.4.1 confidence is diagnostics + the never-solved init gate only —
// it no longer fades the LOD.
//
// v4.4.3 DAY BYTE (px4.g): post pipelines cannot declare the vanilla Fog UBO
// (PostChain builds the bind group layout from inputs + SamplerInfo + JSON
// uniforms only — BindGroupLayouts.FOG is world-pipeline-only; declaring Fog
// in a post shader fails the whole pack with "Unable to find shader defined
// uniform (Fog)"). The day grade therefore rides the camera channel: the
// topmost intact payload row is PURE SKY by construction (clouds, stars and
// the moon break the payload checksum, so those rows never win), and its 19
// pixels sit at one sky altitude. Average their RGB luminance, map through
// the day window, exponentially smooth against the persistent byte. This
// tracks the real sun clock — Globals.GameTime counts raw level ticks while
// the 26.3 sun runs on the WorldClock timeline, so the two diverge after
// beds, /time set or a paused daylight cycle.

uniform sampler2D ScanSampler; // clod:clod_scan (5120x1 slice report)
uniform sampler2D InSampler;   // minecraft:main (payload rows)
uniform sampler2D PrevSampler; // clod:clod_rot_a (1x5 persistent)

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const int SLICES = 5120;
const float GLIDE_DOT = 0.9986; // cos(~3 deg): below this the held basis is stale
const float GLIDE_K = 0.8;      // v4.4.5: 0.6 -> 0.8, converge in ~3 frames
const float FADE_WIN = 0.0873;  // radians(5 deg): residual -> LOD fade scale
const float GAP_UP = 0.05;      // v4.4.6: 0.12 -> 0.05, grace ~5 frames
const float GAP_DOWN = 0.15;    // v4.4.6: 0.3 -> 0.15, materialize ~7 frames

// Day-grade window (luminance of the sky): night ~0.03-0.06, dawn/dusk ramps
// through it, day >= 0.45 (deep-blue zenith and rain-gray sky included).
const float DAY_LO = 0.09;
const float DAY_HI = 0.30;
const float DAY_SMOOTH = 0.1; // per-frame exponential smoothing of the day byte

// (hi,lo) big-endian 16-bit byte pair from RGBA8 (stored n/255), with the
// (c+1)/2 * 65535 signed remap used by the camera channel.
float c16(float hi, float lo) {
    return (hi * 65280.0 + lo * 255.0) / 65535.0 * 2.0 - 1.0;
}

ivec2 enc16(float c) {
    float q = floor(clamp((c + 1.0) * 0.5, 0.0, 1.0) * 65535.0 + 0.5);
    return ivec2(int(floor(q / 256.0)), int(mod(q, 256.0)));
}

float enc8s(float c) {
    return floor(clamp((c + 1.0) * 0.5, 0.0, 1.0) * 255.0 + 0.5);
}

void main() {
    int j = int(gl_FragCoord.x); // 0..4, one output pixel per fragment

    int best = -1;
    for (int s = 0; s < SLICES; ++s) {
        vec4 sc = texelFetch(ScanSampler, ivec2(s, 0), 0);
        if (sc.r > 0.5) {
            int y = int(sc.g * 255.0 + 0.5) * 256 + int(sc.b * 255.0 + 0.5);
            if (y > best) {
                best = y;
            }
        }
    }

    vec4 prev = texelFetch(PrevSampler, ivec2(j, 0), 0);
    vec4 pc = texelFetch(PrevSampler, ivec2(4, 0), 0);

    // v4.4.5 gap counter (px4.a): frames since the last intact payload row,
    // saturating at 1. The grace (gap < 0.25 -> no fade in lod_terrain)
    // keeps single-frame dropout noise (a cloud pixel crossing a column)
    // from dimming the LOD.
    float gapPrev = pc.a;
    float gap = best >= 0 ? max(gapPrev - GAP_DOWN, 0.0)
                          : min(gapPrev + GAP_UP, 1.0);

    // Payload bytes x2..x17 (16 bytes, see assets/glsl/sky.fsh):
    //   b[0..5]  right xyz (16-bit each)   b[6..11] up xyz (16-bit each)
    //   b[12..13] fov 16-bit               b[14] tx  b[15] ty (8-bit signed)
    // Needed by BOTH the diagnostics pixel (residual) and the basis pixels.
    float b[16];
    vec3 Rp = vec3(0.0);
    vec3 Rn = vec3(0.0);
    float k = 1.0; // blend factor: 1 = bit-exact forward, GLIDE_K = gliding
    if (best >= 0) {
        for (int i = 0; i < 16; ++i) {
            b[i] = texelFetch(InSampler, ivec2(2 + i, best), 0).a * 255.0;
        }
        vec4 p0 = texelFetch(PrevSampler, ivec2(0, 0), 0);
        vec4 p1 = texelFetch(PrevSampler, ivec2(1, 0), 0);
        Rp = vec3(c16(p0.x, p0.y), c16(p0.z, p0.w), c16(p1.x, p1.y));
        Rn = vec3(c16(b[0] / 255.0, b[1] / 255.0), c16(b[2] / 255.0, b[3] / 255.0),
                  c16(b[4] / 255.0, b[5] / 255.0));
        // Adaptive glide gate (see header). A zeroed (never-solved) target
        // has length ~0 and is caught by the length checks.
        float lp = length(Rp);
        float ln = length(Rn);
        // v4.4.5 LATCHED GATE: glide only when a gap actually happened
        // (gapPrev) or one is still converging (prev residual). During
        // continuous tracking both are zero — the fresh basis is exact and
        // is forwarded bit-exact at ANY pan speed.
        if (lp > 0.5 && ln > 0.5
            && (gapPrev > 0.001 || pc.b > 0.002)
            && dot(Rp, Rn) / (lp * ln) < GLIDE_DOT) {
            k = GLIDE_K;
        }
    }

    // v4.4.6: diagnostics shared by px4 and the history ring — identical for
    // every fragment, computed once.
    float conf = best >= 0 ? min(pc.r + 0.08, 1.0) : pc.r * 0.995;
    float day = pc.g;
    float res = pc.b;
    if (best >= 0) {
        vec3 skySum = vec3(0.0);
        for (int i = 0; i < 19; ++i) {
            skySum += texelFetch(InSampler, ivec2(i, best), 0).rgb;
        }
        float dayRaw = smoothstep(DAY_LO, DAY_HI,
            dot(skySum / 19.0, vec3(0.299, 0.587, 0.114)));
        float dayByte = floor(clamp(dayRaw, 0.0, 1.0) * 255.0 + 0.5);
        day = pc.r > 0.02
            ? floor(mix(pc.g * 255.0, dayByte, DAY_SMOOTH) + 0.5)
            : dayByte;
        if (k < 1.0) {
            // Angle between the blended basis and the fresh one AFTER this
            // frame's blend, over the fade window (5 deg = fully wrong).
            float d = clamp(dot(normalize(mix(Rp, Rn, k)), normalize(Rn)),
                            -1.0, 1.0);
            res = clamp(acos(d) / FADE_WIN, 0.0, 1.0);
        } else {
            res = 0.0;
        }
    }

    // v4.4.6 HISTORY RING: px5 = write cursor (slot/255, byte-exact round
    // trip), px6..px261 = 256-frame trace of (conf, day, res, gap). Slots
    // that are not the current one forward their previous contents.
    int slot = (int(texelFetch(PrevSampler, ivec2(5, 0), 0).r * 255.0 + 0.5) + 1) % 256;

    if (j == 5) {
        fragColor = vec4(float(slot) / 255.0, 0.0, 0.0, 0.0);
        return;
    }
    if (j >= 6) {
        fragColor = (j - 6 == slot) ? vec4(conf, day, res, gap) : prev;
        return;
    }

    if (j == 4) {
        fragColor = vec4(conf, day, res, gap);
        return;
    }

    if (best < 0) {
        fragColor = prev; // keep the last good basis
        return;
    }

    if (k >= 1.0) {
        if (j == 0) {
            fragColor = vec4(b[0], b[1], b[2], b[3]) / 255.0;
        } else if (j == 1) {
            fragColor = vec4(b[4], b[5], b[6], b[7]) / 255.0;
        } else if (j == 2) {
            fragColor = vec4(b[8], b[9], b[10], b[11]) / 255.0;
        } else {
            fragColor = vec4(b[12], b[13], b[14], b[15]) / 255.0;
        }
        return;
    }

    // Glide path: blend the pair in float space, re-orthonormalize, re-encode.
    vec3 Rb = normalize(mix(Rp, Rn, k));
    vec4 p1 = texelFetch(PrevSampler, ivec2(1, 0), 0);
    vec4 p2 = texelFetch(PrevSampler, ivec2(2, 0), 0);
    vec4 p3 = texelFetch(PrevSampler, ivec2(3, 0), 0);
    vec3 Um = mix(vec3(c16(p1.z, p1.w), c16(p2.x, p2.y), c16(p2.z, p2.w)),
                  vec3(c16(b[6] / 255.0, b[7] / 255.0), c16(b[8] / 255.0, b[9] / 255.0),
                       c16(b[10] / 255.0, b[11] / 255.0)), k);
    vec3 Ub = normalize(Um - Rb * dot(Rb, Um));

    // fov (16-bit, marker 65535 = invalid) and the 8-bit signed eye wobble.
    float fovP = p3.x * 65280.0 + p3.y * 255.0;
    float fovN = b[12] * 65280.0 + b[13] * 255.0;
    float fov;
    if (fovP > 65534.5 || fovN > 65534.5) {
        fov = fovN > 65534.5 ? 70.0 : 5.0 + fovN / 65535.0 * 115.0;
    } else {
        fov = mix(5.0 + fovP / 65535.0 * 115.0, 5.0 + fovN / 65535.0 * 115.0, k);
    }
    float tx = mix(p3.z * 2.0 - 1.0, b[14] / 255.0 * 2.0 - 1.0, k);
    float ty = mix(p3.w * 2.0 - 1.0, b[15] / 255.0 * 2.0 - 1.0, k);

    ivec2 e;
    if (j == 0) {
        e = enc16(Rb.x);
        fragColor = vec4(float(e.x), float(e.y), 0.0, 0.0) / 255.0;
        e = enc16(Rb.y);
        fragColor.z = float(e.x) / 255.0;
        fragColor.w = float(e.y) / 255.0;
    } else if (j == 1) {
        e = enc16(Rb.z);
        fragColor = vec4(float(e.x), float(e.y), 0.0, 0.0) / 255.0;
        e = enc16(Ub.x);
        fragColor.z = float(e.x) / 255.0;
        fragColor.w = float(e.y) / 255.0;
    } else if (j == 2) {
        e = enc16(Ub.y);
        fragColor = vec4(float(e.x), float(e.y), 0.0, 0.0) / 255.0;
        e = enc16(Ub.z);
        fragColor.z = float(e.x) / 255.0;
        fragColor.w = float(e.y) / 255.0;
    } else {
        float fovq = floor(clamp((fov - 5.0) / 115.0, 0.0, 1.0) * 65534.0 + 0.5);
        fragColor = vec4(floor(fovq / 256.0), mod(fovq, 256.0), enc8s(tx), enc8s(ty)) / 255.0;
    }
}
