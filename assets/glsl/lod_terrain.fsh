#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD pass 4: the actual level-of-distant-detail renderer.
//
// v4.2 — Distant-Horizons-style VOXEL renderer. The atlas cells are sampled
// NEAREST (one constant height per cell), so the marched surface is a true
// voxel staircase, not a smooth blob: tops are flat, cliffs are vertical
// walls shaded with vanilla's directional face light (top 1.0, N/S 0.8,
// E/W 0.6, down 0.5), exactly like DH's flat-colour look.
//
// DH-style SLAB geometry: every cell is up to two boxes —
//   surface slab  [bot .. height]   (a canopy, floating island, overhang)
//   ground box    [..    seg2  ]    (the terrain BELOW an air gap)
// so tree canopies get walls and an underside, and the ground below them is
// drawn as well. Cells without a gap are solid down (bot = seg2 = -64).
//
// The march is accelerated by a conservative max-height pyramid: a ray that
// clears a whole 2^k-cell block (checked at BOTH segment ends, so the linear
// ray cannot dip inside) jumps straight to the block boundary. This keeps the
// base step small (cell * 0.75) — crisp voxel edges — while still reaching
// the far edge of a 16k atlas.
//
// For every pixel that shows sky (device depth ~ 1) we march a view ray
// against the atlas shipped in the server resource pack. The ray is built
// from the camera basis delivered by the CAMERA CHANNEL (v4.1): the sky
// shader encodes yaw/pitch/fov, col_scan + rot_solve decode it into the
// clod_rot target. Camera position comes from the Globals UBO.
//
// Non-sky pixels pass through untouched: real chunks always win.

uniform sampler2D InSampler;      // minecraft:main color
uniform sampler2D InDepthSampler; // minecraft:main depth
uniform sampler2D AtlasSampler;   // RG = height, B = water, A = side colour index
uniform sampler2D GeomSampler;    // RG = slab bottom, BA = ground top below gap
uniform sampler2D PyrSampler;     // conservative max-height pyramid
uniform sampler2D ColorSampler;   // top-face block colour (baked texture average)
uniform sampler2D GroundSampler;  // ground-segment top colour below gaps
uniform sampler2D SideSampler;    // 256x1 side/cliff colour LUT
uniform sampler2D RotSampler;     // clod:clod_rot_a (1x1 persistent, camera channel)

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

// x = fog start (blocks), y = fog end (blocks), z = static fov fallback, w = unused
layout(std140) uniform ClodFog {
    vec4 FogParams;
};

// x = debug mode (0 = normal, 1 = confidence heat, 2 = split, 3 = pattern,
// 4 = hit mask, 5 = hit distance)
layout(std140) uniform ClodDebug {
    vec4 DebugParams;
};

// x = device depth near plane (blocks), y = cloud ceiling (atlas max + margin)
layout(std140) uniform ClodSky {
    vec4 SkyParams;
};

layout(std140) uniform ClodColors {
    vec4 ColLow;    // low terrain
    vec4 ColHigh;   // high terrain
    vec4 ColWater;  // water surface
    vec4 VeilColor; // horizon veil (rgb) + snow whiteness (a)
};

