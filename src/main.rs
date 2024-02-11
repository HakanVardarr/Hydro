use hydro::core::{Events, Window};
use hydro::graphics::*;
use hydro::reexports::*;

#[rustfmt::skip]
const VERTICES: [GLfloat; 20] = [
    // Position         Texture Coord
     0.5,  0.5, 0.0,    1.0, 1.0,
     0.5, -0.5, 0.0,    1.0, 0.0,
    -0.5, -0.5, 0.0,    0.0, 0.0,
    -0.5,  0.5, 0.0,    0.0, 1.0,
];

#[rustfmt::skip]
const INDICIES: [GLuint; 6] = [
    0, 1, 3,
    1, 2, 3,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, events) = Window::new(800, 600, "Hydro", false);

    let container = Texture::new("./assets/container.jpg", "containerTex")?;
    let face = Texture::new("./assets/face.png", "faceTex")?;

    let mut shader = Shader::new("./shaders/main.vert", "./shaders/main.frag")?;
    shader.add_texture(container.clone());
    shader.add_texture(face.clone());

    let vertex_buffer = VertexBuffer::new(&VERTICES);
    let vertex_array = VertexArray::new(&vertex_buffer, &[3, 2]);
    let index_buffer = IndexBuffer::new(&INDICIES);

    Renderer::clear_color(0.0, 0.0, 0.0, 1.0);
    Renderer::polygon_mode(gl::FRONT_AND_BACK, gl::FILL);

    while !window.should_close() {
        window.poll_events();
        handle_events(&mut window, &events);

        Renderer::clear();

        let mut trans = glm::mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        // First Box: Apply translation and rotation
        trans = glm::ext::translate(&trans, glm::vec3(0.5, -0.5, 0.0));
        trans = glm::ext::rotate(&trans, window.get_time() as f32, glm::vec3(0.0, 0.0, 1.0));

        // Render first box

        shader.bind();
        shader.set_matrix4("transform", trans);
        shader.set_float("time", window.get_time() as GLfloat);

        Renderer::draw(&shader, &vertex_array, &index_buffer);

        // Second Box: Apply translation only
        let mut trans2 = glm::mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        trans2 = glm::ext::translate(&trans2, glm::vec3(-0.5, 0.5, 0.0));
        trans2 = glm::ext::rotate(&trans2, -window.get_time() as f32, glm::vec3(0.0, 0.0, 1.0));

        // Render second box
        shader.bind();
        shader.set_matrix4("transform", trans2);
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
