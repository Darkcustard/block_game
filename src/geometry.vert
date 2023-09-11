#version 330 core

layout (points) in;
layout (line_strip, max_vertices = 6) out;



void main() {   

    gl_Position = vec4(0.0,1.0,0.0,0.0);
    EmitVertex();

    gl_Position = vec4(-1.0,-1.0,0.0,0.0);
    EmitVertex();
    
    EndPrimitive();
} 