// x = pyramid level count, y = cell size (blocks)
layout(std140) uniform ClodPyr {
    vec4 PyrParams;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;
// 26.3 device depth: d = near/eyeZ (infinite far) — sky == 0.0, any real
// geometry reads positive, so 1e-6 keeps terrain out to ~50 km classified.
const float SKY_DEPTH = 1.0e-6;
const int MAX_STEPS = 460;
const int BISECT = 14;

// DH-look tuning: quantized world-position noise stands in for textures the
// way DH's "fake texture noise" does — luminance-scaled, fading with distance.
const float NOISE_AMP = 0.18;     // max brighten/darken
const float NOISE_STEPS = 2.0;    // noise cell size (blocks)
const float NOISE_FAR = 2000.0;   // start fading the noise out
const float NOISE_END = 6000.0;   // fully gone
// A jump is only safe when the ray clears the block max by this margin.
const float SKIP_MARGIN = 1.0;

// Height (block Y) stored in the atlas: (y + 64) << 6, big-endian RG.
float atlas_height(vec2 uv) {
    vec4 a = texture(AtlasSampler, uv);
    float y16 = a.r * 65280.0 + a.g * 255.0; // r*255*256 + g*255
    return y16 / 64.0 - 64.0;
}

bool atlas_water(vec2 uv) {
    return texture(AtlasSampler, uv).b > 0.5;
}

vec2 atlas_uv(vec2 xz, vec2 origin, float size) {
    return clamp((xz - origin) / size, vec2(0.0015), vec2(0.9985));
}

/// One atlas cell: surface slab [bot..top] plus optional ground below a gap.
/// Unscanned cells are void (valid = false); gap-less columns have
/// bot = seg2 = -64 (solid down, no second surface) — the encoder writes
/// zeros for them, which decodes to exactly that.
struct Cell {
    bool valid;
    bool water;
    float top;   // render height of the surface (sea level for water)
    float bot;   // bottom of the surface slab (-64 = solid down)
    float g2;    // top of the ground segment below the gap (-64 = none)
    float side;  // side/cliff colour LUT index (0..1)
};

Cell cellAt(ivec2 cid, vec2 origin, float cell, int n, float sea) {
    Cell c;
    c.valid = false;
    c.water = false;
    c.top = -64.0;
    c.bot = -64.0;
    c.g2 = -64.0;
    c.side = 0.0;
    if (cid.x < 0 || cid.y < 0 || cid.x >= n || cid.y >= n) {
        return c;
    }
    vec4 a = texelFetch(AtlasSampler, cid, 0);
    float h16 = a.r * 65280.0 + a.g * 255.0;
    if (h16 < 0.5) {
        return c; // never scanned -> void
    }
    c.valid = true;
    c.water = a.b > 0.5;
    c.side = a.a;
    c.top = c.water ? sea : h16 / 64.0 - 64.0;
    vec4 g = texelFetch(GeomSampler, cid, 0);
    float b16 = g.r * 65280.0 + g.g * 255.0;
    float s16 = g.b * 65280.0 + g.a * 255.0;
    c.bot = b16 > 0.5 ? b16 / 64.0 - 64.0 : -64.0;
    c.g2 = s16 > 0.5 ? s16 / 64.0 - 64.0 : -64.0;
    return c;
}

/// Solid test for a ray point: inside the surface slab OR inside the ground
/// below the gap. The open interval at `bot` is the air under a canopy —
/// rays there keep marching (DH's under-canopy view).
bool solid(Cell c, float y) {
    if (!c.valid) {
        return false;
    }
    return (y <= c.top && y > c.bot) || y <= c.g2;
}

/// Max render height over the 2^k-cell block containing `cid`.
/// Pyramid packing (see Atlas::encode_pyramid): level k has dim = n >> k
/// and starts at row sum(n >> j, j in 1..k).
float pyrMax(ivec2 cid, int k, int n) {
    int dim = n >> k;
    if (dim < 1) {
        return -64.0;
    }
    int rowBase = 0;
    for (int j = 1; j < k; ++j) {
        rowBase += n >> j;
    }
    vec4 t = texelFetch(PyrSampler, ivec2(cid.x >> k, rowBase + (cid.y >> k)), 0);
    float v = t.r * 65280.0 + t.g * 255.0;
    return v > 0.5 ? v / 64.0 - 64.0 : -64.0;
}

// The random functions for the fake-texture noise (copied from DH's
// terrain blaze shader so the look matches).
float rand(vec2 co) { return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453); }
float rand(vec3 co) { return rand(co.xy + rand(co.z)); }

