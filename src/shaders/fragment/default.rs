pub const SHADER : &str = r#"#version 300 es
precision mediump float;

out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 1.0f, 0.0f, 1.0f);
}
"#;

use super::super::{compile, Shader, GL};
pub fn new(context: &GL) -> Result<Shader, String> {
    compile(&context, GL::FRAGMENT_SHADER, SHADER)
}