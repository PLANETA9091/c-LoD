#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD pass (v5.0 "SKY-GRID BIT-STREAM"): the actual level-of-distant-detail
// renderer.
//
// For every pixel that shows sky (device depth ~ 0) we march a view ray
// against the streamed heightfield. There is NO atlas in the resource pack:
// the heightfield lives in two persistent post-chain target pairs
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
// Non-sky pixels pass through untouched: real chunks always win.

uniform sampler2D InSampler;      // minecraft:main color
uniform sampler2D InDepthSampler; // minecraft:main depth
uniform sampler2D HeightSampler;  // clod_h_b — streamed height bytes (1 px = 1 cell)
uniform sampler2D ColBufSampler;  // clod_c_b — streamed color index bytes (1 px = 1 cell)
uniform sampler2D RotSampler;     // clod:clod_rot_a (2x2 persistent, camera + fog channel)

// v5.0.3: SamplerInfo never existed as a client UBO — it stayed zeroed
// (the whole capture wrote cell (0,0), the march divided by zero).
// The screen size rides the STANDARD Globals block (ScreenSize), the
// target size is compile-time known.

// x = disk origin x, y = disk origin z, z = plane HI y, w = plane LO y
layout(std140) uniform ClodGrid {
    vec4 GridInfo;
};

