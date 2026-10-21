#version 330

// c-LoD pass 3: ping-pong copy for the persistent 2x2 rotation target
// (clod_rot_b -> clod_rot_a) so the next frame seeds from the newest solve
// without ever reading and writing the same target in one pass.
// v4.2: the target is 2x2 — camera pixel, fog color pixel, fog range pixel.

uniform sampler2D InSampler;

in vec2 texCoord;
out vec4 fragColor;

void main() {
    ivec2 px = ivec2(gl_FragCoord.xy);
    fragColor = texelFetch(InSampler, px, 0);
}
