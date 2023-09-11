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
    let fovx : f32 = 1.5;
    let fovy : f32 = 1.5;

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
    let FRAG_SHADER = std::fs::read_to_string("src/fragment.vert").expect("Failed to read fragment shader.");
    let GEOM_SHADER = std::fs::read_to_string("src/geometry.vert").expect("Failed to read geometry shader.");
    let VERT_SHADER = std::fs::read_to_string("src/vertex.vert").expect("Failed to read vertex shader.");

    let pt:[f32;6] = [-0.5, 0.0, 0.5, 0.0, 0.0, 0.5];


    // Create GPU pipeline
    unsafe {

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        assert_ne!(vao, 0);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        assert_ne!(vbo, 0);   


        // Send Data to defined buffer on GPU
        gl::BufferData(

            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&pt) as isize,
            pt.as_ptr().cast(),
            gl::DYNAMIC_DRAW
        
        );

        // Describe our data
        gl::VertexAttribPointer(
            0, 
            2,
            gl::FLOAT,
            gl::FALSE,
            (2*std::mem::size_of::<f32>()) as i32,
            0 as *const _,


        );

        gl::EnableVertexAttribArray(0);


        // Create and compile shaders
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);


        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
          );

        gl::CompileShader(vertex_shader);


        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
              vertex_shader,
              1024,
              &mut log_len,
              v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));

        }

        // Create Fragment Shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);

        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );

        gl::CompileShader(fragment_shader);


        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(2048);
        let mut log_len = 0_i32;
        gl::GetShaderInfoLog(
            fragment_shader,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }


        // Create Geometry Shader
        let geometry_shader = gl::CreateShader(gl::GEOMETRY_SHADER);
        assert_ne!(geometry_shader, 0);

        gl::ShaderSource(
            geometry_shader,
            1,
            &(GEOM_SHADER.as_bytes().as_ptr().cast()),
            &(GEOM_SHADER.len().try_into().unwrap()),
        );

        gl::CompileShader(geometry_shader);


        let mut success = 0;
        gl::GetShaderiv(geometry_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(2048);
        let mut log_len = 0_i32;
        gl::GetShaderInfoLog(
            geometry_shader,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Geometry Compile Error: {}", String::from_utf8_lossy(&v));
        }



        // Compile shaders into program
        let mut shader_program = gl::CreateProgram();
        assert_ne!(shader_program,0);
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, geometry_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
          let mut v: Vec<u8> = Vec::with_capacity(1024);
          let mut log_len = 0_i32;
          gl::GetProgramInfoLog(
            shader_program,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
          );
          v.set_len(log_len.try_into().unwrap());
          panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(geometry_shader);
        gl::DeleteShader(fragment_shader);
        gl::UseProgram(shader_program);





        println!("{}", gl::GetError());






    }


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

            gl::DrawArrays(gl::POINTS, 0, 9 as i32);
        }




        window.gl_swap_window();
    }



}
