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

// x = fog start (legacy), y = fog end (blocks) == LOD draw distance, z = static fov fallback
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
// A ray crosses at most (cells.x + cells.y) grid boundaries; 256+256 for a
// 1024-block window at cell 4, plus margin.
const int MAX_STEPS = 560;

// DH far-fog defaults (Config.Client.Advanced.Graphics.Fog): EXPONENTIAL_SQUARED,
// farFogStart 0.4, farFogEnd 1.0 (x lodDrawDistance), density 2.5, sky colour.
const float FOG_START_FRAC = 0.4;
const float FOG_DENSITY = 2.5;

// Vanilla directional face shade (ClientLevelWrapper.getShade):
// up 1.0, down 0.5, north/south 0.8, west/east 0.6.
const float SHADE_NS = 0.8;
const float SHADE_EW = 0.6;

// Height (block Y) stored in the atlas: (y + 64) << 6, big-endian RG.
float atlas_height(vec2 uv) {
    vec4 a = texture(AtlasSampler, uv);
    float y16 = a.r * 65280.0 + a.g * 255.0; // r*255*256 + g*255
    return y16 / 64.0 - 64.0;
}

bool atlas_water(vec2 uv) {
    return texture(AtlasSampler, uv).b > 0.5;
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
    float sea = LodParams.z;

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

    // hCur = surface height of the cell the ray is currently in (water -> sea).
    float hCur = world_water(enterXz, origin, size) ? sea : world_height(enterXz, origin, size);

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
        // inside the cell's footprint".
        if (dir.y < 0.0) {
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
        float rayY = cam.y + dir.y * tCur;
        float hNew = world_water(nextXz, origin, size) ? sea : world_height(nextXz, origin, size);
        if (hNew > rayY) {
            hitT = tCur;
            sideHit = true;
            hitAxisX = axisX;
            hHitOld = hCur;
            hHitNew = hNew;
            break;
        }
        hCur = hNew;
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
    bool hitWater = world_water(hitPos.xz, origin, size);

    // ---------------------------------------------------------------
    // DH colour model: one flat colour per column box (the block's baked
    // average, biome tinted), shaded per face with vanilla's directional
    // factors. Side walls exist only on the exposed span and grass material
    // fades to dirt down that span (FADE_TO_DIRT default). Our atlas keeps
    // top and side colours per cell, so the fade is mix(side, top, spanT).
    // ---------------------------------------------------------------
    vec3 topCol = texture(ColorSampler, auv).rgb;
    vec3 sideCol = texture(
        SideSampler,
        vec2((atlas_side(auv) * 255.0 + 0.5) / 256.0, 0.5)
    ).rgb;

    vec3 albedo;
    if (sideHit) {
        // Directional shade from the crossed grid line: X boundary -> E/W wall.
        float shade = hitAxisX ? SHADE_EW : SHADE_NS;
        if (hitWater) {
            albedo = ColWater.rgb * shade; // DH: water box walls stay water
        } else {
            float span = max(hHitNew - hHitOld, 1.0e-3);
            float spanT = clamp((hHitNew - (cam.y + dir.y * hitT)) / span, 0.0, 1.0);
            // top of the span -> top colour, bottom -> side colour (dirt fade)
            albedo = mix(sideCol, topCol, spanT) * shade;
        }
    } else {
        albedo = hitWater ? ColWater.rgb : topCol; // top faces: shade 1.0
    }

    // Light: the sky in THIS frame drives the day factor (the world clock and
    // the rendered sky can disagree). DH samples the vanilla lightmap which
    // follows the real sky — this is our lightmap substitute; moonlight keeps
    // a faint blue cast like the vanilla night lightmap.
    float skyLum = dot(scene.rgb, vec3(0.299, 0.587, 0.114));
    float dayF = smoothstep(0.08, 0.45, skyLum);
    float lightMul = mix(0.22, 1.0, dayF);
    vec3 nightTint = vec3(0.84, 0.89, 1.08);
    vec3 lit = albedo * lightMul * mix(vec3(1.0), nightTint, 1.0 - dayF);

    // Snow: per-block colours cannot see state variants (snowy=true shares the
    // material id with the plain block), so high terrain is whitened by height.
    // DH's snow comes from actual snow blocks — a white TOP surface with the
    // regular box sides below — so only top faces whiten here; whitening side
    // walls turned every high column into a striped white pillar.
    if (!sideHit) {
        float hn = clamp((hitPos.y - sea) / max(maxY - sea, 1.0), 0.0, 1.0);
        float snow = smoothstep(VeilColor.a * 0.55 + 0.42, VeilColor.a * 0.55 + 0.62, hn);
        lit = mix(lit, vec3(0.93, 0.95, 0.97) * lightMul, snow * 0.9);
    }

    // DH far fog: exponential-squared in the sky colour, 0.4..1.0 x LOD draw
    // distance, density 2.5 (GlDhFogShader / Config defaults). The fog target
    // is the actual sky behind the pixel, so the band edge dissolves into the
    // horizon exactly like DH.
    float fogFar = max(FogParams.y, 1.0);
    float fogStart = FOG_START_FRAC * fogFar;
    float fogLen = max(fogFar - fogStart, 1.0);
    float fx = max((hitT - fogStart) / fogLen, 0.0) * FOG_DENSITY;
    float fog = clamp(1.0 - exp(-fx * fx), 0.0, 1.0);
    lit = mix(lit, scene.rgb, fog);

    fragColor = vec4(mix(scene.rgb, lit, fade), scene.a);
}
