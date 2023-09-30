use cgmath::*;
use glfw::{Action, Context, Key};
use std::sync::mpsc::Receiver;

const UP: Vector3<f32> = Vector3 {x: 0.0, y: 1.0, z: 0.0};
const SPEED: f32 = 5.0;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub proj: Matrix4<f32>,
    pub view: Matrix4<f32>,

    pos: Vector3<f32>,
    target: Vector3<f32>,
    direction: Vector3<f32>,
    right: Vector3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,

    dt: f32,
    last_frame: f32,
}

impl Camera {
    pub fn new() -> Self {
        let pos = vec3(0.0, 0.0, 0.0);
        let target = vec3(0.0, 0.0, -1.0);
        let direction = Vector3::normalize(pos - target);
        
        let right = Vector3::normalize(Vector3::cross(UP, direction));
        let up = Vector3::cross(direction, right);
        let front = vec3(0.0, 0.0, -1.0);

        let view = Matrix4::look_at_rh(
            Point3::from_vec(pos),
            Point3::from_vec(pos + front),
            up,
        );

        Self {
            proj: perspective(Deg(45.0), 1.0, 0.1, 100.0),
            view, 

            pos,
            target,
            direction,
            right,
            front,
            up,
    
            dt: 0.0,
            last_frame: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.view = Matrix4::look_at_rh(
            Point3::from_vec(self.pos),
            Point3::from_vec(self.pos + self.front),
            self.up,
        );
    }

    pub fn input(
        &mut self,
        window: &mut glfw::Window, 
        glfw: &glfw::Glfw
    ) {
        let curr_frame = glfw.get_time() as f32;
        self.dt = curr_frame - self.last_frame;
        self.last_frame = curr_frame;

        if window.get_key(Key::W) == Action::Press {
            self.pos += SPEED * self.dt * self.front; 
        }
        if window.get_key(Key::S) == Action::Press {
            self.pos -= SPEED * self.dt * self.front; 
        }
        if window.get_key(Key::A) == Action::Press {
            self.pos -= SPEED * self.dt * self.right; 
        }
        if window.get_key(Key::D) == Action::Press {
            self.pos += SPEED * self.dt * self.right; 
        }
    }
}
