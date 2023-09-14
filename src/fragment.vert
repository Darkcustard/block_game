#version 330 core
out vec4 FragColor;

in vec4 fColor;
in vec3 vertex_pos;
in vec3 normal;
in vec3 midpoint;

uniform vec3 light_pos;
uniform vec3 light_color;
uniform vec3 light_strength;
uniform vec3 light_radius;

vec3 toSun;
float ambient_strength;

void main()
{
    toSun = normalize(light_pos - midpoint);
    ambient_strength = 0.3;

    FragColor = fColor*ambient_strength + fColor*(1.0-ambient_strength) * vec4(light_color,1) * clamp(dot(toSun, normal), 0.0, 1.0);

} 