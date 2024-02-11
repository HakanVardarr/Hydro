use gl::types::GLuint;
use image::EncodableLayout;
use thiserror::Error;

/// Errors that may occur during texture operations.

#[derive(Error, Debug)]
pub enum TextureError {
    /// The specified file for the texture was not found.
    #[error("Texture error: File not found '{0}'")]
    FileNotFount(String),
}

/// Represents a texture object.
#[derive(Clone)]
pub struct Texture {
    id: GLuint,
    name: String,
}

impl Texture {
    /// Creates a new texture object from the image file located at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice representing the file path of the image.
    ///
    /// # Returns
    ///
    /// * `Ok(Texture)` - A new texture object if the texture creation is successful.
    /// * `Err(TextureError)` - An error indicating the reason for texture creation failure.
    pub fn new(path: &str, name: &str) -> Result<Self, TextureError> {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let img = image::open(path)
                .map_err(|_err| TextureError::FileNotFount(path.to_string()))?
                .flipv()
                .into_rgba8();

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const std::ffi::c_void,
            )
        }

        Ok(Self {
            id,
            name: name.to_string(),
        })
    }
    /// Binds the texture to the specified texture unit.
    ///
    /// # Arguments
    ///
    /// * `slot` - A GLuint representing the texture unit to which the texture will be bound.
    pub fn bind(&self, slot: GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
    /// Unbinds the currently bound texture.
    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) };
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for Texture {
    /// Drops the texture object and deallocates associated OpenGL resources.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
