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
//     manual mip strip (16/8/4/2/1 px). uv = fract(worldXZ / 2^Lt) gives
//     the Voxy greedy-quad tiling — one texture tile per 2^Lt BLOCKS (the
//     tile level Lt follows the screen pixel footprint, quads.frag's
//     textureGrad behaviour), side faces tile vertically along world Y.
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
const int MAX_STEPS = 220;
const int BISECT = 7;
const float VOID_Y = -63.5;   // cells below this are unscanned = no data
const float FADE_START = 0.9; // Voxy: fade begins at 90% of the view radius
const float DEBUG = __DEBUG__; // 0 off; 1 = sky->vec3(conf,0,0); 2 = hit distance

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

// Full lightmap texel for a Voxy lightId (analysis/voxy-e2e.md §1.1:
// bits 56-63 = (blockLight << 4) | skyLight — the BLOCK nibble is HIGH;
// the scan writes 0x0F for sunlit surface cells). v4.3.1-rc swapped the
// nibbles AND rounded them with floor(x+0.5), which decoded 0x0F as
// block-15/sky-1 — a near-zero lightmap: the whole LOD rendered BLACK.
// lightId is an exact integer float here, so plain floor division is
// exact for both nibbles.
vec3 voxy_lightmap(float lightId) {
    float block = floor(lightId / 16.0);
    float sky = lightId - block * 16.0;
    vec3 color = vanilla_lightmap0_15(GameTime) * lm_brightness(sky / 15.0);
    // BlockLightColor = mix(BlockLightTint, 1, 0.9 * (2b-1)^2), tint = white.
    float b = block / 15.0;
    vec3 blockColor = mix(vec3(1.0), vec3(1.0), 0.9 * (2.0 * b - 1.0) * (2.0 * b - 1.0));
    color += blockColor * lm_brightness(b);
    return color;
}

// ---- atlas access -----------------------------------------------------------
// The pyramid: level L is a (cells / 2^L)-pixel square sitting at row
// topPx(L) = cells * (2 - 2^(1-L)) of a PNG that is `cells` texels wide
// (levels stack below each other, each left-aligned). The total PNG
// height (levels 0..4) = cells * (2 - 2^-4) = cells * 31/16.
float atlas_cells() { return AtlasInfo.z / AtlasInfo.w; }

// Cells across the atlas at level L (each cell covers cell * 2^L blocks).
// MUST divide by 2^L — the strip SHRINKS with coarser levels (v4.3.0
// multiplied here, which read neighbouring strips as garbage heights).
float level_cells(int L) { return atlas_cells() / exp2(float(L)); }

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
// u is normalized by the FULL PNG width (atlas_cells texels) — the level
// strip is left-aligned, so u must NOT be normalized by the level's cell
// count (that stretched L>=1 over the whole width and into the padding).
void atlas_cell(int L, vec2 local, out float height, out bool water, out float state) {
    float cellsL = level_cells(L);
    float topPx = level_top(L);
    float totalH = atlas_total_h();
    float cx = (floor(local.x * cellsL) + 0.5) / atlas_cells();
    float cy = (topPx + floor(local.y * cellsL) + 0.5) / totalH;
    // textureLod, not texture: these fetches run inside the march loop whose
    // break paths diverge across the quad — implicit derivatives are
    // UNDEFINED there. On strict drivers (llvmpipe in CI) the helper-invocation
    // garbage produced wrong fetches and the whole LOD silently vanished
    // (V3 dark-LOD fraction 0.0%). The data PNGs carry a single mip, so lod 0
    // is byte-identical to the intent and derivative-free.
    vec4 a = textureLod(AtlasSampler, vec2(cx, cy), 0.0);
    height = atlas_height_at(a);
    water = a.b > 0.5;
    state = floor(a.a * 255.0 + 0.5);
}

