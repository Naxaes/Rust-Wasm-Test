use web_sys::{WebGl2RenderingContext, WebGlShader};

type GL = WebGl2RenderingContext;
type Shader = WebGlShader;


pub fn compile(context: &GL, shader_type: u32, source: &str) -> Result<Shader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or("[WEBGL2 - PROGRAM LINKING ERROR]: Unable to create program.")?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let message = context.
            get_shader_info_log(&shader).
            unwrap_or(String::from("Unknown error creating shader"));
        Err(format!("[WEBGL2 - SHADER COMPILATION ERROR]: {}", message))
    }

}
