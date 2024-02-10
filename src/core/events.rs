use glfw::FlushedMessages;

use crate::reexports::WindowEvent;

/// The `Events` struct is responsible for handling and processing window events.
pub struct Events {
    reciever: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

impl Events {
    /// This function is typically used as a constructor for the `Events` struct.
    pub fn new(reciever: glfw::GlfwReceiver<(f64, WindowEvent)>) -> Self {
        Self { reciever }
    }

    /// This function is used to flush the messages from the `receiver`.
    pub fn flush(&self) -> FlushedMessages<'_, (f64, WindowEvent)> {
        glfw::flush_messages(&self.reciever)
    }
}
