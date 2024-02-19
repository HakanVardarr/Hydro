use glm::vec3;
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

const FPS_LIMIT: f64 = 1.0 / 60.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, events) = Window::new(800, 600, "Timsah");

    let container = Texture::new("./assets/container.jpg", "containerTex")?;
    let face = Texture::new("./assets/face.png", "faceTex")?;
    let crocodile = Texture::new("./assets/crocodile.jpg", "crocodileTex")?;

    let mut shader = Shader::new("./shaders/main.vert", "./shaders/main.frag")?;
    // shader.add_texture(container.clone());
    // shader.add_texture(face.clone());
    shader.add_texture(crocodile.clone());

    let vertex_buffer = VertexBuffer::new(&VERTICES);
    let vertex_array = VertexArray::new(&vertex_buffer, &[3, 2]);
    let index_buffer = IndexBuffer::new(&INDICIES);

    Renderer::clear_color(0.0, 0.0, 0.0, 1.0);
    Renderer::polygon_mode(gl::FRONT_AND_BACK, gl::FILL);

    let mut last_update_time = 0.0;
    let mut last_frame_time = 0.0;
    let mut last_print_time = 0.0;
    let mut frame_count = 0.0;

    while !window.should_close() {
        let now = window.get_time();
        let delta_time = now - last_update_time;

        window.poll_events();
        handle_events(&mut window, &events);

        if now - last_frame_time >= FPS_LIMIT {
            Renderer::clear();

            let mut trans = glm::mat4(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            );

            // First Box: Apply translation and rotation
            // trans = glm::ext::translate(&trans, glm::vec3(0.5, -0.5, 0.0));
            trans = glm::ext::rotate(&trans, window.get_time() as f32, glm::vec3(0.0, 0.0, 1.0));

            // Render first box

            let time = window.get_time() as GLfloat;

            shader.bind();
            shader.set_matrix4("transform", trans);
            shader.set_float("time", time);

            Renderer::draw(&shader, &vertex_array, &index_buffer);

            window.swap_buffers();

            last_frame_time = now;
            frame_count += 1.0;

            if now - last_print_time >= 1.0 {
                println!("FPS: {}", frame_count / 1.0);
                frame_count = 0.0;
                last_print_time = now;
            }
        }

        last_update_time = now;
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
