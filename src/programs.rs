use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};

type GL = WebGl2RenderingContext;
type Shader = WebGlShader;
type Program = WebGlProgram;

use crate::shaders::compile;

pub fn create_program(gl: &GL, vertex_source: &str, fragment_source: &str) -> Result<Program, String> {
    link(
        gl,
        &compile(gl, GL::VERTEX_SHADER, vertex_source)?,
        &compile(gl, GL::FRAGMENT_SHADER, fragment_source)?,
    )
}


pub fn link(context: &GL, vertex_shader: &Shader, fragment_shader: &Shader) -> Result<Program, String> {
    let program = context
        .create_program()
        .ok_or("[WEBGL2 - PROGRAM LINKING ERROR]: Unable to create program.")?;

    context.attach_shader(&program, &vertex_shader);
    context.attach_shader(&program, &fragment_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let message = context
            .get_program_info_log(&program)
            .unwrap_or(String::from("Unknown error linking program."));
        Err(format!("[WEBGL2 - PROGRAM LINKING ERROR]: {}", message))
    }
}
