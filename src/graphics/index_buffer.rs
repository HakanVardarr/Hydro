use gl::types::GLuint;

/// The `IndexBuffer` struct is responsible for managing a collection of indices.
pub struct IndexBuffer {
    id: GLuint,
    size: GLuint,
}

impl IndexBuffer {
    /// Creates a new `IndexBuffer` from a given slice of indices.
    ///
    /// # Arguments
    ///
    /// * `indices` - The slice of `GLuint` that specifies the indices.
    ///
    /// # Returns
    ///
    /// A new `IndexBuffer`.
    pub fn new(indicies: &[GLuint]) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (std::mem::size_of::<GLuint>() * indicies.len()) as isize,
                indicies.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            )
        }

        Self {
            id,
            size: (std::mem::size_of::<GLuint>() * indicies.len()) as u32,
        }
    }
    /// Returns the size of the `IndexBuffer`.
    ///
    /// # Returns
    ///
    /// The size of the `IndexBuffer`.
    pub fn size(&self) -> GLuint {
        self.size
    }

    /// Binds the `IndexBuffer` for rendering.
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }
    /// Unbinds the `IndexBuffer`.
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for IndexBuffer {
    /// Deletes the `IndexBuffer` when it goes out of scope.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