// x = region blocks, y = cell size, z = sea level, w = max terrain height
layout(std140) uniform ClodAtlas {
    vec4 AtlasInfo;
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
// One streamed cell = one flat Voxy-style quad. Every read is NEAREST
// (texelFetch on the integer cell coordinate) — bilinear smearing was the
// v4.2 "textures overlay badly" bug and is never coming back.

// Sample the streamed ground at a world xz. Returns the surface Y, or -1e8
// for "no data" (void, never decoded). Water cells stream their real
// surface height; the water FLAG lives in the color word (idx 216).
float ground_at(vec2 xz, out bool water, out bool ok) {
    float cell = AtlasInfo.y;             // blocks per cell
    ivec2 celli = ivec2(floor((xz - GridInfo.xy) / cell)); // 0..255
    ok = false;
    water = false;
    if (any(lessThan(celli, ivec2(0))) || any(greaterThanEqual(celli, ivec2(256)))) {
        return -1.0e8; // outside the streamed disk
    }
    vec4 c = texelFetch(ColBufSampler, celli, 0);
    if (c.a < 0.5) {
        return -1.0e8; // no word ever decoded for this cell
    }
    water = abs(c.r * 255.0 - 216.0) < 0.5;
    vec4 h = texelFetch(HeightSampler, celli, 0);
    if (h.a < 0.5) {
        return -1.0e8; // height word not decoded yet
    }
    ok = true;
    return h.r * 255.0 - 64.0;
}

// Surface color of one streamed cell: the 6x6x6 RGB lattice decode.
// Water (idx 216) returns a negative sentinel — the caller uses its own
// water fallback color.
vec3 color_at(ivec2 celli, out bool water) {
    vec4 c = texelFetch(ColBufSampler, celli, 0);
    float idx = floor(c.r * 255.0 + 0.5);
    water = idx > 215.5;
    if (water) {
        return vec3(-1.0);
    }
    vec3 lattice = vec3(mod(idx, 6.0), mod(floor(idx / 6.0), 6.0), floor(idx / 36.0));
    return lattice * (51.0 / 255.0);
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

void main() {
    vec4 scene = texture(InSampler, texCoord);
    float depth = texture(InDepthSampler, texCoord).r;
    vec2 px = texCoord * ScreenSize;

    // 26.3 device depth (live-verified): d = near/eyeZ, infinite far plane —
    // the sky and the sun/moon quads sit at EXACTLY 0.0, real terrain reads
    // positive values that grow with proximity. Real chunks always win.
    if (depth > 1.0e-6) {
        fragColor = scene;
        return;
    }

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

    float region = AtlasInfo.x;
    float cell = AtlasInfo.y;
    float sea = AtlasInfo.z;
    float maxY = AtlasInfo.w;
    float near = LodParams.x;

    // The march spans two regions in every direction (the 2x2 slot grid);
    // stale data beyond that is stamp-rejected anyway.
    float horizon = region * 2.0;

    // Rays that leave the heightfield from above can never hit: early-out
    // against the ceiling + the march box.
    if (dir.y > 0.0) {
        float toCeil = (maxY + 8.0 - cam.y) / dir.y;
        float toEdge = 1.0e9;
        if (dir.x > 0.0) toEdge = min(toEdge, (cam.x + horizon - cam.x) / dir.x);
        if (dir.x < 0.0) toEdge = min(toEdge, (cam.x - horizon - cam.x) / dir.x);
        if (dir.z > 0.0) toEdge = min(toEdge, (cam.z + horizon - cam.z) / dir.z);
        if (dir.z < 0.0) toEdge = min(toEdge, (cam.z - horizon - cam.z) / dir.z);
        if (toCeil < toEdge) {
            fragColor = scene; // leaves the heightfield from above: sky
            return;
        }
    }
    if (dir.y == 0.0) {
        dir.y = 1.0e-6;
    }

    // March: start at the LOD near edge (real chunks live inside it), step
    // proportional to the screen-space footprint of one streamed cell.
    float t = near;
    float dt = max(cell * 0.5, t * (2.0 * tanH / max(ScreenSize.y, 1.0)));
    float tHit = -1.0;

    // Sign convention: above = groundY - rayY; POSITIVE means the ray is
    // UNDER the surface. A hit is the ENTERING crossing (above-ground ->
    // underground), seeded at the camera (t = 0, clamped above-ground).
    bool dw; bool dok;
    float prevT = 0.0;
    float prevDiff = min(ground_at(cam.xz, dw, dok) - cam.y, -1.0);

    vec2 xz = cam.xz + dir.xz * t;
    bool wflag; bool okflag;
    float above = ground_at(xz, wflag, okflag) - (cam.y + dir.y * t);

    for (int i = 0; i < MAX_STEPS; ++i) {
        prevT = t;
        prevDiff = above;
        t = min(t + dt, horizon);
        dt = max(cell * 0.5, t * (2.0 * tanH / max(ScreenSize.y, 1.0)));

        xz = cam.xz + dir.xz * t;
        above = ground_at(xz, wflag, okflag) - (cam.y + dir.y * t);

        if (above > 0.0 && prevDiff <= 0.0) {
            // Entering crossing between prevT and t; bisection converges on
            // the first sample where the ray is underground.
            float lo = prevT;
            float hi = t;
            for (int b = 0; b < BISECT; ++b) {
                float mid = (lo + hi) * 0.5;
                vec2 mxz = cam.xz + dir.xz * mid;
                bool mw; bool mo;
                float g = ground_at(mxz, mw, mo);
                if (g - (cam.y + dir.y * mid) > 0.0) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            tHit = (lo + hi) * 0.5;
            break;
        }
        if (t >= horizon) {
            break;
        }
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
    float hC = ground_at(hitPos.xz, isWater, okW);
    float hL = ground_at(hitPos.xz - vec2(cell, 0.0), dum, okW);
    float hR = ground_at(hitPos.xz + vec2(cell, 0.0), dum, okW);
    float hD = ground_at(hitPos.xz - vec2(0.0, cell), dum, okW);
    float hU = ground_at(hitPos.xz + vec2(0.0, cell), dum, okW);
    // Void neighbors read -1e8: clamp them to the center height so the
    // normal stays flat instead of exploding at the data frontier.
    hL = hL < -1.0e7 ? hC : hL;
    hR = hR < -1.0e7 ? hC : hR;
    hD = hD < -1.0e7 ? hC : hD;
    hU = hU < -1.0e7 ? hC : hU;
    vec3 nrm = normalize(vec3(hL - hR, 2.0 * cell, hD - hU));

    // ---- COLOR: the Voxy model -------------------------------------------
    bool colWater;
    vec3 col = color_at(tc, colWater);
    float hn = clamp((hitPos.y - sea) / max(maxY - sea, 1.0), 0.0, 1.0);
    vec3 fallback = (isWater || colWater) ? ColWater.rgb : mix(ColLow.rgb, ColHigh.rgb, smoothstep(0.15, 0.9, hn));
    vec3 albedo = colWater ? fallback : (col.r < 0.0 ? fallback : col);

    // Vanilla face shade (meshing weights: up 1.0 / z 0.8 / x 0.6) applied
    // through the height-gradient normal, exactly like Voxy's baked quads.
    vec3 an = vec3(abs(nrm.x), max(nrm.y, 0.0), abs(nrm.z));
    float wsum = max(an.x + an.y + an.z, 1.0e-4);
    float face = (an.x * 0.6 + an.y * 1.0 + an.z * 0.8) / wsum;
    if (isWater) {
        face = 1.0; // water quads face straight up (flat sea surface)
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
    vec3 lit = albedo * face * lightRGB;

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

    vec3 outCol = mix(mix(scene.rgb, lit, fade), scene.rgb, fadeOut);
    fragColor = vec4(outCol, scene.a);
}
