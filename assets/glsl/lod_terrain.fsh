#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD pass 4 (v4.2 "VOXY EXACT"): the actual level-of-distant-detail
// renderer.
//
// For every pixel that shows sky (device depth ~ 1) we march a view ray
// against the heightfield atlas shipped in the server resource pack and
// shade the hit with the Voxy color model, reproduced exactly from the
// Voxy 26.3 source scan (analysis/voxy):
//
//   color = albedo x vanillaFaceShade x lightmap(0, 15)
//   fog   = Voxy NormalRenderPipeline.finish + post/blit_texture_depth_cutout:
//           linear over the VANILLA environmental pair toward the VANILLA
//           FogColor, capped at min(1, max(vanRD, 320)*sqrt(3) scaled);
//   fade  = Voxy FogMode.FADE MODE=1: linear alpha fade over the last 10%
//           of the view radius, horizontal distance only.
//
// The vanilla inputs reach this pass through the CAMERA CHANNEL (sky.fsh):
// group A carries the view basis, group B carries FogColor RGBA,
// FogEnvironmentalStart/8, FogEnvironmentalEnd/8 and FogRenderDistanceEnd/16
// straight out of the Fog UBO — the same numbers Voxy reads from
// fogParameters. No creative grading anywhere: the light term is the exact
// vanilla 26.3 overworld lightmap texel (0,15) = SkyLightColor * SkyFactor
// reproduced from data/minecraft/timeline/day.json keyframes driven by
// GameTime (= (dayTime % 24000 + partialTicks) / 24000, verified in
// GlobalSettingsUniform). Distant terrain therefore dims, blues and warms
// EXACTLY like the real chunks beside it.
//
// Non-sky pixels pass through untouched: real chunks always win.

uniform sampler2D InSampler;      // minecraft:main color
uniform sampler2D InDepthSampler; // minecraft:main depth
uniform sampler2D AtlasSampler;   // clod:textures/effect/atlas.png (RG = height, B = water)
uniform sampler2D ColSampler;     // clod:textures/effect/atlas_col.png (RGB = baked surface albedo)
uniform sampler2D RotSampler;     // clod:clod_rot_a (2x2 persistent, camera + fog channel)

layout(std140) uniform SamplerInfo {
    vec2 OutSize;
    vec2 InSize;
};

// x,y = atlas origin (blocks), z = atlas size (blocks), w = cell size (blocks)
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

