#version 330
#extension GL_ARB_separate_shader_objects : require

#include <minecraft:fog.glsl>
#include <minecraft:dynamictransforms.glsl>
#include <minecraft:projection.glsl>

layout(location = 0) in float sphericalVertexDistance;
layout(location = 1) in float cylindricalVertexDistance;

layout(location = 0) out vec4 fragColor;

// c-LoD camera channel (v4.3) — v4.1 az/el tracker upgraded to the FULL
// bobbed view basis.
//
// The sky program is the only vanilla stage that receives the true camera
// rotation (ModelViewMat: the camera-relative world pass loads the global
// modelview stack with the rotation only) and the projection (ProjMat).
// In 26.3 GameRenderer.renderLevel() folds view bobbing INTO the projection
// (projectionMatrix.mul(bobStack) — verified in the client sources), so
// ProjMat = P * T(tx,ty,0) * Rz(roll) * Rx(pitch) while ModelViewMat stays
// rotation-only. The bob rows are therefore extractable directly from the
// projection's first two rows:
//
//   M row0 = P[0][0] * B row0      (P row0 = (a,0,0,0))
//   M row1 = P[1][1] * B row1      (P row1 = (0,b,0,0))
//
// We encode the BOBBED world-space right/up basis + bob-corrected fov into
// the ALPHA channel of the leftmost 19 columns of every sky fragment. RGB
// stays 100% vanilla — the channel is invisible on screen (alpha of the
// scene target is never displayed).
//
//   x0 = 0xC7, x1 = 0x63            magic (2-byte, false positive ~2^-16/row)
//   x2..x7  = bobbed right, xyz     big-endian 16-bit, (c+1)/2*65535
//   x8..x13 = bobbed up, xyz        big-endian 16-bit, (c+1)/2*65535
//   x14,x15 = fov                   big-endian 16-bit, (fov-5)/115*65534
//                                   (65535 = invalid marker)
//   x16 = eye offset tx             signed 8-bit, (tx+1)/2*255, range +-1
//   x17 = eye offset ty             signed 8-bit, (ty+1)/2*255, range +-1
//   x18 = checksum                  (x0..x17 additive) mod 256
//
// The encoded basis lets the LOD march shake EXACTLY with the world (view
// bobbing roll + pitch wobble + the sub-block eye wobble), and the 16-bit
// fov removes the 0.45-degree stepping the 8-bit byte had during sprint-fov
// animations. The fov is corrected for the bob scale: ProjMat[1][1] reads
// b * Rb_row1.y under bob, so b_true = ProjMat[1][1] / r1.y.
//
// v4.4: the block is written into EIGHT column groups spread across the
// screen width (x % 240 < 19) instead of only the leftmost 19 columns. The
// camera channel previously died whenever terrain covered the left edge
// (every pan across a mountainside), the tracker confidence decayed and
// the whole LOD faded out and popped back — the "flicker when I move the
// camera" bug. With groups spread over the width, at least one of them
// shows sky unless the ENTIRE screen is terrain-covered — in which case
// there are no sky pixels to paint anyway. The additive checksum keeps
// false positives at ~2^-24 per row per group (magic 2^-16 × checksum
// 2^-8), so 8 groups × 2000 rows still stay silent for hours.
//
// Downstream writes (terrain alpha = 1.0, clouds, particles, weather, GUI)
// fail the magic/checksum checks; the post chain scans the column top-down
// for the topmost INTACT row, so transient overdraw self-heals within a
// frame for as long as any sky row is visible in a payload group.
//
// Vertically inverted note: gl_FragCoord (0,0) is the BOTTOM-left texel;
// the "top of the screen" is simply large y — no viewport size is needed.

ivec2 enc16(float c) {
    float q = floor(clamp((c + 1.0) * 0.5, 0.0, 1.0) * 65535.0 + 0.5);
    return ivec2(int(floor(q / 256.0)), int(mod(q, 256.0)));
}

int enc8s(float c) {
    return int(clamp((c + 1.0) * 0.5, 0.0, 1.0) * 255.0 + 0.5);
}

