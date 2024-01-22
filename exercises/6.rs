extern crate nalgebra_glm as glm;

use glium::{
    index::PrimitiveType,
    uniform,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerWrapFunction},
    Surface,
};
use graphics::{app::App, shader::Shader, texture::Texture, vertex::Vertex};
use std::error::Error;
use winit::dpi::LogicalSize;

fn main() -> Result<(), Box<dyn Error>> {
    let window_builder = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(800, 800))
        .with_title("Shader art");

    let app = App::new(window_builder);
    let display = &app.display;
    let window = &app.window;
    let event_loop = app.event_loop;

    let vertex1 = Vertex::new([-0.5, 0.5, 0.0], [0.0, 1.0]);
    let vertex2 = Vertex::new([0.5, 0.5, 0.0], [1.0, 1.0]);
    let vertex3 = Vertex::new([-0.5, -0.5, 0.0], [0.0, 0.0]);
    let vertex4 = Vertex::new([0.5, -0.5, 0.0], [1.0, 0.0]);

    let shape = vec![vertex1, vertex2, vertex3, vertex4];
    let mut shader: Shader<Vertex, u8> =
        Shader::new("./shaders/main.vert", "./shaders/main.frag", &display);

    let texture = Texture::new("./assets/awesomeface.png", &display);

    let behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,

        ..Default::default()
    };

    shader.add_index_buffer(&display, PrimitiveType::TrianglesList, &[0, 1, 2, 1, 2, 3]);
    shader.add_vertex_buffer(&display, &shape);

    let window_size = window.inner_size().to_logical::<f32>(window.scale_factor());
    let resolution = [window_size.width, window_size.height];

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

                    let mut trans = glm::Mat4::identity();
                    // trans = glm::rotate(&trans, elapsed_time, &glm::vec3(1.0, 0.3, 0.0));
                    trans = glm::scale(&trans, &glm::vec3(1.0, 1.0, 1.0));
                    trans = glm::translate(&trans, &glm::vec3(elapsed_time.sin(), 0.0, 0.0));
                    trans = glm::transpose(&trans);

                    let trans = trans.as_slice();
                    let trans = trans.chunks(4);
                    let mut t = [
                        [0.0f32, 0.0, 0.0, 0.0],
                        [0.0, 0.0, 0.0, 0.0],
                        [0.0, 0.0, 0.0, 0.0],
                        [0.0, 0.0, 0.0, 0.0],
                    ];

                    for (i, v) in trans.enumerate() {
                        for (j, k) in v.iter().enumerate() {
                            t[i][j] = *k;
                        }
                    }

                    let uniforms = uniform! {
                        resolution: resolution,
                        time: elapsed_time,
                        tex: glium::uniforms::Sampler(&texture, behavior),
                        matrix: t,
                    };

                    let mut frame = display.draw();
                    frame.clear_color(0.0, 0.0, 0.0, 1.0);
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