void main() {
    vec4 scene = texture(InSampler, texCoord);
    float depth = texture(InDepthSampler, texCoord).r;

    // Debug mode 3: pure geometry probe: cell pattern, viewport edge and four
    // coloured corners over the ENTIRE pass viewport.
    if (DebugParams.x > 2.5 && DebugParams.x < 3.5) {
        vec2 uv = gl_FragCoord.xy / max(OutSize, vec2(1.0));
        vec2 cellId = floor(gl_FragCoord.xy / 64.0);
        vec3 c = vec3(mod(cellId.x, 8.0) / 8.0, mod(cellId.y, 8.0) / 8.0, 0.55);
        if (uv.x < 0.004 || uv.x > 0.996 || uv.y < 0.004 || uv.y > 0.996) {
            c = vec3(1.0, 0.0, 1.0); // magenta border = viewport edge
        }
        if (uv.x < 0.07 && uv.y > 0.93) c = vec3(1.0, 0.0, 0.0); // top-left  red
        if (uv.x > 0.93 && uv.y > 0.93) c = vec3(0.0, 1.0, 0.0); // top-right green
        if (uv.x < 0.07 && uv.y < 0.07) c = vec3(0.0, 0.0, 1.0); // bottom-left  blue
        if (uv.x > 0.93 && uv.y < 0.07) c = vec3(1.0, 1.0, 0.0); // bottom-right yellow
        fragColor = vec4(c, 1.0);
        return;
    }

    bool skyPix = depth <= SKY_DEPTH;
    vec4 rot = texelFetch(RotSampler, ivec2(0), 0);
    float conf = rot.b;
    float minConf = LodParams.w;
    float forceLod = 0.0;

    // Debug mode 4: LOD hit mask + tracker confidence bar.
    if (DebugParams.x > 3.5) {
        if (gl_FragCoord.x < 240.0 && gl_FragCoord.y > OutSize.y - 24.0) {
            vec3 bar = gl_FragCoord.x < conf * 240.0 ? vec3(1.0, 0.0, 0.0) : vec3(0.0);
            fragColor = vec4(bar, 1.0);
            return;
        }
        forceLod = 1.0;
    }

    // Debug view: red channel = tracker confidence over sky pixels.
    if (DebugParams.x > 0.5 && DebugParams.x < 1.5) {
        fragColor = vec4(conf, 0.0, 0.0, 1.0);
        return;
    }

    // Debug mode 2 splits the screen: left = real march, right = raw atlas.
    if (DebugParams.x > 1.5 && DebugParams.x < 2.5) {
        if (texCoord.x >= 0.5) {
            float hm = atlas_height(texCoord);
            float g = clamp((hm + 64.0) / 384.0, 0.0, 1.0);
            vec3 mc = vec3(g);
            if (atlas_water(texCoord)) {
                mc = mix(mc, vec3(0.3, 0.5, 1.0), 0.6);
            }
            fragColor = vec4(mc, 1.0);
            return;
        }
        forceLod = 1.0;
    }

    if (conf < minConf && forceLod < 0.5) {
        fragColor = scene;
        return;
    }

    // v4.0 encoding preserved: R = az/2pi, G = (el+pi/2)/pi, B = conf,
    // A = fov byte measured from the client's ProjMat[1][1].
    float az = rot.r * 6.28318530718;
    float el = rot.g * PI - PI * 0.5;

    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));

    float fov = rot.a > 0.002 ? 5.0 + rot.a * 115.0 : FogParams.z;
    float tanH = tan(radians(fov) * 0.5);
    float aspect = OutSize.x / max(OutSize.y, 1.0);

    vec2 ndc = texCoord * 2.0 - 1.0;
    vec3 dir = normalize(F + R * (ndc.x * tanH * aspect) + U * (ndc.y * tanH));

    // 26.3 Globals: CameraBlockPos = floor(camPos), CameraOffset = floor - camPos.
    vec3 cam = vec3(CameraBlockPos) - CameraOffset;

    // Clouds are opaque geometry above the highest atlas cell; without this
    // the vanilla cloud layer punches holes into distant mountains.
    if (!skyPix) {
        float eyeZ = SkyParams.x / max(depth, 1.0e-9);
        float vlen = length(vec3(ndc.x * tanH * aspect, ndc.y * tanH, 1.0));
        vec3 wp = cam + dir * (eyeZ * vlen);
        skyPix = wp.y > SkyParams.y;
    }
    if (!skyPix) {
        fragColor = scene;
        return;
    }

    vec2 origin = AtlasInfo.xy;
    float size = AtlasInfo.z;
    float cell = AtlasInfo.w;
    float maxY = LodParams.y;
    float sea = LodParams.z;
    int n = int(AtlasInfo.z / AtlasInfo.w + 0.5);
    int K = int(PyrParams.x + 0.5);
    float baseStep = cell * 0.75;

    vec2 minW = origin;
    vec2 maxW = origin + vec2(size);

    // Rays that leave the heightfield from above can never hit: early-out.
    if (dir.y > 0.0) {
        float toCeil = (maxY + 64.0 - cam.y) / dir.y;
        float toEdge = 1.0e9;
        if (dir.x > 0.0) toEdge = min(toEdge, (maxW.x - cam.x) / dir.x);
        if (dir.x < 0.0) toEdge = min(toEdge, (minW.x - cam.x) / dir.x);
        if (dir.z > 0.0) toEdge = min(toEdge, (maxW.y - cam.z) / dir.z);
        if (dir.z < 0.0) toEdge = min(toEdge, (minW.y - cam.z) / dir.z);
        if (toCeil < toEdge) {
            fragColor = DebugParams.x > 3.5 ? vec4(0.6, 0.0, 0.9, 1.0) : scene;
            return;
        }
    }
    if (dir.y == 0.0) {
        dir.y = 1.0e-6;
    }

    // ---------------------------------------------------------------
    // The march. Small constant base step (cell * 0.75) for crisp voxel
    // edges, accelerated by conservative pyramid jumps: the ray may skip a
    // 2^k-cell block only when it clears that block's max height at BOTH
    // ends of the skipped segment (the ray is linear, so endpoint checks are
    // exact). Hits are refined by bisection on the solid() predicate, which
    // with NEAREST cell heights converges onto the exact voxel plane.
    // ---------------------------------------------------------------
    float t = 0.0;
    float prevT = 0.0;
    bool seenAir = false; // camera may start inside a canopy: march out first
    float tHit = -1.0;

    for (int i = 0; i < MAX_STEPS; ++i) {
        vec2 xz = cam.xz + dir.xz * t;
        if (any(lessThan(xz, minW)) || any(greaterThan(xz, maxW)) || t >= size) {
            break;
        }
        float y = cam.y + dir.y * t;
        ivec2 cid = ivec2(floor((xz - origin) / cell));
        Cell cd = cellAt(cid, origin, cell, n, sea);

        if (solid(cd, y)) {
            if (seenAir) {
                float lo = prevT;
                float hi = t;
                for (int b = 0; b < BISECT; ++b) {
                    float mid = (lo + hi) * 0.5;
                    vec2 mxz = cam.xz + dir.xz * mid;
                    float my = cam.y + dir.y * mid;
                    ivec2 mcid = ivec2(floor((mxz - origin) / cell));
                    if (solid(cellAt(mcid, origin, cell, n, sea), my)) {
                        hi = mid;
                    } else {
                        lo = mid;
                    }
                }
                tHit = (lo + hi) * 0.5;
                break;
            }
            // Started inside geometry (camera in a canopy/wall): march out.
        } else {
            seenAir = true;
            prevT = t;
        }

        // Pyramid skip: advance to the boundary of the largest block the ray
        // clears by SKIP_MARGIN at both ends.
        float adv = 0.0;
        if (dir.x != 0.0 || dir.z != 0.0) {
            for (int kk = K; kk >= 1; --kk) {
                float hmax = pyrMax(cid, kk, n);
                if (y - hmax <= SKIP_MARGIN) {
                    continue; // block may contain terrain: try finer level
                }
                float bs = cell * float(1 << kk);
                vec2 bmin = origin + vec2(float(cid.x >> kk), float(cid.y >> kk)) * bs;
                vec2 tv;
                tv.x = abs(dir.x) < 1.0e-9 ? 1.0e9
                    : (dir.x > 0.0 ? (bmin.x + bs - cam.x) : (bmin.x - cam.x)) / dir.x;
                tv.y = abs(dir.z) < 1.0e-9 ? 1.0e9
                    : (dir.z > 0.0 ? (bmin.y + bs - cam.z) : (bmin.y - cam.z)) / dir.z;
                float tExit = min(tv.x, tv.y);
                if (tExit <= t + 1.0e-3) {
                    continue;
                }
                float yExit = cam.y + dir.y * tExit;
                if (yExit - hmax <= SKIP_MARGIN) {
                    continue; // the ray dips into the block before exiting
                }
                adv = tExit + 1.0e-3 - t;
                break;
            }
        }
        t += max(adv, baseStep);
    }

    if (tHit < 0.0) {
        fragColor = DebugParams.x > 3.5 ? vec4(1.0, 0.45, 0.0, 1.0) : scene;
        return;
    }
    if (DebugParams.x > 4.5 && DebugParams.x < 5.5) {
        // Distance tint: red < 300 blocks, green ~600, blue > 900.
        vec3 c = mix(vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), clamp(tHit / 600.0, 0.0, 1.0));
        c = mix(c, vec3(0.0, 0.25, 1.0), clamp((tHit - 600.0) / 600.0, 0.0, 1.0));
        fragColor = vec4(c, 1.0);
        return;
    }
    if (DebugParams.x > 3.5) {
        fragColor = vec4(0.0, 1.0, 0.1, 1.0);
        return;
    }

    float fade = forceLod > 0.5 ? 1.0 : smoothstep(minConf, minConf + 0.15, conf);
    vec3 hitPos = cam + dir * tHit;
    vec2 xzHit = hitPos.xz;
    float yHit = hitPos.y;

    // ---------------------------------------------------------------
    // Face classification, DH model: which face of the voxel did the ray
    // enter through? The state 0.25 blocks back along the ray tells us the
    // entry direction regardless of how big the last march step was.
    //   lateral  -> vertical wall: slab/cliff side (or ground wall below a
    //               gap), vanilla directional shade 0.8 N/S / 0.6 E/W
    //   from above the slab top     -> TOP face
    //   from the gap onto the ground-> ground top below a canopy
    //   from the gap into the slab  -> slab UNDERSIDE (down shade 0.5)
    // ---------------------------------------------------------------
    ivec2 cidHit = ivec2(floor((xzHit - origin) / cell));
    Cell cd = cellAt(cidHit, origin, cell, n, sea);
    vec3 backPos = hitPos - dir * 0.25;
    ivec2 cidBack = ivec2(floor((backPos.xz - origin) / cell));
    bool lateral = any(notEqual(cidHit, cidBack));

    vec2 auv = atlas_uv(xzHit, origin, size);
    vec3 topCol = texture(ColorSampler, auv).rgb;
    vec3 groundCol = texture(GroundSampler, auv).rgb;
    vec3 sideCol = texture(
        SideSampler,
        vec2((cd.side * 255.0 + 0.5) / 256.0, 0.5)
    ).rgb;

    float faceShade;
    vec3 albedo;
    if (lateral) {
        faceShade = abs(cidHit.x - cidBack.x) > 0.5 ? 0.6 : 0.8;
        if (yHit > cd.bot) {
            albedo = sideCol;   // canopy/cliff wall
        } else {
            albedo = groundCol; // wall of the ground under a gap
        }
    } else if (backPos.y > cd.top) {
        faceShade = 1.0;
        albedo = topCol;        // surface top (or sea surface)
    } else if (yHit <= cd.g2) {
        faceShade = 1.0;
        albedo = groundCol;     // ground top below a canopy
    } else {
        faceShade = 0.5;
        albedo = sideCol;       // canopy underside
    }

    // DH fake-texture noise: quantized world-position hash, scaled down on
    // very dark/bright colours, fading with distance. The noise anchor is
    // snapped to the exact surface plane so it never shimmers between frames.
    if (!cd.water) {
        float lum = dot(albedo, vec3(0.299, 0.587, 0.114));
        float amp = NOISE_AMP * (1.0 - pow(2.0 * lum - 1.0, 2.0));
        float sy = lateral ? yHit
                 : (backPos.y > cd.top ? cd.top : (yHit <= cd.g2 ? cd.g2 : cd.bot));
        vec3 q = floor(vec3(xzHit.x, sy, xzHit.y) / NOISE_STEPS);
        float rv = rand(q) * 2.0 * amp - amp;
        vec3 noisy = clamp(albedo + (1.0 - albedo) * rv, 0.0, 1.0);
        float distF = 1.0 - 0.65 * smoothstep(NOISE_FAR, NOISE_END, tHit);
        albedo = mix(albedo, noisy, distF);
    }

    // Light reference from the SKY IN THIS FRAME: the world clock and the
    // rendered sky can disagree, and a wrong day factor made the LOD look
    // lit by moonlight next to sunlit chunks.
    float skyLum = dot(scene.rgb, vec3(0.299, 0.587, 0.114));
    float dayF = smoothstep(0.08, 0.45, skyLum);
    // The fog target is the actual sky behind the pixel, so the far edge of
    // the LOD band dissolves into the horizon.
    vec3 veilCol = mix(VeilColor.rgb * mix(0.35, 1.0, dayF), scene.rgb, 0.82);

    // Vanilla lightmap substitute: full daylight is 1.0, night keeps a small
    // ambient so the silhouette does not vanish.
    float lightMul = mix(0.14, 1.0, dayF);
    vec3 lit = albedo * (faceShade * lightMul);

    // Snow: high terrain is whitened by height (state variants cannot be seen
    // by per-block colours).
    float hn = clamp((hitPos.y - sea) / max(maxY - sea, 1.0), 0.0, 1.0);
    float snow = smoothstep(VeilColor.a * 0.55 + 0.35, VeilColor.a * 0.55 + 0.55, hn);
    lit = mix(lit, vec3(0.93, 0.95, 0.97) * lightMul, snow * 0.9);

    // Distance fog into the day-graded veil color.
    float fog = smoothstep(FogParams.x, FogParams.y, tHit);
    lit = mix(lit, veilCol, fog);

    fragColor = vec4(mix(scene.rgb, lit, fade), scene.a);
}
