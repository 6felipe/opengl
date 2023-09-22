use glfw::{Action, Context, Key};
use gl::*;

use std::sync::mpsc::Receiver;
use std::ffi::CString;

mod shader;
mod vo;
mod util;

use crate::{shader::*, vo::*};

const VS: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    void main() {
        gl_Position = vec4(aPos, 1.0);
    }
"#;

const FS: &str = r#"
    #version 330 core
    out vec4 fragColor;
    uniform float iTime;

    void main() {
        fragColor = vec4(cos(iTime * 1.2), sin(iTime), iTime, 1.0f);
    }
"#;

const MAGO: [&str; 41] = [
    "CONTEMPLEM O MAGO",
    "COM SEUS PODERES",
    "INCRÍVEIS PODERES",
    "SOB O OLHAR DO NECROMANTE",
    "A ESCADA PRATEADA VAI SE ERGUER",
    "AS PESSOAS MARAVILHADAS",
    "COM SEUS OLHOS CHEIOS DE PODER",
    "COMIDA FRIA VAI ESQUENTAR AO ENFEITIÇAR",
    "BALANÇANDO AS SUAS MÃOS",
    "O CACHORRO QUENTE EXPLODIRÁ",
    "NA PRESENÇA DO GRANDE MAGO",
    "O TRÂNSITO PARA DE REPENTE",
    "PODE ATRAVESSAR A RUA",
    "COM OS CARROS PARADOS NA SUA FRENTE",
    "NA TV MUDAM-SE OS CANAIS",
    "SEM QUE SAIA DO SOFÁ",
    "SUA VARINHA PEGA ENTÃO",
    "PRA RECLINAR-SE NO AR",
    "PELA LUZ FRACA DO SOL NEGRO DO REINO DOS SONHOS.",
    "O MAGO SOBE AS CATARATAS CONGELADAS DE VOLDRINI",
    "EM BUSCA DE CELESTIA, A GUARDIÃ DO PODER INFINITO",
    "QUANDO DE REPENTE UM TERRÍVEL GARLON APARECE",
    "E ATACA COM GELO, MAS O MAGO É IMPLACÁVEL",
    "O GARLON RUGE E LIBERA UM VENTO MUITO SINISTRO,",
    "MAS O MAGO É IMPLACÁVEL.",
    "O GARLON INVOCAA AS PEDRAS DE PROPHYNIA,",
    "MAS O MAGO É IMPLACÁVEL.",
    "INVOCANDO OS PODERES DOS ANCESTRAIS,",
    "O MAGO CONJURA O FOGO SAGRADO",
    "E LANÇA SEU FEITIÇO NA LARVA DERRETIDA DE UM GORT INSACIÁVEL.",
    "(OBRIGADO, GORT!)",
    "AHH, CELESTIA. ACHO QUE VOCÊ VAI ADORAR ISSO",
    "O MAGO FICA DIANTE DO PRECIPÍCIO DO PODER DEFINITIVO,",
    "OS PORTÕES SE ABREM PARA REVELAR...",
    "HUM... QUANTO TEMPO EU FIQUEI DORMINDO?",
    "FAMINTO POR CAUSA DA ÚLTIMA MISSÃO",
    "O MAGO QUER LANCHAR",
    "TRAÇA O RUMO DO PRAZER",
    "PRO HABITUAL LUGAR",
    "ELE É O MAGO",
    "O MÍSTICO MAGO",
];

use std::time::Instant;
use std::cell::Cell;
use cgmath::*;

thread_local!{
    pub static START: Cell<Instant> = Cell::new( Instant::now() );
}

pub fn get_time_i32() -> i32 {
    let start = START.with(|start| start.get());
    start.elapsed().as_secs_f32() as i32
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let _rng = rand::thread_rng();

    let (mut window, events) = glfw.create_window(
        300, 
        300, 
        "😎 contemplem o mago 😎", 
        glfw::WindowMode::Windowed
    ).expect("failed to create GLFW window");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (vertex_shader_program, frag_shader_program, vbo, vao) = unsafe {
        let proj = perspective(Deg(45.0), 1.0, 1.0, 100.0);
        
        dbg!(proj);
        let vertex_shader_program = create_shader_program(VERTEX_SHADER, VS);
        let frag_shader_program = create_shader_program(FRAGMENT_SHADER, FS);

        let vertices: Vec<f32> = vec![
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ];

        let (vbo, vao) = vbo_vao(0, vertices.to_vec(), 0);

        (vertex_shader_program, frag_shader_program, vbo, vao)
    };


    let i_time = CString::new("iTime").unwrap();
    let i_time_location = unsafe { GetUniformLocation(frag_shader_program, i_time.as_ptr()) };

    unsafe {
        UseProgram(vertex_shader_program);
        UseProgram(frag_shader_program);

        let time = glfw.get_time() as f32;
        Uniform1f(i_time_location, time);
    }
    while !window.should_close() {
        process_events(&mut window, &events);

        let time = get_time_i32() / 3;
        window.set_title(MAGO[time as usize % MAGO.len()]);

        //render
        unsafe {
            ClearColor(0.2, 0.3, 0.4, 1.0);
            Clear(COLOR_BUFFER_BIT);

            let time = glfw.get_time() as f32;
            Uniform1f(i_time_location, time);

            let triangle = util::random_triangle();
    
            update_array_buffer(vbo, triangle);

            DrawArrays(TRIANGLE_FAN, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { 
                    gl::Viewport(0, 0, width, height); 
                }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

