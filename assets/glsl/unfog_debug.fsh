#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD pass (v6.18.2 "UN-FOG DEBUG"): the engagement mask of clod:post/unfog.
// Same gates, same factors, evidence output — shipped ONLY with
// unfog=true;unfog_debug=true (the 5b evidence lane). GREEN = the swap
// applied, RED = vanilla fog saturated (unrecoverable), DIM = the fogs
// already agree, BLUE = the camera channel was blind (gated before the
// math), untouched = sky.
//
// c-LoD pass (v6.18 "UN-FOG"): replace the vanilla terrain fog on the
// vanilla-rendered pixels with the exact Voxy finish() fog, so our far
// terrain reads like a Voxy mesh frame.
//
// WHY (run #285 verdict, the one residual class on every parity pair): with
// the parity world shipping view-distance 8 on BOTH legs, the 64..128 m band
// renders through the vanilla chunk renderer in our frame and through the
// Voxy mesh in the reference frame. The vanilla terrain shader fogges that
// band with BOTH pairs — the environmental one (which Voxy also applies to
// its mesh, so it is NOT a diff) and the render-distance pair, which reaches
// FULL fog at the chunk edge (128 m) while Voxy's finish() caps its fog at
// maxFog = endDistance/(envEnd-envStart) with
// endDistance = max(vanillaRD, 320) * sqrt(3) = 554 -> cap 0.54, never full.
// The diff maps show the delta as the white forest band (mean 14-18 on
// 08/09/10, 6-10 on the aerials).
//
// THE MATH — every constant cites the 26.3 client or the verified Voxy port:
//
//   WHAT THE CLIENT APPLIED (assets/minecraft/shaders/core/terrain.fsh +
//   include/fog.glsl + FogRenderer.setupFog bytecode, all from client.jar):
//     f_van = max(linear(dSph, envStart, envEnd),
//                 linear(dCyl, rdStart, rdEnd)) * fogColor.a
//     rdEnd = renderDistanceBlocks (= FogRenderDistanceEnd, live);
//     rdStart = rdEnd - clamp(rdEnd / 10, 4, 64)   (bytecode, = 115.2 @128)
//     dCyl = max(length(pos.xz), abs(pos.y)); dSph = length(pos)
//     out = mix(inColor.rgb, fogColor.rgb, f_van)
//
//   WHAT VOXY APPLIES (NormalRenderPipeline.finish, ported byte-exactly in
//   lod_terrain.fsh since v4.2 — the same formula our march paints with):
//     f_tgt = clamp(linear(dSph, envStart, envEnd), 0, maxFog) * fogColor.a
//     maxFog = clamp(endDistance * invEndFogDelta + startDelta, 0, 1)
//     endDistance = max(rdEnd, 320) * sqrt(3); invEndFogDelta = 1/(envEnd-envStart)
//
//   THE PASS: invert the vanilla mix, re-apply the Voxy mix —
//     true_rgb = (fogged - fogColor * f_van) / (1 - f_van)
//     out      = mix(true_rgb, fogColor, f_tgt)
//   Below the rd-pair start the two factors coincide and the pass is an
//   exact no-op (one 8-bit step tolerance); the 115..128 m ring gets its
//   voxy fog back. The pixel then matches a Voxy mesh frame by construction.
//
//   Device depth (src/packgen.rs, v5.0 comment): d = depth_near / eyeZ,
//   infinite far, sky == 0.0 -> eyeZ = depth_near / d.
//
// The per-pixel view ray rebuilds the world-axis position the same way
// capture.fsh projects sub-particle positions (same rot-channel basis, same
// fov byte) — the two passes are exact inverses of each other's projection.
//
// Gates (the pass must be a NO-OP whenever it cannot be exact):
//   - fog payload dead (no sky seen yet) -> passthrough;
//   - blind camera (conf < 0.25, the capture.fsh gate) -> passthrough;
//   - depth 0 (sky — the march owns those pixels) -> passthrough;
//   - |f_tgt - f_van| < 0.004 (one 8-bit step) or f_van >= 0.99 (saturated:
//     (fogged - fogColor) / (1 - f) is 0/0 noise) -> passthrough. The last
//     sliver of the render distance stays vanilla-fogged — information the
//     frame no longer carries.
//
// Chain placement (lod_terrain_unfog.json): AFTER dust_mask (the eraser and
// the four capture passes must read the RAW fogged frame — their strobe
// math owns its own un-fog with the dot's own distance), BEFORE lod_terrain
// (the march composites over the re-fogged pixels), feeding the march's In.

uniform sampler2D InSampler;      // minecraft:main color (fogged)
uniform sampler2D InDepthSampler; // minecraft:main depth (reversed-Z device d)
uniform sampler2D RotSampler;     // clod_rot_a (camera + fog channel, 2x2)

