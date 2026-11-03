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
// frame and the verdict names the blind stage. Colors never collide with
// the cell signatures below (red 102/0/0, black, water 70/130/240, dark
// 31/31/31, frame green 0/153/0).
//
// v6.9.2 STAGE EYES v2 (run #231: rot=ok scan=15 lens_zero=1 with the
// carrier column plainly visible on screen — the relative strobe fires on
// 399/400 of the actual dot pixels measured on the artifact, yet the capture
// committed NOTHING — so the residual suspects are the projection INPUTS,
// not the hue test). FOUR 24x8 strips, each a BUCKET PRINTER: one lit
// (0,220,0) pixel per row marks the value's bucket, the rest stays black:
//   CAM strip  (x0+2)  : row0 = camera x bucket over the atlas (24 buckets,
//                        cell space 0..256 via ClodGrid origin + ClodStream
//                        cell size — the Globals camera the CAPTURE sees),
//                        row1 = camera z bucket. The harness knows the
//                        vantage: a wrong bucket = the Globals camera is not
//                        what the capture assumes (THE smoking gun).
//   ROT strip  (x0+28) : row0 = az bucket (24 over 2pi) from clod_rot_a,
//                        row1 = el bucket (8 over pi) — the decoded camera
//                        direction, comparable to the known vantage.
//   HITS strip (x0+54) : brightness = min(grid_hits,240) — clod_h_b.g>0.5
//                        cells on a 32x32 grid: the strobe FIRED this frame.
//   MASK strip (x0+80) : brightness = min(grid_hits,240) — clod_mask marks
//                        on a 64x36 grid (v6.9.0's 32x18 could miss 2-3px
//                        discs; the dense carrier column needs this).

uniform sampler2D InSampler;      // clod_swap (v6.8.2: the lens REPLACED the
                                  // blit pass — reading minecraft:main while
                                  // writing minecraft:main was a framebuffer
                                  // feedback loop, GL undefined behavior)
uniform sampler2D HeightSampler;  // clod_h_b — committed height bytes (1 px =
                                  // 1 cell; v6.9.2 also reads the .g strobe
                                  // flag for the HITS strip)
uniform sampler2D ColBufSampler;  // clod_c_b — committed color bytes (1 px = 1 cell)
uniform sampler2D ScanSampler;    // clod_scan — col_scan row reports (v6.9.0)
uniform sampler2D RotSampler;     // clod_rot_a — the persistent 2x2 camera/fog
                                  // state (v6.9.0)
uniform sampler2D MaskSampler;    // clod_mask — this frame's carrier mask
                                  // (v6.9.0)

// x = origin_x, y = origin_z, z = HIGH layer y, w = LOW layer y (v6.9.2:
// also the CAM strip's world anchor — the capture probes the SAME origin)
layout(std140) uniform ClodGrid {
    vec4 GridInfo;
};

// x = cell size, y = sub quantum, z/w = mode words (v6.9.2: .x sizes the
// CAM buckets in cells)
layout(std140) uniform ClodStream {
    vec4 StreamInfo;
};

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

// ---- v6.9.2 stage eyes v2 --------------------------------------------------

// Cells whose strobe FIRED this frame on the LO PLANE (clod_c_b.g — the
// color/marker words). v6.9.2 read clod_h_b (the HI plane) which is
// GEOMETRICALLY OFF-SCREEN from the bench vantage (the HI dots at 75+ blocks
// sit 43 deg below the horizon vs the 35-deg half-FOV) — the v6.9.2
// strobe_hits=0 was blind by construction. The LO plane is the one that
// carries the marker.
int strobe_hits() {
    int hits = 0;
    for (int j = 0; j < 32; ++j) {
        for (int i = 0; i < 32; ++i) {
            if (texelFetch(ColBufSampler, ivec2(j * 8 + 4, i * 8 + 4), 0).g > 0.5) {
                ++hits;
            }
        }
    }
    return hits;
}

