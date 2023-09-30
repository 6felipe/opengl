use gl::*;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use gl::types::*;

pub const QUAD: [f32; 12] = [
     0.5,  0.5, 0.0,  // top right
     0.5, -0.5, 0.0,  // bottom right
    -0.5, -0.5, 0.0,  // bottom left
    -0.5,  0.5, 0.0   // top left
];

pub const QUAD_INDICES: [i32; 6] = [ 0, 1, 3,  1, 2, 3];

pub struct VBO {
    vbo: u32,
    ebo: u32,
    vao: u32,
}

impl VBO {
    pub unsafe fn new(vertices: Vec<f32>) -> (u32, u32) {
        let (mut vbo, mut vao) = (0, 0);
        let vertex_ammount = (vertices.len() / 3) as i32;
        
        GenVertexArrays(1, &mut vao);
        GenBuffers(1, &mut vbo);

        // bind vertex array so we can do the vbo
        BindVertexArray(vao);

        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            STATIC_DRAW
        );

        VertexAttribPointer(
            0, 
            3, 
            FLOAT, 
            FALSE, 
            vertex_ammount * mem::size_of::<GLfloat>() as GLsizei, 
            ptr::null()
        );
        EnableVertexAttribArray(0);

        BindBuffer(ARRAY_BUFFER, 0);

        //unbind everything
        BindVertexArray(0);

        (vbo, vao)
    }
    pub unsafe fn new_indexed(
        vertices: Vec<f32>, 
        indices: Vec<i32>
    ) -> (u32, u32, u32) {
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        let vertex_ammount = (vertices.len() / 3) as i32;
        
        GenVertexArrays(1, &mut vao);
        GenBuffers(1, &mut vbo);
        GenBuffers(1, &mut ebo);

        // bind vertex array so we can do the vbo
        BindVertexArray(vao);

        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            STATIC_DRAW
        );

        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &indices[0] as *const i32 as *const c_void,
            STATIC_DRAW
        );

        VertexAttribPointer(
            0, 
            3, 
            FLOAT, 
            FALSE, 
            vertex_ammount * mem::size_of::<GLfloat>() as GLsizei, 
            ptr::null()
        );
        EnableVertexAttribArray(0);


        BindBuffer(ARRAY_BUFFER, 0);

        //unbind everything
        BindVertexArray(0);

        (vbo, ebo, vao)
    }
}
