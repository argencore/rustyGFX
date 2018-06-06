#version 330

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

out vec4 color;

uniform vec3 u_light;

const vec3 ambient_color = vec3(0.1, 0.4, 0.85);
const vec3 diffuse_color = vec3(0.2, 0.6, 0.4);
const vec3 specular_color = vec3(0.3, 1.0, 0.6);

void main() {
    vec3 light = u_light;
    light.x = 0.4;
    light.y =  0.6;
    light.z = -0.2;
    float diffuse = max(dot(normalize(v_normal), normalize(light)), 0.0);
    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
    color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color + v_position, 1.0);
}