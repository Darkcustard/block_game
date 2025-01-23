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
vec4 wColor;
float ambient_strength;

void main()
{
    toSun = normalize(light_pos - midpoint);
    ambient_strength = 0.3;
    wColor = fColor;

    if (midpoint.y < -198.0){
        wColor = vec4(0.0, 0.4, 0.8, 1.0);
    }
    else if (midpoint.y < -185.0){
        wColor = vec4(1.0, 1.0, 0.0, 1.0);

    }
    else if (midpoint.y < -160.0){

    }
    else if (midpoint.y < -150.0){
       wColor = vec4(0.5, 0.5, 0.5, 1.0); 
    }

    else if (midpoint.y < -0.0){
       wColor = vec4(1.0, 1.0, 1.0, 1.0); 
       //wColor = vec4(sin(midpoint.x/100), 0.0, cos(midpoint.z/100), 1.0);
    }

    

    FragColor = wColor*ambient_strength + wColor*(1.0-ambient_strength) * vec4(light_color,1.0) * clamp(dot(toSun, normal), 0.0, 1.0);

} 