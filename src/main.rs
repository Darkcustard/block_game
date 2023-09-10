#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]

//use sdl2;
use gl;

mod render;
mod perlin_2d;



fn build_camera() -> render::Camera{

    let pos : render::Vec3 = render::Vec3::create(0.0, 0.0, 0.0);
    let ang : render::Vec3 = render::Vec3::create(0.0, 0.0, 0.0);
    let fovx : f64 = 1.5;
    let fovy : f64 = 1.5;

    render::Camera::create(pos, ang, fovx, fovy)

}


fn main() {

    println!("\nStarting Block Game.\n----------------------");


    println!("Loading Config");

    // config
    let title = "Block Game";
    let resolution : [u32; 2] = [1000,500];


    // Camera
    println!("Building Camera.");
    let mut camera = build_camera();



    // SDL
    println!("Initializing SDL");
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window(title, resolution[0], resolution[1])
        .position_centered()
        .opengl()
        .build()
        .expect("Failed To Create Window");
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Init GL
    println!("Initializing GL and loading shaders.");
    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsys.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Load Shader files
    let fragment_shader = std::fs::read_to_string("src/fragment.vert").expect("Failed to read fragment shader.");
    let geometry_shader = std::fs::read_to_string("src/fragment.vert").expect("Failed to read geometry shader.");
    let vertex_shader = std::fs::read_to_string("src/fragment.vert").expect("Failed to read vertex shader.");



   // Event Loop
   'gameloop: loop {


        // Handle Events
        for event in event_pump.poll_iter() {

            match event {

                sdl2::event::Event::Quit { .. } => break 'gameloop,
                _ => {}                
            }
        }


        unsafe {
            gl::ClearColor(0.05, 0.1, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }




        window.gl_swap_window();
    }



}
