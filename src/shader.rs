use std::fs::File;
use std::io::Read;

use glium::{
    glutin::surface::WindowSurface,
    index::{Index, PrimitiveType},
    uniforms::Uniforms,
    Display, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer,
};

pub struct Shader<T: Copy + glium::Vertex, I: Index> {
    program: Program,
    index_buffer: Option<IndexBuffer<I>>,
    vertex_buffer: Option<VertexBuffer<T>>,
}

impl<T: Copy + glium::Vertex, I: Index> Shader<T, I> {
    pub fn new(
        vertex_shader_path: &str,
        fragment_shader_path: &str,
        display: &Display<WindowSurface>,
    ) -> Self {
        if let (Ok(mut vertex_shader), Ok(mut fragment_shader)) = (
            File::open(vertex_shader_path),
            File::open(fragment_shader_path),
        ) {
            let mut vertex_shader_src = String::new();
            let mut fragment_shader_src = String::new();

            // TODO: Remove unwraps!
            vertex_shader
                .read_to_string(&mut vertex_shader_src)
                .unwrap();
            fragment_shader
                .read_to_string(&mut fragment_shader_src)
                .unwrap();

            Self {
                program: glium::Program::from_source(
                    display,
                    &vertex_shader_src,
                    &fragment_shader_src,
                    None,
                )
                .unwrap(),
                index_buffer: None,
                vertex_buffer: None,
            }
        } else {
            panic!("TODO: HANDLE THIS")
        }
    }
    pub fn add_index_buffer(
        &mut self,
        display: &Display<WindowSurface>,
        prim: PrimitiveType,
        data: &[I],
    ) {
        self.index_buffer = Some(IndexBuffer::new(display, prim, data).unwrap());
    }

    pub fn add_vertex_buffer(&mut self, display: &Display<WindowSurface>, shape: &Vec<T>) {
        self.vertex_buffer = Some(VertexBuffer::new(display, shape).unwrap());
    }
    pub fn draw<U: Uniforms>(&self, frame: &mut Frame, uniforms: &U, params: &DrawParameters) {
        if let (Some(index_buffer), Some(vertex_buffer)) = (&self.index_buffer, &self.vertex_buffer)
        {
            frame
                .draw(
                    vertex_buffer,
                    index_buffer,
                    &self.program,
                    uniforms,
                    &params,
                )
                .unwrap();
        } else {
            panic!()
        }
    }
}
