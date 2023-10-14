#version 150

in vec4 position;
in vec4 color;
in vec2 texture;
in vec3 normal;

out vec4 v_color;
out vec2 v_texture;
out vec3 v_normal;
out vec4 v_position;

uniform mat4 perspective;
uniform mat4 model;

void main() {
	v_color = color;
	v_texture = texture;
	v_normal = transpose(inverse(mat3(model))) * normal;

    gl_Position = perspective * model * position;
	v_position = gl_Position;
}