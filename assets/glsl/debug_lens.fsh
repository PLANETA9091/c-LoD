#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD v6.7 CAPTURE LENS — CI-only client-side observability pass.
//
// The v6.6 visual runs exposed the frontier: when the on-screen probe says
// "red=17", nobody can tell WHY — did the server stream the right words,
// did the dots render, did the capture COMMIT them? The server belief is
// already observable (/clod-cells); this pass makes the CLIENT belief
// observable: it draws the persistent capture targets themselves onto the
// screen, enlarged, as two panels on the right edge:
//
//   TOP panel (color):    clod_c_b per cell — the 6x6x6 lattice decode
//                         (RED_CONCRETE idx 2 -> exactly (102,0,0) px),
//                         water 216 -> blue, idx 0 -> black, never-marked
//                         cells -> dark gray (0.12).
//   BOTTOM panel (height): clod_h_b per cell — the committed byte as gray;
//                         never-marked cells -> dark red tint.
//
// A screenshot of this overlay IS the client's decode state: a lost single
// slot-dot (the v6.6.2 fragility) shows up as black/navy in the color panel
// while the height panel still shows the column — the shape/color split
// becomes visible per cell, per probe, no fog or vantage in the way.
//
// The pass is appended AFTER clod_blit by packgen when `debug_lens=true`
// (separate chain template: the production pack never ships this pass and
// keeps its content-fixed SHA). Panel layout assumes the bench resolution
// (1280x720 — run_client.sh launches --width 1280 --height 720 since v6.9.0;
// the 640x360 era measured the letterbox instead of the panels) and reads
// texCoord bottom-up like every other post pass — the harness crops the full
// right strip.
//
// v6.9.0 STAGE EYES: three 24x8 px strips sit just under the top frame edge
// of the color panel (GL y [ya+panel-10, ya+panel-2), x from x0+2, each 24
// wide + 2 gap). The harness decodes them relative to the located green
// frame and the verdict names the blind stage:
//   ROT strip    (x0+2)  : green  (0,220,0)  = camera conf >= 0.25 (live)
//                          magenta (200,0,200) = decoded once, conf decayed
//                          yellow (255,255,0)  = camera payload never decoded
//   SCAN strip   (x0+28) : blue (0,0,v), v = min(hits,15)*17 — payload rows
//                          found by col_scan in main this frame (both groups)
//   CARRIER strip(x0+54) : (v,v,0), v = min(grid_hits,240) — carrier pixels
//                          present in main on a 32x18 sample grid of clod_mask
// Colors are chosen to never collide with the cell signatures below
// (red 102/0/0, black, water 70/130/240, dark 31/31/31, frame green 0/153/0).

uniform sampler2D InSampler;      // clod_swap (v6.8.2: the lens REPLACED the
                                  // blit pass — reading minecraft:main while
                                  // writing minecraft:main was a framebuffer
                                  // feedback loop, GL undefined behavior)
uniform sampler2D HeightSampler;  // clod_h_b — committed height bytes (1 px = 1 cell)
uniform sampler2D ColBufSampler;  // clod_c_b — committed color bytes (1 px = 1 cell)
uniform sampler2D ScanSampler;    // clod_scan — col_scan row reports (v6.9.0)
uniform sampler2D RotSampler;     // clod_rot_a — the persistent 2x2 camera/fog
                                  // state (v6.9.0)
uniform sampler2D MaskSampler;    // clod_mask — this frame's carrier mask
                                  // (v6.9.0)

