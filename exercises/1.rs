use glium::{index::PrimitiveType, IndexBuffer, Surface};
use graphics::vertex::Vertex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build()?;
    let window_builder = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("Triangle");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_builder)
        .build(&event_loop);

    let vertex1 = Vertex::new([0.5, 0.5, 0.0]);
    let vertex2 = Vertex::new([0.5, -0.5, 0.0]);
    let vertex3 = Vertex::new([-0.5, -0.5, 0.0]);
    let vertex4 = Vertex::new([-0.5, 0.5, 0.0]);
    let vertex5 = Vertex::new([-1.0, -0.5, 0.0]);
    let vertex6 = Vertex::new([1.0, 0.5, 0.0]);
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices: [u8; 12] = [0, 1, 3, 1, 2, 3, 3, 2, 4, 0, 1, 5];
    let index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices).unwrap();

    let program = glium::Program::from_source(
        &display,
        include_str!("../shaders/1.vert"),
        include_str!("../shaders/1.frag"),
        None,
    )
    .unwrap();

    event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }

                winit::event::WindowEvent::RedrawRequested => {
                    let params = glium::DrawParameters {
                        polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                        ..Default::default()
                    };

                    let mut frame = display.draw();
                    let uniforms = glium::uniform! {};

                    frame.clear_color(0.2, 0.3, 0.3, 1.0);
                    frame
                        .draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params)
                        .unwrap();
                    frame.finish().unwrap();
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            }

            _ => (),
        };
    })?;

    Ok(())
}
