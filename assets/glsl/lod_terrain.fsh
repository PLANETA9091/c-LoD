#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD pass 4: Distant Horizons-exact LOD renderer.
//
// DH renders each LOD column as a flat-coloured BOX: a top quad (when nothing
// sits on it), a bottom quad, and four side quads that exist only where the
// neighbouring column is lower (ColumnBox.addBoxQuadsToBuilder). Every quad is
// shaded with vanilla's directional factor (up 1.0, N/S 0.8, E/W 0.6) at build
// time and lit through the vanilla lightmap; fog is exponential-squared in the
// sky colour from 0.4 to 1.0 of the LOD draw distance (GlDhFogShader:
// uFogScale = 1/lodDrawDistance, farFogStart 0.4, farFogEnd 1.0, density 2.5).
//
// We reproduce that geometry PIXEL-EXACTLY with an Amanatides-Woo voxel DDA
// over the heightfield atlas: each cell boundary crossing tests the neighbour
// wall (hNew > rayY -> side face, exposed span [hPrev..hNew] drives the DH
// grass-side dirt fade), and inside each cell the horizontal plane y=h is
// tested (top face). This is exactly the set of quads DH would rasterise for
// the same data — same faces, same shade, same colours, no tunnelling.
//
// Camera basis comes from the CAMERA CHANNEL (v4.1): the sky shader encodes
// yaw/pitch/fov into the rot target. Non-sky pixels pass through untouched:
// real chunks always win.

uniform sampler2D InSampler;      // minecraft:main color
uniform sampler2D InDepthSampler; // minecraft:main depth
uniform sampler2D AtlasSampler;   // RG = height, B = water, A = side colour index
uniform sampler2D ColorSampler;   // top-face block colour (baked texture average)
uniform sampler2D SideSampler;    // 256x1 side/cliff colour LUT
uniform sampler2D SidePyrSampler; // side-index strip (fallback wall path)
uniform sampler2D SlicesSampler;  // DH vertical slices (texel triples A|B|C)
uniform sampler2D TilesSampler;   // DH ratio-tile atlas (16px tiles, 256/row)
uniform sampler2D TexIdsSampler;  // per-cell tile ids: RG top, BA side (big-endian)
uniform sampler2D RotSampler;     // clod:clod_rot_a (4x1 persistent: camera, day, fp, prev-fp)
uniform sampler2D PrevSampler;    // clod:clod_hist_a (persistent TAA history)
// NOTE: the vanilla `Fog` UBO (FogColor) is NOT declared here on purpose —
// renderpearl rejects any shader UBO that is not registered in the pipeline's
// bind group layout, and post chains can only register the Globals snippet +
// the chain-JSON groups (a JSON "Fog" group would pin a STATIC packed buffer
// that then overrides the live one in PostPass.render). DH reads the vanilla
// world fog colour (USE_WORLD_FOG_COLOR); at the horizon that colour equals
// the sky behind the band, so mixing toward the scene pixel is the closest
// legal equivalent (and is what the approved v4.2 look used).

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

// x = fog start (legacy), y = fog end (blocks) == LOD draw distance,
// z = static fov fallback, w = DH earth curvature radius (0 = off)
layout(std140) uniform ClodFog {
    vec4 FogParams;
};

// x = debug mode (0 = normal, 1 = confidence heat, 2 = split, 3 = pattern,
// 4 = hit mask, 5 = hit distance)
layout(std140) uniform ClodDebug {
    vec4 DebugParams;
};

// x = device depth near plane (blocks), y = cloud ceiling (atlas max + margin),
// z = ratio-tile atlas rows (DH textured LODs), w = max octave with slices
layout(std140) uniform ClodSky {
    vec4 SkyParams;
};

layout(std140) uniform ClodColors {
    vec4 ColLow;
    vec4 ColHigh;
    vec4 ColWater;  // water surface (DH: flat biome water colour, opaque)
    vec4 VeilColor; // a = snow whiteness threshold
};

