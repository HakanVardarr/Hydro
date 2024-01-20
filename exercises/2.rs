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

    let first_shape = vec![vertex1, vertex2, vertex3];

    let first_index_buffer =
        IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u8, 1, 2])?;
    let first_vertex_buffer = glium::VertexBuffer::new(&display, &first_shape)?;

    let first_program = glium::Program::from_source(
        &display,
        include_str!("../shaders/2.vert"),
        include_str!("../shaders/2_1.frag"),
        None,
    )
    .unwrap();

    let vertex4 = Vertex::new([-0.5, 0.5, 0.0]);
    let vertex5 = Vertex::new([0.5, 0.5, 0.0]);
    let vertex6 = Vertex::new([-0.5, -0.5, 0.0]);

    let second_shape = vec![vertex4, vertex5, vertex6];

    let second_index_buffer =
        IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u8, 1, 2])?;
    let second_vertex_buffer = glium::VertexBuffer::new(&display, &second_shape)?;

    let second_program = glium::Program::from_source(
        &display,
        include_str!("../shaders/2.vert"),
        include_str!("../shaders/2_2.frag"),
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
                        polygon_mode: glium::draw_parameters::PolygonMode::Line,
                        ..Default::default()
                    };

                    let mut frame = display.draw();

                    frame.clear_color(0.1, 0.1, 0.1, 1.0);
                    frame
                        .draw(
                            &first_vertex_buffer,
                            &first_index_buffer,
                            &first_program,
                            &glium::uniforms::EmptyUniforms,
                            &params,
                        )
                        .unwrap();
                    frame
                        .draw(
                            &second_vertex_buffer,
                            &second_index_buffer,
                            &second_program,
                            &glium::uniforms::EmptyUniforms,
                            &params,
                        )
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
