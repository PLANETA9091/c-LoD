#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD pass 4: the actual level-of-distant-detail renderer.
//
// For every pixel that shows sky (device depth ~ 1) we march a view ray
// against the heightfield atlas shipped in the server resource pack and
// shade the hit. The ray is built from the camera basis delivered by the
// CAMERA CHANNEL (v4.1): the sky shader encodes yaw/pitch/fov into the
// alpha of the leftmost columns, col_scan + rot_solve decode it into the
// clod_rot target. Works at any time of day and any camera orientation —
// no sun in view required. Camera position comes from the Globals UBO
// (CameraBlockPos + CameraOffset), bound to every post pass.
//
// Non-sky pixels pass through untouched: real chunks always win.

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

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;
// 26.3 device depth: d = near/eyeZ (infinite far) — sky == 0.0, any real
// geometry reads positive, so 1e-6 keeps terrain out to ~50 km classified.
const float SKY_DEPTH = 1.0e-6;
const int MAX_STEPS = 460;
const int BISECT = 7;

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
    vec2 uv = (xz - origin) / size;
    uv = clamp(uv, vec2(0.0015), vec2(0.9985));
    return atlas_height(uv);
}

bool world_water(vec2 xz, vec2 origin, float size) {
    vec2 uv = (xz - origin) / size;
    uv = clamp(uv, vec2(0.0015), vec2(0.9985));
    return atlas_water(uv);
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
    //   green     = the ray hit the heightfield (this pixel WOULD get LOD)
    //   orange    = sky, the march ran but never hit
    //   purple    = the ray leaves the heightfield from above (never hits)
    //   bar (top-left, 240x24 px): red width = tracker confidence 0..1
    if (DebugParams.x > 3.5) {
        if (gl_FragCoord.x < 240.0 && gl_FragCoord.y > OutSize.y - 24.0) {
            vec3 bar = gl_FragCoord.x < conf * 240.0 ? vec3(1.0, 0.0, 0.0) : vec3(0.0);
            fragColor = vec4(bar, 1.0);
            return;
        }
        forceLod = 1.0;
    }

    // Debug view: red channel = tracker confidence over sky pixels.
    //   black sky  -> the tracker never locked (sun never detected)
    //   red sky    -> tracker locked (conf == 1)
    //   normal sky -> this pass never classified the pixel as sky
    if (DebugParams.x > 0.5 && DebugParams.x < 1.5) {
        fragColor = vec4(conf, 0.0, 0.0, 1.0);
        return;
    }

    // Debug mode 2 splits the screen:
    //   left  half -> the real ray march with the confidence gate FORCED open
    //   right half -> the atlas itself drawn as a height map (uv = screen uv)
    // so one look tells us whether the march or the atlas data is at fault.
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
    // A = fov byte (new in v4.1): the REAL vertical fov measured by the sky
    // shader from ProjMat[1][1] — spyglass zoom and fov options just work.
    float az = rot.r * 6.28318530718;
    float el = rot.g * PI - PI * 0.5;

    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));

    // v4.1 camera channel: fov is measured from the client's ProjMat[1][1]
    // (spyglass zoom and the fov slider just work). Fall back to the pack
    // value until the first valid payload row arrives.
    float fov = rot.a > 0.002 ? 5.0 + rot.a * 115.0 : FogParams.z;
    float tanH = tan(radians(fov) * 0.5);
    float aspect = OutSize.x / max(OutSize.y, 1.0);

    vec2 ndc = texCoord * 2.0 - 1.0;
    vec3 dir = normalize(F + R * (ndc.x * tanH * aspect) + U * (ndc.y * tanH));

    // 26.3 Globals: CameraBlockPos = floor(camPos), CameraOffset = floor - camPos
    // (see GlobalSettingsUniform.update), so the camera is floor - offset.
    vec3 cam = vec3(CameraBlockPos) - CameraOffset;

    // Clouds are opaque geometry above the highest atlas cell. Without this the
    // vanilla cloud layer (y=192, far beyond the LOD band) punches holes into
    // distant mountains, because the LOD pass skips every non-sky pixel.
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

    // The atlas is a square around `origin`; stop the ray when it leaves.
    // AtlasInfo.xy is the MIN corner, z the full side: the box is origin..origin+size.
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

    // March from the camera. The pass only overwrites SKY pixels, so the near
    // field is free; starting at 0 also guarantees the first sample is above
    // the heightfield (the camera always is), which the crossing test needs.
    float t = 0.0;
    float dt = max(cell * 0.75, t * 0.008);
    float tHit = -1.0;
    float prevT = t;
    float prevDiff = 1.0e9;
    // Voxy draws real quads, so the visible face decides both the texture and
    // the vanilla face shade. We reconstruct the face from the march: entering a
    // column whose top is above the ray means we are looking at its SIDE (cliff)
    // face, otherwise the ray crossed the surface downwards -> TOP face.
    bool sideHit = false;
    float faceShade = 1.0;

    vec2 xz = cam.xz + dir.xz * t;
    vec2 cellPrev = floor((xz - origin) / cell);
    float hPrev = world_water(xz, origin, size) ? sea : world_height(xz, origin, size);
    float above = hPrev - (cam.y + dir.y * t);

    for (int i = 0; i < MAX_STEPS; ++i) {
        prevT = t;
        prevDiff = above;
        t = min(t + dt, size);
        // Cell-sized steps near the camera, growing with distance so the whole
        // atlas stays reachable at 1 block per cell.
        dt = max(cell * 0.75, t * 0.008);

        xz = cam.xz + dir.xz * t;
        // Left the atlas? No LOD here.
        if (any(lessThan(xz, minW)) || any(greaterThan(xz, maxW))) {
            break;
        }
        float ground = world_water(xz, origin, size) ? sea : world_height(xz, origin, size);
        vec2 cellNow = floor((xz - origin) / cell);
        above = ground - (cam.y + dir.y * t);

        // `above` = ground - rayY: POSITIVE means the ray is below the
        // heightfield. A hit is the crossing from outside (-) to inside (+).
        if (above >= 0.0 && prevDiff < 0.0) {
            sideHit = any(notEqual(cellNow, cellPrev)) && (ground > hPrev + 0.5);
            if (sideHit) {
                // Vanilla directional face shade: N/S = 0.8, E/W = 0.6.
                faceShade = abs(cellNow.x - cellPrev.x) > 0.5 ? 0.6 : 0.8;
            }
            // Bisection between prevT and t.
            float lo = prevT;
            float hi = t;
            for (int b = 0; b < BISECT; ++b) {
                float mid = (lo + hi) * 0.5;
                vec2 mxz = cam.xz + dir.xz * mid;
                float g = world_water(mxz, origin, size) ? sea : world_height(mxz, origin, size);
                if (g - (cam.y + dir.y * mid) >= 0.0) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            tHit = (lo + hi) * 0.5;
            break;
        }
        cellPrev = cellNow;
        hPrev = ground;
        if (t >= size) {
            break;
        }
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

    // ---------------------------------------------------------------
    // Surface colour, Voxy model: the block's real average texture colour
    // (baked from the client jar's blockstate -> model -> texture graph, biome
    // tinted at scan time) picked per face, times vanilla's directional face
    // shade, times the daylight light-map factor. Voxy samples the block model
    // atlas with textureGrad, which at any real distance is exactly this
    // average -- so the colours match instead of being a hand-picked palette.
    // ---------------------------------------------------------------
    vec2 auv = atlas_uv(hitPos.xz, origin, size);
    vec3 topCol = texture(ColorSampler, auv).rgb;
    vec3 sideCol = texture(
        SideSampler,
        vec2((atlas_side(auv) * 255.0 + 0.5) / 256.0, 0.5)
    ).rgb;
    vec3 albedo = sideHit ? sideCol * faceShade : topCol;

    // Light reference from the SKY IN THIS FRAME (see v4.1 notes above): the
    // world clock and the rendered sky can disagree, and a wrong day factor made
    // the LOD look lit by moonlight next to sunlit chunks.
    float skyLum = dot(scene.rgb, vec3(0.299, 0.587, 0.114));
    float dayF = smoothstep(0.08, 0.45, skyLum);
    // The fog target is the actual sky behind the pixel, so the far edge of the
    // LOD band dissolves into the horizon instead of a config colour that can
    // only ever approximate it. VeilColor only tints it.
    vec3 veilCol = mix(VeilColor.rgb * mix(0.35, 1.0, dayF), scene.rgb, 0.82);

    // Vanilla lightmap substitute: full daylight is 1.0 (the baked colours are
    // already the texture colours, which is what a sunlit block shows), night
    // keeps a small ambient so the silhouette does not vanish.
    float lightMul = mix(0.14, 1.0, dayF);
    vec3 lit = albedo * lightMul;

    // Snow: per-block colours cannot see state variants (snowy=true shares the
    // material id with the plain block), so high terrain is whitened by height.
    float hn = clamp((hitPos.y - sea) / max(maxY - sea, 1.0), 0.0, 1.0);
    float snow = smoothstep(VeilColor.a * 0.55 + 0.35, VeilColor.a * 0.55 + 0.55, hn);
    lit = mix(lit, vec3(0.93, 0.95, 0.97) * lightMul, snow * 0.9);

    // Distance fog into the day-graded veil color: merges LOD into the sky.
    float fog = smoothstep(FogParams.x, FogParams.y, tHit);
    lit = mix(lit, veilCol, fog);

    fragColor = vec4(mix(scene.rgb, lit, fade), scene.a);
}
