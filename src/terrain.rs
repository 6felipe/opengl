// draw a triangle strip grid
use gl::*;
use rand::prelude::*;

pub const QUAD_VERTICES: [f32; 12] = [
    // Positions
    0.5,  0.5, 0.0, // Top right
    0.5, -0.5, 0.0, // Bottom right
   -0.5, -0.5, 0.0, // Bottom left
   -0.5,  0.5, 0.0, // Top left
];

pub const QUAD_INDICES: [u32; 6] = [
    0, 1, 3, // First Triangle
    1, 2, 3, // Second Triangle
];

pub fn grid(cols: usize, rows: usize, cell_size: f32) -> (Vec<f32>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut i = 0;

    for row in 0..rows {
        for col in 0..cols {
            let x = row as f32;
            let y = col as f32;

            let quad_vertices = &QUAD_VERTICES.clone().translate_v3f(
                x, 
                y, 
                0.0,
            ).to_vec();

            let quad_indices: Vec<u32> = QUAD_INDICES
                .iter()
                .map(|&index| index + 4 * i)
                .collect();

            vertices.extend_from_slice(&quad_vertices);
            indices.extend_from_slice(&quad_indices);
            
            i += 1;
        }
    }

    (vertices, indices)
}

trait Vertex {
    fn append_vertex(&mut self, x: f32, y: f32, z: f32);
}

impl Vertex for Vec<f32> {
    fn append_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.push(x);
        self.push(y);
        self.push(z);
    }
}

trait Translate {
    fn translate_v3f(&mut self, x: f32, y: f32, z: f32) -> Self;
}

impl Translate for [f32; 12] {
    fn translate_v3f(&mut self, x: f32, y: f32, z: f32) -> Self {
        for i in 0..self.len() {
            if i % 3 == 0 {
                self[i] += x;
            } else if i % 3 == 1 {
                self[i] += y;
            } else {
                self[i] += z;
            }
        }

        *self 
    }
}
