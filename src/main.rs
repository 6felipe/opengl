use gl::*;
use glfw::Context;
use cgmath::*;

mod shader;
mod vbo;
mod camera;
mod util;
mod terrain;
mod ui;
mod mesh;

use crate::shader::Shader;
use crate::camera::Camera;
use crate::util::*;

const VS: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 proj;

    uniform vec3 offsets[100];

    uniform float time;

    void main() {
        vec3 offset = offsets[gl_InstanceID];
        gl_Position = proj * view * model * vec4(aPos + offset, 1.0);
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

const W: u32 = 1000;
const H: u32 = 1000;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(
        glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core)
    );

    let (mut window, events) = glfw.create_window(
        W, 
        H, 
        "mestre dos magos", 
        glfw::WindowMode::Windowed
    ).expect("failed to create glfw window");

    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_mouse_button_polling(true);
    window.set_framebuffer_size_polling(true);

    //gl
    load_with(|s| window.get_proc_address(s) as * const _);

    let grid = terrain::grid(32, 32, 1.0);
    let mesh = mesh::Mesh::new(&grid.0, &grid.1);

    let mut camera = Camera::new();
    let size = mesh.indices.len() as i32;

    let shader = unsafe {
        Enable(BLEND);
        BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        Enable(DEPTH_TEST);
        DepthFunc(LESS);


        PointSize(3.0);
        LineWidth(3.0);

        shader::Shader::new_pipeline(VS, FS)
    }; 
    unsafe {
        shader.use_shader();
        shader.uniform_mat4fv(&"proj".to_cstr().unwrap(), &camera.proj);
    }
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    let mut last_frame = 0.0;
    let (mut dt, mut curr_frame);
    let mut last_dts: Vec<f64> = vec![];

    let mut ctx = imgui::Context::create();
    let mut gui = ui::Imgui::new(&mut ctx, &mut window);
    let model = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));
    
    let mut offsets: Vec<Vector3<f32>> = vec![];
    for i in 0..=100 {
        offsets.push(vec3(0.0,0.0, i as f32));
    }

    while !window.should_close() {
        process_events(
            &mut window, 
            &mut camera, 
            &mut gui,
            &events,
        );
        camera.input(&mut window, &glfw);
        let time = glfw.get_time();
        curr_frame = time;
        dt = curr_frame - last_frame; last_dts.push(dt);
        last_frame = curr_frame;
        
        let average_dt: f64 = 
            last_dts.as_slice().iter().sum::<f64>() / last_dts.len() as f64;
        if last_dts.len() > 512 { last_dts.remove(0); }

        let ui = gui.frame(&mut window); 
        if window.get_cursor_mode() == glfw::CursorMode::Disabled {
            ui.tooltip_text(format!("delta(ms): {:.4} framerate: {:.4} average(512): {:.4} time: {:.4}", 
                                    dt * 1000.0,
                                    1./dt,
                                    1./average_dt,
                                    time,
                                ));
            ui.tooltip_text("controls: F1, F2, F3 - switch view mode; GUI input is not currently supported");
            
        } else {
            ui.show_demo_window(&mut true);
        }

        unsafe {
            ClearColor(0.2, 0.3, 0.3, 1.0);
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

            shader.uniform_1f(&"time".to_cstr().unwrap(), time as f32);
            camera.update();
            shader.uniform_mat4fv(&"view".to_cstr().unwrap(), &camera.view);
            shader.uniform_mat4fv(&"model".to_cstr().unwrap(), &model);
            for i in 0..=100 {
                let string = format!("offsets[{i}]");
                shader.uniform_vec3f(
                    &string.to_cstr().unwrap(), 
                    &offsets[i]
                );
            }
            mesh.draw_instanced(100);
        }

        gui.draw();
        window.swap_buffers();
        glfw.poll_events();
    }
}

