use glfw::Context;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
}

// TODO: make full screen available
impl Window {
    pub fn new(
        width: u32,
        height: u32,
        title: &str,
    ) -> (Self, glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
        use glfw::fail_on_errors;

        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .unwrap();

        window.make_current();
        window.set_all_polling(true);

        gl::load_with(|s| glfw.get_proc_address_raw(s));

        unsafe {
            gl::Viewport(
                0,
                0,
                width as gl::types::GLsizei,
                height as gl::types::GLsizei,
            );
        }

        (Self { glfw, window }, events)
    }
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
    pub fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value)
    }
}
