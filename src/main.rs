use glfw::{Action, Context, Key};
use gl::*;
use cgmath::*;

use std::cell::RefCell;

mod shader;
mod vbo;
use crate::shader::Shader;

const VS: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 proj;

    void main() {
       gl_Position = proj * view * model * vec4(aPos, 1.0);
    }
"#;

const FS: &str = r#"
    #version 330 core
    out vec4 FragColor;

    uniform float time;

    void main() {
       FragColor = vec4(sin(time * 2.0), cos(time * 0.7), sin(time * 0.2), 1.0f);
    }
"#;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(
        glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core)
    );

    let (mut window, events) = glfw.create_window(
        300, 
        300, 
        "mestre dos magos", 
        glfw::WindowMode::Windowed
    ).expect("failed to create glfw window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    //gl
    load_with(|s| window.get_proc_address(s) as * const _);

    let (shader, vbo, vao) = unsafe {
        let shader = Shader::new_pipeline(VS, FS);

        let (mut vbo, mut ebo, mut vao) = vbo::VBO::new_indexed(
            vbo::QUAD.to_vec(), 
            vbo::QUAD_INDICES.to_vec()
        );

        (shader, vbo, vao)
    };

    let proj = perspective(Deg(45.0), 1.0, 1.0, 100.0);

    unsafe {
        shader.use_shader();

        shader.uniform_mat4fv(&"proj".to_cstr().unwrap(), &proj);
    }
    while !window.should_close() {
        process_events(&mut window, &events);
        let time = glfw.get_time() as f32;

        unsafe {
            ClearColor(0.2, 0.3, 0.3, 1.0);
            Clear(COLOR_BUFFER_BIT);

            shader.uniform_1f(&"time".to_cstr().unwrap(), time);

            let radius = 2.0;
            let cam_x = time.sin() as f32 * radius;
            let cam_z = time.cos() as f32 * radius;
            let view = Matrix4::look_at_rh(
                Point3::new(cam_x, 0.0, cam_z), 
                Point3::new(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            );
            shader.uniform_mat4fv(&"view".to_cstr().unwrap(), &view);

            let model = Matrix4::from_translation(vec3(0.0, 0.0, 0.0)); 
            shader.uniform_mat4fv(&"model".to_cstr().unwrap(), &model);

            BindVertexArray(vao);
            DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

use std::sync::mpsc::Receiver;
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { 
                    gl::Viewport(0, 0, width, height); 
                }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
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
trait ToCStr {
    fn to_cstr(&self) -> Result<CString, NulError>;
}

impl ToCStr for str {
    fn to_cstr(&self) -> Result<CString, NulError> {
        CString::new(self)
    }
}
