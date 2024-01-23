#![allow(non_snake_case, dead_code)]
extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::{Action, Context, Key, Window};

#[rustfmt::skip]
const VERTEX_DATA: [GLfloat; 18] = [
     0.0,  0.5, 0.0, // Pos 1
     1.0,  1.0, 0.0, // Color 1
    -0.5, -0.5, 0.0, // Pos 2
     0.0,  1.0, 1.0, // Color 2
     0.5, -0.5, 0.0, // Pos 3
     1.0,  1.0, 1.0, // Color 3
];

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(1600, 1200, "OpenGL Tutorial", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    gl::load_with(|symbol| window.get_proc_address(symbol));

    window.set_key_polling(true);
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_framebuffer_size_callback(framebuffer_size_callback);
    window.set_resizable(false);

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let version = unsafe {
        let data = gl::GetString(gl::VERSION) as *const i8;
        String::from_utf8_lossy(std::ffi::CStr::from_ptr(data).to_bytes()).into_owned()
    };

    println!("OpenGL Version: {}", version);

    let shader = opengl::shader::Shader::new("./main.vert", "./main.frag");

    let mut VBO = 0;
    let mut VAO = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);

        gl::BindVertexArray(VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            std::mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<GLfloat>()) as GLsizei,
            (0 * std::mem::size_of::<GLfloat>()) as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<GLfloat>()) as GLsizei,
            (3 * std::mem::size_of::<GLfloat>()) as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(1);
    }

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    unsafe {
                        gl::DeleteVertexArrays(1, &VAO);
                        gl::DeleteBuffers(1, &VBO);
                        gl::DeleteShader(shader.id);
                    }
                    window.set_should_close(true);
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    framebuffer_size_callback(&mut window, width, height);
                }
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader.use_shader();
            shader.set_float("time", glfw.get_time() as GLfloat);

            gl::BindVertexArray(VAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn framebuffer_size_callback(_window: &mut Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
