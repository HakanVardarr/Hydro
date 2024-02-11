use hydro::core::{Events, Window};
use hydro::graphics::*;
use hydro::reexports::*;

#[rustfmt::skip]
const VERTICES: [GLfloat; 20] = [
     0.5,  0.5, 0.0, 1.0, 1.0,
     0.5, -0.5, 0.0, 1.0, 0.0,
    -0.5, -0.5, 0.0, 0.0, 0.0,
    -0.5,  0.5, 0.0, 0.0, 1.0,
];

#[rustfmt::skip]
const INDICIES: [GLuint; 6] = [
    0, 1, 3,
    1, 2, 3,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, events) = Window::new(800, 600, "Tolga", false);

    let crocodile = Texture::new("./assets/crocodile.jpg")?;

    let shader = Shader::new("./shaders/main.vert", "./shaders/main.frag")?;
    let vertex_buffer = VertexBuffer::new(&VERTICES);
    let vertex_array = VertexArray::new(&vertex_buffer, &[3, 2]);
    let index_buffer = IndexBuffer::new(&INDICIES);

    Renderer::clear_color(0.0, 0.0, 0.0, 1.0);
    Renderer::polygon_mode(gl::FRONT_AND_BACK, gl::FILL);

    while !window.should_close() {
        window.poll_events();
        handle_events(&mut window, &events);

        Renderer::clear();

        shader.bind();
        crocodile.bind(0);
        shader.set_int("crocodileTex", 0);

        shader.set_float("time", window.get_time() as GLfloat);

        Renderer::draw(&shader, &vertex_array, &index_buffer);

        window.swap_buffers();
    }

    Ok(())
}

fn handle_events(window: &mut Window, events: &Events) {
    for (_, event) in events.flush() {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            _ => {}
        }
    }
}
