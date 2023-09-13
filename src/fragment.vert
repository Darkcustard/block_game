#version 330 core
out vec4 FragColor;

in vec4 fColor;
in vec3 vertex_pos;
in vec3 normal;

void main()
{
    //FragColor = fColor;
    FragColor = vec4(0.5, 0.5, 0.3, 1.0);
} 