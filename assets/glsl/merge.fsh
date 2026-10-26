#version 330
#extension GL_ARB_separate_shader_objects : require

// c-LoD v5 pass: PAGE MERGE (512x512).
//
// Routes the freshly scraped 128x128 page into the persistent 512x512 atlas
// target. The target holds 2x2 REGION SLOTS of 256x256 (slot = parity of the
// region coords — the SAME mapping the marcher derives from the world
// position and the server uses when filling pages, stream.rs slot_of). A slot
// holds one region: 2x2 page quadrants of 128x128 per layer.
//
// Output pixel o = (slot, quadrant, texel):
//   slot     = o >> 8, quadrant = (o >> 7) & 1, texel = o & 127.
// The page belongs here when its routing head (temp (0,0)) carries
// slot == my slot && page == my quadrant + PageBase. When routed:
//   - data texels copy through; A is stamped with the region base Y (height
//     layer) so every texel self-describes its vertical anchor;
//   - hole cell (0,0) of the quadrant becomes the REGION STAMP
//     (R = rx&15 /15, G = rz&15 /15, B = 1) — the marcher validates it to
//     reject stale slot content;
//   - other hole cells stay void.
// Otherwise the previous target content passes through (persistent ping-pong;
// the chain copies b back into a right after).

uniform sampler2D PageSampler; // 128x128 scrape output (this layer)
uniform sampler2D PrevSampler; // 512x512 persistent target (previous frame)

// x = first page id of this layer (0 color / 4 height)
layout(std140) uniform ClodMerge {
    vec4 PageBase;
};

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

void main() {
    ivec2 o = ivec2(gl_FragCoord.xy);       // 0..511
    ivec2 slot = o >> 8;                    // 0..1 per axis
    ivec2 in_slot = o & 255;                // 0..255 region cells
    ivec2 quadrant = in_slot >> 7;          // 0..1 per axis
    ivec2 t = in_slot & 127;                // 0..127 page texel
    int my_slot = slot.x + 2 * slot.y;
    int my_page = quadrant.x + 2 * quadrant.y + int(PageBase.x);

    vec4 head = texelFetch(PageSampler, ivec2(0, 0), 0);
    vec4 meta = texelFetch(PageSampler, ivec2(1, 0), 0);
    vec4 stamp = texelFetch(PageSampler, ivec2(2, 0), 0);
    int cur_slot = int(round(head.b * 15.0));
    int cur_page = int(round(head.a * 15.0));
    int hi = int(round(meta.b * 15.0));
    int lo = int(round(meta.a * 15.0));
    int rx = int(round(stamp.b * 15.0));
    int rz = int(round(stamp.a * 15.0));
    bool routed = head.r > 0.5 && meta.r > 0.5 && stamp.r > 0.5
        && cur_slot == my_slot && cur_page == my_page
        && hi < 16 && lo < 16 && rx < 16 && rz < 16;

    bool hole = (t.y == 0 && t.x < 8);
    vec4 prev = texelFetch(PrevSampler, o, 0);

    if (!routed) {
        fragColor = prev;
        return;
    }

    float base_n = float(hi * 16 + lo) / 255.0; // (baseY + 128) / 255
    if (hole) {
        if (t.x == 0) {
            // Region stamp: identity of the streamed region for the marcher.
            fragColor = vec4(float(rx) / 15.0, float(rz) / 15.0, 1.0, base_n);
        } else {
            fragColor = vec4(0.0, 0.0, 0.0, base_n); // void hole
        }
        return;
    }

    vec4 v = texelFetch(PageSampler, t, 0);
    v.a = base_n; // every texel self-describes its vertical anchor
    fragColor = v;
}