// Light + tint fetch. The colour PNG is LEVEL-0 resolution (cells x cells);
// coarse cells resample the L0 winner, which is the group's +x+z corner
// cell (exactly the Mipper's pick: highest corner rank wins ties).
vec4 col_fetch(int L, vec2 local) {
    float stepL = exp2(float(L));          // group size in L0 cells
    float cellsL = atlas_cells() / stepL;
    float n = atlas_cells();
    float ix0 = (floor(local.x * cellsL) + 1.0) * stepL - 1.0;
    float iz0 = (floor(local.y * cellsL) + 1.0) * stepL - 1.0;
    float cx = (min(ix0, n - 1.0) + 0.5) / n;
    float cy = (min(iz0, n - 1.0) + 0.5) / n;
    // textureLod: same divergence argument as atlas_cell (this runs after
    // early returns, so quad derivatives are already undefined).
    return textureLod(ColSampler, vec2(cx, cy), 0.0);
}

// Heightfield lookup helpers for the march.
// Unscanned (void) cells return -inf: the ray is always ABOVE them, so an
// entry test can never fire inside unknown territory (v4.3.0 used +inf,
// which under entry semantics made void cells an infinite wall to enter).
float ground_at(int L, vec2 local) {
    float h; bool w; float s;
    atlas_cell(L, local, h, w, s);
    if (h < VOID_Y) {
        return -1.0e8;
    }
    return w ? LodParams.z : h;
}

// ---- states atlas (the blockModelAtlas) ------------------------------------
// Layout per state: 3 face-columns of 31 px (16/8/4/2/1 mip strip), 2 rows
// of 16 px; 16 states per atlas row. Mip m tile width = 16 >> m.
const float FACE_STRIDE = 31.0;
const float STATES_PER_ROW = 16.0;