void main() {
    vec4 color = apply_fog(ColorModulator, sphericalVertexDistance,
        cylindricalVertexDistance, 0.0, FogSkyEnd, FogSkyEnd, FogSkyEnd, FogColor);

    ivec2 px = ivec2(gl_FragCoord.xy);
    if (px.x % 240 < 19) {
        // World-space view basis (rotation-only modelview): view space looks
        // down -Z, so transpose(mat3(MV)) maps view axes to world axes.
        mat3 mv = mat3(ModelViewMat);
        vec3 Rw = transpose(mv) * vec3(1.0, 0.0, 0.0);
        vec3 Uw = transpose(mv) * vec3(0.0, 1.0, 0.0);
        vec3 Fw = transpose(mv) * vec3(0.0, 0.0, -1.0);

        // Bob rows out of the folded projection (see header): rows 0/1 of
        // ProjMat are positive-scaled rows of the bob rotation. Normalize
        // recovers them exactly; the scale (a, b) is irrelevant here.
        vec3 r0 = normalize(vec3(ProjMat[0][0], ProjMat[1][0], ProjMat[2][0]));
        vec3 r1 = normalize(vec3(ProjMat[0][1], ProjMat[1][1], ProjMat[2][1]));

        // Bobbed world-space basis: world -> view' is Rb * Rv, so the view'
        // axes in world space are Rv^T * (rows of Rb), with
        // Rv^T * (x,y,z) = x*Rw + y*Uw - z*Fw. Orthonormalize for the
        // 16-bit packing (mv and the extraction are float-exact enough).
        vec3 Rb = r0.x * Rw + r0.y * Uw - r0.z * Fw;
        vec3 Ub = r1.x * Rw + r1.y * Uw - r1.z * Fw;
        Rb = normalize(Rb);
        Ub = normalize(Ub - Rb * dot(Rb, Ub));

        // Bob-corrected vertical fov: ProjMat[1][1] = b * Rb_row1.y, so the
        // true projection scale is b / r1.y. r1.y = cos(roll)*cos(pitch),
        // bounded well away from 0 for every vanilla pose (max roll ~40deg
        // death animation), but clamp defensively anyway.
        float bTrue = ProjMat[1][1] / max(abs(r1.y), 0.25);
        float fovDeg = degrees(atan(1.0 / bTrue)) * 2.0;

        // Eye wobble: the bob translation lives in ProjMat's last column,
        // scaled by the same a/b factors as rows 0/1.
        float tx = ProjMat[3][0] / ProjMat[0][0] * max(abs(r0.x), 0.25);
        float ty = ProjMat[3][1] / ProjMat[1][1] * max(abs(r1.y), 0.25);

        int pay[19];
        pay[0] = 199;                                   // 0xC7
        pay[1] = 99;                                    // 0x63
        ivec2 e;
        e = enc16(Rb.x); pay[2] = e.x; pay[3] = e.y;
        e = enc16(Rb.y); pay[4] = e.x; pay[5] = e.y;
        e = enc16(Rb.z); pay[6] = e.x; pay[7] = e.y;
        e = enc16(Ub.x); pay[8] = e.x; pay[9] = e.y;
        e = enc16(Ub.y); pay[10] = e.x; pay[11] = e.y;
        e = enc16(Ub.z); pay[12] = e.x; pay[13] = e.y;
        int fovq = int(clamp((fovDeg - 5.0) / 115.0, 0.0, 1.0) * 65534.0 + 0.5);
        e = ivec2(int(floor(fovq / 256.0)), int(mod(fovq, 256.0)));
        pay[14] = e.x; pay[15] = e.y;
        pay[16] = enc8s(tx);
        pay[17] = enc8s(ty);
        int chk = 0;
        for (int i = 0; i < 18; ++i) {
            chk = (chk + pay[i]) & 255;
        }
        pay[18] = chk;

        color.a = float(pay[px.x]) / 255.0;
    }

    fragColor = color;
}