// Scanned box (packgen: Chebyshev ring extent of the valid cells, blocks):
// x,y = box center, z = half size. The march and the fade use this instead of
// the full atlas footprint, so rays stop at the scan frontier (no fake hits,
// no wasted steps) and the LOD fades out exactly where the data ends.
layout(std140) uniform ClodScan {
    vec4 ScanBox;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;
const int MAX_STEPS = 260;
const int BISECT = 7;
const float VOID_Y = -63.5;   // cells below this are unscanned = no data
const float FADE_START = 0.9; // Voxy: fade begins at 90% of the view radius
// Floor for the FADE radius (blocks). Voxy fades against its fixed LOD view
// distance; a camera near the data edge must keep a full-length fade ramp or
// the terrain beside it dissolves into sky (v4.3 bench: west-flat-blue).
// 150 blocks ~= the visible LOD band for RD8 (96..256) at 2-block cells.
const float FADE_MIN_RADIUS = 150.0;

// ---- vanilla 26.3 overworld lightmap texel (0,15) --------------------------
// data/minecraft/timeline/day.json, tracks visual/sky_light_color (+_factor),
// modifier multiply over the defaults (white / 1.0), linear keyframe lerp
// (ARGB::srgbLerp). Keyframes: 730 white/1.0, 11270 white/1.0,
// 13140 #7a7aff/0.24, 22860 #7a7aff/0.24, wrap to 730 (+24000).
// Overworld ambient_light = 0.0 -> lightmap(0,15) = SkyLightColor * SkyFactor.
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
// clod:post/rot_solve (the resolve_day_tick function). Identical constants
// and ramps; the persisted day-state pixel rot (1,1) provides the branch
// continuity across frames.
const vec3 FOG_BASE_DAY = vec3(192.0, 216.0, 255.0) / 255.0;  // #c0d8ff
const vec3 FOG_SCALAR_NIGHT = vec3(15.0, 15.0, 22.0) / 255.0; // #0f0f16

float resolve_day_tick(vec3 fogRgb, float prevTick, bool prevValid) {
    vec3 s = fogRgb / FOG_BASE_DAY;
    vec3 dvec = FOG_SCALAR_NIGHT - vec3(1.0);
    float a = clamp(dot(s - vec3(1.0), dvec) / dot(dvec, dvec), 0.0, 1.0);
    // Plateaus map to the middle of the respective window (the exact tick is
    // unobservable from the fog there — and irrelevant, the light is flat):
    // day -> 6000 (noon), night -> 18000 (midnight).
    float tEve = a <= 0.0 ? 6000.0 : (a >= 1.0 ? 18000.0 : 11867.0 + a * 1803.0);
    float tMor = a <= 0.0 ? 6000.0 : (a >= 1.0 ? 18000.0 : 24133.0 - a * 1803.0);
    if (prevValid) {
        return abs(tEve - prevTick) <= abs(tMor - prevTick) ? tEve : tMor;
    }
    return tEve; // bootstrap
}

// ---- atlas reads: NEAREST, per cell ---------------------------------------
// The atlas IS the LOD geometry: one cell = one flat Voxy-style quad. Every
// attribute is read NEAREST (texelFetch on the integer cell coordinate).
// The v4.2-first build sampled bilinearly and that reproduced EXACTLY the
// "textures overlay badly" report: neighbor colors smeared across cell
// borders, coastlines turned half-water, and — the worst — void border
// cells blended into the valid heights as ghost walls the march hit at the
// scan frontier. texelFetch bypasses the sampler filter, so the bilinear
// flags in the chain JSON only remain relevant for nothing here.

// Atlas cell count per axis (atlas_size / cell, both uniforms).
int atlas_n(float cell) {
    return int(AtlasInfo.z / cell + 0.5);
}

// Integer atlas cell for a world xz, clamped into the atlas.
ivec2 cell_tc(vec2 xz, vec2 origin, float cell, int n) {
    return clamp(ivec2(floor((xz - origin) / cell)), ivec2(0), ivec2(n - 1));
}

// Height (block Y) of one cell: (y + 64) << 6, big-endian RG.
float cell_height(ivec2 tc) {
    vec4 a = texelFetch(AtlasSampler, tc, 0);
    float y16 = a.r * 65280.0 + a.g * 255.0; // r*255*256 + g*255
    return y16 / 64.0 - 64.0;
}

bool cell_water(ivec2 tc) {
    return texelFetch(AtlasSampler, tc, 0).b > 0.5;
}

// Distance-to-ground sample for the march: void reads -1e8 — VERY far above
// the ray in the `above = ground - rayY` convention, i.e. unscanned cells can
// NEVER register as the underground side of an entering crossing. (Reading
// +1e8 made every ray that left the scan disc "hit" the disc boundary as a
// fake terrain wall — the towering striped curtains of the first v4.2 build.)
// Water reads the flat sea level.
float ground_at(vec2 xz, vec2 origin, float cell, int n, float sea) {
    ivec2 tc = cell_tc(xz, origin, cell, n);
    float h = cell_height(tc);
    if (h < VOID_Y) {
        return -1.0e8; // unscanned: nothing to hit, nothing to exit
    }
    return cell_water(tc) ? sea : h;
}

// Horizontal distance from cam to the atlas boundary along dir.xz.
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
    float az = rot.r * 6.28318530718;
    float el = rot.g * PI - PI * 0.5;

    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));

    float fov = 5.0 + rot.a * 115.0; // inverse of (fov-5)/115*254
    float tanH = tan(radians(fov) * 0.5);
    float aspect = OutSize.x / max(OutSize.y, 1.0);

    vec2 ndc = texCoord * 2.0 - 1.0;
    vec3 dir = normalize(F + R * (ndc.x * tanH * aspect) + U * (ndc.y * tanH));

    vec3 cam = vec3(CameraBlockPos) + CameraOffset;

    vec2 origin = AtlasInfo.xy;
    float size = AtlasInfo.z;
    float cell = AtlasInfo.w;
    int n = atlas_n(cell);
    float near = LodParams.x;
    float maxY = LodParams.y;
    float sea = LodParams.z;

    // AtlasInfo.xy is the atlas MIN EDGE (packgen: center - half), so the
    // footprint is [origin, origin + size]. The old code treated origin as
    // the CENTER (minW = origin - size/2), which displaced the whole LOD
    // half an atlas west/north, mirrored the far west onto the edge texels
    // and killed the LOD entirely on the eastern/southern half of the
    // compass (verified on the v4.1 bench shots: yaw south/east = no LOD).
    // The MARCH window is the SCANNED box when available (ScanBox.z > 0):
    // beyond it there is no data — nothing can hit, and the fade must
    // reference the true data edge, not the nominal atlas edge.
    vec2 minW = origin;
    vec2 maxW = origin + vec2(size);
    if (ScanBox.z > 1.0) {
        minW = ScanBox.xy - vec2(ScanBox.z);
        maxW = ScanBox.xy + vec2(ScanBox.z);
    }

    // Rays that leave the heightfield from above can never hit: early-out
    // against the REAL scanned max height (uniform, packgen) + margin.
    if (dir.y > 0.0) {
        float toCeil = (maxY + 8.0 - cam.y) / dir.y;
        float toEdge = 1.0e9;
        if (dir.x > 0.0) toEdge = min(toEdge, (maxW.x - cam.x) / dir.x);
        if (dir.x < 0.0) toEdge = min(toEdge, (minW.x - cam.x) / dir.x);
        if (dir.z > 0.0) toEdge = min(toEdge, (maxW.y - cam.z) / dir.z);
        if (dir.z < 0.0) toEdge = min(toEdge, (minW.y - cam.z) / dir.z);
        if (toCeil < toEdge) {
            fragColor = scene; // leaves the heightfield from above: sky
            return;
        }
    }
    if (dir.y == 0.0) {
        dir.y = 1.0e-6;
    }

    // March: start at the LOD near edge (real chunks live inside it), step
    // proportional to screen-space footprint of one atlas cell. With the
    // corrected window the camera can sit anywhere inside the atlas, so the
    // march limit is the full atlas span (worst case: player at one edge,
    // looking across).
    float horizon = size;
    float t = near;
    // Conservative stride: the minimum step must stay at or below half a
    // cell, or near-parallel rays punch through slopes between samples and
    // the surface tears into moire stripes. (Nearest-cell reads need >= 2
    // samples per crossed cell for the same reason.)
    float dt = max(cell * 0.5, t * (2.0 * tanH / max(OutSize.y, 1.0)));
    float tHit = -1.0;

    // Sign convention: above = groundY - rayY; POSITIVE means the ray is
    // UNDER the surface. A hit is the ENTERING crossing (above-ground ->
    // underground). The v4.0-v4.2-first builds had this INVERTED: they
    // detected the EXIT (+ -> -) — descending rays (the whole LOD below the
    // horizon!) never registered a hit, and the only visible pixels came
    // from grazing backface crossings — the v4.1 dark column artifact.
    // Seed the sign at t = 0 (the camera itself): rays that dive below the
    // heightfield BEFORE the near edge (aerial down-look over terrain taller
    // than the near ring) then register their crossing inside [0, near].
    float prevT = 0.0;
    // min(.., -1): a VOID camera column reads 1e8 ("underground") and would
    // poison the whole march — the camera is in the AIR for every pixel this
    // pass sees (sky depth gate), so clamp the seed to the above-ground side.
    float prevDiff = min(ground_at(cam.xz, origin, cell, n, sea) - cam.y, -1.0);

    vec2 xz = cam.xz + dir.xz * t;
    float above = ground_at(xz, origin, cell, n, sea) - (cam.y + dir.y * t);

    for (int i = 0; i < MAX_STEPS; ++i) {
        prevT = t;
        prevDiff = above;
        t = min(t + dt, horizon);
        dt = max(cell * 0.5, t * (2.0 * tanH / max(OutSize.y, 1.0)));

        xz = cam.xz + dir.xz * t;
        // Left the atlas? No LOD here.
        if (any(lessThan(xz, minW)) || any(greaterThan(xz, maxW))) {
            break;
        }
        above = ground_at(xz, origin, cell, n, sea) - (cam.y + dir.y * t);

        if (above > 0.0 && prevDiff <= 0.0) {
            // Entering crossing between prevT and t; bisection converges on
            // the first sample where the ray is underground (void stays
            // +inf: an unscanned cell reads as solid no-data and can never
            // produce the above-ground side of a crossing).
            float lo = prevT;
            float hi = t;
            for (int b = 0; b < BISECT; ++b) {
                float mid = (lo + hi) * 0.5;
                vec2 mxz = cam.xz + dir.xz * mid;
                float g = ground_at(mxz, origin, cell, n, sea);
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
        fragColor = scene; // open sky (or out of atlas): vanilla wins
        return;
    }

    float fade = smoothstep(minConf, minConf + 0.15, conf);
    vec3 hitPos = cam + dir * tHit;

    // ---- LIVE FOG STATE (camera channel group B) --------------------------
    // Read BEFORE the shading: the same fog color also reconstructs the day
    // tick for the light term. Static pack fog is the pre-first-frame
    // fallback (used until the first valid fog payload row arrives).
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
    ivec2 tc = cell_tc(hitPos.xz, origin, cell, n);
    bool isWater = cell_water(tc);

    // Side-entry detection (the hanging-curtain fix). A march crossing can
    // enter a column through its WALL: a real cliff (terrain rose faster
    // than the ray descended between samples) or a valid island rising out
    // of unscanned void (hPre < VOID_Y). Painting those pixels with the top
    // gradient normal (face ~= 1.0, full brightness) produced the bright
    // vertical curtains at every void boundary on the v4.3 bench. DH bakes
    // separate darker side quads for the same geometry; we detect the wall
    // at the hit and use the vanilla horizontal face weights instead.
    ivec2 tcPre = cell_tc(cam.xz + dir.xz * prevT, origin, cell, n);
    float hPre = cell_height(tcPre);
    float hHit = cell_height(tc);
    float rayPreY = cam.y + dir.y * prevT;
    bool sideWall = (hHit - hPre) > (rayPreY - hitPos.y) + cell * 0.5;
    float wallFace = 0.8;
    if (sideWall) {
        ivec2 dtc = tc - tcPre;
        wallFace = abs(dtc.x) >= abs(dtc.y) ? 0.6 : 0.8; // vanilla x/z side weights
    }

    float hL = cell_height(tc - ivec2(1, 0));
    float hR = cell_height(tc + ivec2(1, 0));
    float hD = cell_height(tc - ivec2(0, 1));
    float hU = cell_height(tc + ivec2(0, 1));
    vec3 nrm = normalize(vec3(hL - hR, 2.0 * cell, hD - hU));

    // ---- COLOR: the Voxy model -------------------------------------------
    // albedo = baked surface color (block palette x biome tint, palette.rs);
    // the elevation gradient stays only as the fallback for colorless cells.
    vec3 col = texelFetch(ColSampler, tc, 0).rgb;
    float colorless = step(max(col.r, max(col.g, col.b)), 1.0 / 255.0);
    float hn = clamp((hitPos.y - sea) / max(maxY - sea, 1.0), 0.0, 1.0);
    vec3 fallback = isWater ? ColWater.rgb : mix(ColLow.rgb, ColHigh.rgb, smoothstep(0.15, 0.9, hn));
    vec3 albedo = mix(col, fallback, colorless);

    // Vanilla face shade (meshing weights: up 1.0 / z 0.8 / x 0.6) applied
    // through the height-gradient normal, exactly like Voxy's baked quads.
    vec3 an = vec3(abs(nrm.x), max(nrm.y, 0.0), abs(nrm.z));
    float wsum = max(an.x + an.y + an.z, 1.0e-4);
    float face = (an.x * 0.6 + an.y * 1.0 + an.z * 0.8) / wsum;
    if (isWater) {
        face = 1.0; // water quads face straight up (flat sea surface)
    }
    if (sideWall) {
        face = wallFace;          // vertical wall: horizontal weight only
        albedo *= vec3(0.85);     // and a depth hint (DH side quads are darker)
    }

    // Light = EXACT vanilla lightmap texel (0,15): distant surface cells are
    // sky-lit; the vanilla timeline drives day/dusk/night/dawn. The day tick
    // is reconstructed live from the fog color (the timeline scalar inverted
    // against the known #c0d8ff fog base — GameTime is the world age, NOT the
    // day time, verified on the bench: it ignores `time set` entirely).
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
        // Voxy blits with length(point.xyz): our ray param t IS that length.
        fog = clamp(tHit * invEndFogDelta + startDelta, 0.0, maxFog) * fogOpacity;
    }
    lit = mix(lit, fogTint, fog);

    // ---- FADE: Voxy FogMode.FADE, MODE=1 (xz only), last 10% ------------
    float lenXZ = tHit * length(dir.xz);
    // Voxy's FADE radius is its FIXED render distance, not the distance to
    // the data edge. Using the raw camera->edge distance collapses the whole
    // fade into the first blocks whenever the camera stands near the atlas
    // boundary (bench v4.3: player 36 cells from the west edge -> the entire
    // western view faded to sky even though every cell was scanned). Floor
    // the radius at a sane view span so the fade only ever touches the true
    // frontier, never the terrain next to the player.
    float rEdge = max(edge_distance_xz(cam, dir, minW, maxW), FADE_MIN_RADIUS);
    // Voxy FogMode.FADE verbatim: colour.a *= (1 - fadeOut) with fadeOut =
    // (len - start)/(end - start), start = 90% of the radius. With SRC_ALPHA
    // blending that is mix(lod, scene, fadeOut) — fadeOut is 0 INSIDE the
    // radius (LOD fully visible) and 1 AT the edge (vanilla shows through).
    // The previous build computed 1 - fadeOut and fed it to the same mix,
    // which inverted the whole term: the LOD was suppressed everywhere
    // except the outermost 10% ring — visually "no LOD at all".
    float fadeOut = clamp((lenXZ - FADE_START * rEdge) / max((1.0 - FADE_START) * rEdge, 1.0), 0.0, 1.0);

    vec3 outCol = mix(mix(scene.rgb, lit, fade), scene.rgb, fadeOut);
    fragColor = vec4(outCol, scene.a);
}
