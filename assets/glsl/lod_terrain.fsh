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
uniform sampler2D TilesSampler;   // DH ratio-tile atlas (16px tiles, 256/row)
uniform sampler2D TexIdsSampler;  // per-cell tile ids: RG top, BA side (big-endian)
uniform sampler2D RotSampler;     // clod:clod_rot_a (1x1 persistent, camera channel)
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

// x = fog start (legacy), y = fog end (blocks) == LOD draw distance, z = static fov fallback
layout(std140) uniform ClodFog {
    vec4 FogParams;
};

// x = debug mode (0 = normal, 1 = confidence heat, 2 = split, 3 = pattern,
// 4 = hit mask, 5 = hit distance)
layout(std140) uniform ClodDebug {
    vec4 DebugParams;
};

// x = device depth near plane (blocks), y = cloud ceiling (atlas max + margin),
// z = ratio-tile atlas rows (DH textured LODs)
layout(std140) uniform ClodSky {
    vec4 SkyParams;
};

layout(std140) uniform ClodColors {
    vec4 ColLow;
    vec4 ColHigh;
    vec4 ColWater;  // water surface (DH: flat biome water colour, opaque)
    vec4 VeilColor; // a = snow whiteness threshold
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;
// 26.3 device depth: d = near/eyeZ (infinite far) — sky == 0.0.
const float SKY_DEPTH = 1.0e-6;
// A ray crosses at most (fogFar/cell in x + fogFar/cell in z) grid boundaries
// before the fog early-out stops the march: 2 x (1024/4) = 512 for the
// 2048-block atlas (fogFar = 1024, cell 4), plus margin.
const int MAX_STEPS = 560;

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

// DH 3.3.4 textured LODs (BlockTextureRegistry + terrain/gl/frag.frag):
// tile pixels are ratios, 128/255 == 1.0, so the sampler value doubles.
const float TILE_PX = 16.0;
const float TILES_PER_ROW = 256.0;

// DH zeroes the tile id at MESH BUILD time once the datapoint is coarser
// than Config.Client.Advanced.Graphics.Texture.maxTexturedLodDetailLevel
// (default 2): near LODs carry textures, far LODs fall back to the flat
// colour + noise. Our cells are a fixed-size grid, so the same rule is
// expressed as a DISTANCE fade: a 16px tile spans one cell, and once a
// cell covers only a couple of screen pixels the bilinear (mipless) tile
// sampling shimmers — the snow-field speckle and the water checkerboard
// in the e2e matrix. The fade is proportional to the cell size so the
// texel density (16 texels per cell) stays the limiting factor: full
// texture at the LOD start (the vanilla-overlap band), flat around the
// point where the DH fog itself reaches ~50% (fog 0.5 at ~0.73 x fogFar).
const float TILE_FADE_CELL_START = 64.0;
const float TILE_FADE_CELL_END = 160.0;

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
vec3 dh_noise(vec3 col, vec3 worldPos, vec3 faceN, float viewDist) {
    float lum = (col.r + col.g + col.b) / 3.0;
    float amp = (1.0 - pow(lum * 2.0 - 1.0, 2.0)) * NOISE_INTENSITY;
    vec3 fixedPos = worldPos + faceN * 0.001; // keep the grain on one side of a face
    float r = dh_rand3(dh_quantize(fixedPos, NOISE_STEPS)) * 2.0 * amp - amp;
    vec3 newCol = clamp(col + (1.0 - col) * r, 0.0, 1.0);
    float distF = min(viewDist / NOISE_DROPOFF, 1.0);
    return mix(newCol, col, distF);
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

    if (DebugParams.x > 0.5 && DebugParams.x < 1.5) {
        fragColor = vec4(conf, 0.0, 0.0, 1.0);
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
            fragColor = vec4(mc, 1.0);
            return;
        }
        forceLod = 1.0;
    }

    if (conf < minConf && forceLod < 0.5) {
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

    float fov = rot.a > 0.002 ? 5.0 + rot.a * 115.0 : FogParams.z;
    float tanH = tan(radians(fov) * 0.5);
    float aspect = OutSize.x / max(OutSize.y, 1.0);

    vec2 ndc = texCoord * 2.0 - 1.0;
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
        fragColor = scene;
        return;
    }

    vec2 origin = AtlasInfo.xy;
    float size = AtlasInfo.z;
    float cell = AtlasInfo.w;
    float near = LodParams.x;
    float maxY = LodParams.y;

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
            fragColor = DebugParams.x > 3.5 ? vec4(1.0, 0.45, 0.0, 1.0) : scene;
            return;
        }
        tCur = tEnter;
        enterXz = cam.xz + dir.xz * tCur;
    }

    vec2 cellF = floor((enterXz - origin) / cell);
    float cellsSide = size / cell;

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
    // — the hit would be >= 99.8% fog. The early-out keeps the step count bound
    // by the FOG RANGE instead of the atlas size, so a 2048-block atlas costs
    // no more steps than the old 1024 one.
    float fogFar = max(FogParams.y, 1.0);

    float hitT = -1.0;
    bool sideHit = false;
    bool hitAxisX = false;
    // DH FADE_TO_DIRT state: the wall's exposed span [hHitOld..hHitNew].
    float hHitOld = 0.0;
    float hHitNew = 0.0;

    for (int i = 0; i < MAX_STEPS; ++i) {
        // TOP face: inside this cell the ray may cross the plane y = hCur from
        // above. DH skips the top quad only when another box covers it — for a
        // heightfield that case is exactly "the ray reaches y=hCur while still
        // inside the cell's footprint". Void cells have no top face.
        if (!curVoid && dir.y < 0.0) {
            float tTop = (hCur - cam.y) / dir.y;
            float tExit = min(tMaxX, tMaxZ);
            if (tTop >= tCur && tTop < tExit) {
                hitT = tTop;
                sideHit = false;
                break;
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

        // SIDE face: entering a cell whose surface is above the ray. The ray
        // is guaranteed to be above hCur (we never continue below terrain),
        // so this is exactly DH's wall on the exposed span [hCur..hNew].
        // Walls exist only between two SCANNED cells: a void neighbour is
        // un-generated data, not a pit — the ray passes through it and the
        // next scanned cell resets the span (no white cliff walls out of
        // scan holes, no cross-sections at a deadline-cut scan edge).
        float rayY = cam.y + dir.y * tCur;
        bool newVoid = atlas_void(atlas_uv(nextXz, origin, size));
        if (!newVoid) {
            float hNew = world_height(nextXz, origin, size);
            if (!curVoid && hNew > rayY) {
                hitT = tCur;
                sideHit = true;
                hitAxisX = axisX;
                hHitOld = hCur;
                hHitNew = hNew;
                break;
            }
            hCur = hNew;
        }
        curVoid = newVoid;
    }

    if (hitT < 0.0) {
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
    vec3 topCol = texture(ColorSampler, auv).rgb;
    vec3 sideCol = texture(
        SideSampler,
        vec2((atlas_side(auv) * 255.0 + 0.5) / 256.0, 0.5)
    ).rgb;

    vec3 albedo;
    if (sideHit) {
        // Directional shade from the crossed grid line: X boundary -> E/W wall.
        float shade = hitAxisX ? SHADE_EW : SHADE_NS;
        float span = max(hHitNew - hHitOld, 1.0e-3);
        float spanT = clamp((hHitNew - (cam.y + dir.y * hitT)) / span, 0.0, 1.0);
        // wall top edge -> column colour, wall bottom edge -> side (dirt) colour
        albedo = mix(topCol, sideCol, spanT) * shade;
    } else {
        albedo = topCol; // top faces: shade 1.0
    }

    // DH 3.3.4 textured LODs: the ratio tile of this column's top block.
    // Horizontal repeats = one per CELL (vBlockPos is in data-point units in
    // DH), vertical repeats on walls = one per BLOCK. The tile weight
    // implements DH's maxTexturedLodDetailLevel as a distance fade — far
    // LODs are flat + noise, exactly like DH's coarse datapoints.
    float tileRows = SkyParams.z;
    float tileId = tile_id(auv, sideHit);
    float tileW = 1.0 - smoothstep(
        TILE_FADE_CELL_START * cell, TILE_FADE_CELL_END * cell, hitT);
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

    // DH order: the texture multiplies the ALREADY-LIT flat colour
    // (vertexColor = lightmap * color, then the tile mix in frag).
    lit = apply_dh_tile(lit, tileId, faceUv, tileRows, tileW);

    // DH fake-texture grain (terrain pass applies it after lighting, before
    // the fog pass) — ONLY on untextured pixels (vTextureTileId == 0u):
    // textured blocks carry their real grain. The noise cell is anchored to
    // the FACE — tops use the up normal, walls the normal of the crossed grid
    // line pointing back at the camera side (-stepX / -stepZ). Beyond the
    // tile fade the flat colour takes the grain over, exactly like DH's
    // coarse-level LODs.
    if (tileId < 0.5 || tileW < 0.996) {
        vec3 faceN = sideHit
            ? (hitAxisX ? vec3(-stepX, 0.0, 0.0) : vec3(0.0, 0.0, -stepZ))
            : vec3(0.0, 1.0, 0.0);
        lit = dh_noise(lit, hitPos, faceN, hitT);
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

    fragColor = vec4(mix(scene.rgb, lit, fade), scene.a);
}
