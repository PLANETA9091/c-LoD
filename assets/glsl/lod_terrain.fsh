#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD pass 4 (v4.4.0 "VOXY CORE"): the LOD renderer, with the SUBSTANCE
// of Voxy's LOD pipeline (analysis/voxy-e2e.md), not just its colors:
//
//   - The height PNG carries the real Voxy mip pyramid: level 0 cells plus
//     levels 1..4 reduced by the ported Mipper. v4.3.3 marches the LEVEL-0
//     field exclusively: the old distance-keyed level schedule handed
//     64x64 regions to a single Mipper-picked column, whose corner pick
//     leaked ocean through canopies, floated trunk tips as flat bark slabs
//     and sprayed sky dots along every terrace edge. The L0 field is the
//     real scanned surface: real heights, no picks, no leaks.
//   - v4.4.0 SHADING detail selection: geometry stays L0, but the shading
//     inputs (state tile, gradient normal, tint, light) coarsen through the
//     SAME pyramid once a surface pixel covers more than one L0 cell
//     (Voxy's screen-area detail rule). Below that rate every L0 cell is
//     its own texel and the per-column variety (stone/grass/dirt/leaves on
//     one mountainside) IS the texture — sub-pixel detail sampled at the
//     tile rate: the far horizon rendered as 1-px salt-and-pepper mush
//     (run-30 evidence). Coarsened, neighbouring pixels share one coarse
//     cell: one state, one face, one tint — the smooth Voxy far masses
//     (green forests, gray peaks, blue water) with zero data changes.
//   - Every hit is textured from the blockModelAtlas (baked server-side from
//     the real vanilla 26.3 block textures): per state 3x2 face tiles +
//     manual mip strip (16/8/4/2/1 px). The tile grid is WORLD-ANCHORED at
//     vanilla scale (one 16px tile per 16 blocks = 1 texel/block) and
//     therefore seamless across whatever geometry the march hits; the mip
//     follows the true texel rate like quads.frag's textureGrad
//     (tpp = 16*pxSurface/16, mip = log2(tpp), trilinear across the strip):
//     distant terrain auto-blurs to the tile's average colour, side faces
//     tile vertically along world Y.
//   - Fragment math = Voxy quads.frag: albedo x (lightmap x faceShade) x
//     biomeTint, cutout rendered SOLID (every heightfield cell is a merged
//     quad — makeQuadFlags disables discard for quadSize > 1 — and a
//     posteffect pixel has no geometry behind it to reveal).
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
const int MAX_STEPS = 260;
const int BISECT = 7;
const float VOID_Y = -63.5;   // cells below this are unscanned = no data
const float FADE_START = 0.9; // Voxy: fade begins at 90% of the view radius
const float TILE_BLOCKS = 16.0; // one 16px texture tile per block (vanilla scale)
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

    // March on the LEVEL-0 field (v4.3.3). The heightfield is sampled at the
    // RAW scan resolution (`cell`-block cells) for BOTH the stepping and the
    // hit — Voxy's screen-area detail selection keeps the finest level while
    // a cell still covers more than ~a pixel, and inside this atlas it
    // always does: even at the horizon a 4-block cell spans 384*0.75*2*tanH
    // / OutSize.y ≈ 1.5 blocks/px on a 360p render — 2.7 px per cell (more
    // on every real resolution). The old schedule L = floor(log2(t/cell))
    // ignored the screen metric entirely and reached L4 (64-block cells)
    // already at lod_near — ONE scanned column then decided the height of a
    // 64x64 region (the Mipper corner pick): rays sailed over canopies whose
    // corner column was low (sky-leak rectangles showing the ocean behind),
    // trunk-tip columns floated as flat bark slabs above the canopy line,
    // and every height discontinuity sprayed single-pixel vanilla-sky dots
    // along terrace edges. The L0 field is the real scanned surface — no
    // pick, no average, no leaks.
    //
    // HIT = ENTRY into the heightfield: above = ground - p.y crosses from
    // <= 0 (ray point above the surface) to > 0 (ray point under it).
    float horizon = size * 0.75;
    float t = LodParams.x;
    float tHit = -1.0;

    // Sample the START point before stepping: the entry test needs a real
    // "above surface" reference. A sentinel would fake a hit at the first
    // step (> 0) or suppress the first legit entry (< 0). The START point is
    // sampled for real (not a sentinel): a sentinel > 0 fakes a hit at the
    // first step, a sentinel < 0 fakes an entry for rays that are already
    // inside the heightfield at lod_near (steep-down rays; see the
    // SKY-LEAK RESCUE below for how those are painted).
    float prevT = t;
    float prevDiff;
    // The llvmpipe JIT (CI client + local harness) segfaults on a bool flag
    // kept alive across the march loop together with the rescue assignment
    // below — an exact float 0/1 flag compiles and runs identically.
    float startUnder; // 1.0 = the ray is already INSIDE the heightfield at lod_near
    {
        vec3 p0 = cam + dir * t;
        vec2 local0 = (p0.xz - origin) / size;
        if (any(lessThan(local0, vec2(0.0))) || any(greaterThan(local0, vec2(1.0)))) {
            prevDiff = -1.0; // start off-atlas: open sky, entries ahead are real
            startUnder = 0.0;
        } else {
            prevDiff = ground_at(0, local0) - p0.y;
            startUnder = prevDiff > 0.0 ? 1.0 : 0.0;
        }
    }

    for (int i = 0; i < MAX_STEPS; ++i) {
        float dt = max(cell * 0.35, t * (2.0 * tanH / max(OutSize.y, 1.0)));
        prevT = t;
        t = min(t + dt, horizon);

        vec3 p = cam + dir * t;
        // origin is the atlas CORNER (packgen: center - half) — no +size/2.
        vec2 local = (p.xz - origin) / size;
        if (any(lessThan(local, vec2(0.0))) || any(greaterThan(local, vec2(1.0)))) {
            break; // left the atlas
        }
        float above = ground_at(0, local) - p.y;

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
                float g = ground_at(0, mlocal);
                if (g - mp.y <= 0.0) {
                    lo = mid;   // still air: the hit is farther
                } else {
                    hi = mid;   // underground: the hit is closer
                }
            }
            tHit = (lo + hi) * 0.5;
            break;
        }
        prevDiff = above;
        if (t >= horizon) {
            break;
        }
    }

    // SKY-LEAK RESCUE: a ray that is under the heightfield at lod_near pierced
    // the surface INSIDE the real-chunk band (t < lod_near). When the server's
    // chunk view distance is SHORTER than lod_near (CI e2e: view-distance=4 =
    // 64 blocks < lod_near 96) that piercing is NOT depth-covered, the march
    // never re-enters on flat/downhill ground, and the pixel showed raw
    // vanilla sky/fog — flat pale rectangles with ruler-straight edges
    // (run-30 02_noon_high). Paint the START cell instead: the terrain that
    // continues seamlessly from the real-chunk edge.
    if (tHit < 0.0 && startUnder > 0.5) {
        tHit = LodParams.x; // the march's own start distance (t was mutated)
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

    // ---- data at the hit cell (the RAW L0 scan — v4.3.3) -------------------
    float cellL = cell;
    float h; bool isWater; float sid;
    atlas_cell(0, local, h, isWater, sid);

    // Provisional L0 height-gradient normal (feeds the grazing factor below;
    // re-derived at the shading level when one is selected).
    vec2 e = vec2(1.0 / level_cells(0), 0.0);
    float hL; bool wtmp; float stmp;
    atlas_cell(0, clamp(local - e.xy, vec2(0.0), vec2(0.999999)), hL, wtmp, stmp);
    float hR; atlas_cell(0, clamp(local + e.xy, vec2(0.0), vec2(0.999999)), hR, wtmp, stmp);
    float hD; atlas_cell(0, clamp(local - e.yx, vec2(0.0), vec2(0.999999)), hD, wtmp, stmp);
    float hU; atlas_cell(0, clamp(local + e.yx, vec2(0.0), vec2(0.999999)), hU, wtmp, stmp);
    vec3 n = normalize(vec3(hL - hR, 2.0 * cellL, hD - hU));

    // ---- screen footprint (quads.frag textureGrad, vanilla scale) ----------
    // pxSurface is the footprint ALONG THE SURFACE: a ray grazing the
    // heightfield sweeps pxFoot / |dot(dir,n)| blocks per pixel — without
    // the grazing factor mountainsides under-mip and alias. tpp < 1 (pixel
    // smaller than a texel) clamps to mip 0 = vanilla-crisp magnification;
    // at the far edge tpp >> 16 reaches the strip's 1px average — Voxy's
    // solid-colour distant terrain.
    float pxFoot = tHit * (2.0 * tanH / max(OutSize.y, 1.0));
    float grazing = max(abs(dot(dir, n)), 0.08);
    float pxSurface = pxFoot / grazing;

    // ---- v4.4.0 SCREEN-AREA SHADING LEVEL ----------------------------------
    // Voxy's detail rule: keep the finest level while a cell still covers
    // more than ~a pixel — past that rate the level's own granularity IS
    // sub-pixel and rendering it per-column aliases into noise. The march
    // stays L0 (real silhouette), but once a surface pixel spans more than
    // one L0 cell the shading inputs switch to the pyramid level whose cell
    // matches the footprint: state/water from the Mipper-max cell (the
    // region's top surface), the gradient normal over one coarse step, and
    // tint/light from that group's winner. Neighbouring pixels then share
    // one state, one face, one tint — smooth far masses instead of the
    // per-column stone/grass/dirt speckle the far band degraded into.
    // Discrete like every LOD scheme; the switch ring sits where one cell
    // ≈ one pixel and is invisible at that rate.
    float cpp = pxSurface / cell; // L0 cells per surface pixel
    int Ls = int(clamp(floor(log2(max(cpp, 1.0))), 0.0, 4.0));
    if (Ls > 0) {
        float stepBlocks = cell * exp2(float(Ls));
        atlas_cell(Ls, local, h, isWater, sid);
        vec2 ec = vec2(1.0 / level_cells(Ls), 0.0);
        atlas_cell(Ls, clamp(local - ec.xy, vec2(0.0), vec2(0.999999)), hL, wtmp, stmp);
        atlas_cell(Ls, clamp(local + ec.xy, vec2(0.0), vec2(0.999999)), hR, wtmp, stmp);
        atlas_cell(Ls, clamp(local - ec.yx, vec2(0.0), vec2(0.999999)), hD, wtmp, stmp);
        atlas_cell(Ls, clamp(local + ec.yx, vec2(0.0), vec2(0.999999)), hU, wtmp, stmp);
        n = normalize(vec3(hL - hR, 2.0 * stepBlocks, hD - hU));
    }

    // ---- FACE + TEXTURE (quads.frag math) ----------------------------------
    float face = n.y > 0.577 ? 1.0 : (abs(n.x) > abs(n.z)
        ? (n.x > 0.0 ? 5.0 : 4.0)
        : (n.z > 0.0 ? 3.0 : 2.0));

    // Tint + light follow the same level as the state: at Ls > 0 the group's
    // +x+z winner cell carries the region's biome tint and light id exactly
    // like the Mipper's own reduction would.
    vec4 colData = col_fetch(Ls, local);
    float lightId = floor(colData.a * 255.0 + 0.5);

    // ---- texture detail (quads.frag textureGrad, vanilla scale) ------------
    // v4.3.3 decouples the tile grid from the heightfield level. The old
    // uvf = fract(uvWorld / 2^L) repeated one tile per 2^L blocks while the
    // hit quads were 64-block L4 cells: the 16-block tile grid never aligned
    // with the quad edges, textures re-anchored at every level switch, and
    // neighbouring cells at different heights showed offset copies of the
    // same tile — the "textures overlap" report. The tile now repeats every
    // TILE_BLOCKS (16) blocks of WORLD space — vanilla's 1 texel/block —
    // world-anchored and identical across every cell, so the grid is ALWAYS
    // seamless regardless of what geometry the march hits.
    //
    // The mip follows the true texel rate, exactly quads.frag's
    // textureGrad(blockModelAtlas, texPos, dFdx(uv), dFdy(uv)):
    //   texels-per-pixel at mip 0 = 16 * pxSurface / TILE_BLOCKS
    //   mip m divides the rate by 2^m; a real GPU trilinearly resolves
    //   m = log2(tpp), blending the two strips around it.
    // pxSurface was computed above from the L0-provisional grazing factor —
    // the shading-level normal differs only in smooth regions where the
    // mip is already clamped, so reusing it keeps one footprint for both
    // the level pick and the strip pick.
    float tpp = 16.0 * pxSurface / TILE_BLOCKS; // texels per pixel at mip 0
    // Continuous LOD for the manual strip: a real GPU trilinearly BLENDS the
    // two mips around the continuous LOD log2(tpp); rounding alone hard-
    // switches strips per pixel, and the round() contour lines across a
    // terraced surface painted visible ring/dither zones (harness zoom on
    // the sand terraces). Mix floor/ceil strips by the fractional part -
    // the exact behaviour quads.frag gets from the sampler for free.
    float mipF = clamp(log2(max(tpp, 0.03125)), 0.0, 4.0);

    // World-anchored uv: UP faces tile in xz; side faces tile along the
    // horizontal axis AND vertically along world Y — without the Y term
    // every cliff showed one smeared texel row instead of stacked tiles.
    vec2 uvWorld = (abs(n.x) > abs(n.z)) ? vec2(hitPos.z, hitPos.y)
                                         : vec2(hitPos.x, hitPos.y);
    if (face < 1.5) {
        uvWorld = hitPos.xz; // UP (the heightfield never yields DOWN)
    }
    vec2 uvf = fract(uvWorld / TILE_BLOCKS);

    vec3 albedo;
    // StatesInfo.x = the pack's state count. A client can hold a STALE pack
    // (join-time pack swap races: "post effect does not exist" window) while
    // the server atlas already uses a newer registry — out-of-range sids then
    // clamp into a random neighbouring tile. The count guard falls back to
    // the height-gradient colours instead of garbage block textures.
    bool hasState = sid > 0.5 && sid < StatesInfo.x;
    if (hasState) {
        // Manual trilinear across the mip strip (see mipF above): m0/m1 and
        // the blend factor reproduce the sampler's behaviour between two
        // adjacent strip tiles.
        int m0 = int(floor(mipF));
        int m1 = min(m0 + 1, 4);
        float mf = mipF - float(m0);
        vec4 tex = mix(state_tile(sid, face, m0, uvf),
                       state_tile(sid, face, m1, uvf), mf);
        // Cutout: NO discard (v4.3.3). quads.frag only discards UNMERGED
        // quads (makeQuadFlags disables it for quadSize > 1), and every cell
        // of this heightfield IS a merged surface — 4x4 columns behind one
        // hit. A discard here would punch through to `scene` = raw vanilla
        // sky (there is no geometry behind a posteffect pixel), spraying
        // sky-speckle over every canopy. The atlas tiles are SOLIDIFIED at
        // bake time (transparent texels carry their nearest-opaque RGB), so
        // rendering them opaque gives exactly the merged-quad look: distant
        // forests with no sky holes.
        albedo = tex.rgb;
    } else {
        // Unknown state: height gradient fallback (v4.1 look).
        float hn = clamp((hitPos.y - sea) / max(LodParams.y - sea, 1.0), 0.0, 1.0);
        albedo = isWater ? ColWater.rgb : mix(ColLow.rgb, ColHigh.rgb, smoothstep(0.15, 0.9, hn));
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
