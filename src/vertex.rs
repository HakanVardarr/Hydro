#![allow(non_snake_case)]
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    aPos: [f32; 3],
}

impl Vertex {
    pub fn new(aPos: [f32; 3]) -> Self {
        Self { aPos }
    }
}

implement_vertex!(Vertex, aPos);
