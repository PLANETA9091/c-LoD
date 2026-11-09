#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>
// __MESH_DEBUG__

// c-LoD pass (v5.0 "SKY-GRID BIT-STREAM"): the actual level-of-distant-detail
// renderer.
//
// For every pixel that shows sky (device depth ~ 0) we march a view ray
// against the streamed heightfield. The resource pack carries ZERO world
// data: the heightfield lives in two persistent post-chain target pairs
// (clod_h_a/b heights, clod_c_a/b colors, 256x256, 1 px = 1 cell) that the
// earlier capture passes of this very chain fill every frame by decoding the
// dust-particle sky-grid (see assets/glsl/capture.fsh). The data arrives as
// vanilla dust particles — the client received it with zero downloads after
// the one-time pack push.
//
// Geometry (must match src/stream.rs exactly):
//   cell = floor((xz - GridOrigin) / CELL)                      (0..255)
//   height pixel = HeightSampler[cell]  (R byte - 64)
//   color pixel  = ColBufSampler[cell]  (R byte = 6x6x6 lattice idx, 216 = water)
//
// Shading reproduces the Voxy color model exactly, as in v4.2:
//   color = albedo x vanillaFaceShade x lightmap(0, 15)
//   ssao  = voxy post/ssao.comp BETTER path ported onto the heightfield
//           (v6.8.1 — every constant cites its ssao.comp line below)
//   fog   = Voxy NormalRenderPipeline.finish over the VANILLA environmental
//           pair toward the VANILLA FogColor;
//   fade  = Voxy FogMode.FADE MODE=1: linear alpha fade over the last 10% of
//           the hit region's span, horizontal distance only.
//
// The vanilla inputs reach this pass through the CAMERA CHANNEL (sky.fsh):
// group A carries the view basis, group B carries FogColor RGBA,
// FogEnvironmentalStart/8, FogEnvironmentalEnd/8 and FogRenderDistanceEnd/16.
// The light term is the exact vanilla 26.3 overworld lightmap texel (0,15)
// reconstructed from the day.json keyframes driven by the live fog color.
//
// v6.19 G8d MESH PAINT: non-sky pixels no longer pass through untouched.
// The vanilla fragment's ray distance is reconstructed from the device depth,
// the heightfield is marched up to it, and a hit paints the unified march
// look OVER the vanilla pixel (mesh_paint=true): our whole frame becomes one
// representation — the march — exactly like the voxy leg renders its mesh
// everywhere it holds data. Where vanilla is genuinely closer, vanilla keeps
// the pixel. mesh_paint=false keeps the pre-G8d behavior byte-for-byte.

uniform sampler2D InSampler;      // minecraft:main color
uniform sampler2D InDepthSampler; // minecraft:main depth
uniform sampler2D HeightSampler;  // clod_h_b — streamed height bytes (1 px = 1 cell)
uniform sampler2D ColBufSampler;  // clod_c_b — streamed color index bytes (1 px = 1 cell)
uniform sampler2D RotSampler;     // clod:clod_rot_a (2x2 persistent, camera + fog channel)
uniform sampler2D MaskSampler;    // clod_mask — dust eraser mask (this frame, v5.3)

// v5.0.3: SamplerInfo never existed as a client UBO — it stayed zeroed
// (the whole capture wrote cell (0,0), the march divided by zero).
// The screen size rides the STANDARD Globals block (ScreenSize), the
// target size is compile-time known.

// x = disk origin x, y = disk origin z, z = plane HI y, w = plane LO y
layout(std140) uniform ClodGrid {
    vec4 GridInfo;
};

// x = region blocks, y = cell size, z = sea level, w = max terrain height
layout(std140) uniform ClodRegion {
    vec4 RegionInfo;
};

// x = lod near (blocks), y = max terrain height, z = sea level, w = min confidence
layout(std140) uniform ClodParams {
    vec4 LodParams;
};

// STATIC fallback fog (x = start, y = end) used only before the first valid
// fog payload row arrives; live fog replaces it the moment the sky is seen.
layout(std140) uniform ClodFog {
    vec4 FogParams;
};

// v6.19 G8d: x = 26.3 projection near plane (device depth linearization,
// live-verified: d = near/eyeZ, infinite far — the same constant the capture
// passes decode with), y = mesh paint enable (1.0 = the march depth-tests
// and paints over the vanilla band).
layout(std140) uniform ClodPaint {
    vec4 PaintParams;
};

