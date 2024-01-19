#[macro_use]
extern crate glium;

use std::error::Error;

use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 position;
    in vec3 color;
    out vec3 vertex_color;

    uniform mat4 matrix;

    void main() {    
        vertex_color = color;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    in vec3 vertex_color;
    out vec4 color;

    void main() {
        color = vec4(vertex_color, 1.0);
    }
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build()?;
    let window_builder = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("Triangle");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_builder)
        .build(&event_loop);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        color: [0.0, 1.0, 0.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5],
        color: [0.0, 0.0, 1.0],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None)
            .unwrap();

    let mut t = 0f32;
    event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into())
                }

                winit::event::WindowEvent::RedrawRequested => {
                    t += 0.02;
                    let x = t.sin() * 0.5;

                    let uniforms = uniform! {
                        matrix: [
                            [x.cos(), -x.sin(), 0.0, 0.0],
                            [x.sin(), x.cos(), 0.0, 0.0],
                            [0.0, 0.0, 0.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0],
                        ]
                    };

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();

                    target.finish().unwrap();
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => window.request_redraw(),
            _ => (),
        };
    })?;

    Ok(())
}
