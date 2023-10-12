use crate::vbo::VBO;
use gl::*;

pub struct Mesh<'indices> {
    pub indices: &'indices Vec<u32>,
    pub vbo: (u32, u32, u32),
}

impl <'indices>Mesh<'indices> {
    pub fn new(verts: &Vec<f32>, indices: &'indices Vec<u32>, ) -> Self {
        let (vbo, ebo, vao) = 
            unsafe { VBO::new_indexed(verts, indices) };
        
        Self {
            indices,
            vbo: (vbo, ebo, vao),
        }
    }

    pub unsafe fn draw(&self) {
        let (vbo, ebo, vao) = self.vbo;
        BindVertexArray(vao);
        DrawElements(
            TRIANGLES, 
            self.indices.len() as i32, 
            UNSIGNED_INT, 
            std::ptr::null(),
        );
        BindVertexArray(0);
    }

    pub unsafe fn draw_instanced(&self, n: i32) {
        let (vbo, ebo, vao) = self.vbo;
        BindVertexArray(vao);
        DrawElementsInstanced(
            TRIANGLES, 
            self.indices.len() as i32, 
            UNSIGNED_INT, 
            std::ptr::null(),
            n,
        );
        BindVertexArray(0);
    }
}
