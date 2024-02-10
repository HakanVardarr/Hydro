use gl::types::{GLfloat, GLuint};

/// The `VertexBuffer` struct represents a vertex buffer object (VBO) in OpenGL.
/// It contains an id of type `GLuint` which is used to reference the buffer in OpenGL API calls.
pub struct VertexBuffer {
    id: GLuint,
}

impl VertexBuffer {
    /// The `new` function creates a new `VertexBuffer` instance.
    ///
    /// # Arguments
    ///
    /// * `verticies` - A slice of `GLfloat` values representing the vertices data.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` because it directly interacts with the OpenGL API.
    ///
    /// # Returns
    ///
    /// A new `VertexBuffer` instance.
    pub fn new(verticies: &[GLfloat]) -> Self {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<GLfloat>() * verticies.len()) as isize,
                verticies.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            )
        }

        Self { id }
    }

    /// The `bind` function binds the `VertexBuffer` instance to the current context.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` because it directly interacts with the OpenGL API.
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    /// The `unbind` function unbinds the current `VertexBuffer` from the context.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` because it directly interacts with the OpenGL API.
    pub fn unbind() {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
