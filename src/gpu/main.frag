#version 150

in vec4 v_color;
in vec2 v_texture;
out vec4 out_color;

uniform sampler2D tex;

void main() {
    // out_color = v_color;
	out_color = texture(tex, v_texture);
}