#version 330 core

//uniforms


layout (points) in;
layout (triangle_strip, max_vertices = 24) out;

out vec4 fColor;
out vec3 vertex_pos;
out vec3 normal;

vec3 origin;

vec3 tfl;
vec3 tfr;
vec3 tbl;
vec3 tbr;

vec3 bfl;
vec3 bfr;
vec3 bbl;
vec3 bbr;

vec3 block_size;
vec3 shift;

uniform vec3 cam_pos;
uniform vec3 cam_ang;
uniform float cam_fovx;
uniform float cam_fovy;


void get_projected(in vec3 pos, out vec4 projected ) {

    vec3 relative;
    float aspect_ratio;
    float farclip;

    relative.x = cos(cam_ang.y)*(sin(cam_ang.z)*(pos.y-cam_pos.y)+cos(cam_ang.z)*(pos.x-cam_pos.x))-sin(cam_ang.y)*(pos.z-cam_pos.z);
    relative.y = sin(cam_ang.x)*(cos(cam_ang.y)*(pos.z-cam_pos.z)+sin(cam_ang.y)*(sin(cam_ang.z)*(pos.y-cam_pos.y)+cos(cam_ang.z)*(pos.x-cam_pos.x)))+cos(cam_ang.x)*(cos(cam_ang.z)*(pos.y-cam_pos.y)-sin(cam_ang.z)*(pos.x-cam_pos.x));
    relative.z = cos(cam_ang.x)*(cos(cam_ang.y)*(pos.z-cam_pos.z)+sin(cam_ang.y)*(sin(cam_ang.z)*(pos.y-cam_pos.y)+cos(cam_ang.z)*(pos.x-cam_pos.x)))-sin(cam_ang.x)*(cos(cam_ang.z)*(pos.y-cam_pos.y)-sin(cam_ang.z)*(pos.x-cam_pos.x));

    aspect_ratio = 1080.0/1920.0;
    farclip = 1000.0;

    projected.x = ( relative.x  * aspect_ratio ) / ( relative.z * tan( cam_fovx/2.0 ) ) ;
    projected.y = ( relative.y ) / ( relative.z * tan( cam_fovy/2.0 ) ) ;


    if ( (relative.z > 0.0 )) {
        projected = vec4(projected.x, projected.y , relative.z/farclip, 1);

    }

    else {
        projected = vec4(2, 2 , 100, 0);
      
    }



}


void main() {   

    // Init
    origin = vec3(gl_in[0].gl_Position.x, gl_in[0].gl_Position.y, gl_in[0].gl_Position.z);
    block_size = vec3(1.0,1.0,1.0);
    shift = block_size/2.0;

    // Create Vertices
    tfl = origin + vec3(-shift.x, shift.y, shift.z);
    tfr = origin + vec3(shift.x, shift.y, shift.z);
    tbl = origin + vec3(-shift.x, shift.y, -shift.z);
    tbr = origin + vec3(shift.x, shift.y, -shift.z);
    bfl = origin + vec3(-shift.x, -shift.y, shift.z);
    bfr = origin + vec3(shift.x, -shift.y, shift.z);
    bbl = origin + vec3(-shift.x, -shift.y, -shift.z);
    bbr = origin + vec3(shift.x, -shift.y, -shift.z);
    

    // Front Face
    get_projected(tbl,gl_Position);
    EmitVertex();
    get_projected(tbr,gl_Position);
    EmitVertex();
    get_projected(bbl,gl_Position);
    EmitVertex();
    get_projected(bbr,gl_Position);
    EmitVertex();
    
    EndPrimitive();


    
} 