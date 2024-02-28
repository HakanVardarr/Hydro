use glium::{implement_vertex, Surface};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn new(position: [f32; 2]) -> Self {
        Self { position }
    }
}

implement_vertex!(Vertex, position);

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("Failed to create event loop.");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Hydro")
        .build(&event_loop);

    let shape = vec![
        Vertex::new([-0.5, -0.5]),
        Vertex::new([0.0, 0.5]),
        Vertex::new([0.5, -0.5]),
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indicies = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 1.0);
    frame
        .draw(
            &vertex_buffer,
            &indicies,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    frame.finish().unwrap();

    let _ = event_loop.run(move |event, window_target| match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => window_target.exit(),
            _ => {}
        },
        _ => {}
    });
}
