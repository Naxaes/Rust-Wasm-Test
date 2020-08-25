pub const SHADER : &str = r#"#version 300 es
precision mediump float;

in vec4 out_position;

out vec4 FragColor;

void main()
{
    FragColor = (out_position + 1.0) / 2.0;
}
"#;

use super::super::{compile, Shader, GL};
pub fn new(context: &GL) -> Result<Shader, String> {
    compile(&context, GL::FRAGMENT_SHADER, SHADER)
}