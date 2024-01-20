use glium::Surface;
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

    let vertex1 = Vertex::new([-0.5, -0.5], [1.0, 1.0, 1.0]);
    let vertex2 = Vertex::new([0.0, 0.5], [1.0, 1.0, 0.0]);
    let vertex3 = Vertex::new([0.5, -0.5], [0.0, 1.0, 1.0]);
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(
        &display,
        include_str!("../triangle.vert"),
        include_str!("../triangle.frag"),
        None,
    )
    .unwrap();

    let mut t: f32 = 4.0;
    let mut velocity: f32 = 0.02;
    event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }

                winit::event::WindowEvent::RedrawRequested => {
                    t += velocity;
                    if t < -4.0 || t > 4.0 {
                        velocity *= -1.0;
                    };

                    let mut frame = display.draw();
                    let uniforms = glium::uniform! {
                        x: t,
                    };

                    frame.clear_color(0.1, 0.1, 0.1, 1.0);
                    frame
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
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
