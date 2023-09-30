use glfw::{Action, Context, Key};
use gl::*;
use cgmath::*;

use std::cell::RefCell;

mod shader;
mod vbo;
mod camera;
mod util;

use crate::shader::Shader;
use crate::camera::Camera;
use crate::util::*;

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
    //glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
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

    let mut camera = Camera::new();

    unsafe {
        shader.use_shader();

        shader.uniform_mat4fv(&"proj".to_cstr().unwrap(), &camera.proj);
    }
    while !window.should_close() {
        process_events(&mut window, &events);
        camera.input(&mut window, &glfw);
        let time = glfw.get_time() as f32;

        unsafe {
            ClearColor(0.2, 0.3, 0.3, 1.0);
            Clear(COLOR_BUFFER_BIT);

            shader.uniform_1f(&"time".to_cstr().unwrap(), time);

            camera.update();
            shader.uniform_mat4fv(&"view".to_cstr().unwrap(), &camera.view);

            let model = Matrix4::from_translation(vec3(0.0, 0.0, 0.0)); 
            shader.uniform_mat4fv(&"model".to_cstr().unwrap(), &model);

            BindVertexArray(vao);
            DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}


