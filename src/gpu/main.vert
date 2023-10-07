#version 150

in vec4 position;
in vec4 color;
in vec2 texture;

out vec4 v_color;
out vec2 v_texture;

uniform mat4 perspective;
uniform mat4 matrix;

void main() {
	v_color = color;
	v_texture = texture;
    gl_Position = perspective * matrix * position;
}