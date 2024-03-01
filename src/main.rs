use glium::{implement_vertex, Surface};
use hydro::{Renderer, Shader};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}

implement_vertex!(Vertex, position, color);

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("Failed to create event loop.");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Hydro")
        .with_inner_size(800, 600)
        .build(&event_loop);

    let renderer = Renderer::new(&display);

    let shape = vec![
        Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
        Vertex::new([0.0, 0.5, 0.0], [0.0, 1.0, 0.0]),
        Vertex::new([0.5, -0.5, 0.0], [0.0, 0.0, 1.0]),
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &[0 as u8, 1, 2],
    )
    .expect("Failed to create index buffer");

    let shader = Shader::new(&display, "shaders/triangle.vert", "shaders/trinagle.frag").unwrap();
    let mut frame = display.draw();

    frame.clear_color(0.0, 0.0, 0.0, 1.0);
    frame
        .draw(
            &vertex_buffer,
            &index_buffer,
            &shader.program,
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
