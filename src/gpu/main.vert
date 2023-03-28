#version 150

in vec3 position;
out vec4 color;

uniform mat4 matrix;

void main() {
	color = vec4(
		(position.x + 1.0) / 2.0,
		(position.y + 1.0) / 2.0,
		(position.z + 1.0) / 2.0,
		1.0
	);
    gl_Position = matrix * vec4(position, 1.0);
}