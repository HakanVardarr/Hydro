use gl::types::*;
use std::{ffi::CString, fs::File, io::Read};

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        unsafe {
            let mut vertex_code = String::new();
            let mut fragment_code = String::new();

            // TODO: remove unwrap!

            let mut vertex_file = File::open(vertex_path).unwrap();
            let mut fragment_file = File::open(fragment_path).unwrap();

            vertex_file.read_to_string(&mut vertex_code).unwrap();
            fragment_file.read_to_string(&mut fragment_code).unwrap();

            let vertex_code = CString::new(vertex_code.as_bytes()).unwrap();
            let fragment_code = CString::new(fragment_code.as_bytes()).unwrap();

            // --------------------------------------------------------------------------
            // Compile Shaders

            let vertex = compile_shader(vertex_code, gl::VERTEX_SHADER);
            let fragment = compile_shader(fragment_code, gl::FRAGMENT_SHADER);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);

            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    std::str::from_utf8(&buf)
                        .ok()
                        .expect("ProgramInfoLog not valid utf8")
                );
            }
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            return Self { id };
        }
    }

    pub fn use_shader(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        let name_cstr = CString::new(name.as_bytes()).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id.clone(), name_cstr.as_ptr()),
                value as GLint,
            );
        }
    }

    pub fn set_int(&self, name: &str, value: GLint) {
        let name_cstr = CString::new(name.as_bytes()).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id.clone(), name_cstr.as_ptr()),
                value,
            );
        }
    }
    pub fn set_float(&self, name: &str, value: GLfloat) {
        let name_cstr = CString::new(name.as_bytes()).unwrap();
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.id.clone(), name_cstr.as_ptr()),
                value,
            );
        }
    }
}

fn compile_shader(code: CString, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader

        gl::ShaderSource(shader, 1, &code.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                std::str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}