// x,y = window min cell (cells), z = window side in cells, w = px per cell.
layout(std140) uniform ClodLens {
    vec4 LensInfo;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

// The 6x6x6 lattice decode (matches lod_terrain.fsh color_at): idx -> rgb.
// Water (216) renders as a fixed blue; idx 0 stays black.
vec3 color_lattice(float idxF) {
    int idx = int(floor(idxF + 0.5));
    if (idx > 215) {
        return vec3(70.0, 130.0, 240.0) / 255.0;
    }
    ivec3 l = ivec3(idx % 6, (idx / 6) % 6, idx / 36);
    return vec3(l) * (51.0 / 255.0);
}

// ---- v6.9.0 stage eyes -----------------------------------------------------
// Payload rows found in main this frame (clod_scan, both group rows).
int scan_hits() {
    int hits = 0;
    for (int j = 0; j < 640; ++j) {
        if (texelFetch(ScanSampler, ivec2(j, 0), 0).r > 0.5) { ++hits; }
        if (texelFetch(ScanSampler, ivec2(j, 1), 0).r > 0.5) { ++hits; }
    }
    return hits;
}

// Carrier pixels present in main this frame (coarse 32x18 grid of clod_mask).
int carrier_hits() {
    ivec2 ms = textureSize(MaskSampler, 0);
    int hits = 0;
    for (int j = 0; j < 32; ++j) {
        for (int i = 0; i < 18; ++i) {
            ivec2 p = ivec2((j * 2 + 1) * ms.x / 64, (i * 2 + 1) * ms.y / 36);
            if (texelFetch(MaskSampler, p, 0).r > 0.5) { ++hits; }
        }
    }
    return hits;
}

// Draws the three strips; true when the fragment was consumed.
bool stage_eyes(vec2 px, float x0, float ya, float panel) {
    float yTop = ya + panel - 10.0;
    if (px.y < yTop || px.y >= yTop + 8.0) {
        return false;
    }
    float v;
    if (px.x >= x0 + 2.0 && px.x < x0 + 26.0) {
        // ROT strip: the persistent camera pixel (0,0).
        vec4 rot = texelFetch(RotSampler, ivec2(0, 0), 0);
        if (rot.a <= 0.001) {
            fragColor = vec4(1.0, 1.0, 0.0, 1.0);          // never decoded
        } else if (rot.b >= 0.25) {
            fragColor = vec4(0.0, 220.0 / 255.0, 0.0, 1.0); // live camera
        } else {
            fragColor = vec4(200.0 / 255.0, 0.0, 200.0 / 255.0, 1.0); // decayed
        }
        return true;
    }
    if (px.x >= x0 + 28.0 && px.x < x0 + 52.0) {
        v = float(min(scan_hits(), 15)) * 17.0 / 255.0;
        fragColor = vec4(0.0, 0.0, v, 1.0);
        return true;
    }
    if (px.x >= x0 + 54.0 && px.x < x0 + 78.0) {
        v = float(min(carrier_hits(), 240)) / 255.0;
        fragColor = vec4(v, v, 0.0, 1.0);
        return true;
    }
    return false;
}

void main() {
    vec2 px = texCoord * ScreenSize;
    float win = LensInfo.z;
    float ppc = LensInfo.w;
    float panel = win * ppc;
    float x0 = ScreenSize.x - panel - 8.0;
    vec2 minCell = vec2(LensInfo.x, LensInfo.y);

    // Panel A: the client's committed COLOR map.
    float ya = 8.0;
    if (px.x >= x0 && px.x < x0 + panel && px.y >= ya && px.y < ya + panel) {
        if (stage_eyes(px, x0, ya, panel)) {
            return;
        }
        vec2 cell = minCell + floor((px - vec2(x0, ya)) / ppc);
        vec4 c = texelFetch(ColBufSampler, ivec2(cell), 0);
        vec3 col = vec3(0.12);
        if (c.a >= 0.5) {
            col = color_lattice(c.r * 255.0);
        }
        vec2 f = px - vec2(x0, ya);
        if (f.x < 1.0 || f.y < 1.0 || f.x >= panel - 1.0 || f.y >= panel - 1.0) {
            col = vec3(0.0, 0.6, 0.0);
        }
        fragColor = vec4(col, 1.0);
        return;
    }

    // Panel B: the client's committed HEIGHT map (byte as gray).
    float yb = ya + panel + 8.0;
    if (px.x >= x0 && px.x < x0 + panel && px.y >= yb && px.y < yb + panel) {
        vec2 cell = minCell + floor((px - vec2(x0, yb)) / ppc);
        vec4 h = texelFetch(HeightSampler, ivec2(cell), 0);
        vec3 col = vec3(0.25, 0.02, 0.02);
        if (h.a >= 0.5) {
            col = vec3(h.r);
        }
        vec2 f = px - vec2(x0, yb);
        if (f.x < 1.0 || f.y < 1.0 || f.x >= panel - 1.0 || f.y >= panel - 1.0) {
            col = vec3(0.0, 0.6, 0.0);
        }
        fragColor = vec4(col, 1.0);
        return;
    }

    fragColor = texture(InSampler, texCoord);
}
