use gl::types::{GLchar, GLfloat, GLint, GLuint};
use std::{ffi::CString, fs::File, io::Read};
use thiserror::Error;

/// `ShaderError` is an enumeration that represents the types of errors that can occur when working with shaders.
/// It can either be a Vertex shader error, a Fragment shader error, or a Program error.
///
/// Each variant of `ShaderError` carries a `String` payload that provides more details about the error.
#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("Vertex shader error: '{0}' ")]
    Vertex(String),
    #[error("Fragment shader error: '{0}' ")]
    Fragment(String),
    #[error("Program error: '{0}' ")]
    Program(String),
}

/// `ShaderType` is an enumeration that represents the type of shader.
/// It can either be a Vertex shader or a Fragment shader.
enum ShaderType {
    Vertex,
    Fragment,
}

/// `Shader` is a structure that represents a shader.
/// It contains an id of type GLuint.
pub struct Shader {
    id: GLuint,
}

/// `compile_shader` is a function that compiles a shader from a given file path and shader type.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the shader file.
/// * `shader_type` - A ShaderType value that specifies the type of shader (Vertex or Fragment).
///
/// # Returns
///
/// * `Ok(GLuint)` - The id of the compiled shader if the shader compilation is successful.
/// * `Err(ShaderError)` - An error of type ShaderError if the shader compilation fails.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The shader file cannot be opened.
/// * The contents of the shader file cannot be read.
/// * The shader compilation fails.
fn compile_shader(path: &str, shader_type: ShaderType) -> Result<GLuint, ShaderError> {
    if let Ok(mut file) = File::open(path) {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .map_err(|_err| match shader_type {
                ShaderType::Vertex => ShaderError::Vertex(format!(
                    "error while reading the contenst of the vertex shader"
                )),
                ShaderType::Fragment => ShaderError::Fragment(format!(
                    "error while reading the contenst of the fragment shader"
                )),
            })?;

        let c_string = CString::new(buffer).unwrap();

        unsafe {
            let shader: GLuint = gl::CreateShader(match shader_type {
                ShaderType::Vertex => gl::VERTEX_SHADER,
                ShaderType::Fragment => gl::FRAGMENT_SHADER,
            });
            gl::ShaderSource(shader, 1, &(c_string.as_ptr()), std::ptr::null());
            gl::CompileShader(shader);

            let mut success: GLint = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut len: GLint = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );

                buffer.push(0);

                let error_message = std::str::from_utf8(&buffer)
                    .expect("Shader info log was not valid UTF-8")
                    .to_string();

                match shader_type {
                    ShaderType::Fragment => return Err(ShaderError::Fragment(error_message)),
                    ShaderType::Vertex => return Err(ShaderError::Fragment(error_message)),
                };
            }

            Ok(shader)
        }
    } else {
        match shader_type {
            ShaderType::Vertex => {
                return Err(ShaderError::Vertex(format!("file {} is not found", path)))
            }
            ShaderType::Fragment => {
                return Err(ShaderError::Fragment(format!("file {} is not found", path)))
            }
        }
    }
}

impl Shader {
    /// `new` is a function that creates a new Shader.
    ///
    /// # Arguments
    ///
    /// * `vertex` - A string slice that holds the path to the vertex shader file.
    /// * `fragment` - A string slice that holds the path to the fragment shader file.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - A new Shader if the shader program is successfully created and linked.
    /// * `Err(ShaderError)` - An error of type ShaderError if the shader program creation or linking fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// * The shader program cannot be created.
    /// * The shaders cannot be attached to the program.
    /// * The shader program cannot be linked.
    /// * The shaders cannot be deleted after linking.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it calls unsafe OpenGL functions.
    pub fn new(vertex: &str, fragment: &str) -> Result<Self, ShaderError> {
        let vertex_shader = compile_shader(vertex, ShaderType::Vertex)?;
        let fragment_shader = compile_shader(fragment, ShaderType::Fragment)?;

        unsafe {
            let id: GLuint = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            gl::LinkProgram(id);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            let mut success: GLint = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len: gl::types::GLint = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut gl::types::GLchar,
                );
                // Ensure that the buffer is null-terminated
                buffer.push(0);

                let error_message = std::str::from_utf8(&buffer)
                    .expect("Program info log was not valid UTF-8")
                    .to_string();

                return Err(ShaderError::Program(error_message));
            }

            Ok(Self { id })
        }
    }

    /// `bind` is a function that binds the Shader for use.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it calls unsafe OpenGL functions.
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    /// `unbind` is a function that unbinds the Shader.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it calls unsafe OpenGL functions.
    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// `set_bool` is a function that sets a boolean uniform value for the Shader.
    ///
    /// # Parameters
    ///
    /// * `name`: A string slice that holds the name of the uniform variable in the shader.
    /// * `value`: A boolean value to be set for the uniform variable.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it calls unsafe OpenGL functions.
    pub fn set_bool(&self, name: &str, value: bool) {
        let c_str = CString::new(name.to_string()).unwrap();

        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, c_str.as_ptr()),
                value as i32,
            )
        }
    }

    /// `set_int` is a function that sets an integer uniform value for the Shader.
    ///
    /// # Parameters
    ///
    /// * `name`: A string slice that holds the name of the uniform variable in the shader.
    /// * `value`: An integer value to be set for the uniform variable.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it calls unsafe OpenGL functions.
    pub fn set_int(&self, name: &str, value: GLint) {
        let c_str = CString::new(name.to_string()).unwrap();

        unsafe { gl::Uniform1i(gl::GetUniformLocation(self.id, c_str.as_ptr()), value) }
    }

    /// `set_float` is a function that sets a floating-point uniform value for the Shader.
    ///
    /// # Parameters
    ///
    /// * `name`: A string slice that holds the name of the uniform variable in the shader.
    /// * `value`: A floating-point value to be set for the uniform variable.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it calls unsafe OpenGL functions.
    pub fn set_float(&self, name: &str, value: GLfloat) {
        let c_str = CString::new(name.to_string()).unwrap();

        unsafe { gl::Uniform1f(gl::GetUniformLocation(self.id, c_str.as_ptr()), value) }
    }
}

impl Drop for Shader {
    /// The `drop` function is called when an instance of `Shader` goes out of scope.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` because it calls `gl::DeleteProgram`,
    /// which is an unsafe function that interacts with the OpenGL API.
    ///
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
