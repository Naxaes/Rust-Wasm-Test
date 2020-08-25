pub const SHADER : &str = r#"#version 300 es
precision mediump float;

in vec4 out_position;
in vec2 out_texture_coordinate;
in vec3 out_normal;

out vec4 FragColor;

void main()
{
    FragColor = vec4(out_normal + 1.0 / 2.0, 1.0);
}
"#;

use super::super::{compile, Shader, GL};
pub fn new(context: &GL) -> Result<Shader, String> {
    compile(&context, GL::FRAGMENT_SHADER, SHADER)
}