layout(std140) uniform ClodColors {
    vec4 ColLow;    // fallback low terrain
    vec4 ColHigh;   // fallback high terrain
    vec4 ColWater;  // fallback water surface
    vec4 VeilColor; // fallback fog tint (rgb) + snow whiteness (a)
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;
const int MAX_STEPS = 260;
const int BISECT = 7;
const float FADE_START = 0.9; // Voxy: fade begins at 90% of the view radius

// ---- vanilla 26.3 overworld lightmap texel (0,15) --------------------------
// data/minecraft/timeline/day.json, tracks visual/sky_light_color (+_factor),
// modifier multiply over the defaults (white / 1.0), linear keyframe lerp
// (ARGB::srgbLerp). Keyframes: 730 white/1.0, 11270 white/1.0,
// 13140 #7a7aff/0.24, 22860 #7a7aff/0.24, wrap to 730 (+24000).
vec3 vanilla_lightmap0_15(float tick) {
    const vec3 DAY = vec3(1.0);
    const vec3 NIGHT = vec3(122.0, 122.0, 255.0) / 255.0; // #7a7aff
    float t = tick;
    if (t >= 24000.0) t -= 24000.0;
    vec3 col;
    float fac;
    if (t < 11270.0) {
        col = DAY; fac = 1.0;
    } else if (t < 13140.0) {
        float a = (t - 11270.0) / 1870.0;
        col = mix(DAY, NIGHT, a); fac = mix(1.0, 0.24, a);
    } else if (t < 22860.0) {
        col = NIGHT; fac = 0.24;
    } else {
        float a = (t - 22860.0) / 1870.0;
        col = mix(NIGHT, DAY, a); fac = mix(0.24, 1.0, a);
    }
    return col * fac;
}

// Day-time reconstruction from the live fog color — see the long comment in
// clod:post/rot_solve (the resolve_day_tick function).
const vec3 FOG_BASE_DAY = vec3(192.0, 216.0, 255.0) / 255.0;  // #c0d8ff
const vec3 FOG_SCALAR_NIGHT = vec3(15.0, 15.0, 22.0) / 255.0; // #0f0f16

float resolve_day_tick(vec3 fogRgb, float prevTick, bool prevValid) {
    vec3 s = fogRgb / FOG_BASE_DAY;
    vec3 dvec = FOG_SCALAR_NIGHT - vec3(1.0);
    float a = clamp(dot(s - vec3(1.0), dvec) / dot(dvec, dvec), 0.0, 1.0);
    float tEve = a <= 0.0 ? 6000.0 : (a >= 1.0 ? 18000.0 : 11867.0 + a * 1803.0);
    float tMor = a <= 0.0 ? 6000.0 : (a >= 1.0 ? 18000.0 : 24133.0 - a * 1803.0);
    if (prevValid) {
        return abs(tEve - prevTick) <= abs(tMor - prevTick) ? tEve : tMor;
    }
    return tEve;
}

// ---- streamed heightfield: the fixed disk ----------------------------------
// One streamed cell = one flat Voxy-style quad. Base reads are NEAREST
// (texelFetch on the integer cell coordinate) — bilinear smearing was the
// v4.2 "textures overlay badly" bug and is never coming back.
//
// v5.2.0 DH-style distance smoothing: past ~100 blocks a 2-block cell is
// ~1 px on screen and NEAREST reads alias into a speckled wall (v5.1.1
// 400 m finding). Instead of bilinear (the v4.2 bug) we average a 2x2 cell
// neighborhood of DECODED values, valid cells only, blended in by a
// distance weight. Two explicit LOD levels in-shader, no mip chain, the
// data frontier stays exact (void neighbors are skipped, never smeared).

// NEAREST detail within 96 blocks, 2x2 average over 96..288, and a wider
// +-2 cross average past 288 (v5.2.1: second DH-style LOD level — one
// 2x2 step is still ~1 texel at 400 m). Both weights are computed from
// the ray distance by the caller and passed in.
float smooth_w(float t) {
    return smoothstep(96.0, 288.0, t);
}

// Wide-average weight: fades the +-2 cross in over 288..448 blocks.
float smooth_w2(float t) {
    return smoothstep(288.0, 448.0, t);
}

// Sample the streamed ground at a world xz. Returns the surface Y, or -1e8
// for "no data" (void, never decoded). Water cells stream their real
// surface height; the water FLAG lives in the color word (idx 216).
// v6.21.0: the COLOR word no longer gates the GEOMETRY — the §7b world's
// mid-band cells (80..120 m, below the HI plane) decode their height word
// (the dots at Y=120 stand above them) while their color dots sit at Y=80
// INSIDE the terrain and can never decode; the old color gate made the
// march blind to exactly the band the transport CAN carry. A cell with a
// height word but no color word is a valid surface whose paint falls to
// the caller's height fallback (color_at returns the sentinel); a cell
// with neither word is still void. Water stays color-gated (the flag lives
// in the color word — an undecoded color word cannot claim water).
float ground_at(vec2 xz, float w, float w2, out bool water, out bool ok) {
    float cell = RegionInfo.y;             // blocks per cell
    ivec2 celli = ivec2(floor((xz - GridInfo.xy) / cell)); // 0..255
    ok = false;
    water = false;
    if (any(lessThan(celli, ivec2(0))) || any(greaterThanEqual(celli, ivec2(256)))) {
        return -1.0e8; // outside the streamed disk
    }
    vec4 c = texelFetch(ColBufSampler, celli, 0);
    water = c.a >= 0.5 && abs(c.r * 255.0 - 216.0) < 0.5;
    vec4 h = texelFetch(HeightSampler, celli, 0);
    if (h.a < 0.5) {
        return -1.0e8; // height word not decoded yet (or the cell is void)
    }
    ok = true;
    float h0 = h.r * 255.0 - 64.0;
    if (w <= 0.0) {
        return h0;
    }
    // 2x2 cell average, valid neighbors only (data frontier stays exact).
    float sum = h0;
    float cnt = 1.0;
    ivec2 nb[3] = ivec2[3](ivec2(1, 0), ivec2(0, 1), ivec2(1, 1));
    for (int i = 0; i < 3; ++i) {
        ivec2 q = celli + nb[i];
        if (any(lessThan(q, ivec2(0))) || any(greaterThanEqual(q, ivec2(256)))) {
            continue;
        }
        vec4 hq = texelFetch(HeightSampler, q, 0);
        if (hq.a < 0.5) {
            continue;
        }
        sum += hq.r * 255.0 - 64.0;
        cnt += 1.0;
    }
    float avg = mix(h0, sum / cnt, w);
    if (w <= 0.0) {
        return avg;
    }
    // Level 2: +-2 cross average (v5.2.1) — kills the residual 1-texel
    // shimmer at 400 m. ONLY the shading calls pass w2 > 0: the march
    // stays on the 2x2 level (the +50% march cost of the cross made the
    // llvmpipe post-frame slower than the shot interval — paint flickered).
    if (w2 <= 0.0) {
        return avg;
    }
    float s2 = h0;
    float c2 = 1.0;
    ivec2 nb2[4] = ivec2[4](ivec2(2, 0), ivec2(-2, 0), ivec2(0, 2), ivec2(0, -2));
    for (int i = 0; i < 4; ++i) {
        ivec2 q = celli + nb2[i];
        if (any(lessThan(q, ivec2(0))) || any(greaterThanEqual(q, ivec2(256)))) {
            continue;
        }
        vec4 hq = texelFetch(HeightSampler, q, 0);
        if (hq.a < 0.5) {
            continue;
        }
        s2 += hq.r * 255.0 - 64.0;
        c2 += 1.0;
    }
    return mix(avg, s2 / c2, w2);
}

// Surface color of one streamed cell: the 6x6x6 RGB lattice decode.
// Water (idx 216) returns a negative sentinel — the caller uses its own
// water fallback color. v6.21.0: an undecoded color word ALSO returns the
// sentinel (the height-fallback paint), flagged via `colorless` so the
// callers can tell the two classes apart — v6.21.1 uses it to keep the
// vanilla path off the height-only cells (repainting REAL vanilla textures
// with the height fallback cost the #307 aerials 14-24pp of near%).
vec3 color_at(ivec2 celli, float w, float w2, out bool water, out bool colorless) {
    vec4 c = texelFetch(ColBufSampler, celli, 0);
    // v6.21.0: an undecoded color word is the height-fallback sentinel (the
    // mid-band cells paint ColLow/ColHigh by height — see ground_at). It is
    // NOT lattice index 0: before this gate the (0,0,0,0) texel decoded as
    // idx 0 = pure black, one of the #305 dark classes.
    if (c.a < 0.5) {
        water = false;
        colorless = true;
        return vec3(-1.0);
    }
    colorless = false;
    float idx = floor(c.r * 255.0 + 0.5);
    water = idx > 215.5;
    if (water) {
        return vec3(-1.0);
    }
    vec3 lattice = vec3(mod(idx, 6.0), mod(floor(idx / 6.0), 6.0), floor(idx / 36.0));
    vec3 rgb = lattice * (51.0 / 255.0);
    if (w <= 0.0) {
        return rgb;
    }
    // 2x2 average of decoded RGB (land cells only, water stays a flag:
    // averaging the lattice index itself would decode to garbage colors).
    vec3 sum = rgb;
    float cnt = 1.0;
    float wc = 0.0;
    ivec2 nb[3] = ivec2[3](ivec2(1, 0), ivec2(0, 1), ivec2(1, 1));
    for (int i = 0; i < 3; ++i) {
        ivec2 q = celli + nb[i];
        if (any(lessThan(q, ivec2(0))) || any(greaterThanEqual(q, ivec2(256)))) {
            continue;
        }
        vec4 cq = texelFetch(ColBufSampler, q, 0);
        if (cq.a < 0.5) {
            continue;
        }
        float iq = floor(cq.r * 255.0 + 0.5);
        if (iq > 215.5) {
            wc += 1.0;
            continue;
        }
        vec3 lq = vec3(mod(iq, 6.0), mod(floor(iq / 6.0), 6.0), floor(iq / 36.0));
        sum += lq * (51.0 / 255.0);
        cnt += 1.0;
    }
    if (wc > (cnt + wc) * 0.5) {
        water = true; // neighborhood turned mostly water
        return vec3(-1.0);
    }
    vec3 avg = mix(rgb, sum / cnt, w);
    // Level 2: +-2 cross average of decoded RGB (v5.2.1).
    if (w2 <= 0.0) {
        return avg;
    }
    vec3 s2 = rgb;
    float c2 = 1.0;
    float w2c = 0.0;
    ivec2 nb2[4] = ivec2[4](ivec2(2, 0), ivec2(-2, 0), ivec2(0, 2), ivec2(0, -2));
    for (int i = 0; i < 4; ++i) {
        ivec2 q = celli + nb2[i];
        if (any(lessThan(q, ivec2(0))) || any(greaterThanEqual(q, ivec2(256)))) {
            continue;
        }
        vec4 cq = texelFetch(ColBufSampler, q, 0);
        if (cq.a < 0.5) {
            continue;
        }
        float iq = floor(cq.r * 255.0 + 0.5);
        if (iq > 215.5) {
            w2c += 1.0;
            continue;
        }
        vec3 lq = vec3(mod(iq, 6.0), mod(floor(iq / 6.0), 6.0), floor(iq / 36.0));
        s2 += lq * (51.0 / 255.0);
        c2 += 1.0;
    }
    if (w2c > c2 * 0.5) {
        water = true;
        return vec3(-1.0);
    }
    return mix(avg, s2 / c2, w2);
}

// ---- SSAO: the Voxy post/ssao.comp BETTER path, ported (v6.8.1) -----------
// Evidence: analysis/voxy/.../shaders/post/ssao.comp (voxy 0.2.20-beta,
// MCRcortex/voxy branch 263) — the run order in NormalRenderPipeline.java is
// terrain -> SSAO.computeSSAO -> finish(fog), so the ao multiplier enters
// the albedo HERE, before the fog mix at the bottom of main. No invented
// constants — every number below cites its line:
//
//   SSAO_STEPS 12                    ssao.comp:95-97  (BETTER; BEST = 24)
//   phi1 = 1.6180339887              ssao.comp:101
//   r1(n) = fract(0.5 + n/phi1)      ssao.comp:105-110 (seed 0.5)
//   offset_i = r_i * (cos, sin) of   ssao.comp:112-119, call at :186
//     theta = (i+0.5)/STEPS*tau      — CONSTANT dither vec2(0.5): voxy has
//                                      no TAA jitter here, the result is
//                                      stable per frame, no noise seed
//   SSAO_RADIUS 1.5                  ssao.comp:121
//   per-sample accumulate            ssao.comp:210-223:
//     v = occluder - position
//     cos_theta = clamp(dot(v,n)/|v|, 0, 1)
//     falloff = 1/(1 + |v|/RADIUS)
//     ao += cos_theta * falloff
//   final pow(clamp(1-ao/STEPS),2.5) ssao.comp:226
//   applied as colour.rgb *= ao      ssao.comp:272
//   gate = metadata bit 6            set when the section holds fully-opaque
//                                    blocks (RenderDataFactory.java:262);
//                                    pure-fluid sections never carry it.
//                                    Per-pixel mapping: land hits only —
//                                    ocean cells are the pure-fluid case; a
//                                    mixed 2 m cell decodes as land anyway
//                                    (color_at flips water only when the
//                                    neighborhood is MOSTLY water).
//
// The one substitution the port REQUIRES (documented, not invented): voxy
// probes occluders through its depth buffer (ssao.comp:190-209); a
// heightfield marcher has no second geometry source — the occluder height
// IS ground_at() at the sample xz. The screen-space clamp
// (SSAO_MAX_RADIUS_SCREEN 0.1, ssao.comp:120) bounds the projected sample
// footprint; the world-space disc is already bounded by SSAO_RADIUS, so
// there is nothing left to clamp. Samples read ground_at with w=0
// (NEAREST, 2 fetches): the dh-style smoothing exists for the far field,
// a 1.5-block AO probe gains nothing from it, and the llvmpipe march
// budget is real (see the level-2 note inside ground_at).

const float SSAO_STEPS_F = 12.0;   // ssao.comp:96
const float SSAO_PHI1 = 1.6180339887; // ssao.comp:101
const float SSAO_TAU = 6.2831853;  // ssao.comp:99
const float SSAO_RADIUS = 1.5;     // ssao.comp:121

float ssao_r1(int n) {
    return fract(0.5 + float(n) / SSAO_PHI1); // ssao.comp:105-110, seed 0.5
}

// voxy get_tbn_matrix (ssao.comp:131-137) in world space: tangent frame of
// the surface normal, the y-up special case verbatim.
void ssao_tbn(vec3 n, out vec3 t0, out vec3 t1) {
    vec3 tangent = (n.y == 1.0)
        ? vec3(1.0, 0.0, 0.0)
        : normalize(cross(vec3(0.0, 1.0, 0.0), n));
    t0 = tangent;
    t1 = normalize(cross(tangent, n));
}

// BETTER_SSAO computeAO (ssao.comp:150-227) with the depth-buffer probe
// replaced by the heightfield probe (header note above).
float march_ssao(vec3 pos, vec3 n) {
    vec3 t0; vec3 t1;
    ssao_tbn(n, t0, t1);
    float ao = 0.0;
    for (int i = 0; i < 12; ++i) {
        float a = (float(i) + 0.5) * (1.0 / SSAO_STEPS_F); // ssao.comp:113
        float r = sqrt(ssao_r1(i));                        // ssao.comp:115
        float theta = a * SSAO_TAU;                        // ssao.comp:116
        vec2 off = r * vec2(cos(theta), sin(theta));       // ssao.comp:118
        vec2 q = pos.xz + (t0.xz * off.x + t1.xz * off.y) * SSAO_RADIUS;
        bool sw; bool sok;
        float hq = ground_at(q, 0.0, 0.0, sw, sok);
        if (!sok) {
            // no occluder data = voxy's depth==FAR skip (ssao.comp:193)
            continue;
        }
        vec3 v = vec3(q.x - pos.x, hq - pos.y, q.y - pos.z); // ssao.comp:210
        float slen = dot(v, v);                              // ssao.comp:212
        float rlen = inversesqrt(max(slen, 1.0e-6));         // ssao.comp:218
        float angle = dot(v, n) * rlen;                      // ssao.comp:219
        float cosTheta = clamp(angle, 0.0, 1.0);             // ssao.comp:220
        float falloff = 1.0 / (1.0 + (1.0 / rlen) * (1.0 / SSAO_RADIUS)); // :221
        ao += cosTheta * falloff;                            // ssao.comp:223
    }
    return pow(clamp(1.0 - ao * (1.0 / SSAO_STEPS_F), 0.0, 1.0), 2.5); // :226
}

// Horizontal distance from cam to the box boundary along dir.xz.
float edge_distance_xz(vec3 cam, vec3 dir, vec2 minW, vec2 maxW) {
    float e = 1.0e9;
    if (dir.x > 0.0) e = min(e, (maxW.x - cam.x) / dir.x);
    if (dir.x < 0.0) e = min(e, (minW.x - cam.x) / dir.x);
    if (dir.z > 0.0) e = min(e, (maxW.y - cam.z) / dir.z);
    if (dir.z < 0.0) e = min(e, (minW.y - cam.z) / dir.z);
    return e * length(dir.xz);
}

// v5.3 DUST ERASER: fill a mask-disc from unmasked surroundings.
// v5.3.2: ADAPTIVE rings. Dense spawn waves mask whole neighborhoods — a
// fixed 3-ring kernel landed entirely inside the mask (cnt == 0) and the
// raw green survived at the dot-band edge (measured live: 59 px/frame).
// The kernel now doubles its radius (r+1 .. 16r+16) until it has enough
// unmasked background; the early exit keeps ordinary dots filled locally.
vec3 erase_dust(float r) {
    vec3 sum = vec3(0.0);
    float cnt = 0.0;
    for (int k = 0; k < 5; ++k) {
        float rad = (k == 0) ? (r + 1.0) : (2.0 * r + 2.0) * float(1 << (k - 1));
        vec2 s = vec2(rad) / ScreenSize;
        for (int i = 0; i < 8; ++i) {
            float a = float(i) * 0.7853981634;
            vec2 d = vec2(cos(a), sin(a));
            vec2 q = texCoord + d * s;
            if (texture(MaskSampler, q).r < 0.5) {
                sum += texture(InSampler, q).rgb;
                cnt += 1.0;
            }
        }
        if (cnt > 3.0) {
            break;
        }
    }
    if (cnt > 0.5) {
        return sum / cnt;
    }
    return texture(InSampler, texCoord).rgb;
}

// ---- the march (v6.19: extracted for the G8d dual-path caller) -------------
// First ENTERING crossing of the streamed surface along the view ray,
// sampled from t0 up to tCap. Steps grow with the screen-space footprint of
// one streamed cell; a 7-step bisection converges on the crossing. Returns
// -1.0 when the ray never enters the field (open sky, out-of-data, or the
// ceiling/box early-out).
float march_terrain(vec3 cam, vec3 dir, float t0, float tCap, float tanH,
                    float cell, float horizon) {
    // Rays that leave the heightfield from above can never hit: early-out
    // against the ceiling + the march box.
    if (dir.y > 0.0) {
        float toCeil = (RegionInfo.w + 8.0 - cam.y) / dir.y;
        float toEdge = 1.0e9;
        if (dir.x > 0.0) toEdge = min(toEdge, (cam.x + horizon - cam.x) / dir.x);
        if (dir.x < 0.0) toEdge = min(toEdge, (cam.x - horizon - cam.x) / dir.x);
        if (dir.z > 0.0) toEdge = min(toEdge, (cam.z + horizon - cam.z) / dir.z);
        if (dir.z < 0.0) toEdge = min(toEdge, (cam.z - horizon - cam.z) / dir.z);
        if (toCeil < toEdge) {
            return -1.0;
        }
    }

    float t = t0;
    float dt = max(cell * 0.5, t * (2.0 * tanH / max(ScreenSize.y, 1.0)));

    // Sign convention: above = groundY - rayY; POSITIVE means the ray is
    // UNDER the surface. A hit is the ENTERING crossing (above-ground ->
    // underground). The seed is the UNCLAMPED sample at the march start —
    // a ray born inside the field (the eye under a canopy cell) flags no
    // entering crossing until it exits and re-enters, so those pixels stay
    // vanilla (conservative; the v5.x camera seed was dead — the first loop
    // iteration overwrote it with exactly this unclamped start sample).
    bool wflag; bool okflag;
    float prevT = t0;
    float prevDiff = ground_at(cam.xz + dir.xz * t0, smooth_w(t0), 0.0, wflag, okflag)
                     - (cam.y + dir.y * t0);

    for (int i = 0; i < MAX_STEPS; ++i) {
        t = min(t + dt, tCap);
        dt = max(cell * 0.5, t * (2.0 * tanH / max(ScreenSize.y, 1.0)));

        vec2 xz = cam.xz + dir.xz * t;
        float above = ground_at(xz, smooth_w(t), 0.0, wflag, okflag) - (cam.y + dir.y * t);

        if (above > 0.0 && prevDiff <= 0.0) {
            // Entering crossing between prevT and t; bisection converges on
            // the first sample where the ray is underground.
            float lo = prevT;
            float hi = t;
            for (int b = 0; b < BISECT; ++b) {
                float mid = (lo + hi) * 0.5;
                vec2 mxz = cam.xz + dir.xz * mid;
                bool mw; bool mo;
                float g = ground_at(mxz, smooth_w(mid), 0.0, mw, mo);
                if (g - (cam.y + dir.y * mid) > 0.0) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            return (lo + hi) * 0.5;
        }
        prevT = t;
        prevDiff = above;
        if (t >= tCap) {
            break;
        }
    }
    return -1.0;
}

void main() {
#ifdef MESH_DEBUG
    // v6.19.2 MESH DEBUG probe (compile-time — a #define CANNOT fail to
    // bind, unlike the PaintParams UBO): three sky-row pixels carry the
    // live markers; the runner decodes BOTH PNG ends (GL y-flip: a block
    // written at GL y=2 lands at PNG y=h-3).
    //   x=306: constant WHITE — the NEW shader is live (zero uniforms)
    //   x=307: r = PaintParams.x (delivered DEPTH_NEAR, ~0.05 -> ~13/255)
    //          g = PaintParams.y (delivered MESH PAINT arm, 1.0 -> 254/255)
    //   x=308: r = 255 iff MESH_DEBUG && PaintParams.y < 0.5 — shader live
    //          but uniform dead (the binding-failure signature)
    //          g = conf (the rot channel confidence)
    ivec2 sxy = ivec2(gl_FragCoord.xy);
    if (sxy.y == 2 || sxy.y == int(ScreenSize.y) - 3) {
        if (sxy.x == 306) {
            fragColor = vec4(1.0, 1.0, 1.0, 1.0);
            return;
        }
        if (sxy.x == 307) {
            fragColor = vec4(clamp(PaintParams.x * 254.0, 0.0, 255.0) / 255.0,
                             clamp(PaintParams.y * 254.0, 0.0, 255.0) / 255.0,
                             0.0, 1.0);
            return;
        }
        if (sxy.x == 308) {
            vec4 rot0 = texelFetch(RotSampler, ivec2(0, 0), 0);
            fragColor = vec4(PaintParams.y < 0.5 ? 1.0 : 0.0,
                             clamp(rot0.b * 254.0, 0.0, 255.0) / 255.0,
                             0.0, 1.0);
            return;
        }
        // v6.19.6 THE CAPTURE HEARTBEAT: the delivered uniform is proven, the
        // #299 round left exactly one suspect class — the CAPTURE. v6.19.7:
        // the heartbeat moves to the VISIBLE BAND — the 01 vantage looks
        // WEST (yaw 90), the west-band cells (48..96, 128) sit 64..160 m
        // west of the disk center, their HI-plane dots are on screen (the
        // #300 center-cell read (0,0) in BOTH lanes — the cell below the
        // eye is never in view, the heartbeat was blind, not the capture).
        // x=309.r = the MAX raw height byte over the band (0 = no word),
        // x=310.r = the MAX color-word ALPHA (the decoded flag itself —
        // 254 = at least one west-band cell holds a decoded word).
        if (sxy.x == 309) {
            float mh = 0.0;
            for (int i = 0; i < 5; ++i) {
                vec4 h = texelFetch(HeightSampler, ivec2(48 + i * 12, 128), 0);
                mh = max(mh, h.r);
            }
            fragColor = vec4(clamp(mh * 254.0, 0.0, 255.0) / 255.0, 0.0, 0.0, 1.0);
            return;
        }
        if (sxy.x == 310) {
            float ma = 0.0;
            for (int i = 0; i < 5; ++i) {
                vec4 c = texelFetch(ColBufSampler, ivec2(48 + i * 12, 128), 0);
                ma = max(ma, c.a);
            }
            fragColor = vec4(clamp(ma * 254.0, 0.0, 255.0) / 255.0, 0.0, 0.0, 1.0);
            return;
        }
    }
    // v6.24 BUFFER DUMP: the #317 round killed the OR-ratchet (the purple
    // wedge died, 11 of 13 rows rose) but the eastern fake-wall columns
    // survive on 04/06/02/10 with heights the rank-0 byte windows can never
    // encode — every armchair mechanism was refuted, so the round carries
    // the instrument instead of a guess: a RAW window of the persistent
    // buffers in the frame corners. Block A (GL rows 0..31 = PNG bottom
    // rows, the y-flip lesson) = the HEIGHT bytes: R = the byte, G = the
    // decoded flag. Block B (GL rows 32..63) = the COLOR bytes: R = the
    // lattice index, G = the decoded flag. The window = cells x 120..183,
    // z 112..143 — the disk center's east sector (the marker, both
    // mountains, the 64..110 m east band the wall lives in). The mask8
    // evidence lane (mesh_debug=true) carries the dump; the production
    // parity boot compiles without MESH_DEBUG and the gate frames stay
    // clean. The forensics decode names the garbage cells exactly.
    if (sxy.x < 64 && sxy.y < 64) {
        bool heightBlock = sxy.y < 32;
        ivec2 celli = ivec2(120 + sxy.x, 112 + (heightBlock ? sxy.y : sxy.y - 32));
        vec4 b = heightBlock ? texelFetch(HeightSampler, celli, 0)
                             : texelFetch(ColBufSampler, celli, 0);
        fragColor = vec4(b.r, b.a, 0.0, 1.0);
        return;
    }
#endif
    vec4 scene = texture(InSampler, texCoord);
    float depth = texture(InDepthSampler, texCoord).r;
    vec2 px = texCoord * ScreenSize;

    // v5.3 DUST ERASER runs FIRST: masked pixels are repainted from their
    // surroundings before anything else. Over real terrain the early return
    // below then passes the cleaned color through; over sky the dome path
    // uses the cleaned color as its fog-blend base. The transport is
    // invisible from every vantage while capture decoded it unharmed.
    vec4 mk = texture(MaskSampler, texCoord);
    if (mk.r > 0.5) {
        scene.rgb = erase_dust(mk.g * 64.0);
    }

    // 26.3 device depth (live-verified): d = near/eyeZ, infinite far plane —
    // the sky and the sun/moon quads sit at EXACTLY 0.0, real terrain reads
    // positive values that grow with proximity.
    vec4 rot = texelFetch(RotSampler, ivec2(0, 0), 0);
    float conf = rot.b;
    float minConf = LodParams.w;

    if (conf < minConf) {
        fragColor = scene;
        return;
    }

    // v4.0 encoding preserved: R = az/2pi, G = (el+pi/2)/pi, B = conf.
    // A = fov byte (v4.1): the REAL vertical fov measured by the sky shader.
    float az = rot.r * 2.0 * PI;
    float el = rot.g * PI - PI * 0.5;

    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));

    float fov = 5.0 + rot.a * 115.0; // inverse of (fov-5)/115*254
    float tanH = tan(radians(fov) * 0.5);
    float aspect = ScreenSize.x / max(ScreenSize.y, 1.0);

    vec2 ndc = texCoord * 2.0 - 1.0;
    vec3 dir = normalize(F + R * (ndc.x * tanH * aspect) + U * (ndc.y * tanH));

    vec3 cam = vec3(CameraBlockPos) + CameraOffset;

    float region = RegionInfo.x;
    float cell = RegionInfo.y;
    float sea = RegionInfo.z;
    float maxY = RegionInfo.w;
    float near = LodParams.x;

    // The march spans two regions in every direction (the 2x2 slot grid);
    // stale data beyond that is stamp-rejected anyway.
    float horizon = region * 2.0;

    // v6.19 G8d MESH PAINT — the vanilla path. The vanilla fragment's ray
    // distance is reconstructed from the device depth (eyeZ = DEPTH_NEAR /
    // depth; a point t along the view ray sits at view-z t*dot(dir,F)), the
    // heightfield is marched from just ahead of the eye UP TO that distance,
    // and a hit paints the unified march look OVER the vanilla pixel (the
    // run #292 verdict: the representation gap is the 01-10 diff class —
    // voxy renders its greedy-quad mesh of the same blocks, our client the
    // vanilla chunk renderer; the march is the only painter we own). The
    // march cap tVan*1.15+4 IS the paint rule — the surface must stand in
    // front of the vanilla geometry, with a +15%/4 m slack absorbing the
    // 2 m cell quantization and the reconstructed-basis error. Where
    // vanilla is genuinely closer (builds, entities), vanilla keeps the
    // pixel. mesh_paint=false keeps the pre-G8d pass-through.
    float tVan = -1.0;
    if (depth > 1.0e-6) {
        if (PaintParams.y < 0.5) {
#ifdef MESH_DEBUG
            // Armed shader + dead uniform: the whole screen goes magenta.
            fragColor = vec4(1.0, 0.0, 1.0, scene.a);
            return;
#else
            fragColor = scene; // real chunks always win (pre-G8d behavior)
            return;
#endif
        }
        float eyeZ = PaintParams.x / depth;
        tVan = eyeZ / max(dot(dir, F), 1.0e-4);
    }
    if (dir.y == 0.0) {
        dir.y = 1.0e-6;
    }

    float tHit;
    if (tVan > 0.0) {
        // Vanilla path: march from just ahead of the eye; the cap bounds
        // both the march and the ownership test.
        tHit = march_terrain(cam, dir, 1.0, min(horizon, tVan * 1.15 + 4.0), tanH, cell, horizon);
    } else {
        // Sky path: start at the LOD near edge (real chunks live inside it).
        tHit = march_terrain(cam, dir, near, horizon, tanH, cell, horizon);
    }

    if (tHit < 0.0) {
        fragColor = scene; // open sky (or out of data): vanilla wins
        return;
    }


    float fade = smoothstep(minConf, minConf + 0.15, conf);
    vec3 hitPos = cam + dir * tHit;

    // ---- LIVE FOG STATE (camera channel group B) --------------------------
    vec4 fogCol = texelFetch(RotSampler, ivec2(1, 0), 0);
    vec4 fogRng = texelFetch(RotSampler, ivec2(0, 1), 0);
    bool fogLive = fogRng.a > 0.5 && fogCol.a > 0.004;
    float fStart = fogLive ? fogRng.r * 255.0 * 8.0 : FogParams.x;
    float fEnd = fogLive ? fogRng.g * 255.0 * 8.0 : FogParams.y;
    float vanRD = fogRng.b * 255.0 * 16.0;
    vec3 fogTint = fogLive ? fogCol.rgb : VeilColor.rgb;
    float fogOpacity = fogLive ? fogCol.a : 1.0;

    // Shade inputs at the hit: per-cell flags + per-cell gradient normal
    // (nearest reads; the normal is constant per cell = flat Voxy quads).
    float rb = region;
    vec2 hr = floor((hitPos.xz - GridInfo.xy) / rb);
    vec2 hmin = GridInfo.xy + hr * rb;
    vec2 hmax = hmin + vec2(rb);
    ivec2 tc = ivec2(floor((hitPos.xz - GridInfo.xy) / cell));

    bool isWater = false; bool okW; bool dum;
    float sw = smooth_w(tHit);  // level-1 weight
    float sw2 = smooth_w2(tHit); // level-2 weight (v5.2.1)
    float hC = ground_at(hitPos.xz, sw, sw2, isWater, okW);
    float hL = ground_at(hitPos.xz - vec2(cell, 0.0), sw, sw2, dum, okW);
    float hR = ground_at(hitPos.xz + vec2(cell, 0.0), sw, sw2, dum, okW);
    float hD = ground_at(hitPos.xz - vec2(0.0, cell), sw, sw2, dum, okW);
    float hU = ground_at(hitPos.xz + vec2(0.0, cell), sw, sw2, dum, okW);
    // Void neighbors read -1e8: clamp them to the center height so the
    // normal stays flat instead of exploding at the data frontier.
    hL = hL < -1.0e7 ? hC : hL;
    hR = hR < -1.0e7 ? hC : hR;
    hD = hD < -1.0e7 ? hC : hD;
    hU = hU < -1.0e7 ? hC : hU;
    vec3 nrm = normalize(vec3(hL - hR, 2.0 * cell, hD - hU));

    // ---- COLOR: the Voxy model -------------------------------------------
    bool colWater; bool colorless;
    vec3 col = color_at(tc, sw, sw2, colWater, colorless);
    float hn = clamp((hitPos.y - sea) / max(maxY - sea, 1.0), 0.0, 1.0);
    vec3 fallback = (isWater || colWater) ? ColWater.rgb : mix(ColLow.rgb, ColHigh.rgb, smoothstep(0.15, 0.9, hn));
    vec3 albedo = colWater ? fallback : (col.r < 0.0 ? fallback : col);

    // v6.21.1 THE VANILLA-PATH COLOR GATE: the height-only cells (no color
    // word — the §7b mid-band 80..120 m whose color dots sit inside the
    // terrain) may own SKY pixels, never VANILLA pixels: the G8d depth test
    // would repaint REAL vanilla chunk textures with the height fallback,
    // and the #307 aerials paid 14-24pp of near% for it (the mountains'
    // mid-terraces sit at 16-64 m, inside the vanilla band). Over sky there
    // is no vanilla texture to lose — the far flanks keep the honest
    // fallback paint.
    if (tVan > 0.0 && colorless) {
        fragColor = scene;
        return;
    }

    // Vanilla face shade (meshing weights: up 1.0 / z 0.8 / x 0.6) applied
    // through the height-gradient normal, exactly like Voxy's baked quads.
    vec3 an = vec3(abs(nrm.x), max(nrm.y, 0.0), abs(nrm.z));
    float wsum = max(an.x + an.y + an.z, 1.0e-4);
    float face = (an.x * 0.6 + an.y * 1.0 + an.z * 0.8) / wsum;
    if (isWater) {
        face = 1.0; // water quads face straight up (flat sea surface)
    }

    // ---- SSAO: the voxy bit6 gate — land hits only (header note) --------
    float ao = 1.0;
    if (!isWater) {
        ao = march_ssao(hitPos, nrm);
    }

    // Light = EXACT vanilla lightmap texel (0,15) via the live fog color.
    vec4 dayState = texelFetch(RotSampler, ivec2(1, 1), 0);
    vec3 lightRGB;
    if (fogLive) {
        float dayTick = resolve_day_tick(fogCol.rgb, dayState.r * 24000.0, dayState.g > 0.5);
        lightRGB = vanilla_lightmap0_15(dayTick);
    } else {
        lightRGB = vanilla_lightmap0_15(6000.0); // pre-first-frame: noon
    }
    // ssao.comp:272 — ocolour.rgb *= computeAO(...): the ao multiplier rides
    // the albedo BEFORE finish/fog (NormalRenderPipeline.java run order).
    vec3 lit = albedo * face * lightRGB * ao;

    // NormalRenderPipeline.finish: linear fog over the environmental pair,
    // saturated at max(vanillaRD, 320) * sqrt(3) blocks (3D distance).
    float fog = 0.0;
    if (abs(fEnd - fStart) > 1.0) {
        float invEndFogDelta = 1.0 / (fEnd - fStart);
        float startDelta = -fStart * invEndFogDelta;
        float endDistance = max(vanRD, 320.0) * 1.7320508;
        float maxFog = clamp(endDistance * invEndFogDelta + startDelta, 0.0, 1.0);
        fog = clamp(tHit * invEndFogDelta + startDelta, 0.0, maxFog) * fogOpacity;
    }
    lit = mix(lit, fogTint, fog);

    // ---- FADE: Voxy FogMode.FADE, MODE=1 (xz only), last 10% ------------
    // The radius reference is the streamed disk edge: data beyond it is
    // void — the march stops at the true data frontier on its own.
    float lenXZ = tHit * length(dir.xz);
    float rEdge = max(edge_distance_xz(cam, dir, hmin, hmax), 1.0);
    float fadeOut = clamp((lenXZ - FADE_START * rEdge) / max((1.0 - FADE_START) * rEdge, 1.0), 0.0, 1.0);

    // v5.3 NEAR GATE (backlog "paint spills onto the near dome"): the dome
    // paints ONLY beyond the vanilla chunk bubble. Real chunks own the
    // inside of vanRD by depth; chunk-load gaps during fast motion now show
    // vanilla sky instead of a flat dome wall.
    // v6.19 G8d: the depth-tested vanilla path bypasses the near gate — the
    // per-pixel depth comparison already decided ownership; the gate stays
    // on the sky path exactly as v5.3 shipped it.
    float rd = fogLive ? max(vanRD, 64.0) : 128.0;
    float gate = (tVan > 0.0) ? 1.0 : smoothstep(rd * 0.92, rd * 1.10, tHit);

    vec3 outCol = mix(mix(scene.rgb, lit, fade * gate), scene.rgb, fadeOut);
#ifdef MESH_DEBUG
    // v6.19.2: the vanilla-path ENGAGEMENT MAP — every pixel the depth-
    // tested march owns carries a 35% magenta lift, unmistakable in the
    // mask8 frames and in the pixel forensics.
    if (tVan > 0.0) {
        outCol = mix(outCol, vec3(1.0, 0.0, 1.0), 0.35);
    }
#endif
    fragColor = vec4(outCol, scene.a);
}