// Eraser mask marks on a DENSE 128x72 grid (9216 samples — v6.9.2's 64x36
// expected ~3 hits on the measured column and read 0 at P(0)~3%; the dense
// grid expects ~15 on the same dots).
int carrier_hits() {
    ivec2 ms = textureSize(MaskSampler, 0);
    int hits = 0;
    for (int j = 0; j < 128; ++j) {
        for (int i = 0; i < 72; ++i) {
            ivec2 p = ivec2((j * 2 + 1) * ms.x / 256, (i * 2 + 1) * ms.y / 144);
            if (texelFetch(MaskSampler, p, 0).r > 0.5) {
                ++hits;
            }
        }
    }
    return hits;
}

// Draws the four strips; true when the fragment was consumed. Each strip is
// 24 px wide + 2 px gap; a lit (0,220,0) pixel marks the value's bucket.
bool stage_eyes(vec2 px, float x0, float ya, float panel) {
    float yTop = ya + panel - 10.0;
    if (px.y < yTop || px.y >= yTop + 8.0) {
        return false;
    }
    int col = int(px.x - x0);
    int row = int(px.y - yTop);
    vec3 lit = vec3(0.0, 220.0 / 255.0, 0.0);

    // CAM strip [2,26): row0 = camera x bucket, row1 = camera z bucket, in
    // CELL space over the atlas (24 buckets x ~10.67 cells).
    if (col >= 2 && col < 26) {
        vec3 cam = vec3(CameraBlockPos) + CameraOffset;
        float camCx = (cam.x - GridInfo.x) / max(StreamInfo.x, 0.001);
        float camCz = (cam.z - GridInfo.y) / max(StreamInfo.x, 0.001);
        int b;
        if (row == 0) {
            b = int(clamp(camCx / 256.0, 0.0, 0.999) * 24.0);
        } else if (row == 1) {
            b = int(clamp(camCz / 256.0, 0.0, 0.999) * 24.0);
        } else {
            fragColor = vec4(vec3(0.0), 1.0);
            return true;
        }
        fragColor = vec4((col - 2 == b) ? lit : vec3(0.0), 1.0);
        return true;
    }

    // ROT strip [28,52): row0 = az bucket (24 over 2pi), row1 = el bucket
    // (8 over pi, drawn across the same 24 px — the bucket is col-28 scaled).
    if (col >= 28 && col < 52) {
        vec4 rot = texelFetch(RotSampler, ivec2(0, 0), 0);
        if (rot.a <= 0.001) {
            // never decoded: light the FULL strip yellow on row 0
            fragColor = vec4((row == 0) ? vec3(1.0, 1.0, 0.0) : vec3(0.0), 1.0);
            return true;
        }
        int b;
        if (row == 0) {
            b = int(clamp(rot.r, 0.0, 0.999) * 24.0);
        } else if (row == 1) {
            b = int(clamp(rot.g, 0.0, 0.999) * 8.0);
            if (b >= 8) { b = 7; }
            if (col - 28 != b * 3 && col - 28 != b * 3 + 1 && col - 28 != b * 3 + 2) {
                fragColor = vec4(vec3(0.0), 1.0);
                return true;
            }
            fragColor = vec4(lit, 1.0);
            return true;
        } else {
            fragColor = vec4(vec3(0.0), 1.0);
            return true;
        }
        fragColor = vec4((col - 28 == b) ? lit : vec3(0.0), 1.0);
        return true;
    }

    // HITS strip [54,78): the LO-plane strobe-fired cell count this frame
    // (x8 scale, cap 240).
    if (col >= 54 && col < 78) {
        float v = float(min(strobe_hits() * 8, 240)) / 255.0;
        fragColor = vec4(v, v, 0.0, 1.0);
        return true;
    }

    // MASK strip [80,104): the eraser-mask mark count this frame (x8 scale,
    // cap 240).
    if (col >= 80 && col < 104) {
        float v = float(min(carrier_hits() * 8, 240)) / 255.0;
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
