use gl::types::{GLfloat, GLuint};

use super::vertex_buffer::VertexBuffer;

/// The `VertexArray` struct is responsible for managing a collection of `VertexBuffer`s.
pub struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    /// Creates a new `VertexArray` from a given `VertexBuffer` and a list of attributes.
    ///
    /// # Arguments
    ///
    /// * `vertex_buffer` - The `VertexBuffer` to use.
    /// * `attributes` - A slice of `GLuint` that specifies the attribute locations.
    ///
    /// # Returns
    ///
    /// A new `VertexArray`.
    pub fn new(vertex_buffer: &VertexBuffer, attributes: &[GLuint]) -> Self {
        vertex_buffer.bind();
        let mut id: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
        }

        let count: u32 = attributes.iter().sum();

        let mut last = 0;

        for (idx, attr) in attributes.iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(
                    idx as GLuint,
                    *attr as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    count as i32 * std::mem::size_of::<GLfloat>() as i32,
                    (last * std::mem::size_of::<GLfloat>()) as *const std::ffi::c_void,
                );
                gl::EnableVertexAttribArray(idx as GLuint);

                last += *attr as usize;
            }
        }

        Self { id }
    }

    /// Binds the `VertexArray` for rendering.
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    /// Unbinds the `VertexArray`.
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    /// Deletes the `VertexArray` when it goes out of scope.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
