#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD pass 4 (v4.3 "VOXY CORE"): the LOD renderer, now with the SUBSTANCE
// of Voxy's LOD pipeline (analysis/voxy-e2e.md), not just its colors:
//
//   - The height PNG carries the real Voxy mip pyramid: level 0 cells plus
//     levels 1..4 reduced by the ported Mipper (max-OPACITY corner voxel
//     keeps its full blockstate id — distant LODs are real blocks, never
//     averaged colors). The ray marches the level matching its distance,
//     exactly like Voxy's screen-area detail selection.
//   - Every hit is textured from the blockModelAtlas (baked server-side from
//     the real vanilla 26.3 block textures): per state 3x2 face tiles +
//     manual mip strip (16/8/4/2/1 px). uv = fract(worldXZ / cellL) gives
//     the Voxy greedy-quad tiling — one texture tile per 2^L cell.
//   - Fragment math = Voxy quads.frag: albedo x (lightmap x faceShade) x
//     biomeTint, cutout discard at alpha <= 0.1, water translucent.
//     faceShade = vanilla cardinal light: UP 1.0 / N,S 0.8 / E,W 0.6 /
//     DOWN 0.5 (NormalRenderPipeline cardinalLighting).
//   - lightmap(b, s) = Ambient + SkyLightColor*brightness(s)*SkyFactor +
//     BlockLightColor*brightness(b)*BlockFactor — the exact 26.3
//     lightmap.fsh formula; brightness(l) = l/(4-3l). The sky term is the
//     v4.2-verified timeline reconstruction of lightmap(0,15); block light
//     comes from the cell's Voxy lightId (lava glows).
//   - fog/fade/composite stay the v4.2-verified Voxy recipe: linear fog over
//     the vanilla environmental pair capped at max(vanRD,320)*sqrt(3), the
//     10% xz fade, and real chunks always win (device depth gate).
//
// Non-sky pixels pass through untouched: real chunks always win.

uniform sampler2D InSampler;      // minecraft:main color
uniform sampler2D InDepthSampler; // minecraft:main depth
uniform sampler2D AtlasSampler;   // clod:atlas — Voxy mip pyramid (RG=height, B=water, A=state)
uniform sampler2D ColSampler;     // clod:atlas_col — RGB=biome tint, A=Voxy lightId
uniform sampler2D StatesSampler;  // clod:atlas_states — the blockModelAtlas
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

// x = state count, y = states atlas width, z = states atlas height, w = 16
layout(std140) uniform ClodStates {
    vec4 StatesInfo;
};

// STATIC fallback fog (x = start, y = end) used only before the first valid
// fog payload row arrives; live fog replaces it the moment the sky is seen.
layout(std140) uniform ClodFog {
    vec4 FogParams;
};

