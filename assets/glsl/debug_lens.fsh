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
// (1280x720; the bottom panel ends at y=656) and reads texCoord bottom-up
// like every other post pass — the harness crops the full right strip.

uniform sampler2D InSampler;      // clod_swap (v6.8.2: the lens REPLACED the
                                  // blit pass — reading minecraft:main while
                                  // writing minecraft:main was a framebuffer
                                  // feedback loop, GL undefined behavior)
uniform sampler2D HeightSampler;  // clod_h_b — committed height bytes (1 px = 1 cell)
uniform sampler2D ColBufSampler;  // clod_c_b — committed color bytes (1 px = 1 cell)

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
