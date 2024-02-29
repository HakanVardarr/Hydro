use std::io::Read;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("Shader named: {0} is not found")]
    ShaderNotFound(String),
    #[error("Error happened while reading the shader file")]
    ShaderRead,
}

pub struct Shader {
    pub program: glium::Program,
}

impl Shader {
    pub fn new(
        display: &glium::Display<glium::glutin::surface::WindowSurface>,
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<Self, ShaderError> {
        let vertex_shader_src = Shader::get_shader(vertex_path)?;
        let fragment_shader_src = Shader::get_shader(fragment_path)?;

        let program =
            glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None)
                .unwrap();

        Ok(Self { program })
    }

    fn get_shader(path: &str) -> Result<String, ShaderError> {
        if let Ok(mut file) = std::fs::File::open(path) {
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)
                .map_err(|_err| ShaderError::ShaderRead)?;
            return Ok(buffer);
        }

        Err(ShaderError::ShaderNotFound(path.to_string()))
    }
}
