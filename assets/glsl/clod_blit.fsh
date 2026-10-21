#version 330

// c-LoD pass 6: composite the LOD frame back into minecraft:main.

uniform sampler2D InSampler;

layout(std140) uniform BlitConfig {
    vec4 ColorModulate;
};

in vec2 texCoord;
out vec4 fragColor;

void main() {
    fragColor = texture(InSampler, texCoord) * ColorModulate;
}
