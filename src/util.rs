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
            glfw::WindowEvent::Key(Key::LeftAlt, _, Action::Press, _) => {
                if window.get_cursor_mode() == glfw::CursorMode::Disabled {
                    window.set_cursor_mode(glfw::CursorMode::Normal);
                } else {
                    window.set_cursor_mode(glfw::CursorMode::Disabled);
                }
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                gui.on_mouse_move(xpos as f32, ypos as f32, &window);
                camera.mouse_callback(xpos as f32, ypos as f32, &window);
            }
            glfw::WindowEvent::MouseButton(button, action, _) => {
                gui.on_mouse_click(button, action);
            }
            glfw::WindowEvent::Scroll(x, y) => {
                gui.on_mouse_scroll(x as f32, y as f32);
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

use cgmath::*;
pub fn usize_as_vec3(
    usize: usize, 
    max: f32, 
    max_x: f32, max_y: f32, max_z: f32
) -> Vector3<f32> {
    let usize = usize::min(max as usize, usize::max(usize, 0));

    let frac_val = usize as f32 / max;

    vec3(frac_val * max_x, frac_val * max_y, frac_val * max_z)
}

