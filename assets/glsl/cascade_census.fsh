// c-LoD v6.59.9 cascade census strip (reconstruction; the v6.59 census rode
// the march shader, which died with the sandbox recycle — this standalone
// pass paints the atlas telemetry the verdict on the constant-186 needs):
//   monitor 640x360, the strip = x 200..208, row h-3:
//     200 r1H   — rh_b byte at the ring-1 window-w center cell
//     201 r2H   — rh_b at ring 2
//     202 conf  — rot (0,0).g (the camera liveness)
//     203 r1C   — rc_b at ring 1
//     204 r3H   — rh_b at ring 3
//     205 rwinW — rwin.r (the broadcast window, 0..15)
//     206 mx    — MAX height byte over the ring-1 window-w cells (4096 taps)
//     207 mn    — MIN height byte over the same window
//     208 seen  — rwin.b (the beacon projection aliveness)
// mx==mn everywhere at a live camera = the constant-186 signature; a spread
// = the decode is real and window 11 is simply a flat 122 m plain.
uniform sampler2D In;        // minecraft:main
uniform sampler2D Height;    // clod_rh_b — the HI atlas (2048x256)
uniform sampler2D ColBuf;    // clod_rc_b — the LO atlas (2048x256)
uniform sampler2D Rot;       // clod_rot_a
uniform sampler2D Rwin;      // clod_rwin

out vec4 fragColor;

layout(location = 0) in vec2 texCoord;

void main() {
    vec4 base = texture(In, texCoord);
    ivec2 res = ivec2(640.0, 360.0);
    ivec2 px = ivec2(texCoord * vec2(res));
    if (px.y != res.y - 3 || px.x < 200 || px.x > 208) {
        fragColor = base;
        return;
    }
    // The ring-1 cell grid: ring k = atlas columns 256k..256(k+1); the
    // window w center = ((w%4)*64 + 32, (w/4)*64 + 32) in ring-local cells.
    vec4 rwin = texelFetch(Rwin, ivec2(0, 0), 0);
    int w = int(floor(rwin.r * 16.0 + 0.5));
    vec2 ctr = vec2(float((w % 4) * 64 + 32), float((w / 4) * 64 + 32));
    vec4 rh = texelFetch(Height, ivec2(px.x - 200 + 1, 1), 0); // placeholder tap keeps the branch cheap
    vec4 col;
    if (px.x == 200) { col = texelFetch(Height, ivec2(256 + int(ctr.x), int(ctr.y)), 0); }
    else if (px.x == 201) { col = texelFetch(Height, ivec2(512 + int(ctr.x), int(ctr.y)), 0); }
    else if (px.x == 202) { col = texelFetch(Rot, ivec2(0, 0), 0); }
    else if (px.x == 203) { col = texelFetch(ColBuf, ivec2(256 + int(ctr.x), int(ctr.y)), 0); }
    else if (px.x == 204) { col = texelFetch(Height, ivec2(768 + int(ctr.x), int(ctr.y)), 0); }
    else if (px.x == 205) { col = rwin; }
    else if (px.x == 206 || px.x == 207) {
        // mx/mn over the whole window w of ring 1 (64x64 cells).
        float mx = 0.0;
        float mn = 1.0;
        for (int cy = 0; cy < 64; ++cy) {
            for (int cx = 0; cx < 64; ++cx) {
                float b = texelFetch(Height, ivec2(256 + (w % 4) * 64 + cx, (w / 4) * 64 + cy), 0).r;
                mx = max(mx, b);
                mn = min(mn, b);
            }
        }
        col = vec4(px.x == 206 ? mx : mn, 0.0, 0.0, 1.0);
    }
    else { col = rwin; } // 208: seen = rwin.b
    fragColor = vec4(col.r, col.g, col.b, 1.0);
}
