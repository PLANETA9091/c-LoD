#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD v5 MAP CARRIER — the vanilla core/text vertex shader with one
// surgical addition: the streamed LOD map quad (RenderTypes.text on a
// minecraft:map/<id> DynamicTexture, local coords 0..128 at z = -0.01) is
// parked into a FIXED 128x128 device-pixel slot at the bottom-left of
// minecraft:main with depth 0 (sky class). The clod:lod_terrain post chain
// scrapes the slot into a persistent target and the terrain pass repaints
// those pixels — the map carries data, it is never seen.
//
// Everything else (glyphs, nametags, GUI text under IS_GUI, see-through
// text, OIT variants) runs the vanilla path byte-for-byte. The Globals UBO
// (ScreenSize) is bound for every world text pipeline (TEXT_SNIPPET), which
// is exactly where map quads render; the include is guarded like the vanilla
// UV2/Sampler2 world-only surface.

#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
#include <minecraft:fog.glsl>
#include <minecraft:sample_lightmap.glsl>
#endif
// Globals (ScreenSize) is only referenced by the map-carrier path, which is
// exclusive to the plain world pipeline: OIT variants must stay byte-identical
// to vanilla or their extra passes fail to compile.
#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
#include <minecraft:globals.glsl>
#endif

#include <minecraft:dynamictransforms.glsl>
#include <minecraft:projection.glsl>

layout(location = 0) in vec3 Position;
layout(location = 1) in vec4 Color;
layout(location = 2) in vec2 UV0;
#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
layout(location = 3) in ivec2 UV2;
#endif

#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
uniform sampler2D Sampler2;
layout(location = 0) out float sphericalVertexDistance;
layout(location = 1) out float cylindricalVertexDistance;
#endif

layout(location = 2) out vec4 vertexColor;
layout(location = 3) out vec2 texCoord0;
#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
layout(location = 4) out float vClodMap;
#endif

void main() {
#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
    vClodMap = 0.0;
    // Map-quad detection by UV: MapRenderer submits the map quad with UVs
    // EXACTLY at the (0,0)/(1,0)/(1,1)/(0,1) corners. Glyph quads carry
    // font-atlas sub-rectangles whose corners never land on both 0/1 edges
    // of BOTH axes. Detection must be UV-based because addVertex(pose, ...)
    // transforms positions on the CPU — the Position attribute arrives in
    // view space, not in the 0..128 quad space.
    // The slot position derives from the SAME UV: texel = uv * 128 device
    // pixels at the bottom-left of the render target, so the scrape reads
    // page texel (x, y) from screen texel (x, y) 1:1.
    bool cu = UV0.x <= 0.001 || UV0.x >= 0.999;
    bool cv = UV0.y <= 0.001 || UV0.y >= 0.999;
    if (cu && cv) {
        vec2 uvq = vec2(UV0.x >= 0.999 ? 1.0 : 0.0, UV0.y >= 0.999 ? 1.0 : 0.0);
        gl_Position = vec4(
            uvq.x * 128.0 / ScreenSize.x * 2.0 - 1.0,
            uvq.y * 128.0 / ScreenSize.y * 2.0 - 1.0,
            -1.0, 1.0);
        vClodMap = 1.0;
        vertexColor = Color;           // raw vertex color: NO lightmap multiply
        sphericalVertexDistance = 0.0; // fog skipped in the fragment stage
        cylindricalVertexDistance = 0.0;
        texCoord0 = UV0;
        return;
    }
#endif

    gl_Position = ProjMat * ModelViewMat * vec4(Position, 1.0);

#if !defined(IS_GUI) && !defined(IS_SEE_THROUGH)
    sphericalVertexDistance = fog_spherical_distance(Position);
    cylindricalVertexDistance = fog_cylindrical_distance(Position);
    vertexColor = Color * sample_lightmap(Sampler2, UV2);
#else
    vertexColor = Color;
#endif
    texCoord0 = UV0;
}
