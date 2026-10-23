#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// NOTE: post pipelines cannot declare the vanilla Fog UBO — PostChain builds
// the bind group layout from inputs + SamplerInfo + JSON uniforms only
// (BindGroupLayouts.FOG is world-pipeline-only), and an undeclared uniform
// fails the whole pack ("Unable to find shader defined uniform (Fog)"). The
// day grade comes through the camera channel instead (rot_solve px4.g).

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
//
// v4.4.4 GLIDE FADE: the camera channel publishes a glide residual (px4.b,
// rot_solve) — the angle between the blended basis and the fresh payload
// after the solve, over a 12-degree window. While the solver converges onto
// a re-acquired payload after an occlusion gap (looked down, clouds over
// every group column), the blended basis is the WRONG pose for this frame
// and showing it was the residual flicker: the LOD horizon visibly tilted
// and swept for up to 8 frames after every look-down/look-up cycle. The LOD
// is now faded BY that residual: sky pixels show the real sky while the
// pose is far off, and the LOD materializes as it converges. Bit-exact
// tracking frames carry residual 0 and render at full strength, exactly as
// in v4.4.1.

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
const int MAX_STEPS = 600;
const int BISECT = 14;

// v4.4 adaptive level-of-detail march (DH-style LOD levels on a raymarcher):
// the sampled cell grows with the pixel footprint so the far field renders
// through coarse pyramid levels (stable, no per-pixel speckle) while the
// near field keeps the exact cell grid. PIXF scales how aggressively the
// level coarsens; 2.5 keeps one cell ~2.5 pixels wide before coarsening.
const float PIXF = 2.5;

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

