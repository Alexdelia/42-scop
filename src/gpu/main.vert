#version 150

in vec3 position;
in vec3 color;
in vec2 texture;
out vec4 v_color;
out vec2 v_texture;

uniform mat4 matrix;

void main() {
	v_color = vec4(color, 1.0);
	v_texture = texture;
    gl_Position = matrix * vec4(position, 1.0);
}