use glium::{glutin::surface::WindowSurface, Display};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct App {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub display: Display<WindowSurface>,
}

impl App {
    pub fn new(builder: WindowBuilder) -> Self {
        // Todo: Remove unwrap!
        let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .set_window_builder(builder)
            .build(&event_loop);

        App {
            event_loop,
            window,
            display,
        }
    }
}
