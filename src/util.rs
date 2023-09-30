use gl::*;
use glfw::{Action, Key,};

use std::sync::mpsc::Receiver;
pub fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { 
                    gl::Viewport(0, 0, width, height); 
                }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => 
                window.set_should_close(true),
            glfw::WindowEvent::Key(Key::F2, _, Action::Press, _) => {
                unsafe { PolygonMode(FRONT_AND_BACK, LINE); }
            }
            glfw::WindowEvent::Key(Key::F3, _, Action::Press, _) => {
                unsafe { PolygonMode(FRONT_AND_BACK, FILL); }
            }
            _ => {}
        }
    }
}

use std::ffi::{CString, NulError};
pub trait ToCStr {
    fn to_cstr(&self) -> Result<CString, NulError>;
}

impl ToCStr for str {
    fn to_cstr(&self) -> Result<CString, NulError> {
        CString::new(self)
    }
}
