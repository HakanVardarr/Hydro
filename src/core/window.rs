use super::events::Events;
use glfw::Context;

/// The `Window` struct is responsible for managing a GLFW window and its associated events.
pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
}

// TODO: make full screen available
impl Window {
    /// Constructs a new `Window` and its associated `Events`.
    /// The window is created with the specified `width`, `height`, and `title`.
    /// The OpenGL context is also set up within this method.
    pub fn new(mut width: u32, mut height: u32, title: &str, fullscreen: bool) -> (Self, Events) {
        use glfw::fail_on_errors;

        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        glfw.window_hint(glfw::WindowHint::Resizable(false));

        if cfg!(target_os = "macos") {
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        }

        let (mut window, events) = glfw
            .with_primary_monitor(|glfw, m| {
                if fullscreen {
                    glfw.create_window(
                        width,
                        height,
                        title,
                        m.map_or(glfw::WindowMode::Windowed, |m| {
                            glfw::WindowMode::FullScreen(m)
                        }),
                    )
                } else {
                    glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
                }
            })
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_all_polling(true);

        window.set_framebuffer_size_callback(|_, width, height| unsafe {
            gl::Viewport(0, 0, width, height);
        });

        let events = Events::new(events);

        gl::load_with(|s| glfw.get_proc_address_raw(s));

        if cfg!(target_os = "macos") {
            width *= 2;
            height *= 2;
        }

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
    /// Checks if the window should close.
    /// Returns `true` if the window should close, `false` otherwise.
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
    /// Polls for any pending window events.
    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }
    /// Swaps the front and back buffers of the window.
    /// This should be called after all rendering operations for a frame are complete.
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
    /// Sets whether the window should close.
    /// `value`: `true` if the window should close, `false` otherwise.
    pub fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value)
    }

    pub fn get_time(&self) -> f64 {
        self.glfw.get_time()
    }
}