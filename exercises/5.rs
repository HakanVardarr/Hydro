use glium::{
    index::PrimitiveType,
    uniform,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerWrapFunction},
    Surface,
};
use graphics::{shader::Shader, texture::Texture, vertex::Vertex};
use std::error::Error;
use winit::{
    event::ElementState, keyboard::Key,
    platform::modifier_supplement::KeyEventExtModifierSupplement,
};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build()?;
    let window_builder = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("Triangle");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_builder)
        .build(&event_loop);

    let vertex1 = Vertex::new([0.5, 0.5, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0]);
    let vertex2 = Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]);
    let vertex3 = Vertex::new([-0.5, -0.5, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]);
    let vertex4 = Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0]);
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let mut shader: Shader<Vertex, u8> = Shader::new(
        "./shaders/triangle.vert",
        "./shaders/triangle.frag",
        &display,
    );

    let texture1 = Texture::new("./assets/container.jpg", &display);
    let texture2 = Texture::new("./assets/awesomeface.png", &display);

    shader.add_index_buffer(&display, PrimitiveType::TrianglesList, &[0, 1, 2, 0, 3, 2]);
    shader.add_vertex_buffer(&display, &shape);

    let behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Linear,
        magnify_filter: MagnifySamplerFilter::Linear,
        ..Default::default()
    };

    let mut v: f32 = 0.0;
    // let start_time = std::time::Instant::now();
    event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }

                winit::event::WindowEvent::RedrawRequested => {
                    // let elapsed_time = start_time.elapsed().as_secs_f32();

                    let params = glium::DrawParameters {
                        polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                        ..Default::default()
                    };

                    let uniforms = uniform! {
                        texture1: glium::uniforms::Sampler(&texture1, behavior),
                        texture2: glium::uniforms::Sampler(&texture2, behavior),
                        v: v,
                    };

                    let mut frame = display.draw();
                    frame.clear_color(0.0, 0.0, 0.0, 1.0);
                    shader.draw(&mut frame, &uniforms, &params);
                    frame.finish().unwrap();
                }
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        match event.key_without_modifiers() {
                            Key::Named(winit::keyboard::NamedKey::ArrowUp) => {
                                if v < 1.0 {
                                    v += 0.05;
                                }
                            }
                            Key::Named(winit::keyboard::NamedKey::ArrowDown) => {
                                if v > 0.0 {
                                    v -= 0.05;
                                }
                            }
                            _ => {}
                        }
                    }
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
