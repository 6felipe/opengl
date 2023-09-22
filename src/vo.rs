use gl::types::*;
use gl::*;

use std::ffi::c_void;

pub unsafe fn vao(mut vao: u32) -> u32 {
    GenVertexArrays(1, &mut vao);
    BindVertexArray(vao);

    vao
}

pub unsafe fn vbo_vao(mut vbo: u32, vertices: Vec<f32>, idx: u32) -> (u32, u32) {
    let vertex_ammout = (vertices.len() / 3) as i32;

    GenBuffers(1, &mut vbo);
    BindBuffer(ARRAY_BUFFER, vbo);
    let vao = vao(0);

    BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW,
    );

    VertexAttribPointer(
        idx,
        3,
        gl::FLOAT,
        gl::FALSE,
        vertex_ammout * std::mem::size_of::<GLfloat>() as GLsizei,
        std::ptr::null(),
    );

    EnableVertexAttribArray(idx);
    BindBuffer(ARRAY_BUFFER, vbo);
    BindVertexArray(vao);

    (vbo, vao)
}

pub unsafe fn update_array_buffer(buffer: u32, data: Vec<f32>) {
    BindBuffer(ARRAY_BUFFER, buffer);
    BufferSubData(
        ARRAY_BUFFER,
        0,
        (data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
        &data[0] as *const f32 as *const c_void,
    );
    BindBuffer(ARRAY_BUFFER, 0);
}

pub unsafe fn create_buffer(
    buf_type: GLenum, 
    mut dst: u32, 
    data: Vec<f32>,
    rw_type: GLenum,
) -> u32 {
    GenBuffers(1, &mut dst);
    BindBuffer(buf_type, dst);

    BufferData(
        buf_type,
        (data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
        &data[0] as *const f32 as *const c_void,
        rw_type,
    );

    dst
}
