pub const SHADER : &str = r#"#version 300 es
layout (location = 0) in vec3 position;

uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * vec4(position, 1.0);
}
"#;


use super::super::{compile, Shader, GL};
pub fn new(context: &GL) -> Result<Shader, String> {
    compile(&context, GL::VERTEX_SHADER, SHADER)
}