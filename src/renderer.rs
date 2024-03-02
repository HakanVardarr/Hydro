use crate::{shader::ShaderError, Shader};

pub struct Renderer<'a> {
    display: &'a glium::Display<glium::glutin::surface::WindowSurface>,
}

impl<'a> Renderer<'a> {
    pub fn new(display: &'a glium::Display<glium::glutin::surface::WindowSurface>) -> Self {
        Self { display }
    }

    pub fn create_shader(
        &self,
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<Shader, ShaderError> {
        Shader::new(&self.display, vertex_path, fragment_path)
    }
}
