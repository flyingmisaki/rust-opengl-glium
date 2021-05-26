#[macro_use]
extern crate glium;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    // Define glium opengl window, ect... stuff
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Represents a vertex with a position
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    // Define vertices and shape here
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    // Define vertex buffer and put stuff in idk
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Vertex shader GLSL now takes in matrix
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    // Fragment shader GLSL
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    
    // Create glium program and pass it shaders
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
    // Define animation step
    let mut t: f32 = 1.0;
    event_loop.run(move |event, _, control_flow| {

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(8_333_334);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // we update `t`
        t += 0.05;
        

        // Define target object
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        // Draw target (triangle) here with all the needed data and parameters
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    });
}