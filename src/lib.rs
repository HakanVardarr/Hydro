//! # Hydro
//!
//! Hydro is a Rust game engine with a focus on simplicity, performance, and flexibility. It provides a modular architecture that allows developers to easily extend and customize its features.
//!
//! ## Modules
//!
//! ### `core`
//!
//! The `core` module contains fundamental components and utilities for building applications with Hydro. It includes features such as window management, input handling, and event processing.
//!
//! ### `graphics`
//!
//! The `graphics` module provides functionality for rendering graphics using OpenGL. It includes abstractions for managing vertex buffers, shaders, textures, and more.
//!
//! ### `reexports`
//!
//! The `reexports` module re-exports types and functions from external libraries for convenience. It allows users to access commonly used types from the `glfw` crate without directly importing them.
//!
//! #### Reexported Types
//!
//! - `Action`: Represents an action performed on a GLFW window or input device.
//! - `Key`: Represents a keyboard key.
//! - `Modifiers`: Represents modifiers such as shift, control, and alt keys.
//! - `MouseButton`: Represents a mouse button.
//! - `Scancode`: Represents a platform-specific scancode.
//! - `WindowEvent`: Represents events that occur on a GLFW window.
//!
//! ## Usage
//!
//! To use Hydro in your Rust project, you can import the desired modules:
//!
//! ```rust
//! use hydro::core::*;
//! use hydro::graphics::*;
//! use hydro::reexports::*;
//! ```
//!
//! With the `reexports` module, you have access to GLFW types without directly importing them from the `glfw` crate:
//!
//! ```rust
//! #[rustfmt::skip]
//! const VERTICES: [gl::types::GLfloat; 12] = [
//!     -0.5, -0.5, 0.0,
//!      0.5, -0.5, 0.0,
//!     -0.5,  0.5, 0.0,
//!      0.5,  0.5, 0.0,
//! ];
//!
//! #[rustfmt::skip]
//! const INDICIES: [gl::types::GLuint; 6] = [
//!     0, 1, 2,
//!     1, 2, 3,
//! ];
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let (mut window, events) = Window::new(800, 600, "Hydro");
//!
//!     let shader = Shader::new("./shaders/main.vert", "./shaders/main.frag")?;
//!     let vertex_buffer = VertexBuffer::new(&VERTICES);
//!     let vertex_array = VertexArray::new(&vertex_buffer, &[3]);
//!     let index_buffer = IndexBuffer::new(&INDICIES);
//!
//!     Renderer::clear_color(0.05, 0.0, 0.18, 1.0);
//!
//!     while !window.should_close() {
//!         window.poll_events();
//!          handle_events(&mut window, &events);
//!
//!         Renderer::polygon_mode(gl::FRONT_AND_BACK, gl::LINE);
//!
//!         Renderer::clear();
//!
//!         Renderer::draw(&shader, &vertex_array, &index_buffer);
//!
//!         window.swap_buffers();
//!     }
//!
//!     Ok(())
//! }
//!
//! fn handle_events(window: &mut Window, events: &Events) {
//!     for (_, event) in events.flush() {
//!         match event {
//!             WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
//!                 window.set_should_close(true);
//!             }
//!             _ => {}
//!         }
//!     }
//! }
//! ```

pub mod core;
pub mod graphics;

pub mod reexports;
