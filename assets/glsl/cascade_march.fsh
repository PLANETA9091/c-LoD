// c-LoD v6.59 THE CASCADE — the six-ring march (functional rewrite; the
// v6.59.8y lod_terrain.fsh died with the sandbox recycle, the paint model
// here follows the living v6.39 march: camera basis from the rot channel,
// ray-march with entering-crossing bisection, flat per-cell normal, Voxy
// face shade + lightmap-ish tint, dimension fog, FADE at the data edge).
//
// Data source = the cascade atlas (clod_rh_b heights / clod_rc_b colors,
// 2048x256, 256 px per ring slice, alpha = the decoded mark):
//   ring r cell size = 16 * 4^r m, grid origin = GridInfo.xy (ring origin),
//   slice px = (r*256 + cx, cz), h0 = byte - 64 (the +-64 symmetric code).
// Ring selection by ray distance (cell ~ t): the coarsest ring whose
// half-span 2048*4^r >= t, then the nearest MARKED ring around it — the
// data frontier stays exact, unmarked rings read void.
//
// Water stays color-gated (byte 216 = the water flag), heights march
// geometry exactly like v6.21.0+ (the color word never gates geometry).

uniform sampler2D InSampler;        // minecraft:main color
uniform sampler2D InDepthSampler;   // minecraft:main depth
uniform sampler2D HeightSampler;    // clod_rh_b — the cascade height atlas
uniform sampler2D ColBufSampler;    // clod_rc_b — the cascade color atlas
uniform sampler2D RotSampler;       // clod_rot_a (camera + fog channels)

layout(std140) uniform ClodGrid {
    vec4 GridInfo;    // x = ring origin x, y = ring origin z, z/w = spare
};

layout(std140) uniform ClodRegion {
    vec4 RegionInfo;  // x = region blocks, y = disk cell, z = sea, w = max Y
};

layout(std140) uniform ClodParams {
    vec4 LodParams;   // x = lod near, y = max height, z = sea, w = min conf
};

layout(std140) uniform ClodPaint {
    vec4 PaintParams; // x = depth near, y = mesh paint enable
};

layout(std140) uniform ClodColors {
    vec4 ColLow;
    vec4 ColHigh;
    vec4 ColWater;
    vec4 VeilColor;
};

const float PI = 3.14159265358979;
const int MAX_STEPS = 320;
const int BISECT = 6;
const float FADE_START = 0.9;
const float CASCADE_CAP = 34816.0; // r2 half-span 32768 + one ring-0 span

out vec4 fragColor;

layout(location = 0) in vec2 texCoord;

float ring_cell(int r) {
    return 16.0 * exp2(2.0 * float(r));
}

// The coarsest ring whose half-span (2048 * 4^r) covers the distance t.
int ring_for(float t) {
    float x = max(t * (1.0 / 2048.0), 1.0);
    // r = ceil(log4(x)); log4(x) = log2(x)/2
    int r = int(ceil(log2(x) * 0.5));
    return clamp(r, 0, 5);
}

// Ring r's ground height at world xz; ok = the slice cell is marked.
float ground_ring(vec2 xz, int r, out bool water, out bool ok) {
    water = false;
    ok = false;
    float cell = ring_cell(r);
    ivec2 ci = ivec2(floor((xz - GridInfo.xy) / cell));
    if (any(lessThan(ci, ivec2(0))) || any(greaterThanEqual(ci, ivec2(256)))) {
        return -1.0e8;
    }
    ivec2 px = ivec2(r * 256 + ci.x, ci.y);
    vec4 h = texelFetch(HeightSampler, px, 0);
    if (h.a < 0.5) {
        return -1.0e8; // the ring cell never decoded — void to the marcher
    }
    vec4 c = texelFetch(ColBufSampler, px, 0);
    if (c.a >= 0.5) {
        float idx = floor(c.r * 255.0 + 0.5);
        water = idx > 215.5 && idx < 216.5;
    }
    ok = true;
    return h.r * 255.0 - 64.0;
}

