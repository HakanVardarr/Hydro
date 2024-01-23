#![allow(non_snake_case, dead_code)]
extern crate gl;
extern crate glfw;
extern crate stb_image;

use std::{ffi::c_char, ffi::CString, os::raw::c_void};

use gl::types::*;
use glfw::{Action, Context, Key, Window};
use stb_image::stb_image::{stbi_image_free, stbi_load};

#[rustfmt::skip]
const VERTEX_DATA: [GLfloat; 32] = [
    // positions          // colors           // texture coords
     0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // top right
     0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // bottom right
    -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // bottom left
    -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // top left 
];

const INDICIES: [GLuint; 6] = [
    0, 1, 3, // first triangle
    1, 2, 3, // second triangle
];
fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "OpenGL Tutorial", glfw::WindowMode::Windowed)
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
    let mut EBO = 0;
    let mut texture = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);

        gl::BindVertexArray(VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            std::mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (INDICIES.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
            std::mem::transmute(&INDICIES[0]),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<GLfloat>()) as GLsizei,
            (0 * std::mem::size_of::<GLfloat>()) as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<GLfloat>()) as GLsizei,
            (3 * std::mem::size_of::<GLfloat>()) as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<GLfloat>()) as GLsizei,
            (6 * std::mem::size_of::<GLfloat>()) as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(2);

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as GLint,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

        let mut height = 0;
        let mut width = 0;
        let mut channels_in_file = 0;
        let file_path = "./wall.jpg";
        let file_path_c_str = CString::new(file_path.as_bytes()).unwrap();

        let image_data = stbi_load(
            file_path_c_str.as_ptr() as *const c_char,
            &mut width,
            &mut height,
            &mut channels_in_file,
            0,
        );

        if image_data.is_null() {
            panic!("Error while loading the image");
        }

        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            width,
            height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            image_data as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        stbi_image_free(image_data as *mut c_void);
    }

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    unsafe {
                        gl::DeleteVertexArrays(1, &VAO);
                        gl::DeleteBuffers(1, &VBO);
                        gl::DeleteBuffers(1, &EBO);
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

            gl::BindTexture(gl::TEXTURE_2D, texture);

            shader.use_shader();
            gl::BindVertexArray(VAO);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);
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
