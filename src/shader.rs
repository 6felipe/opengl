use gl::types::*;
use gl::*;

use std::ffi::CString;
use std::ptr;

pub unsafe fn create_shader_program(shader_type: GLenum, code: &str) -> u32 {
    let shader = shader(shader_type, code);
    let program = link_shader(shader);

    DeleteShader(shader);

    program
}

pub unsafe fn shader(shader_type: GLenum, code: &str) -> u32 {
    let shader = CreateShader(shader_type);
    let shader_cstring = CString::new(code.as_bytes()).unwrap();
    ShaderSource(shader, 1, &shader_cstring.as_ptr(), ptr::null());
    CompileShader(shader);

    // check for errors
    check_shader_error(shader);

    shader
}

pub unsafe fn link_shader(shader: u32) -> u32 {
    let shader_program = CreateProgram();
    AttachShader(shader_program, shader);
    LinkProgram(shader_program);

    //check for errors
    check_shader_link_error(shader_program);

    shader_program
}

pub unsafe fn check_shader_error(shader: u32) {
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1); // skip the trailing null char
    GetShaderiv(shader, COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        GetShaderInfoLog(
            shader,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
            "ERROR::SHADER::COMPILATION::FAILED\n{}",
            std::str::from_utf8(&info_log).unwrap()
        );
    }
}

pub unsafe fn check_shader_link_error(program: u32) {
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(
            program,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
            "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
            std::str::from_utf8(&info_log).unwrap()
        );
    }
}