layout(std140) uniform ClodColors {
    vec4 ColLow;    // fallback low terrain (state 0 cells)
    vec4 ColHigh;   // fallback high terrain
    vec4 ColWater;  // fallback water surface
    vec4 VeilColor; // fallback fog tint (rgb) + snow whiteness (a)
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;
const int MAX_STEPS = 420;
const int BISECT = 7;
const float VOID_Y = -63.5;   // cells below this are unscanned = no data
const float FADE_START = 0.9; // Voxy: fade begins at 90% of the view radius

// ---- vanilla 26.3 lightmap ------------------------------------------------
// lightmap.fsh: brightness(l) = l / (4 - 3l); color = Ambient
// + SkyLightColor * skyBr + BlockLightColor * blockBr (overworld Ambient 0).
float lm_brightness(float level) {
    return level / (4.0 - 3.0 * level);
}

// The (0, 15) texel: data/minecraft/timeline/day.json keyframes (v4.2-verified).
vec3 vanilla_lightmap0_15(float dayFrac) {
    const vec3 DAY = vec3(1.0);
    const vec3 NIGHT = vec3(122.0, 122.0, 255.0) / 255.0; // #7a7aff
    float t = dayFrac < 730.0 / 24000.0 ? dayFrac * 24000.0 + 24000.0 : dayFrac * 24000.0;
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

// Full lightmap texel for a Voxy lightId ((block<<4)|sky), lightmap.fsh math.
vec3 voxy_lightmap(float lightId) {
    // Voxy lightId layout: (block << 4) | sky — the scan probes the REAL
    // server light engine for the air cell above the surface, so sky is the
    // per-column shadow channel (under canopies/overhangs sky < 15) and
    // block carries lava/lantern glow. Integer split (no +0.5 rounding: it
    // turned sky=15 into sky=1 — 2% brightness, i.e. black LOD).
    float block = floor(lightId / 16.0);
    float sky = lightId - block * 16.0;
    // Sky term: daylight colour graded by the cell's own sky light. Open sky
    // (15) keeps the exact good-pack full brightness (0.22 + 0.78*1 = 1.0);
    // shadowed columns fall toward a 0.22 ambient floor — the Voxy-style
    // relief shadows without ever going pitch black.
    vec3 color = vanilla_lightmap0_15(GameTime)
        * (0.22 + 0.78 * lm_brightness(sky / 15.0));
    // BlockLightColor = mix(BlockLightTint, 1, 0.9 * (2b-1)^2), tint = white.
    float b = block / 15.0;
    vec3 blockColor = mix(vec3(1.0), vec3(1.0), 0.9 * (2.0 * b - 1.0) * (2.0 * b - 1.0));
    color += blockColor * lm_brightness(b);
    return color;
}

// ---- atlas access -----------------------------------------------------------
// The pyramid: level L occupies rows [topPx(L), topPx(L)+cellsL) of a PNG
// that is `cells` wide. topPx(L) = cells * (2 - 2^(1-L)); the total PNG
// height (levels 0..4) = cells * (2 - 2^-4) = cells * 31/16.
float atlas_cells() { return AtlasInfo.z / AtlasInfo.w; }

/// Cells in level L: the pyramid halves per level (L1 = cells/2), while the
/// PNG keeps the full L0 width — `atlas_cell` therefore samples x over the
/// full width but quantises in level cells.
float level_cells(int L) { return atlas_cells() * exp2(-float(L)); }

float level_top(int L) {
    return atlas_cells() * (2.0 - exp2(1.0 - float(L)));
}

float atlas_total_h() {
    return atlas_cells() * (2.0 - exp2(-4.0));
}

// Height (block Y) from a level's data: (y + 64) << 6, big-endian RG.
float atlas_height_at(vec4 a) {
    float y16 = a.r * 65280.0 + a.g * 255.0;
    return y16 / 64.0 - 64.0;
}

// Full data fetch for one level cell: height, water, state id.
void atlas_cell(int L, vec2 local, out float height, out bool water, out float state) {
    float cellsL = level_cells(L);
    float topPx = level_top(L);
    float totalH = atlas_total_h();
    // Each level is left-aligned inside the L0-wide PNG: one level cell = one
    // pixel, so x normalises over `atlas_cells()` (the width), not over cellsL.
    float cx = (floor(local.x * cellsL) + 0.5) / atlas_cells();
    float cy = (topPx + floor(local.y * cellsL) + 0.5) / totalH;
    vec4 a = texture(AtlasSampler, vec2(cx, cy));
    height = atlas_height_at(a);
    water = a.b > 0.5;
    state = floor(a.a * 255.0 + 0.5);
}

// Light + tint fetch (level 0 PNG only — light/tint are per L0 cell; the
// pyramid keeps the winner's id, so coarse levels resample L0's winner by
// taking the +x+z cell of each coarse cell, which IS the Mipper's pick).
vec4 col_fetch(int L, vec2 local) {
    // The colour PNG is level 0 only; a coarse cell samples the Mipper's pick,
    // i.e. the +x+z L0 cell inside it (Voxy takes the same corner voxel).
    float n0 = atlas_cells();
    float cx = (floor(local.x * n0) + 0.5) / n0;
    float cy = (floor(local.y * n0) + 0.5) / n0;
    return texture(ColSampler, vec2(cx, cy));
}

// Heightfield lookup helpers for the march (void = +inf: never hit).
float ground_at(int L, vec2 local) {
    float h; bool w; float s;
    atlas_cell(L, local, h, w, s);
    if (h < VOID_Y) {
        // No data (unscanned chunk / cell): the ray must pass through, i.e.
        // the ground is FAR BELOW. Returning +1e8 made every void cell read as
        // a solid column above the ray, so the marcher "hit" it and painted a
        // fog-coloured slab (the flat sky-coloured rectangle).
        return -1.0e8;
    }
    return w ? LodParams.z : h;
}

// ---- states atlas (the blockModelAtlas) ------------------------------------
// Layout per state: 3 face-columns of 31 px (16/8/4/2/1 mip strip), 2 rows
// of 16 px; 16 states per atlas row. Mip m tile width = 16 >> m.
const float FACE_STRIDE = 31.0;
const float STATES_PER_ROW = 16.0;

vec4 state_tile(float sidf, float face, int mip, vec2 uvf) {
    // Exact texel fetch: the block atlas is pixel art packed edge to edge
    // (mip strip 16/8/4/2/1 inside a 31px face column), so any normalised
    // sample bleeds into the neighbouring mip/state — that was the confetti
    // (grass rows reading a transparent leaf texel -> cutout -> sky dots).
    int sid = int(sidf + 0.5);
    int tw = 16 >> mip;                       // 16, 8, 4, 2, 1
    int off = 32 - (32 >> mip);               // 0, 16, 24, 28, 30
    int sx = sid - (sid / 16) * 16;
    int sy = sid / 16;
    int ox = sx * 93 + int(floor(face * 0.5)) * int(FACE_STRIDE) + off;
    int oy = sy * 32 + int(mod(face, 2.0)) * 16;
    ivec2 t = clamp(ivec2(floor(uvf * float(tw))), ivec2(0), ivec2(tw - 1));
    return texelFetch(StatesSampler, ivec2(ox, oy) + t, 0);
}

// ---- vanilla cardinal face shading (Voxy computeDirectionalFaceTint) -------
float face_shade(float face) {
    if (face < 0.5) return 0.5;        // DOWN
    if (face < 1.5) return 1.0;        // UP
    if (face < 3.5) return 0.8;        // NORTH/SOUTH (z axis)
    return 0.6;                        // WEST/EAST (x axis)
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

vec2 atlas_uv(vec2 xz, vec2 origin, float size) {
    return clamp((xz - origin) / size, vec2(0.0015), vec2(0.9985));
}

/// Index of the side/cliff colour stored in the atlas alpha channel.
float atlas_side(vec2 uv) {
    return texture(AtlasSampler, uv).a;
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
    float sea = LodParams.z;

    vec2 minW = origin - vec2(size * 0.5);
    vec2 maxW = origin + vec2(size * 0.5);

    if (dir.y > 0.0) {
        float toCeil = (LodParams.y + 8.0 - cam.y) / dir.y;
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

    // March (Voxy level stepping): level L = floor(log2(t / cell)) clamped
    // to the pyramid depth; each step samples that level's heightfield with
    // a stride proportional to the level's cell size.
    float horizon = size * 0.75;
    float t = LodParams.x;
    float tHit = -1.0;
    int hitL = 0;
    float prevT = t;
    float prevDiff = 1.0e9;
    // Voxy draws real quads, so the visible face decides both the texture and
    // the vanilla face shade. We reconstruct the face from the march: entering a
    // column whose top is above the ray means we are looking at its SIDE (cliff)
    // face, otherwise the ray crossed the surface downwards -> TOP face.
    bool sideHit = false;
    float faceShade = 1.0;

    for (int i = 0; i < MAX_STEPS; ++i) {
        // Voxy picks the LOD level from the on-screen footprint, not distance:
        // one level cell ~= one pixel. At 1080p with cell=1 that is L0 out to
        // the whole atlas, i.e. full block detail (the coarse levels only kick
        // in at low resolutions / much longer ranges).
        float wpp = 2.0 * t * tanH / max(OutSize.y, 1.0);
        int L = int(clamp(floor(log2(max(wpp, cell) / cell)), 0.0, 4.0));
        float cellL = cell * exp2(float(L));
        float dt = max(cellL * 0.75, t * 0.010);
        prevT = t;
        t = min(t + dt, horizon);

        vec3 p = cam + dir * t;
        // origin is the atlas CORNER (packgen: center - half) — no +size/2.
        vec2 local = (p.xz - origin) / size;
        if (any(lessThan(local, vec2(0.0))) || any(greaterThan(local, vec2(1.0)))) {
            break; // left the atlas
        }
        float above = ground_at(L, local) - p.y;

        // above = ground - rayY: negative outside, positive inside the terrain.
        // A hit is the outside -> inside crossing (the v4.1 test; the ported
        // WIP had it reversed, so the marcher "hit" the first sky sample and
        // painted a fog-coloured slab over the sky).
        if (above >= 0.0 && prevDiff < 0.0) {
            float lo = prevT;
            float hi = t;
            for (int b = 0; b < BISECT; ++b) {
                float mid = (lo + hi) * 0.5;
                vec3 mp = cam + dir * mid;
                vec2 mlocal = (mp.xz - origin) / size;
                float mwpp = 2.0 * mid * tanH / max(OutSize.y, 1.0);
                int mL = int(clamp(floor(log2(max(mwpp, cell) / cell)), 0.0, 4.0));
                float g = ground_at(mL, mlocal);
                if (g - mp.y <= 0.0) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            tHit = (lo + hi) * 0.5;
            float hwpp = 2.0 * tHit * tanH / max(OutSize.y, 1.0);
            hitL = int(clamp(floor(log2(max(hwpp, cell) / cell)), 0.0, 4.0));
            break;
        }
        prevDiff = above;
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
    vec2 local = clamp((hitPos.xz - origin) / size, vec2(0.0), vec2(0.999999));

    // ---- data at the hit cell (Voxy Mipper-picked level L) -----------------
    int L = hitL;
    float cellL = cell * exp2(float(L));
    float h; bool isWater; float sid;
    atlas_cell(L, local, h, isWater, sid);

    // ---- FACE: which side of the column did the ray enter? -----------------
    // A heightfield raycast enters a column either through its TOP plane (the
    // ray descends across the surface) or through a SIDE wall (the column's
    // top sits above the ray). The old height-gradient normal called almost
    // every 1-block step a top face, so cliffs wore grass-top textures.
    // face: 1 UP, 2 NORTH, 3 SOUTH, 4 WEST, 5 EAST.
    bool sideHitNow = hitPos.y < h - 0.5;
    float face = !sideHitNow ? 1.0
        : (abs(dir.x) > abs(dir.z)
            ? (dir.x > 0.0 ? 4.0 : 5.0)   // travelling east -> the WEST wall
            : (dir.z > 0.0 ? 2.0 : 3.0)); // travelling south -> the NORTH wall

    vec4 colData = col_fetch(L, local);
    float lightId = floor(colData.a * 255.0 + 0.5);

    // Voxy greedy-quad tiling: one texture tile per 2^L cell. The UV plane
    // follows the FACE (a cliff shows the block side mapped in x/y, not x/z —
    // using xz everywhere made every wall a set of horizontal stripes).
    vec2 uvf = face < 1.5
        ? fract(hitPos.xz / cellL)                                  // UP / DOWN
        : (face < 3.5
            ? vec2(fract(hitPos.x / cellL), 1.0 - fract(hitPos.y / cellL))
            : vec2(fract(hitPos.z / cellL), 1.0 - fract(hitPos.y / cellL)));

    vec3 albedo;
    float texAlpha = 1.0;
    bool hasState = sid > 0.5;
    if (hasState) {
        // Voxy picks the TEXTURE mip from the on-screen footprint (one texel
        // ~= one pixel); sampling mip 0 at 900 blocks is pure shimmer/aliasing.
        float wppHit = 2.0 * tHit * tanH / max(OutSize.y, 1.0);
        int texMip = int(clamp(floor(log2(max(wppHit * 16.0 / cellL, 1.0))), 0.0, 4.0));
        vec4 tex = state_tile(sid, face, texMip, uvf);
        // Cutout on the SAMPLED mip: a mip-0 test dissolves distant canopies
        // into sky-dot confetti, a mip-averaged one keeps them solid.
        if (tex.a <= 0.1) {
            fragColor = scene;
            return;
        }
        albedo = tex.rgb;
        texAlpha = isWater ? 0.75 : tex.a;
    } else {
        // Unknown state: height gradient fallback (v4.1 look).
        float hn = clamp((hitPos.y - sea) / max(LodParams.y - sea, 1.0), 0.0, 1.0);
        albedo = isWater ? ColWater.rgb : mix(ColLow.rgb, ColHigh.rgb, smoothstep(0.15, 0.9, hn));
        texAlpha = isWater ? 0.75 : 1.0;
    }

    // Vanilla face shade (cardinal light), exact Voxy constants.
    float shade = face_shade(face);
    if (isWater) {
        face = 1.0;
        shade = 1.0;
    }

    // Biome tint: multiply into UP faces when the cell carries one
    // (white = untinted — matches Voxy's biome LUT behavior for the rest).
    vec3 tint = colData.rgb;
    bool tinted = any(greaterThan(abs(tint - vec3(1.0)), vec3(0.004)));
    if (tinted && ((face > 0.5 && face < 1.5) || isWater)) {
        albedo *= tint;
    }

    // Light = the exact lightmap texel for the cell's Voxy lightId.
    vec3 lightRGB = voxy_lightmap(lightId);
    vec3 lit = albedo * shade * lightRGB;

    // ---- FOG: the Voxy recipe, verbatim (v4.2-verified) --------------------
    vec4 fogCol = texelFetch(RotSampler, ivec2(1, 0), 0);
    vec4 fogRng = texelFetch(RotSampler, ivec2(0, 1), 0);
    bool fogLive = fogRng.a > 0.5 && fogCol.a > 0.004;
    float fStart = fogLive ? fogRng.r * 255.0 * 8.0 : FogParams.x;
    float fEnd = fogLive ? fogRng.g * 255.0 * 8.0 : FogParams.y;
    float vanRD = fogRng.b * 255.0 * 16.0;
    vec3 fogTint = fogLive ? fogCol.rgb : VeilColor.rgb;
    float fogOpacity = fogLive ? fogCol.a : 1.0;

    float fog = 0.0;
    if (abs(fEnd - fStart) > 1.0) {
        float invEndFogDelta = 1.0 / (fEnd - fStart);
        float startDelta = -fStart * invEndFogDelta;
        float endDistance = max(vanRD, 320.0) * 1.7320508;
        float maxFog = clamp(endDistance * invEndFogDelta + startDelta, 0.0, 1.0);
        fog = clamp(tHit * invEndFogDelta + startDelta, 0.0, maxFog) * fogOpacity;
    }
    lit = mix(lit, fogTint, fog);

    // ---- FADE: Voxy FogMode.FADE, MODE=1 (xz only), last 10% --------------
    float lenXZ = tHit * length(dir.xz);
    float rEdge = max(edge_distance_xz(cam, dir, minW, maxW), 1.0);
    float edge = 1.0 - clamp((lenXZ - FADE_START * rEdge) / max((1.0 - FADE_START) * rEdge, 1.0), 0.0, 1.0);

    vec3 outCol = mix(mix(scene.rgb, lit, fade), scene.rgb, edge);
    fragColor = vec4(outCol, scene.a);
}
