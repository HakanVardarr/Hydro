#![allow(non_snake_case)]
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    aPos: [f32; 3],
    aTex: [f32; 2],
}

impl Vertex {
    pub fn new(aPos: [f32; 3], aTex: [f32; 2]) -> Self {
        Self { aPos, aTex }
    }
}

implement_vertex!(Vertex, aPos, aTex);
