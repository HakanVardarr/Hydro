use gl::types::GLfloat;

use crate::graphics::*;

/// The `Renderer` struct is responsible for rendering operations.
pub struct Renderer;

impl Renderer {
    /// Sets the color used to clear the screen.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component of the color.
    /// * `g` - Green component of the color.
    /// * `b` - Blue component of the color.
    /// * `a` - Alpha (transparency) component of the color.
    pub fn clear_color(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    /// Draws a shape using the provided shader, vertex array, and index buffer.
    ///
    /// # Arguments
    ///
    /// * `shader` - The shader to use for rendering.
    /// * `vertex_array` - The vertex array defining the shape's points.
    /// * `index_buffer` - The index buffer defining the order of the vertices.
    pub fn draw(shader: &Shader, vertex_array: &VertexArray, index_buffer: &IndexBuffer) {
        shader.bind();
        vertex_array.bind();
        index_buffer.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                index_buffer.size() as i32,
                gl::UNSIGNED_INT,
                0 as *const std::ffi::c_void,
            );
        }

        shader.unbind();
        vertex_array.unbind();
        index_buffer.unbind();
    }

    /// Clears the screen using the color set by `clear_color`.
    pub fn clear() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    /// Sets the polygon rasterization mode.
    ///
    /// # Arguments
    ///
    /// * `face` - Specifies the polygons that mode applies to. Can be `gl::FRONT_AND_BACK`, `gl::FRONT`, or `gl::BACK`.
    /// * `mode` - Specifies how polygons are rasterized. Can be `gl::POINT`, `gl::LINE` or `gl::FILL`.
    pub fn polygon_mode(face: u32, mode: u32) {
        unsafe { gl::PolygonMode(face, mode) }
    }
}
