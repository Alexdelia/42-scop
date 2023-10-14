#version 430

in vec4 v_position;
in vec4 v_color;
in vec2 v_texture;
in vec3 v_normal;

out vec4 out_color;

uniform sampler2D tex;
uniform bool textured;
uniform bool enlighten;
uniform vec3 light;
uniform float strength;
uniform bool use_color_buffer;

layout(std140) buffer ColorBuffer {
	vec4 colors[5];
	uint len;
};

vec4 get_enlightened_color(vec4 base_color) {
    float diffuse = max(dot(normalize(v_normal), normalize(light)), 0.0);
    vec3 camera_dir = normalize(vec3(-v_position));
    vec3 half_direction = normalize(normalize(light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
    
    vec3 dark_color = vec3(base_color[0] - strength, base_color[1] - strength, base_color[2] - strength);
    vec3 regular_color = vec3(base_color[0], base_color[1], base_color[2]);
    vec3 specular_color = vec3(base_color[0] + strength, base_color[1] + strength, base_color[2] + strength);
    return vec4(dark_color + diffuse * regular_color + specular * specular_color, base_color[3]);
}

void main() {
	vec4 raw_color;

    if (textured) {
        raw_color = texture(tex, v_texture);
    } else {
		if (use_color_buffer)
        	raw_color = colors[gl_PrimitiveID % len];
		else
			raw_color = v_color;
    }

    if (enlighten) {
        out_color = get_enlightened_color(raw_color);
    } else {
        out_color = raw_color;
    }
}