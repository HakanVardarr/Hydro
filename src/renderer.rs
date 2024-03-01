pub struct Renderer<'a> {
    display: &'a glium::Display<glium::glutin::surface::WindowSurface>,
}

impl<'a> Renderer<'a> {
    pub fn new(display: &'a glium::Display<glium::glutin::surface::WindowSurface>) -> Self {
        Self { display }
    }
}