// (hi,lo) big-endian 16-bit byte pair from RGBA8 (stored n/255), with the
// (c+1)/2 * 65535 signed remap used by the v4.3 camera channel.
float c16(float hi, float lo) {
    return (hi * 65280.0 + lo * 255.0) / 65535.0 * 2.0 - 1.0;
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
    // v4.4: render the water surface at its STORED height, not the sea
    // level — the scan captures the actual fluid block (ocean = 63 = sea,
    // but mountain lakes sit at their own Y and used to render as pits).
    // (`sea` stays an argument for signature stability; unused here.)
    c.top = h16 / 64.0 - 64.0;
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
// terrain blaze shader so the look matches). v4.2.1: restore the scalar
// overload — rand(vec3) calls rand(co.z), and without rand(float) the
// compile dies with 'rand' : no matching overloaded function found, which
// made MC drop the whole pack on join.
float rand(float co) { return fract(sin(co * 91.3458) * 47453.5453); }
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
    vec4 pc4 = texelFetch(RotSampler, ivec2(4, 0), 0);
    float conf = pc4.r;
    float minConf = LodParams.w; // v4.4.1: diagnostics only, see the gate below
    float forceLod = 0.0;

    // Debug modes 4 and 6 draw the tracker HUD (confidence bar + v4.4.6
    // channel oscilloscope). Mode 4 is INVASIVE: it forces the LOD over
    // every pixel and flat-greens hits (geometry debugging) — the 19:03
    // screenshots showed why that hides the real picture (the whole sky
    // becomes debug-orange, far fog-hidden hits become visible green).
    // Mode 6 is TRANSPARENT: the game renders normally, only the HUD
    // overlays it — a screenshot of a bad moment pairs the artifact with
    // the channel trace. The 2px white tick at the scope's top-right
    // marks "now".
    bool hud4 = DebugParams.x > 3.5 && DebugParams.x < 4.5;
    bool hud6 = DebugParams.x > 5.5 && DebugParams.x < 6.5;
    if (hud4 || hud6) {
        if (gl_FragCoord.x < 240.0 && gl_FragCoord.y > OutSize.y - 24.0) {
            vec3 bar = gl_FragCoord.x < conf * 240.0 ? vec3(1.0, 0.0, 0.0) : vec3(0.0);
            fragColor = vec4(bar, 1.0);
            return;
        }
        // v4.4.6 CHANNEL OSCILLOSCOPE: 256x56 trace under the conf bar,
        // backed by the history ring rot_solve keeps in px6..px261.
        // red = conf, green = gap (payload occlusion), blue = glide
        // residual; newest frame at the RIGHT edge. One screenshot of a
        // bad moment now shows exactly what the camera channel did.
        float gx = gl_FragCoord.x - 8.0;
        float gy = gl_FragCoord.y - (OutSize.y - 24.0 - 56.0);
        if (gx >= 0.0 && gx < 256.0 && gy >= 0.0 && gy < 56.0) {
            int slotNow = int(texelFetch(RotSampler, ivec2(5, 0), 0).r * 255.0 + 0.5);
            int sx = int(mod(float(slotNow) - (255.0 - gx), 256.0));
            vec4 h = texelFetch(RotSampler, ivec2(6 + sx, 0), 0);
            float v = (gy - 4.0) / 48.0; // 4px padding, 48px span = 0..1
            vec3 col = vec3(0.02);
            if (abs(v - h.r) < 0.021) col = vec3(1.0, 0.25, 0.25); // conf
            if (abs(v - h.a) < 0.021) col = vec3(0.25, 1.0, 0.35); // gap
            if (abs(v - h.b) < 0.021) col = vec3(0.35, 0.6, 1.0);  // res
            if (abs(v - 0.5) < 0.011) col = vec3(0.15);            // mid grid
            if (gy < 2.0 || gy > 53.0) col = vec3(0.15);           // frame
            if (gy < 2.0 && gx > 250.0) col = vec3(1.0);           // "now" tick
            fragColor = vec4(col, 1.0);
            return;
        }
        if (hud4) {
            forceLod = 1.0;
        }
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

    // v4.4.1 FLICKER FIX: confidence no longer gates VISIBILITY, only the
    // very first frame. The old chain (gate at minConf=0.25 + fade
    // smoothstep) made the LOD alpha pump every time a pan carried terrain
    // across the eight camera-channel column groups: conf decayed x0.99
    // while all groups were occluded, crossed the threshold -> the LOD
    // popped out, then ramped back at +0.08/frame on the next sky row ->
    // pop-in. That oscillation WAS the "flicker when I move the camera".
    // Now the gate only protects the never-solved state (rot target starts
    // zeroed; first valid payload pushes conf to 0.08 on the next frame),
    // and the blend below is always 1: a stale basis is only ever visible
    // when NO group column sees sky (overcast clouds / GUI over every
    // payload column) while sky pixels elsewhere still exist -- and the
    // moment any group sees sky again the basis resyncs bit-exact in one
    // frame. Rendering with a held basis beats fading the whole horizon
    // out and back.
    if (conf < 0.02 && forceLod < 0.5) {
        fragColor = scene;
        return;
    }

    // v4.3 camera channel: the sky shader encodes the BOBBED world-space
    // right/up basis (16-bit per component, raw bytes in the RGBA8 persistent
    // target), the bob-corrected vertical fov (16-bit) and the bob eye
    // wobble (8-bit signed each). This is what makes the LOD march shake
    // EXACTLY with the world: 26.3 folds view bobbing into the projection
    // (GameRenderer.renderLevel: projectionMatrix.mul(bobStack)), the sky
    // extracts it, we consume it. See assets/glsl/sky.fsh + rot_solve.fsh.
    vec4 rp0 = texelFetch(RotSampler, ivec2(0, 0), 0);
    vec4 rp1 = texelFetch(RotSampler, ivec2(1, 0), 0);
    vec4 rp2 = texelFetch(RotSampler, ivec2(2, 0), 0);
    vec4 rp3 = texelFetch(RotSampler, ivec2(3, 0), 0);

    vec3 R = vec3(c16(rp0.x, rp0.y), c16(rp0.z, rp0.w), c16(rp1.x, rp1.y));
    vec3 U = vec3(c16(rp1.z, rp1.w), c16(rp2.x, rp2.y), c16(rp2.z, rp2.w));
    // The quantized pair is near-orthonormal; re-orthonormalize so the
    // reconstructed horizon can never drift against the real one.
    R = normalize(R);
    U = normalize(U - R * dot(R, U));
    vec3 F = cross(U, R);

    float fovq = rp3.x * 65280.0 + rp3.y * 255.0;
    float fov = fovq < 65534.5 ? 5.0 + fovq / 65535.0 * 115.0 : FogParams.z;
    float tanH = tan(radians(fov) * 0.5);
    float aspect = OutSize.x / max(OutSize.y, 1.0);

    vec2 ndc = texCoord * 2.0 - 1.0;
    vec3 dir = normalize(F + R * (ndc.x * tanH * aspect) + U * (ndc.y * tanH));

    // 26.3 Globals: CameraBlockPos = floor(camPos), CameraOffset = floor - camPos.
    // The bob eye wobble (view-space tx/ty) shifts the LOD eye with the
    // real camera — sub-pixel at LOD distances, but free and exact.
    vec2 eyeWob = rp3.zw * 2.0 - 1.0;
    vec3 cam = vec3(CameraBlockPos) - CameraOffset - (eyeWob.x * R + eyeWob.y * U);

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

    vec2 minW = origin;
    vec2 maxW = origin + vec2(size);

    // v4.4 early-out, FIXED: the old test killed every upward ray that
    // crossed the ceiling plane (maxY + 64) before leaving the atlas side
    // walls — but such a ray STARTS below the ceiling and can hit terrain
    // on the way up. That was the "high LODs cut off at cloud level when
    // standing below" bug (from a low eye the cut line sat right around
    // y≈196, i.e. exactly where the user saw it). The only safe early-out
    // is an eye that is itself above the whole heightfield: such a ray can
    // never descend back into it.
    if (dir.y > 0.0 && cam.y > maxY + 64.0) {
        fragColor = hud4 ? vec4(0.6, 0.0, 0.9, 1.0) : scene;
        return;
    }
    if (dir.y == 0.0) {
        dir.y = 1.0e-6;
    }

    // ---------------------------------------------------------------
    // The march (v4.4 adaptive, DH-style LOD levels on a raymarcher).
    // The sampled cell size grows with the pixel footprint: level k tests
    // the 2^k max-height pyramid (one conservative height per 2^k*cell
    // block) and steps by that coarse cell. The far field becomes a
    // STABLE coarse staircase (one column several pixels wide, like DH's
    // distant LOD rings) instead of per-pixel speckle, and the same step
    // budget now reaches the far atlas border — the old constant
    // cell*0.75 step died at ~460*0.75 ≈ 345 blocks on grazing rays and
    // punched flickering holes into the horizon while panning. Level 0
    // keeps the exact v4.2 voxel path: slab/ground geometry, NEAREST
    // heights, crisp voxel edges. Hits are refined by bisection on the
    // level's own solid predicate, which converges onto that level's
    // exact voxel plane.
    // ---------------------------------------------------------------
    float t = 0.0;
    float prevT = 0.0;
    bool seenAir = false; // camera may start inside a canopy: march out first
    float tHit = -1.0;
    int hitLvl = 0;
    float hitSurf = 0.0; // coarse surface plane (noise/snow anchor, level > 0)
    // World size covered by one pixel per block of ray distance.
    float pixAng = 2.0 * tanH / max(OutSize.y, 1.0);

    for (int i = 0; i < MAX_STEPS; ++i) {
        vec2 xz = cam.xz + dir.xz * t;
        if (any(lessThan(xz, minW)) || any(greaterThan(xz, maxW)) || t >= size) {
            break;
        }
        float y = cam.y + dir.y * t;
        ivec2 cid = ivec2(floor((xz - origin) / cell));

        // Adaptive level: the pyramid level whose cells are about PIXF
        // pixels wide at this distance (floor keeps the near field exact).
        float foot = max(t * pixAng * PIXF, cell);
        int lvl = foot > cell ? min(int(floor(log2(foot / cell))), K) : 0;
        float csize = cell * float(1 << lvl);

        bool sol;
        float hmax = -64.0;
        if (lvl == 0) {
            sol = solid(cellAt(cid, origin, cell, n, sea), y);
        } else {
            hmax = pyrMax(cid, lvl, n);
            sol = hmax > -63.5 && y <= hmax;
        }

        if (sol) {
            if (seenAir) {
                float lo = prevT;
                float hi = t;
                for (int b = 0; b < BISECT; ++b) {
                    float mid = (lo + hi) * 0.5;
                    vec2 mxz = cam.xz + dir.xz * mid;
                    float my = cam.y + dir.y * mid;
                    ivec2 mcid = ivec2(floor((mxz - origin) / cell));
                    bool ms;
                    if (lvl == 0) {
                        ms = solid(cellAt(mcid, origin, cell, n, sea), my);
                    } else {
                        float mh = pyrMax(mcid, lvl, n);
                        ms = mh > -63.5 && my <= mh;
                    }
                    if (ms) {
                        hi = mid;
                    } else {
                        lo = mid;
                    }
                }
                tHit = (lo + hi) * 0.5;
                hitLvl = lvl;
                hitSurf = hmax;
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
        t += max(adv, csize * 0.75);
    }

    if (tHit < 0.0) {
        fragColor = hud4 ? vec4(1.0, 0.45, 0.0, 1.0) : scene;
        return;
    }
    if (DebugParams.x > 4.5 && DebugParams.x < 5.5) {
        // Distance tint: red < 300 blocks, green ~600, blue > 900.
        vec3 c = mix(vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), clamp(tHit / 600.0, 0.0, 1.0));
        c = mix(c, vec3(0.0, 0.25, 1.0), clamp((tHit - 600.0) / 600.0, 0.0, 1.0));
        fragColor = vec4(c, 1.0);
        return;
    }
    if (hud4) {
        fragColor = vec4(0.0, 1.0, 0.1, 1.0);
        return;
    }

    // v4.4.4: fade by the camera-channel glide residual (px4.b, rot_solve).
    // Bit-exact frames: residual 0 -> fade 1 (full strength, as v4.4.1).
    // Glide frames: fade ramps 0 -> 1 as the pose converges, so the tilted
    // intermediate basis is never shown (see the header note above).
    // v4.4.5: AND by the camera-channel gap counter (px4.a): while no group
    // column sees sky the held basis is stale by however far the camera
    // turned during the gap — the LOD dissolves into the sky instead of
    // showing a tilted world ("LODs rotate depending on where I look").
    // Grace ~2 frames kills single-frame dropout noise, gone by ~9 frames,
    // materializes over ~3 frames on re-acquisition while the pose is
    // already bit-exact.
    float fade = clamp(1.0 - pc4.b * 1.05, 0.0, 1.0)
               * (1.0 - smoothstep(0.25, 0.8, pc4.a));
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
    float csize = cell * float(1 << hitLvl);

    vec2 auv = atlas_uv(xzHit, origin, size);
    vec3 topCol = texture(ColorSampler, auv).rgb;
    vec3 groundCol = texture(GroundSampler, auv).rgb;
    vec3 sideCol = texture(
        SideSampler,
        vec2((cd.side * 255.0 + 0.5) / 256.0, 0.5)
    ).rgb;

    float faceShade;
    vec3 albedo;
    float sy;             // noise anchor: the exact surface plane of the hit
    bool waterFace = false;

    if (hitLvl > 0) {
        // Coarse hit: pyramid columns carry no slab/gap data, so the entry
        // face comes from the last air sample one step back. Side colours
        // read the base cell under the COARSE cell centre, so a whole
        // coarse column shades (and texts) as one stable block — no
        // per-pixel colour speckle on distant cliffs.
        ivec2 cHit = ivec2(floor((xzHit - origin) / csize));
        vec3 prevPos = cam + dir * prevT;
        ivec2 cPrev = ivec2(floor((prevPos.xz - origin) / csize));
        if (any(notEqual(cHit, cPrev))) {
            faceShade = abs(cHit.x - cPrev.x) > 0.5 ? 0.6 : 0.8;
            vec2 cc = origin + (vec2(cHit) + 0.5) * csize;
            Cell cdc = cellAt(ivec2(floor((cc - origin) / cell)), origin, cell, n, sea);
            albedo = texture(
                SideSampler,
                vec2((cdc.side * 255.0 + 0.5) / 256.0, 0.5)
            ).rgb;
            sy = yHit;
        } else {
            faceShade = 1.0;
            albedo = topCol;
            sy = hitSurf;
        }
    } else {
    // ---------------------------------------------------------------
    // Face classification (level 0, unchanged v4.2 model): which face of
    // the voxel did the ray enter through? The state 0.25 blocks back
    // along the ray tells us the entry direction regardless of how big
    // the last march step was.
    //   lateral  -> vertical wall: slab/cliff side (or ground wall below a
    //               gap), vanilla directional shade 0.8 N/S / 0.6 E/W
    //   from above the slab top     -> TOP face
    //   from the gap onto the ground-> ground top below a canopy
    //   from the gap into the slab  -> slab UNDERSIDE (down shade 0.5)
    // ---------------------------------------------------------------
        vec3 backPos = hitPos - dir * 0.25;
        ivec2 cidBack = ivec2(floor((backPos.xz - origin) / cell));
        bool lateral = any(notEqual(cidHit, cidBack));
        waterFace = cd.water;
        if (lateral) {
            faceShade = abs(cidHit.x - cidBack.x) > 0.5 ? 0.6 : 0.8;
            if (yHit > cd.bot) {
                albedo = sideCol;   // canopy/cliff wall
            } else {
                albedo = groundCol; // wall of the ground under a gap
            }
            sy = yHit;
        } else if (backPos.y > cd.top) {
            faceShade = 1.0;
            albedo = topCol;        // surface top (or sea surface)
            sy = cd.top;
        } else if (yHit <= cd.g2) {
            faceShade = 1.0;
            albedo = groundCol;     // ground top below a canopy
            sy = cd.g2;
        } else {
            faceShade = 0.5;
            albedo = sideCol;       // canopy underside
            sy = cd.bot;
        }
    }

    // DH fake-texture noise: quantized world-position hash, scaled down on
    // very dark/bright colours, fading with distance. The noise anchor is
    // snapped to the exact surface plane so it never shimmers between frames.
    if (!waterFace) {
        float lum = dot(albedo, vec3(0.299, 0.587, 0.114));
        float amp = NOISE_AMP * (1.0 - pow(2.0 * lum - 1.0, 2.0));
        vec3 q = floor(vec3(xzHit.x, sy, xzHit.y) / NOISE_STEPS);
        float rv = rand(q) * 2.0 * amp - amp;
        vec3 noisy = clamp(albedo + (1.0 - albedo) * rv, 0.0, 1.0);
        float distF = 1.0 - 0.65 * smoothstep(NOISE_FAR, NOISE_END, tHit);
        albedo = mix(albedo, noisy, distF);
    }

    // Day factor from the camera channel (px4.g, rot_solve): the topmost
    // intact payload row is pure sky by construction (clouds/stars/moon
    // break the payload checksum), its RGB luminance maps through the day
    // window and is exponentially smoothed in the persistent target. This
    // tracks the REAL sun clock: 26.3 drives the sun from the WorldClock
    // timeline while Globals.GameTime counts raw level ticks — beds, /time
    // set and doDaylightCycle=false make the two disagree, which graded the
    // LOD for night while the sky showed day (the "black in daytime" bug).
    // The never-solved state holds the LOD off entirely (gate above), so no
    // clock fallback is needed.
    float dayF = pc4.g;

    // The fog target tracks the actual sky behind the pixel so the far edge
    // of the LOD band dissolves into the horizon — but through a 4-tap min
    // that kills 1-3px stars, and with a night luminance cap so the moon can
    // never backlight the fog into a glow patch.
    vec2 texel = 1.0 / max(InSize, vec2(1.0));
    vec3 s10 = texture(InSampler, texCoord + vec2(2.0, 0.0) * texel).rgb;
    vec3 s01 = texture(InSampler, texCoord + vec2(0.0, 2.0) * texel).rgb;
    vec3 s11 = texture(InSampler, texCoord + vec2(2.0, 2.0) * texel).rgb;
    vec3 skyRef = min(min(scene.rgb, s10), min(s01, s11));
    vec3 nightVeil = VeilColor.rgb * mix(0.14, 1.0, dayF);
    nightVeil = mix(nightVeil * vec3(0.85, 0.9, 1.12), nightVeil, dayF);
    vec3 veilCol = mix(nightVeil, skyRef, 0.55);
    float vCap = mix(0.10, 2.0, dayF);
    float vLum = dot(veilCol, vec3(0.299, 0.587, 0.114));
    veilCol *= min(1.0, vCap / max(vLum, 1.0e-4));

    // Vanilla lightmap substitute: full daylight is 1.0, night keeps a small
    // ambient so the silhouette does not vanish.
    float lightMul = mix(0.14, 1.0, dayF);
    vec3 lit = albedo * (faceShade * lightMul);

    // Snow: high terrain is whitened by height (state variants cannot be seen
    // by per-block colours).
    float hn = clamp((hitPos.y - sea) / max(maxY - sea, 1.0), 0.0, 1.0);
    float snow = smoothstep(VeilColor.a * 0.55 + 0.35, VeilColor.a * 0.55 + 0.55, hn);
    lit = mix(lit, vec3(0.93, 0.95, 0.97) * lightMul, snow * 0.9);

    // Distance fog into the day-graded veil color. The 1.5 power keeps the
    // near-mid band (300-800 blocks) solid — the "translucent LOD" fix —
    // while still dissolving fully before the atlas border.
    float fog = pow(smoothstep(FogParams.x, FogParams.y, tHit), 1.5);
    lit = mix(lit, veilCol, fog);

    fragColor = vec4(mix(scene.rgb, lit, fade), scene.a);
}