// The cascade ground sample: the distance-chosen ring, then finer rings
// (near detail outranks coarse), then coarser ones (the wall keeps its
// shape past a fine-ring data hole).
float ground_cascade(vec2 xz, float t, out bool water, out bool ok) {
    water = false;
    ok = false;
    int r0 = ring_for(t);
    for (int r = r0; r >= 0; --r) {
        float g = ground_ring(xz, r, water, ok);
        if (ok) {
            return g;
        }
    }
    for (int r = r0 + 1; r < 6; ++r) {
        float g = ground_ring(xz, r, water, ok);
        if (ok) {
            return g;
        }
    }
    return -1.0e8;
}

float march_cascade(vec3 cam, vec3 dir, float t0, float tCap, float tanH) {
    // Rays leaving upward can never hit below the ceiling.
    if (dir.y > 0.0) {
        float toCeil = (RegionInfo.w + 8.0 - cam.y) / dir.y;
        if (toCeil < 0.0) {
            return -1.0;
        }
    }
    if (dir.y == 0.0) {
        dir.y = 1.0e-6;
    }
    float t = t0;
    float dt = max(ring_cell(ring_for(t)), t * (2.0 * tanH / max(ScreenSize.y, 1.0)));
    bool wflag;
    bool okflag;
    float prevT = t0;
    float prevDiff = ground_cascade(cam.xz + dir.xz * t0, t0, wflag, okflag)
                     - (cam.y + dir.y * t0);
    for (int i = 0; i < MAX_STEPS; ++i) {
        t = min(t + dt, tCap);
        int ring = ring_for(t);
        dt = max(ring_cell(ring), t * (2.0 * tanH / max(ScreenSize.y, 1.0)));
        vec2 xz = cam.xz + dir.xz * t;
        float above = ground_cascade(xz, t, wflag, okflag) - (cam.y + dir.y * t);
        if (above > 0.0 && prevDiff <= 0.0) {
            float lo = prevT;
            float hi = t;
            for (int b = 0; b < BISECT; ++b) {
                float mid = (lo + hi) * 0.5;
                bool mw;
                bool mo;
                float g = ground_cascade(cam.xz + dir.xz * mid, mid, mw, mo);
                if (g - (cam.y + dir.y * mid) > 0.0) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            return (lo + hi) * 0.5;
        }
        prevT = t;
        prevDiff = above;
        if (t >= tCap) {
            break;
        }
    }
    return -1.0;
}

void main() {
    vec4 scene = texture(InSampler, texCoord);
    float depth = texture(InDepthSampler, texCoord).r;

    // The rot channel carries the camera + confidence state.
    vec4 rot = texelFetch(RotSampler, ivec2(0, 0), 0);
    float conf = rot.b;
    float minConf = LodParams.w;
    if (conf < minConf) {
        fragColor = scene;
        return;
    }

    float az = rot.r * 2.0 * PI;
    float el = rot.g * PI - PI * 0.5;
    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));
    float fov = 5.0 + rot.a * 115.0;
    float tanH = tan(radians(fov) * 0.5);
    float aspect = ScreenSize.x / max(ScreenSize.y, 1.0);
    vec2 ndc = texCoord * 2.0 - 1.0;
    vec3 dir = normalize(F + R * (ndc.x * tanH * aspect) + U * (ndc.y * tanH));
    vec3 cam = vec3(CameraBlockPos) + CameraOffset;

    float near = LodParams.x;
    float tVan = -1.0;
    if (depth > 1.0e-6) {
        if (PaintParams.y < 0.5) {
            fragColor = scene; // real chunks always win
            return;
        }
        float eyeZ = PaintParams.x / depth;
        tVan = eyeZ / max(dot(dir, F), 1.0e-4);
    }

    float tHit;
    if (tVan > 0.0) {
        tHit = march_cascade(cam, dir, 1.0, min(CASCADE_CAP, tVan * 1.15 + 4.0), tanH);
    } else {
        tHit = march_cascade(cam, dir, near, CASCADE_CAP, tanH);
    }
    if (tHit < 0.0) {
        fragColor = scene;
        return;
    }

    vec3 hitPos = cam + dir * tHit;
    int hitRing = ring_for(tHit);
    float cell = ring_cell(hitRing);

    bool isWater = false;
    bool okC = false;
    bool dum = false;
    float hC = ground_cascade(hitPos.xz, tHit, isWater, okC);
    if (!okC) {
        fragColor = scene;
        return;
    }
    float hL = ground_cascade(hitPos.xz - vec2(cell, 0.0), tHit, dum, okC);
    float hR = ground_cascade(hitPos.xz + vec2(cell, 0.0), tHit, dum, okC);
    float hD = ground_cascade(hitPos.xz - vec2(0.0, cell), tHit, dum, okC);
    float hU = ground_cascade(hitPos.xz + vec2(0.0, cell), tHit, dum, okC);
    hL = hL < -1.0e7 ? hC : hL;
    hR = hR < -1.0e7 ? hC : hR;
    hD = hD < -1.0e7 ? hC : hD;
    hU = hU < -1.0e7 ? hC : hU;
    vec3 nrm = normalize(vec3(hL - hR, 2.0 * cell, hD - hU));

    // ---- color: the 6x6x6 lattice at the hit cell's ring slice -----------
    ivec2 ci = ivec2(floor((hitPos.xz - GridInfo.xy) / cell));
    ivec2 px = ivec2(hitRing * 256 + ci.x, ci.y);
    vec4 c = texelFetch(ColBufSampler, px, 0);
    bool colWater = false;
    bool colorless = c.a < 0.5;
    vec3 col = vec3(-1.0);
    if (!colorless) {
        float idx = floor(c.r * 255.0 + 0.5);
        colWater = idx > 215.5;
        if (!colWater) {
            vec3 lat = vec3(mod(idx, 6.0), mod(floor(idx / 6.0), 6.0), floor(idx / 36.0));
            col = lat * (51.0 / 255.0);
        }
    }

    // ---- live fog state (camera channel group B) --------------------------
    vec4 fogCol = texelFetch(RotSampler, ivec2(1, 0), 0);
    vec4 fogRng = texelFetch(RotSampler, ivec2(0, 1), 0);
    bool fogLive = fogRng.a > 0.5 && fogCol.a > 0.004;
    float vanRD = fogRng.b * 255.0 * 16.0;
    vec3 fogTint = fogLive ? fogCol.rgb : VeilColor.rgb;
    float fogOpacity = fogLive ? fogCol.a : 1.0;

    float sea = RegionInfo.z;
    float maxY = RegionInfo.w;
    float hn = clamp((hitPos.y - sea) / max(maxY - sea, 1.0), 0.0, 1.0);
    vec3 fallback = (isWater || colWater)
        ? ColWater.rgb
        : mix(ColLow.rgb, ColHigh.rgb, smoothstep(0.15, 0.9, hn));
    vec3 albedo = (colWater || colorless) ? fallback : col;
    if (tVan > 0.0 && colorless) {
        fragColor = scene; // height-only cells never repaint real chunks
        return;
    }

    vec3 an = vec3(abs(nrm.x), max(nrm.y, 0.0), abs(nrm.z));
    float wsum = max(an.x + an.y + an.z, 1.0e-4);
    float face = (an.x * 0.6 + an.y * 1.0 + an.z * 0.8) / wsum;
    if (isWater) {
        face = 1.0;
    }
    // Day light from the live fog tint's luminance (noon ~1, night ~0.25) —
    // the compact march carries no lightmap reconstruction.
    float day = clamp(dot(fogTint, vec3(0.33)), 0.25, 1.0);
    vec3 lit = albedo * face * day;

    float fog = fogLive ? clamp(tHit * (1.0 / 1024.0), 0.0, 1.0) * fogOpacity : 0.0;
    lit = mix(lit, fogTint, fog);

    // FADE over the last 10% toward the marched cap.
    float lenXZ = tHit * length(dir.xz);
    float fadeOut = clamp((lenXZ - FADE_START * CASCADE_CAP)
                          / max((1.0 - FADE_START) * CASCADE_CAP, 1.0), 0.0, 1.0);

    // Near gate: the wall paints only beyond the vanilla chunk bubble on
    // the sky path; the depth-tested vanilla path bypasses it.
    float rd = fogLive ? max(vanRD, 64.0) : 128.0;
    float gate = (tVan > 0.0) ? 1.0 : smoothstep(rd * 0.92, rd * 1.10, tHit);
    float fade = smoothstep(minConf, minConf + 0.15, conf);

    vec3 outCol = mix(mix(scene.rgb, lit, fade * gate), scene.rgb, fadeOut);
    fragColor = vec4(outCol, scene.a);
}
