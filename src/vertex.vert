#version 330 core

layout (location = 0) in vec3 block_pos;

void main()

    {
        gl_Position = vec4(block_pos.x, block_pos.y, 0.0, 1.0); 

    }