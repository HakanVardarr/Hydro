use glium::{index::PrimitiveType, uniform, Surface};
use graphics::{shader::Shader, vertex::Vertex};
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build()?;
    let window_builder = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("Triangle");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_builder)
        .build(&event_loop);

    let vertex1 = Vertex::new([0.0, 0.5, 0.0]);
    let vertex2 = Vertex::new([-0.5, -0.5, 0.0]);
    let vertex3 = Vertex::new([0.5, -0.5, 0.0]);
    let shape = vec![vertex1, vertex2, vertex3];

    let mut shader = Shader::new(
        PathBuf::from("./shaders/triangle.vert"),
        PathBuf::from("./shaders/triangle.frag"),
        &display,
    );
    shader.add_index_buffer(&display, PrimitiveType::TrianglesList, &[0, 1, 2]);
    shader.add_vertex_buffer(&display, &shape);

    let start_time = std::time::Instant::now();
    event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }

                winit::event::WindowEvent::RedrawRequested => {
                    let elapsed_time = start_time.elapsed().as_secs_f32();
                    let params = glium::DrawParameters {
                        polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                        ..Default::default()
                    };

                    let uniforms = uniform! {
                        t: elapsed_time,
                        h_offset: 0.1f32,
                    };

                    let mut frame = display.draw();

                    frame.clear_color(0.1, 0.1, 0.1, 1.0);
                    shader.draw(&mut frame, &uniforms, &params);

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