// x = TAA enable, y = history weight (the DH client-TAA blend stand-in)
layout(std140) uniform ClodTaa {
    vec4 TaaParams;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;
// 26.3 device depth: d = near/eyeZ (infinite far) — sky == 0.0.
const float SKY_DEPTH = 1.0e-6;

// DH far-fog defaults (Config.Client.Advanced.Graphics.Fog): EXPONENTIAL_SQUARED,
// farFogStart 0.4, farFogEnd 1.0 (x lodDrawDistance), density 2.5, spherical
// (heightFogMixMode SPHERICAL -> 3D distance), colour = world fog colour.
const float FOG_START_FRAC = 0.4;
const float FOG_DENSITY = 2.5;

// DH Config.Client.Advanced.Graphics.NoiseTexture defaults: the fake-texture
// grain that makes flat-coloured LODs look like terrain. enableNoiseTexture
// true, steps 4, intensity 0.05, dropoff 1024 blocks (fades out past that).
const float NOISE_STEPS = 4.0;
const float NOISE_INTENSITY = 0.05;
const float NOISE_DROPOFF = 1024.0;

// Vanilla directional face shade (ClientLevelWrapper.getShade):
// up 1.0, down 0.5, north/south 0.8, west/east 0.6.
const float SHADE_NS = 0.8;
const float SHADE_EW = 0.6;

// ------------------------------------------------------------------
// DH 3.3.4 TAA (terrain/gl/vert.vert, verbatim taps): the LOD geometry
// shifts by `taps[frame % 8] * w / viewport` in clip space — an image-space
// shift of taps/2 pixels — and the client temporal anti-aliaser averages the
// frames ("-1 if TAA is disabled" -> gate `uFrameMod8 > 0`, frame 0 = the
// identity tap). Our march shifts the RAY by the same pixel-space offsets
// (taps / OutSize in NDC) and accumulates the composite into a persistent
// post-effect target: every frame the fingerprint pass (clod:post/fp_solve)
// hashes the solved camera + day factor into 24 bits; while the fingerprint
// matches the previous frame the march blends 1/8 of the new sample over the
// history — the full 8-tap average after < 1 s. Any camera/time move breaks
// the match: the raw frame passes through (no jitter, no ghosting), exactly
// the no-TAA mode DH ships when the client cannot accumulate.
// ------------------------------------------------------------------
const vec2 TAA_JITTER_OFFSETS[8] = vec2[8](
    vec2( 0.125, -0.375),
    vec2(-0.125,  0.375),
    vec2( 0.625,  0.125),
    vec2( 0.375, -0.625),
    vec2(-0.625,  0.625),
    vec2(-0.875, -0.125),
    vec2( 0.375, -0.875),
    vec2( 0.875,  0.875)
);

// DH zeroes the tile id at MESH BUILD time once the datapoint is coarser
// than Config.Client.Advanced.Graphics.Texture.maxTexturedLodDetailLevel
// (default 2): near LODs carry textures, far LODs fall back to the flat
// colour + noise. Our cells are a fixed-size grid, so the same rule is
// expressed as a FOOTPRINT fade: a 16px tile spans one cell (texel =
// cell/16 blocks) and once a texel is smaller than the pixel's world
// footprint the mipless point sample shimmers — the water checkerboard and
// the snow speckle of the e2e matrix. The fade runs across the octave
// where the texel crosses the footprint (2x texel/pixel -> full tile,
// 0.5x -> flat), which adapts to any zoom/fov/resolution automatically.
const float TILE_PX = 16.0;
const float TILES_PER_ROW = 256.0;

// Height (block Y) stored in the atlas: (y + 64) << 6, big-endian RG.
float atlas_height(vec2 uv) {
    vec4 a = texture(AtlasSampler, uv);
    float y16 = a.r * 65280.0 + a.g * 255.0; // r*255*256 + g*255
    return y16 / 64.0 - 64.0;
}

bool atlas_water(vec2 uv) {
    return texture(AtlasSampler, uv).b > 0.5;
}

// Side index 255 is the reserved VOID marker (packgen writes it for cells the
// scan never filled). DH semantics for un-generated data: NO geometry — no
// top faces, no walls against the hole, the scene shows through (the fog
// pass's `drawnTo` check). This is what kills the white cliff walls that used
// to rise out of scan holes.
bool atlas_void(vec2 uv) {
    return texture(AtlasSampler, uv).a > 0.99;
}

// Heightfield lookup in world space; clamps to the atlas edge (plateau).
float world_height(vec2 xz, vec2 origin, float size) {
    return atlas_height(clamp((xz - origin) / size, vec2(0.0015), vec2(0.9985)));
}

bool world_water(vec2 xz, vec2 origin, float size) {
    return atlas_water(clamp((xz - origin) / size, vec2(0.0015), vec2(0.9985)));
}

vec2 atlas_uv(vec2 xz, vec2 origin, float size) {
    return clamp((xz - origin) / size, vec2(0.0015), vec2(0.9985));
}

// ------------------------------------------------------------------
// DH detail pyramid (colour strip, packgen: encode_colors_pyramid).
// Distant Horizons aggregates coarse datapoints 2x2 per detail level,
// which is why its far LODs read as smooth terrain. A fixed 2-block
// grid point-sampled per pixel is white noise instead (the far "noisy
// pixels" of v4.6). The pack therefore stacks the aggregation levels
// vertically into ONE n x (2n-1) image — level L occupies rows
// [2n - 2*sz, 2n - sz) with sz = n>>L — and the shader picks the
// octave from the pixel's world footprint, trilinearly blended
// between adjacent levels (the same filter DH's GPU sampling gets).
// ------------------------------------------------------------------
float pyr_footprint(float hitT, float tanH) {
    // World size of one screen pixel at the hit distance (vertical fov).
    return max(hitT * 2.0 * tanH / max(OutSize.y, 1.0), 1.0e-5);
}

vec3 pyr_level(vec2 uv01, float L, float n) {
    float sz = max(floor(n / exp2(L)), 1.0);
    // DH datapoint colours are FLAT — each coarse datapoint renders as a
    // solid quad, so the per-octave sample is NEAREST cell lookup (hard
    // cell edges like DH). A bilinear mix of two datapoint colours is a
    // third colour DH never produces: an isolated warm cell (flower field,
    // lichen) smeared into a soft glowing blob — the v4.8.1 "stains". The
    // far-distance noise this strip exists to kill is handled by the octave
    // selection (coarse cells are big), not by interpolation.
    vec2 t = clamp(floor(uv01 * sz), vec2(0.0), vec2(sz - 1.0));
    float off = 2.0 * n - 2.0 * sz;
    return texture(ColorSampler, vec2((t.x + 0.5) / n, (off + t.y + 0.5) / (2.0 * n - 1.0))).rgb;
}

vec3 pyr_color(vec2 uv01, float lvl, float n) {
    float L0 = floor(lvl);
    vec3 c0 = pyr_level(uv01, L0, n);
    vec3 c1 = pyr_level(uv01, L0 + 1.0, n);
    return mix(c0, c1, lvl - L0);
}

// Side INDEX from the same strip layout — NEAREST only (a linear mix of two
// LUT keys is a third, wrong colour), hard octave switch like DH's datapoint
// selection.
float pyr_side(vec2 uv01, float lvl, float n) {
    float L = floor(lvl);
    float sz = max(floor(n / exp2(L)), 1.0);
    vec2 t = clamp(floor(uv01 * sz), vec2(0.0), vec2(sz - 1.0));
    float off = 2.0 * n - 2.0 * sz;
    return texture(
        SidePyrSampler,
        vec2((t.x + 0.5) / n, (off + t.y + 0.5) / (2.0 * n - 1.0))
    ).r;
}

// ------------------------------------------------------------------
// DH vertical slices (v4.9/v4.10, slices.rs + atlas.encode_slices): the
// banded cliff walls of Distant Horizons. Every datapoint column stores up
// to EDhApiVerticalQuality.MEDIUM[detail] vertical runs (detail = log2(cell)
// + octave); each wall pixel samples the band containing its hit Y. The
// strip layout: octave L rows [rowBase, rowBase + K*(n>>L)), cell (x,z)
// slice j -> texel A at column 3x (RGB colour, A = (top+64)<<6 hi byte),
// texel B at 3x+1 (R = lo byte, GB = side tile id, A = flags), texel C at
// 3x+2 (R/G = (bottom+64)<<6 — the run's REAL solid floor; the air between
// bottom and the next band's top is DH's cave air datapoint: no wall face,
// the ray flies through). The DEEPEST present band always reaches the
// world floor (DH full-data columns span to min_y).
// ------------------------------------------------------------------
const int SLICE_MEDIUM[11] = int[11](6, 6, 6, 4, 4, 4, 4, 4, 4, 4, 1);

int slice_k(float detail) {
    return SLICE_MEDIUM[int(clamp(detail, 0.0, 10.0))];
}

float slice_row_base(float detail0, float L, int n) {
    int base = 0;
    int li = int(L);
    for (int l = 0; l < 12; ++l) {
        if (l >= li) break;
        base += slice_k(detail0 + float(l)) * (n >> l);
    }
    return float(base);
}

/// Band of cell cellId (octave-L indices) containing block yHit.
/// Outputs: band colour (0..1), side tile id, liquid/grass flags and the
/// band's block-Y bounds (for the grass fade). gapBelow = the hit Y falls
/// into the CAVE AIR under the matched band (DH: no wall face there);
/// floorTop = the next band's top (the cave floor plane); the gap* outputs
/// carry the deeper band's appearance for the cave-floor top face.
/// bandLight / floorLight (v4.11.0) = the DH datapoint LIGHT bytes
/// (sky<<4|block, 0-255) of the matched band and of the first band BELOW
/// the hit (the cave-gap floor — the wall light source). Defaults 240
/// (open sky) when the query fails or no deeper band exists.
/// Returns false when the cell carries no slice data (fallback path) or
/// yHit is above every band top.
bool slice_band(ivec2 cellId, float L, float n, float yHit,
                out vec3 col, out float tile, out float liquid,
                out float grass, out float bandTop, out float bandBot,
                out float gapBelow, out float floorTop,
                out vec3 gapCol, out float gapTile,
                out float bandLight, out float floorLight) {
    col = vec3(0.0); tile = 0.0; liquid = 0.0; grass = 0.0;
    bandTop = 0.0; bandBot = -1.0e4;
    gapBelow = 0.0; floorTop = -1.0e4;
    gapCol = vec3(0.0); gapTile = 0.0;
    bandLight = 240.0; floorLight = 240.0;
    float detail0 = log2(max(AtlasInfo.w, 2.0));
    int K = slice_k(detail0 + L);
    int nI = int(n);
    int sz = nI >> int(L);
    if (cellId.x < 0 || cellId.y < 0 || cellId.x >= sz || cellId.y >= sz) {
        return false;
    }
    int base = int(slice_row_base(detail0, L, nI));
    int match = -1;
    float topY = 0.0;
    float botY = 0.0;
    bool deeper = false;
    for (int j = 0; j < K; ++j) {
        int row = base + cellId.y * K + j;
        vec4 b = texelFetch(SlicesSampler, ivec2(3 * cellId.x + 1, row), 0);
        if ((int(b.a * 255.0 + 0.5) & 1) == 0) break; // absent (packed top-down)
        vec4 a = texelFetch(SlicesSampler, ivec2(3 * cellId.x, row), 0);
        vec4 c = texelFetch(SlicesSampler, ivec2(3 * cellId.x + 2, row), 0);
        float ty = (a.a * 65280.0 + b.r * 255.0) / 64.0 - 64.0;
        float by = (c.g * 65280.0 + c.r * 255.0) / 64.0 - 64.0;
        if (ty >= yHit) {
            // deepest band whose top is still above the hit
            match = j;
            topY = ty;
            botY = by;
            col = a.rgb;
            tile = floor(b.g * 255.0 + 0.5) * 256.0 + floor(b.b * 255.0 + 0.5);
            int fl = int(b.a * 255.0 + 0.5);
            liquid = float((fl >> 1) & 1);
            grass = float((fl >> 2) & 1);
            bandLight = floor(c.b * 255.0 + 0.5); // DH datapoint light
        } else {
            // the first band BELOW the hit: its top is the cave floor plane;
            // its light is the CAVE-GAP light (DH: a wall against a
            // transparent neighbour is lit from the point below it)
            deeper = true;
            floorTop = ty;
            gapCol = a.rgb;
            gapTile = floor(b.g * 255.0 + 0.5) * 256.0 + floor(b.b * 255.0 + 0.5);
            floorLight = floor(c.b * 255.0 + 0.5);
            break; // tops strictly descend
        }
    }
    if (match < 0) {
        return false;
    }
    bandTop = topY;
    if (deeper) {
        // a band exists below: the stored bottom is the REAL solid floor;
        // the hit inside (top, bottom) air of the matched band is cave air.
        bandBot = botY;
        gapBelow = yHit < botY ? 1.0 : 0.0;
    } else {
        // deepest present band reaches the world floor (DH full-data
        // columns span to min_y) — always solid.
        bandBot = -1.0e4;
        gapBelow = 0.0;
    }
    return true;
}

/// Index of the side/cliff colour stored in the atlas alpha channel.
float atlas_side(vec2 uv) {
    return texture(AtlasSampler, uv).a;
}

// ------------------------------------------------------------------
// DH 3.3.4 textured LODs (terrain/gl/frag.frag): the per-cell tile id
// picks a 16px ratio tile; the UV repeats once per CELL horizontally
// (vBlockPos is in data-point units) and once per BLOCK vertically
// (wall heights stay in block units). Orientation mirrors DH's
// blockFaceUv: up (x,z), north (1-x,1-y), south (x,1-y),
// west (z,1-y), east (1-z,1-y).
// ------------------------------------------------------------------
float tile_id(vec2 uv, bool side) {
    vec4 t = texture(TexIdsSampler, uv);
    vec2 pair = side ? t.ba : t.rg;
    return floor(pair.x * 255.0 + 0.5) * 256.0 + floor(pair.y * 255.0 + 0.5);
}

/// DH tile application: albedo = mix(albedo, albedo * (tile.rgb * 2.0), tile.a)
/// with the tile's 16x16 texels mapped into the fract() UV cell. Half-texel
/// clamping keeps bilinear sampling inside the tile (no neighbour bleed).
/// w = the DH maxTexturedLodDetailLevel distance weight (1 near, 0 far).
vec3 apply_dh_tile(vec3 albedo, float tileId, vec2 faceUv, float tileRows, float w) {
    if (tileId < 0.5 || w < 0.004) {
        return albedo; // tile 0 = the reserved flat tile
    }
    float tx = mod(tileId, TILES_PER_ROW);
    float ty = floor(tileId / TILES_PER_ROW);
    vec2 halfTexel = vec2(0.5) / vec2(TILE_PX, TILE_PX);
    vec2 f = clamp(faceUv, halfTexel, 1.0 - halfTexel);
    vec2 uv = vec2(
        (tx + f.x) * TILE_PX / (TILES_PER_ROW * TILE_PX),
        (ty + f.y) * TILE_PX / (tileRows * TILE_PX)
    );
    vec4 tile = texture(TilesSampler, uv);
    return mix(albedo, albedo * (tile.rgb * 2.0), tile.a * w);
}

// ------------------------------------------------------------------
// DH terrain/gl/frag.frag applyNoise — ported verbatim (defaults above).
// Random grain quantised to 1/NOISE_STEPS-block world cells, strongest on
// mid-luminance colours, faded out by view distance. This is what makes DH
// LODs look "textured" without any texture.
// ------------------------------------------------------------------
float dh_rand1(float co) {
    return fract(sin(co * 91.3458) * 47453.5453);
}
float dh_rand2(vec2 co) {
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}
float dh_rand3(vec3 co) {
    return dh_rand2(co.xy + dh_rand1(co.z));
}
vec3 dh_quantize(vec3 v, float steps) {
    return floor(v * steps) / steps;
}
vec3 dh_noise(vec3 col, vec3 worldPos, vec3 faceN, float viewDist, float ampW) {
    float lum = (col.r + col.g + col.b) / 3.0;
    float amp = (1.0 - pow(lum * 2.0 - 1.0, 2.0)) * NOISE_INTENSITY * ampW;
    vec3 fixedPos = worldPos + faceN * 0.001; // keep the grain on one side of a face
    float r = dh_rand3(dh_quantize(fixedPos, NOISE_STEPS)) * 2.0 * amp - amp;
    vec3 newCol = clamp(col + (1.0 - col) * r, 0.0, 1.0);
    float distF = min(viewDist / NOISE_DROPOFF, 1.0);
    return mix(newCol, col, distF);
}

float gEarthR = 0.0;

/// DH earth curvature (terrain/gl/vert.vert, stduhpf): a ground vertex at
/// horizontal distance d and height y renders at
///   y' = y + (cos(phi)-1) * (R + y),  phi = d / (R + y)
/// (the xz sin(phi)/phi compression is second-order and skipped). The march
/// evaluates the same transform at every ground test: the surface visible
/// along a straight ray at distance d has dropped by curve_drop(d, ground).
/// uEarthRadius = 6371000/ratio; |R| <= 1 disables (DH semantic).
float curve_drop(float dist, float groundY) {
    if (gEarthR < 1.0 && gEarthR > -1.0) {
        return 0.0;
    }
    float lr = gEarthR + groundY;
    float phi = dist / lr;
    return (cos(phi) - 1.0) * lr;
}

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
        fragColor = vec4(c, 0.0);
        return;
    }

    bool skyPix = depth <= SKY_DEPTH;
    vec4 rot = texelFetch(RotSampler, ivec2(0), 0);
    float conf = rot.b;
    float minConf = LodParams.w;
    float forceLod = 0.0;

    // -----------------------------------------------------------------
    // TAA history state (DH client-TAA stand-in — see the taps note).
    // fps: rot pixel 2 = this frame's 3-byte camera fingerprint, pixel 3 =
    // the previous frame's (rot_solve). A mismatch (camera / day-factor /
    // fov move) invalidates BOTH the jitter and the blend for this frame.
    // The history alpha marks which pixels were march-written last frame
    // (pass-through pixels carry 0 — vanilla content is live, accumulating
    // it would smear chunk fade-ins).
    // -----------------------------------------------------------------
    vec4 fpCur = texelFetch(RotSampler, ivec2(2, 0), 0);
    vec4 fpPrev = texelFetch(RotSampler, ivec2(3, 0), 0);
    bool fpMatch = all(lessThan(abs(fpCur.rgb - fpPrev.rgb), vec3(0.002)));
    int taaPhase = int(fpCur.a * 255.0 + 0.5);
    vec4 hist = texelFetch(PrevSampler, ivec2(gl_FragCoord.xy), 0);
    bool taaOk = TaaParams.x > 0.5
        && fpMatch
        && hist.a > 0.5
        && DebugParams.x < 0.5;
    // DH: `gl_Position.xy = TAAJitter(...)` gated by `uFrameMod8 > 0` —
    // frame 0 of the cycle and the disabled mode both stay identity.
    vec2 taaJit = (taaOk && taaPhase > 0)
        ? TAA_JITTER_OFFSETS[taaPhase] / OutSize
        : vec2(0.0);

    // Debug mode 4: LOD hit mask + tracker confidence bar.
    if (DebugParams.x > 3.5) {
        if (gl_FragCoord.x < 240.0 && gl_FragCoord.y > OutSize.y - 24.0) {
            vec3 bar = gl_FragCoord.x < conf * 240.0 ? vec3(1.0, 0.0, 0.0) : vec3(0.0);
            fragColor = vec4(bar, 0.0);
            return;
        }
        forceLod = 1.0;
    }

    if (DebugParams.x > 0.5 && DebugParams.x < 1.5) {
        fragColor = vec4(conf, 0.0, 0.0, 0.0);
        return;
    }

    if (DebugParams.x > 1.5 && DebugParams.x < 2.5) {
        if (texCoord.x >= 0.5) {
            float hm = atlas_height(texCoord);
            float g = clamp((hm + 64.0) / 384.0, 0.0, 1.0);
            vec3 mc = vec3(g);
            if (atlas_water(texCoord)) {
                mc = mix(mc, vec3(0.3, 0.5, 1.0), 0.6);
            }
            fragColor = vec4(mc, 0.0);
            return;
        }
        forceLod = 1.0;
    }

    if (conf < minConf && forceLod < 0.5) {
        fragColor = vec4(scene.rgb, 0.0);
        return;
    }

    // v4.0 encoding preserved: R = az/2pi, G = (el+pi/2)/pi, B = conf.
    // A = fov byte (v4.1): the REAL vertical fov measured by the sky shader.
    float az = rot.r * 6.28318530718;
    float el = rot.g * PI - PI * 0.5;

    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));

    float fov = rot.a > 0.002 ? 5.0 + rot.a * 115.0 : FogParams.z;
    float tanH = tan(radians(fov) * 0.5);
    float aspect = OutSize.x / max(OutSize.y, 1.0);

    vec2 ndc = texCoord * 2.0 - 1.0 + taaJit;
    vec3 dir = normalize(F + R * (ndc.x * tanH * aspect) + U * (ndc.y * tanH));

    // 26.3 Globals: CameraBlockPos = floor(camPos), CameraOffset = floor - camPos.
    vec3 cam = vec3(CameraBlockPos) - CameraOffset;

    // Clouds are opaque geometry above the highest atlas cell.
    if (!skyPix) {
        float eyeZ = SkyParams.x / max(depth, 1.0e-9);
        float vlen = length(vec3(ndc.x * tanH * aspect, ndc.y * tanH, 1.0));
        vec3 wp = cam + dir * (eyeZ * vlen);
        skyPix = wp.y > SkyParams.y;
    }
    if (!skyPix) {
        fragColor = vec4(scene.rgb, 0.0);
        return;
    }

    vec2 origin = AtlasInfo.xy;
    float size = AtlasInfo.z;
    float cell = AtlasInfo.w;
    float near = LodParams.x;
    float maxY = LodParams.y;

    vec2 minW = origin;
    vec2 maxW = origin + vec2(size);
    gEarthR = FogParams.w;
    float hlen = length(dir.xz);

    // Rays that leave the heightfield from above can never hit: early-out.
    if (dir.y > 0.0) {
        float toCeil = (maxY + 64.0 - cam.y) / dir.y;
        float toEdge = 1.0e9;
        if (dir.x > 0.0) toEdge = min(toEdge, (maxW.x - cam.x) / dir.x);
        if (dir.x < 0.0) toEdge = min(toEdge, (minW.x - cam.x) / dir.x);
        if (dir.z > 0.0) toEdge = min(toEdge, (maxW.y - cam.z) / dir.z);
        if (dir.z < 0.0) toEdge = min(toEdge, (minW.y - cam.z) / dir.z);
        if (toCeil < toEdge) {
            fragColor = DebugParams.x > 3.5 ? vec4(0.6, 0.0, 0.9, 0.0) : vec4(scene.rgb, 0.0);
            return;
        }
    }
    if (abs(dir.y) < 1.0e-6) {
        dir.y = 1.0e-6;
    }

    // -------------------------------------------------------------
    // Amanatides-Woo DDA over the cell grid — the DH quad set, exactly.
    // -------------------------------------------------------------
    // Clip the ray to the atlas box (xz). The camera is normally inside;
    // debug shots can park it outside — enter through the box face then.
    float tCur = 0.0;
    vec2 enterXz = cam.xz;
    if (any(lessThan(cam.xz, minW)) || any(greaterThan(cam.xz, maxW))) {
        float tEnter = -1.0e9;
        if (dir.x > 0.0) tEnter = max(tEnter, (minW.x - cam.x) / dir.x);
        if (dir.x < 0.0) tEnter = max(tEnter, (maxW.x - cam.x) / dir.x);
        if (dir.z > 0.0) tEnter = max(tEnter, (minW.y - cam.z) / dir.z);
        if (dir.z < 0.0) tEnter = max(tEnter, (maxW.y - cam.z) / dir.z);
        if (tEnter < 0.0) {
            fragColor = DebugParams.x > 3.5 ? vec4(1.0, 0.45, 0.0, 0.0) : vec4(scene.rgb, 0.0);
            return;
        }
        tCur = tEnter;
        enterXz = cam.xz + dir.xz * tCur;
    }

    vec2 cellF = floor((enterXz - origin) / cell);
    float cellsSide = size / cell;

    // Atlas cell grid (level-0 slice queries in the march + the pyramid
    // colour reads after the hit share it).
    float cellsN = AtlasInfo.z / max(AtlasInfo.w, 1.0);

    // Classic DDA setup: tMax = t to the first x/z grid line, tDelta = t per cell.
    float tMaxX = 1.0e9;
    float tMaxZ = 1.0e9;
    float tDeltaX = 1.0e9;
    float tDeltaZ = 1.0e9;
    float stepX = dir.x > 0.0 ? 1.0 : -1.0;
    float stepZ = dir.z > 0.0 ? 1.0 : -1.0;
    if (abs(dir.x) > 1.0e-6) {
        float boundary = cellF.x + (dir.x > 0.0 ? 1.0 : 0.0);
        tMaxX = (boundary * cell + origin.x - cam.x) / dir.x;
        tDeltaX = cell / abs(dir.x);
    }
    if (abs(dir.z) > 1.0e-6) {
        float boundary = cellF.y + (dir.z > 0.0 ? 1.0 : 0.0);
        tMaxZ = (boundary * cell + origin.y - cam.z) / dir.z;
        tDeltaZ = cell / abs(dir.z);
    }

    // hCur = surface height of the cell the ray is currently in. Liquid cells
    // store their WATER SURFACE height (the scan reconciles heightmap variants
    // against sea level), so the march treats water boxes exactly like DH: a
    // column whose top quad sits at the water surface, coloured with the
    // biome-tinted water colour.
    float hCur = world_height(enterXz, origin, size);
    bool curVoid = atlas_void(atlas_uv(enterXz, origin, size));

    // DH far-fog plane (see the composite below): marching past it is pointless
    // — the hit would be >= 99.8% fog. The early-out keeps the AVERAGE step
    // count bound by the fog range rather than the atlas side.
    float fogFar = max(FogParams.y, 1.0);

    // Step budget: a ray crosses at most (fogFar/cell in x + fogFar/cell in z)
    // grid boundaries before the fog early-out stops the march. This scales
    // with the CONFIGURED atlas — the old constant 560 silently truncated the
    // walk on the 4096-block product default (fogFar 2048 needs up to 1024+
    // crossings at cell 4, 2048+ at cell 2) and cut the terrain mid-band.
    int maxSteps = int(2.0 * fogFar / cell) + 64;

    float hitT = -1.0;
    bool sideHit = false;
    bool hitAxisX = false;
    // DH FADE_TO_DIRT state: the wall's exposed span [hHitOld..hHitNew].
    float hHitOld = 0.0;
    float hHitNew = 0.0;
    // DH cave air: set once the ray passed through a slice air gap (a cave
    // mouth). Hits after that are CAVE walls/floors — DH lights them from
    // the stored light data (v4.11.0: the real per-band sky/block light);
    // cleared when the ray re-emerges above a column's surface (open sky
    // again).
    bool caveDepth = false;

    for (int i = 0; i < maxSteps; ++i) {
        // TOP face: inside this cell the ray may cross the plane y = hCur from
        // above. DH skips the top quad only when another box covers it — for a
        // heightfield that case is exactly "the ray reaches y=hCur while still
        // inside the cell's footprint". Void cells have no top face.
        if (!curVoid && dir.y < 0.0) {
            float tTop = (hCur - cam.y) / dir.y;
            // Curvature: the top plane drops with distance; two fixed-point
            // refinements converge far below cell precision.
            if (gEarthR > 1.0 || gEarthR < -1.0) {
                float y1 = hCur + curve_drop(tTop * hlen, hCur);
                tTop = (y1 - cam.y) / dir.y;
                float y2 = hCur + curve_drop(tTop * hlen, hCur);
                tTop = (y2 - cam.y) / dir.y;
            }
            float tExit = min(tMaxX, tMaxZ);
            if (tTop >= tCur && tTop < tExit) {
                hitT = tTop;
                sideHit = false;
                break;
            }
        }

        // CAVE FLOOR: inside a cave (the ray passed a gap) the descending ray
        // can land on the top face of a DEEPER band — DH renders every
        // datapoint's top quad. The floor plane is the first band top under
        // the ray; the crossing must still happen inside this cell.
        if (caveDepth && dir.y < 0.0 && !curVoid) {
            float rayYc = cam.y + dir.y * tCur;
            vec3 fq; float ft_, fl_, fg_, fTop_, fBot_, fGap, fFloor, fBL_, fFL_;
            vec3 fgc; float fgt_;
            if (slice_band(ivec2(cellF), 0.0, cellsN, floor(rayYc),
                           fq, ft_, fl_, fg_, fTop_, fBot_, fGap, fFloor, fgc, fgt_,
                           fBL_, fFL_)
                && fGap > 0.5 && fFloor > -9.0e3) {
                float tFloor = (fFloor - cam.y) / dir.y;
                if (gEarthR > 1.0 || gEarthR < -1.0) {
                    float y1 = fFloor + curve_drop(tFloor * hlen, fFloor);
                    tFloor = (y1 - cam.y) / dir.y;
                    float y2 = fFloor + curve_drop(tFloor * hlen, fFloor);
                    tFloor = (y2 - cam.y) / dir.y;
                }
                float tExitC = min(tMaxX, tMaxZ);
                if (tFloor >= tCur && tFloor < tExitC) {
                    hitT = tFloor;
                    sideHit = false;
                    break;
                }
            }
        }

        // Step to the next grid boundary.
        float tNext;
        bool axisX;
        if (tMaxX < tMaxZ) {
            tNext = tMaxX;
            axisX = true;
        } else {
            tNext = tMaxZ;
            axisX = false;
        }
        tCur = tNext;

        // Beyond the fog end nothing is visible — stop marching.
        if (tCur > fogFar) {
            break;
        }

        // Moved out of the atlas? No LOD here (fog is 1.0 at the band edge).
        vec2 nextXz = cam.xz + dir.xz * tCur;
        if (any(lessThan(nextXz, minW)) || any(greaterThan(nextXz, maxW))) {
            break;
        }

        if (axisX) {
            cellF.x += stepX;
            tMaxX += tDeltaX;
        } else {
            cellF.y += stepZ;
            tMaxZ += tDeltaZ;
        }
        if (cellF.x < 0.0 || cellF.y < 0.0 || cellF.x >= cellsSide || cellF.y >= cellsSide) {
            break;
        }

        // SIDE face: entering a cell whose surface is above the ray. With
        // the DH slice table the wall face exists only where the ENTERED
        // column is SOLID at this Y — a cave air gap has NO face: the ray
        // flies through the mouth (caveDepth = the unlit-cave state). Cells
        // without slice data keep the v4.9 heightfield behaviour.
        // Walls exist only between two SCANNED cells: a void neighbour is
        // un-generated data, not a pit — the ray passes through it and the
        // next scanned cell resets the span (no white cliff walls out of
        // scan holes, no cross-sections at a deadline-cut scan edge).
        float rayY = cam.y + dir.y * tCur;
        bool newVoid = atlas_void(atlas_uv(nextXz, origin, size));
        if (!newVoid) {
            float hNew = world_height(nextXz, origin, size);
            // Curvature: the wall's top edge renders at the curved height of
            // the neighbour's ground vertex at this distance.
            float hNewR = hNew + curve_drop(tCur * hlen, hNew);
            if (!curVoid && hNewR > rayY) {
                vec3 sq; float st_, sl_, sg_, sTop_, sBot_, sGap, sFloor, sBL_, sFL_;
                vec3 sgc; float sgt_;
                if (slice_band(ivec2(cellF), 0.0, cellsN, floor(rayY),
                               sq, st_, sl_, sg_, sTop_, sBot_, sGap, sFloor, sgc, sgt_,
                               sBL_, sFL_)
                    && sGap > 0.5) {
                    // DH: no wall face inside the air gap — keep marching.
                    caveDepth = true;
                } else {
                    hitT = tCur;
                    sideHit = true;
                    hitAxisX = axisX;
                    hHitOld = hCur;
                    hHitNew = hNew;
                    break;
                }
            } else if (hNewR <= rayY) {
                // the ray passed above the neighbour's surface: open sky
                // again (a cave mouth opening onto lower ground).
                caveDepth = false;
            }
            hCur = hNew;
        } else {
            caveDepth = false; // unscanned data — no cave state past it
        }
        curVoid = newVoid;
    }

    if (hitT < 0.0) {
        if (caveDepth && DebugParams.x <= 3.5) {
            // DH: looking deeper into a cave shows unlit datapoints (light 0).
            // Flat unlit stand-in, then the same distance fog as real hits
            // (tCur = where the march gave up).
            float fogStart = FOG_START_FRAC * fogFar;
            float fogLen = max(fogFar - fogStart, 1.0);
            float fx = max((tCur - fogStart) / fogLen, 0.0) * FOG_DENSITY;
            float fog = clamp(1.0 - exp(-fx * fx), 0.0, 1.0);
            vec3 caveDark = vec3(0.045, 0.048, 0.058);
            fragColor = vec4(mix(caveDark, scene.rgb, fog), 1.0);
            return;
        }
        fragColor = DebugParams.x > 3.5 ? vec4(1.0, 0.45, 0.0, 1.0) : scene;
        return;
    }
    if (DebugParams.x > 4.5 && DebugParams.x < 5.5) {
        // Distance tint: red < 300 blocks, green ~600, blue > 900.
        vec3 c = mix(vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), clamp(hitT / 600.0, 0.0, 1.0));
        c = mix(c, vec3(0.0, 0.25, 1.0), clamp((hitT - 600.0) / 600.0, 0.0, 1.0));
        fragColor = vec4(c, 1.0);
        return;
    }
    if (DebugParams.x > 3.5) {
        fragColor = vec4(0.0, 1.0, 0.1, 1.0);
        return;
    }

    // DH renders LOD quads from the vanilla render distance out; nearer hits
    // would be occluded by real chunks (and are scan noise anyway).
    if (hitT < near && forceLod < 0.5) {
        fragColor = scene;
        return;
    }

    // NOTE ON DH's NEAR-CLIP BAYER DITHER (terrain/gl/frag.frag:137-161):
    // deliberately NOT ported. DH's LODs exist UNDER the vanilla chunks
    // (overdraw 0.2-0.9 of the vanilla radius), so discarding dithered LOD
    // fragments there reveals the REAL TERRAIN behind them. Our LOD becomes
    // visible exactly AT the vanilla boundary — a discarded fragment would
    // reveal bare SKY, sprinkling bright dither dots over the near band
    // (verified on the real client, v4.8.0 e2e). The hard cut at `near` IS
    // the correct DH-equivalent for a posteffect pipeline; the chunk->LOD
    // seam is clean because vanilla chunk fog is disabled and the LOD band
    // shares the fog stack (verified e2e v4.6-v4.8).

    float fade = forceLod > 0.5 ? 1.0 : smoothstep(minConf, minConf + 0.15, conf);
    vec3 hitPos = cam + dir * hitT;
    vec2 auv = atlas_uv(hitPos.xz, origin, size);

    // DH colour model: one flat colour per column box (the block's baked
    // average, biome tinted), shaded per face with vanilla's directional
    // factors. Water columns carry their own biome-tinted water colour in the
    // atlas (DH: the water data point's colour — no global water constant),
    // so every hit reads the cell colours and the UBO colour stays unused.
    // Side walls exist only on the exposed span; grass material fades from
    // the top colour at the wall's TOP edge to the side (dirt) colour at its
    // BOTTOM edge — LodQuadBuilder FADE_TO_DIRT: the bottom vertex row of a
    // grass wall is recoloured to getDirtBlockColor() and the GPU interpolates
    // the span linearly.
    //
    // The top colour is read from the DH detail pyramid at the pixel's world
    // footprint: near hits stay on the per-cell level 0 (identical to the
    // v4.6 look), far hits aggregate 2x2 per octave exactly like DH's coarse
    // datapoints — point-sampling the fine grid out there was per-pixel
    // white noise (the "noisy pixels").
    float footprint = pyr_footprint(hitT, tanH);
    float pyrLvl = clamp(log2(max(footprint / AtlasInfo.w, 1.0)), 0.0, 24.0);
    vec3 topCol = pyr_color(atlas_uv(hitPos.xz, origin, size), pyrLvl, cellsN);
    vec3 sideCol = texture(
        SideSampler,
        vec2((pyr_side(atlas_uv(hitPos.xz, origin, size), pyrLvl, cellsN) * 255.0 + 0.5) / 256.0, 0.5)
    ).rgb;

    // Wall colour resolution. DH renders each datapoint box in ITS OWN flat
    // colour (the block's first-face average): the band containing the hit Y
    // wins. Grass bands fade to the dirt colour across their own REAL span
    // (LodQuadBuilder FADE_TO_DIRT, default EDhApiGrassSideRendering). A
    // hit inside a band's cave-air span (gapBelow — far octaves only: the
    // near march never lands there) and cells without slice data (un-scanned,
    // or octaves above the strip budget) fall back to the v4.8 top->side
    // gradient.
    float rayHitY = cam.y + dir.y * hitT;
    float shade = 1.0;
    float spanT = 0.0;
    float tileId = 0.0;
    vec3 albedo;
    // DH datapoint light at the hit (sky/block, 0..15). Surface hits keep
    // open-sky (15, 0) — the existing day/night stand-in; cave hits carry
    // the REAL light the scan captured (v4.11.0).
    float lightSky = 15.0;
    float lightBlk = 0.0;
    if (sideHit) {
        // Directional shade from the crossed grid line: X boundary -> E/W wall.
        shade = hitAxisX ? SHADE_EW : SHADE_NS;
        float span = max(hHitNew - hHitOld, 1.0e-3);
        spanT = clamp((hHitNew - rayHitY) / span, 0.0, 1.0);
        int Ls = int(floor(pyrLvl));
        vec2 bandUv = atlas_uv(hitPos.xz, origin, size);
        int szL = int(cellsN) >> Ls;
        ivec2 cellId = ivec2(floor(bandUv * float(szL)));
        vec3 bandCol;
        float bandTile, bandLiq, bandGrass, bandTop, bandBot, bandGap, bandFloor;
        float bandLight, bandFloorLight;
        vec3 bandGapCol;
        float bandGapTile;
        bool hasBand = float(Ls) <= SkyParams.w + 0.5
            && slice_band(cellId, float(Ls), cellsN, floor(rayHitY),
                          bandCol, bandTile, bandLiq, bandGrass, bandTop, bandBot,
                          bandGap, bandFloor, bandGapCol, bandGapTile,
                          bandLight, bandFloorLight);
        if (hasBand && bandGap < 0.5) {
            // DH wall light (makeAdjVerticalQuad): sky = the NEIGHBOUR's
            // light at the wall (the air the ray flew through — the incoming
            // cell's gap-floor band), block = the wall datapoint's OWN block
            // light. A missing neighbour column lights the wall sky-MAX (the
            // slice_band default 240 = 0xF0).
            float wSky = 240.0;
            {
                ivec2 adjId = cellId - ivec2(hitAxisX ? int(stepX) : 0,
                                             hitAxisX ? 0 : int(stepZ));
                vec3 aCol; float aTile, aLiq, aGrass, aTop, aBot, aGap, aFloor;
                float aBL, aFL;
                vec3 aGapCol; float aGapTile;
                if (slice_band(adjId, float(Ls), cellsN, floor(rayHitY),
                               aCol, aTile, aLiq, aGrass, aTop, aBot, aGap, aFloor,
                               aGapCol, aGapTile, aBL, aFL)
                    && aGap > 0.5) {
                    wSky = aFL; // the light of the air gap the ray is in
                }
            }
            lightSky = floor(wSky / 16.0);
            lightBlk = mod(bandLight, 16.0);
            vec3 base = bandCol;
            if (bandGrass > 0.5) {
                // DH: the bottom vertex row of a grass wall is recoloured to
                // getDirtBlockColor() and the GPU interpolates across the
                // datapoint's own wall span.
                float bt = clamp(
                    (bandTop + 1.0 - rayHitY) / max(bandTop + 1.0 - bandBot, 1.0),
                    0.0, 1.0);
                base = mix(base, ColLow.rgb, bt);
            }
            albedo = base * shade;
            tileId = bandTile;
        } else {
            // fallback: v4.8 top->side (dirt) wall gradient
            albedo = mix(topCol, sideCol, spanT) * shade;
            tileId = tile_id(auv, true);
        }
    } else if (caveDepth) {
        // CAVE FLOOR (the march landed on a deeper band's top quad): its
        // colour is that band's flat datapoint colour — level 0, the same
        // query the march used to find the plane.
        vec2 fUv = atlas_uv(hitPos.xz, origin, size);
        ivec2 fCell = ivec2(clamp(floor(fUv * cellsN), vec2(0.0), vec2(cellsN - 1.0)));
        vec3 fCol;
        float fTile, fLiq, fGrass, fTop, fBot, fGap, fFloor2, fBandL, fFloorL;
        vec3 fGapCol;
        float fGapTile;
        if (slice_band(fCell, 0.0, cellsN, floor(rayHitY),
                       fCol, fTile, fLiq, fGrass, fTop, fBot, fGap, fFloor2,
                       fGapCol, fGapTile, fBandL, fFloorL)
            && fGap < 0.5) {
            albedo = fCol; // top face: shade 1.0
            tileId = fTile;
            // DH top face: the datapoint's OWN sky/block light (addQuadUp).
            lightSky = floor(fBandL / 16.0);
            lightBlk = mod(fBandL, 16.0);
        } else {
            albedo = vec3(0.05); // degenerate: unlit fallback
            lightSky = 0.0;
            lightBlk = 0.0;
        }
    } else {
        albedo = topCol; // top faces: shade 1.0
        tileId = tile_id(auv, false);
    }

    // DH 3.3.4 textured LODs: the ratio tile of THIS datapoint (the band's
    // side tile on walls, the cell's top tile on top faces — tileId came out
    // of the albedo resolution above).
    // Horizontal repeats = one per CELL (vBlockPos is in data-point units in
    // DH), vertical repeats on walls = one per BLOCK. The tile weight
    // implements DH's maxTexturedLodDetailLevel as a FOOTPRINT fade: a texel
    // is cell/16 blocks, and once it drops under the pixel footprint the
    // mipless point sample shimmers (water checkerboard, snow speckle) —
    // fade across that octave; far LODs flatten to colour + noise exactly
    // like DH's coarse datapoints.
    float tileRows = SkyParams.z;
    float texelB = AtlasInfo.w / TILE_PX;
    float tileW = clamp((texelB / footprint - 0.5) / 1.5, 0.0, 1.0);
    vec2 faceUv;
    if (sideHit) {
        float yF = fract(hitPos.y); // walls repeat per block vertically
        if (hitAxisX) {
            // ray moving +X hits a west-facing wall: (z, 1-y); +X wall: (1-z, 1-y)
            float u = fract(hitPos.z / cell);
            faceUv = vec2((stepX > 0.0) ? u : (1.0 - u), yF);
        } else {
            // ray moving +Z hits a north-facing wall: (1-x, 1-y); south: (x, 1-y)
            float u = fract(hitPos.x / cell);
            faceUv = vec2((stepZ > 0.0) ? (1.0 - u) : u, yF);
        }
    } else {
        faceUv = fract(hitPos.xz / cell); // up: (x, z)
    }

    // Light: the day factor comes from the camera-channel's celestial probe
    // (rot pixel 1: the darkest true-sky luminance across the frame, mapped
    // through the vanilla lightmap-ish curve by rot_solve). DH lights distant
    // LODs through the vanilla lightmap at (skyLight 15, blockLight 0), which
    // follows the celestial angle — the horizon-glow-independent probe is the
    // faithful stand-in; moonlight keeps a faint blue cast like the vanilla
    // night lightmap.
    float dayF = texelFetch(RotSampler, ivec2(1, 0), 0).r;
    float lightMul = mix(0.22, 1.0, dayF);
    vec3 nightTint = vec3(0.84, 0.89, 1.08);
    vec3 lit = albedo * lightMul * mix(vec3(1.0), nightTint, 1.0 - dayF);

    // DH cave light (v4.11.0 — the light column is ported): datapoints
    // carry (sky, block) light — the light at the block above the run's top
    // (LodDataBuilder lines 200-201). Enclosed caves read (0,0): the vanilla
    // lightmap curve at the origin, day-INDEPENDENT dark; a torch-lit cave
    // band carries its block light and glows warm. The curve is the classic
    // x/(4-3x) lightmap ramp: sky scaled by the reconstructed celestial
    // brightness (dayF — the surface stand-in above), block carries the
    // vanilla torch tint. The 0.10 floor keeps unlit caves visible exactly
    // like the old stand-in (lightmap 0.05-0.25 by brightness setting).
    if (caveDepth) {
        float sN = lightSky / 15.0;
        float bN = lightBlk / 15.0;
        float sCurve = sN / (4.0 - 3.0 * sN);
        float bCurve = bN / (4.0 - 3.0 * bN);
        vec3 skyLit = vec3(sCurve * dayF) * mix(vec3(1.0), nightTint, 1.0 - dayF);
        vec3 blockLit = vec3(bCurve) * vec3(1.0, 0.90, 0.70);
        vec3 lm = max(max(skyLit, blockLit), vec3(0.10));
        lit = albedo * lm;
    }

    // DH order: the texture multiplies the ALREADY-LIT flat colour
    // (vertexColor = lightmap * color, then the tile mix in frag).
    lit = apply_dh_tile(lit, tileId, faceUv, tileRows, tileW);

    // DH fake-texture grain (terrain pass applies it after lighting, before
    // the fog pass) — ONLY on untextured pixels (vTextureTileId == 0u):
    // textured blocks carry their real grain. The noise cell is anchored to
    // the FACE — tops use the up normal, walls the normal of the crossed grid
    // line pointing back at the camera side (-stepX / -stepZ). The grain
    // cell is 1/NOISE_STEPS blocks; below the pixel footprint a point sample
    // of it is white noise, so the amplitude fades across that octave (DH's
    // near-field grain is preserved verbatim, the far "noisy pixels" die).
    if (tileId < 0.5 || tileW < 0.996) {
        vec3 faceN = sideHit
            ? (hitAxisX ? vec3(-stepX, 0.0, 0.0) : vec3(0.0, 0.0, -stepZ))
            : vec3(0.0, 1.0, 0.0);
        float noiseW = clamp((1.0 / NOISE_STEPS) / footprint, 0.0, 1.0);
        lit = dh_noise(lit, hitPos, faceN, hitT, noiseW);
    }

    // DH far fog: exponential-squared, 0.4..1.0 x LOD draw distance, density
    // 2.5, spherical distance (GlDhFogShader + config defaults). The fog
    // target is the vanilla WORLD FOG COLOUR read as the sky behind the pixel
    // (the legal stand-in — see the uniform note above); at the band edge the
    // LOD dissolves into the horizon exactly like DH.
    float fogStart = FOG_START_FRAC * fogFar;
    float fogLen = max(fogFar - fogStart, 1.0);
    float fx = max((hitT - fogStart) / fogLen, 0.0) * FOG_DENSITY;
    float fog = clamp(1.0 - exp(-fx * fx), 0.0, 1.0);
    lit = mix(lit, scene.rgb, fog);

    // DH TAA: the march result joins the temporal accumulator (alpha = 1
    // marks the pixel as march-written for the next frame's validity test;
    // clod_blit forces alpha back to 1.0 for minecraft:main). History weight
    // 7/8 converges to the 8-tap average in ~40 frames, fast enough for the
    // static reference shots and slow camera pans, while any move re-anchors
    // on the raw frame within one step.
    vec3 outRgb = mix(scene.rgb, lit, fade);
    if (taaOk) {
        outRgb = mix(outRgb, hist.rgb, TaaParams.y);
    }
    fragColor = vec4(outRgb, 1.0);
}
