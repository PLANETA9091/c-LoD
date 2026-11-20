// c-LoD v6.59.13 THE PYRAMID BUILDER (the atlas-path port, part 1):
// packed conservative max-height pyramid over the streamed height atlas
// (clod_h_b 256x256). Level k (k=1..4) has dim = 256>>k cells and lives at
// rows [rowBase..rowBase+dim): L1 rows 0..127, L2 128..191, L3 192..223,
// L4 224..239 (rows 240..255 spare). One pass per level, ping-ponged through
// clod_py_b -> clod_py_a by rot_copy; the march reads clod_py_a.
//
// Each level-k pixel = max height byte over its MARKED children (unmarked
// fine cells read void in the march and can never hit — they must not raise
// the block max; a fully unmarked block stays A=0 and the march skips it
// outright). Rows outside the level are passthrough copies from Prev (the
// freshest pyramid so far) — the chain builds L1..L4 in sequence and the
// march always reads the settled clod_py_a.

uniform sampler2D Atlas; // clod_h_b — the settled height atlas (R byte, A mark)
uniform sampler2D Prev;  // clod_py_a — the freshest pyramid rows

layout(std140) uniform ClodPyr {
    vec4 LevelK;         // x = k (1..4), y = rowBase, z = dim
};

out vec4 fragColor;

void main() {
    ivec2 p = ivec2(gl_FragCoord.xy);
    int k = int(LevelK.x + 0.5);
    int rowBase = int(LevelK.y + 0.5);
    int dim = int(LevelK.z + 0.5);

    if (p.y >= rowBase && p.y < rowBase + dim && p.x < dim) {
        int cx = p.x * 2;
        int cy = (p.y - rowBase) * 2;
        int childBase = rowBase - dim * 2; // level k-1 rows (k=1: unused)
        float mx = 0.0;
        float marked = 0.0;
        for (int i = 0; i < 4; ++i) {
            ivec2 q = ivec2(cx + (i & 1), cy + (i >> 1));
            vec4 c;
            if (k == 1) {
                c = texelFetch(Atlas, q, 0);
            } else {
                c = texelFetch(Prev, ivec2(q.x, childBase + q.y), 0);
            }
            if (c.a > 0.5) {
                mx = max(mx, c.r);
                marked = 1.0;
            }
        }
        fragColor = vec4(mx, 0.0, 0.0, marked);
        return;
    }

    // Passthrough rows: whatever Prev holds (fresh for built levels, stale
    // for the levels this frame's later passes will rebuild).
    fragColor = texelFetch(Prev, p, 0);
}
