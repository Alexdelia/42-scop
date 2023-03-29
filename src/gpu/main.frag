#version 150

in vec4 v_color;
in vec2 v_texture;
out vec4 out_color;

uniform sampler2D tex;
uniform bool texture_on;

void main() {
	if (texture_on)
		out_color = texture(tex, v_texture);
	else
    	out_color = v_color;
}