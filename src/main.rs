#[macro_use]
extern crate glium;

use std::error::Error;

use glium::Surface;

const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build()?;
    let window_builder = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("Triangle");

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_builder)
        .build(&event_loop);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None)
            .unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(
        &vertex_buffer,
        &indices,
        &program,
        &glium::uniforms::EmptyUniforms,
        &Default::default(),
    )?;
    target.finish()?;

    event_loop.run(move |event, window_target| match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                window_target.exit();
            }
            _ => (),
        },
        _ => (),
    })?;

    Ok(())
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