vec4 state_tile(float sid, float face, int mip, vec2 uvf) {
    float tw = exp2(float(4 - mip));          // 16 >> mip
    // mip strip offsets within one 31px face column: 0,16,24,28,30 = 32-2^(5-m).
    float off = 32.0 - exp2(5.0 - float(mip));
    float sx = mod(sid, STATES_PER_ROW);
    float sy = floor(sid / STATES_PER_ROW);
    float ox = sx * 3.0 * FACE_STRIDE + floor(face / 2.0) * FACE_STRIDE + off;
    float oy = sy * 32.0 + mod(face, 2.0) * 16.0;
    // The clamp already insets half a texel: texUV spans EXACT texel
    // centers [ox+0.5, ox+tw-0.5]. The extra +0.5 the v4.3.0 pass added
    // shifted every tile one texel over — its right/bottom edge then
    // bilinear-blended with the NEIGHBOURING tile (another face or another
    // block state) — the reported "block textures overlap" artifact.
    vec2 texUV = vec2(ox, oy) + clamp(uvf, 0.5 / tw, 1.0 - 0.5 / tw) * tw;
    // textureLod: the mip strip is selected MANUALLY via `off`; implicit
    // derivatives would let the driver pick mips of the whole states atlas
    // (and are undefined under divergent control flow anyway).
    return textureLod(StatesSampler, texUV / StatesInfo.yz, 0.0);
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
        if (DEBUG > 0.5 && DEBUG < 1.5) {
            fragColor = vec4(conf, 0.0, 0.0, 1.0); // black = never locked
            return;
        }
        fragColor = scene;
        return;
    }
    if (DEBUG > 0.5 && DEBUG < 1.5) {
        fragColor = vec4(conf, 0.0, 0.0, 1.0); // red = camera locked
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

    vec2 minW = origin;
    vec2 maxW = origin + vec2(size);
    // origin is the atlas CORNER (packgen: center - half) — v4.3.1-rc built
    // the window as origin ∓ size/2, i.e. the quadrant [origin-size, origin]
    // — half the atlas. The edge fade then only released one screen half
    // (a hard vertical boundary through the camera) and the dir.y>0 exit
    // test culled the wrong rays on the other half.

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
    //
    // HIT = ENTRY into the heightfield: above = ground - p.y crosses from
    // <= 0 (ray point above the surface) to > 0 (ray point under it). The
    // v4.3-and-earlier test had the signs swapped — it caught the EXIT from
    // terrain, i.e. the far side of ridges. Consequences: on flat ground the
    // ray dives below the surface and never exits, so flat/low-relief worlds
    // rendered NO LOD at all (CI V3 dark-LOD fraction 0.0% on every v4.3
    // run), and relief worlds rendered back-slopes with inverted lighting.
    float horizon = size * 0.75;
    float t = LodParams.x;
    float tHit = -1.0;
    int hitL = 0;

    // Sample the START point before stepping: the entry test needs a real
    // "above surface" reference. A sentinel would fake a hit at the first
    // step (> 0) or suppress the first legit entry (< 0).
    float prevT = t;
    // The camera stands in AIR above the heightfield: the surface hit is the
    // air -> underground transition of 'above' (<= 0 -> > 0). v4.3.1-rc
    // tested the OPPOSITE transition with prevDiff = +1e9, so the very first
    // sample fired the branch and the bisect collapsed to ~lod_near — every
    // downward ray got a ghost hit at a fixed distance (the "ghost wall").
    // The START point is sampled for real (not a sentinel): a sentinel > 0
    // fakes a hit at the first step, a sentinel < 0 fakes an entry for rays
    // that are already inside the heightfield at lod_near (steep-down rays;
    // those crossings sit inside the real-chunk band, where the depth gate
    // wins anyway, so they must not paint anything).
    float prevDiff;
    {
        vec3 p0 = cam + dir * t;
        vec2 local0 = (p0.xz - origin) / size;
        if (any(lessThan(local0, vec2(0.0))) || any(greaterThan(local0, vec2(1.0)))) {
            prevDiff = -1.0; // start off-atlas: open sky, entries ahead are real
        } else {
            int L0 = int(clamp(floor(log2(max(t, cell) / cell)), 0.0, 4.0));
            prevDiff = ground_at(L0, local0) - p0.y;
        }
    }

    for (int i = 0; i < MAX_STEPS; ++i) {
        int L = int(clamp(floor(log2(max(t, cell) / cell)), 0.0, 4.0));
        float cellL = cell * exp2(float(L));
        float dt = max(cellL * 0.35, t * (2.0 * tanH / max(OutSize.y, 1.0)));
        prevT = t;
        t = min(t + dt, horizon);

        vec3 p = cam + dir * t;
        // origin is the atlas CORNER (packgen: center - half) — no +size/2.
        vec2 local = (p.xz - origin) / size;
        if (any(lessThan(local, vec2(0.0))) || any(greaterThan(local, vec2(1.0)))) {
            break; // left the atlas
        }
        float above = ground_at(L, local) - p.y;

        if (above > 0.0 && prevDiff <= 0.0) {
            // lo tracks the air side (ground below the ray), hi the
            // underground side (ground above the ray); converge on the
            // surface crossing.
            float lo = prevT;
            float hi = t;
            for (int b = 0; b < BISECT; ++b) {
                float mid = (lo + hi) * 0.5;
                vec3 mp = cam + dir * mid;
                vec2 mlocal = (mp.xz - origin) / size;
                int mL = int(clamp(floor(log2(max(mid, cell) / cell)), 0.0, 4.0));
                float g = ground_at(mL, mlocal);
                if (g - mp.y <= 0.0) {
                    lo = mid;   // still air: the hit is farther
                } else {
                    hi = mid;   // underground: the hit is closer
                }
            }
            tHit = (lo + hi) * 0.5;
            hitL = int(clamp(floor(log2(max(tHit, cell) / cell)), 0.0, 4.0));
            break;
        }
        prevDiff = above;
        if (t >= horizon) {
            break;
        }
    }

    if (tHit < 0.0) {
        if (DEBUG > 1.5 && DEBUG < 2.5) {
            // debug 2: sky where the march found nothing (depth = miss)
            fragColor = vec4(0.0, 0.0, 1.0, 1.0);
            return;
        }
        fragColor = scene; // open sky (or out of atlas): vanilla wins
        return;
    }
    if (DEBUG > 1.5 && DEBUG < 2.5) {
        // debug 2: green = LOD hit, brightness = tHit / 512 blocks
        fragColor = vec4(0.0, clamp(tHit / 512.0, 0.0, 1.0), 0.0, 1.0);
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

    // Height-gradient normal at the hit level (Voxy-flat quads).
    vec2 e = vec2(1.0 / level_cells(L), 0.0);
    float hL; bool wtmp; float stmp;
    atlas_cell(L, clamp(local - e.xy, vec2(0.0), vec2(0.999999)), hL, wtmp, stmp);
    float hR; atlas_cell(L, clamp(local + e.xy, vec2(0.0), vec2(0.999999)), hR, wtmp, stmp);
    float hD; atlas_cell(L, clamp(local - e.yx, vec2(0.0), vec2(0.999999)), hD, wtmp, stmp);
    float hU; atlas_cell(L, clamp(local + e.yx, vec2(0.0), vec2(0.999999)), hU, wtmp, stmp);
    vec3 n = normalize(vec3(hL - hR, 2.0 * cellL, hD - hU));

    // ---- FACE + TEXTURE (quads.frag math) ----------------------------------
    float face = n.y > 0.577 ? 1.0 : (abs(n.x) > abs(n.z)
        ? (n.x > 0.0 ? 5.0 : 4.0)
        : (n.z > 0.0 ? 3.0 : 2.0));

    vec4 colData = col_fetch(L, local);
    float lightId = floor(colData.a * 255.0 + 0.5);

    // ---- Voxy texture detail level (quads.frag textureGrad) ----------------
    // The TILE level follows the screen pixel footprint (one texture tile
    // per 2^Lt blocks, one mip step per level: the mip-m strip tile is
    // (16>>m) px over 2^m blocks = 1 texel per block at EVERY distance),
    // NOT the march's data level — a 64-block cell of one state still
    // repeats its texture per block, exactly like a Voxy greedy quad.
    float pxFoot = tHit * (2.0 * tanH / max(OutSize.y, 1.0));
    float Lt = clamp(ceil(log2(max(pxFoot, 0.5))), 0.0, 4.0);

    // Greedy-quad uv, world-anchored. UP faces tile in xz; side faces tile
    // along the horizontal axis AND vertically along world Y (Voxy quad uv
    // is 2D in cells) — without the Y term every cliff showed one smeared
    // texel row instead of stacked texture tiles.
    vec2 uvWorld = (abs(n.x) > abs(n.z)) ? vec2(hitPos.z, hitPos.y)
                                         : vec2(hitPos.x, hitPos.y);
    if (face < 1.5) {
        uvWorld = hitPos.xz; // UP (the heightfield never yields DOWN)
    }
    vec2 uvf = fract(uvWorld / exp2(Lt));

    vec3 albedo;
    float texAlpha = 1.0;
    bool hasState = sid > 0.5;
    if (hasState) {
        vec4 tex = state_tile(sid, face, int(Lt), uvf);
        // Cutout (Voxy: alpha <= 0.1 shows sky). The sample IS the mip-Lt
        // texel: L0 = per-block holes (leaves), coarse = the tile's averaged
        // alpha (distant leaves render solid — no random hole-punching).
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

    // v4.3.0 REGRESSION FIX: the factor below was passed INVERTED — `edge` is
    // 1.0 in the atlas interior and 0.0 at the rim, so mix(..., scene, edge)
    // drew VANILLA SKY everywhere except the outer 10% ring of the atlas.
    // The LOD was invisible across ~90% of the window on every GPU (CI V3
    // dark fraction 0.0%; live testing only ever saw the far rim).
    // 1.0 - edge is the rim factor: 0 interior (draw the LOD), 1 at the rim
    // (hand back to vanilla sky) — the actual Voxy FogMode.FADE behaviour.
    vec3 outCol = mix(mix(scene.rgb, lit, fade), scene.rgb, 1.0 - edge);
    fragColor = vec4(outCol, scene.a);
}