// x = device depth near plane (the __DEPTH_NEAR__ token, src/packgen.rs)
layout(std140) uniform ClodUnfog {
    vec4 UnfogInfo;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;

// fog.glsl linear_fog_value (client.jar): 0 at start, 1 at end.
float fog_linear(float d, float s, float e) {
    if (d <= s) { return 0.0; }
    if (d >= e) { return 1.0; }
    return (d - s) / (e - s);
}

void main() {
    vec4 color = texelFetch(InSampler, ivec2(clamp(texCoord, 0.0, 1.0) * ScreenSize), 0);
    float depth = texture(InDepthSampler, texCoord).r;

    vec4 rotP = texelFetch(RotSampler, ivec2(0, 0), 0);
    vec4 fogCol = texelFetch(RotSampler, ivec2(1, 0), 0);
    vec4 fogRng = texelFetch(RotSampler, ivec2(0, 1), 0);
    bool fogLive = fogRng.a > 0.5 && fogCol.a > 0.004;
    if (!fogLive || rotP.b < 0.25) {
        // channel blind -> BLUE overlay (the pass cannot be exact here)
        fragColor = vec4(mix(color.rgb, vec3(0.2, 0.3, 1.0), 0.6), color.a);
        return;
    }
    if (depth <= 0.0) {
        fragColor = color; // sky: the march owns these pixels
        return;
    }

    // World-axis view basis — identical to capture.fsh probe_word().
    float az = rotP.r * 2.0 * PI;
    float el = rotP.g * PI - PI * 0.5;
    float fov = 5.0 + rotP.a * 115.0;
    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));
    float tanH = tan(radians(fov) * 0.5);
    float aspect = ScreenSize.x / max(ScreenSize.y, 1.0);
    vec2 ndc = clamp(texCoord, 0.0, 1.0) * 2.0 - 1.0;
    vec3 dir = normalize(F + R * (ndc.x * tanH * aspect) + U * (ndc.y * tanH));

    // eyeZ = forward distance (the depth axis); pos = eyeZ * dir / (dir . F)
    // is the camera-relative world-axis position terrain.vsh computed.
    float eyeZ = UnfogInfo.x / depth;
    vec3 pos = dir * (eyeZ / max(dot(dir, F), 1e-4));

    float dSph = length(pos);
    float dCyl = max(length(pos.xz), abs(pos.y)); // fog.glsl fog_cylindrical_distance

    // Live pairs from sky.fsh group B (same decode as capture.fsh / lod_terrain.fsh).
    float envStart = fogRng.r * 255.0 * 8.0;
    float envEnd = fogRng.g * 255.0 * 8.0;
    float rdEnd = fogRng.b * 255.0 * 16.0;
    // FogRenderer.setupFog bytecode: rdStart = rdEnd - clamp(rdEnd/10, 4, 64).
    float rdStart = rdEnd - clamp(rdEnd * 0.1, 4.0, 64.0);

    // WHAT THE CLIENT APPLIED — fog.glsl total_fog_value: max of the pairs.
    float envLin = fog_linear(dSph, envStart, envEnd);
    float fVan = max(envLin, fog_linear(dCyl, rdStart, rdEnd)) * fogCol.a;

    // WHAT VOXY APPLIES — finish() (the lod_terrain.fsh port, v4.2): the
    // environmental pair capped by endDistance = max(rdEnd, 320) * sqrt(3).
    float invEndFogDelta = 1.0 / max(envEnd - envStart, 1.0);
    float startDelta = -envStart * invEndFogDelta;
    float endDistance = max(rdEnd, 320.0) * 1.7320508;
    float maxFog = clamp(endDistance * invEndFogDelta + startDelta, 0.0, 1.0);
    float fTgt = clamp(envLin, 0.0, maxFog) * fogCol.a;

    // v6.18.2 ENGAGEMENT MASK (evidence, not a render): the SAME gates and
    // factors as clod:post/unfog, but the output names its decision per
    // pixel — GREEN = the vanilla->voxy fog swap applied here, RED = the
    // vanilla fog is saturated (fVan >= 0.99, the signal is destroyed),
    // DIM = the two fogs already agree (no-op needed), untouched = sky.
    if (fVan >= 0.99) {
        fragColor = vec4(mix(color.rgb, vec3(1.0, 0.15, 0.1), 0.7), color.a);
        return;
    }
    if (abs(fTgt - fVan) <= 0.004) {
        fragColor = vec4(color.rgb * 0.45, color.a);
        return;
    }
    fragColor = vec4(mix(color.rgb, vec3(0.15, 1.0, 0.2), 0.65), color.a);
}
