use gl::COLOR_BUFFER_BIT;

use glfw::{Action, Key, WindowEvent};

use hydro::Window;

fn main() {
    let (mut window, events) = Window::new(800, 600, "Hydro");

    unsafe {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    }

    while !window.should_close() {
        handle_events(&mut window, &events);

        unsafe {
            gl::Clear(COLOR_BUFFER_BIT);
        }

        window.poll_events();
        window.swap_buffers();
    }
}

fn handle_events(window: &mut Window, events: &glfw::GlfwReceiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            WindowEvent::CursorPos(x, y) => unsafe {
                gl::ClearColor(
                    (x / 800.0) as f32,
                    (y / 800.0) as f32,
                    (x / 800.0 / 2.0 + y / 600.0 / 2.0) as f32,
                    1.0,
                );
            },
            _ => {}
        }
    }
}
