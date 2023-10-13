use imgui::*;
use glfw::Window;

use imgui_opengl_renderer::Renderer;

pub struct Imgui<'ctx> {
    last_frame: f64,

    /* 
     * INPUT IS YET TO BE IMPLEMENTED
     */

    renderer: Renderer,
    ctx: &'ctx mut Context,
}

impl <'ctx>Imgui<'ctx> {
    pub fn new(ctx: &'ctx mut Context, window: &mut Window) -> Self {
        let renderer = Renderer::new(ctx, |s| window.get_proc_address(s) as _);

        Self {
            last_frame: window.glfw.get_time(),

            renderer, 
            ctx,
        }
    }

    pub fn frame(&mut self, window: &mut Window) -> &mut Ui {
        let io = self.ctx.io_mut();

        let now = window.glfw.get_time();
        let delta = now - self.last_frame;
        self.last_frame = now;
        io.delta_time = delta as f32;

        let (w, h) = window.get_size();
        io.display_size = [w as f32, h as f32];

        self.ctx.frame()
    }

    pub fn on_mouse_move(
        &mut self, xpos: f32, ypos: f32, window: &glfw::Window
    ) {
        if window.get_cursor_mode() == glfw::CursorMode::Disabled {
            self.ctx.io_mut().mouse_pos = [0., 0.];
        } else {
            self.ctx.io_mut().mouse_pos = [xpos, ypos];
        }
    }
    pub fn on_mouse_click(
        &mut self, button: glfw::MouseButton, action: glfw::Action,
    ) {
        let is_pressed = if action == glfw::Action::Press {true} else {false};

        match button {
            glfw::MouseButton::Button1 => { 
                self.ctx.io_mut().mouse_down[0] = is_pressed;
            },
            glfw::MouseButton::Button2 => {
                self.ctx.io_mut().mouse_down[1] = is_pressed;
            },
            glfw::MouseButton::Button3 => {
                self.ctx.io_mut().mouse_down[2] = is_pressed;
            },
            glfw::MouseButton::Button4 => {
                self.ctx.io_mut().mouse_down[3] = is_pressed;
            },
            glfw::MouseButton::Button5 => {
                self.ctx.io_mut().mouse_down[4] = is_pressed;
            },
            _ => {},
        }
    }

    pub fn on_mouse_scroll(&mut self, x: f32, y: f32) {
        self.ctx.io_mut().mouse_wheel = y;
        self.ctx.io_mut().mouse_wheel_h = x;
    }

    pub fn draw(&mut self) {
        self.renderer.render(self.ctx);
    }
}
