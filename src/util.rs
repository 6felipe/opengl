use gl::*;
use glfw::{self, Action, Key}; 
use crate::camera::Camera;
use crate::ui::Imgui;

use std::sync::mpsc::Receiver;
pub fn process_events(
    window: &mut glfw::Window, 
    camera: &mut Camera,
    gui: &mut Imgui,
    events: &Receiver<(f64, glfw::WindowEvent)>,
) {
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
            glfw::WindowEvent::Key(Key::F1, _, Action::Press, _) => {
                unsafe { PolygonMode(FRONT_AND_BACK, POINT); }
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                camera.mouse_callback(xpos as f32, ypos as f32);
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
