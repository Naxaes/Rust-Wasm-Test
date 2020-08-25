pub const SHADER : &str = r#"#version 300 es

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texture_coordinate;
layout (location = 2) in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec4 out_position;
out vec2 out_texture_coordinate;
out vec3 out_normal;

void main()
{
    out_position = model * vec4(position, 1.0);
    out_texture_coordinate = texture_coordinate;
    out_normal = mat3(transpose(inverse(model))) * normal;

    gl_Position = projection * view * out_position;
}
"#;


use super::super::{compile, Shader, GL};
pub fn new(context: &GL) -> Result<Shader, String> {
    compile(&context, GL::VERTEX_SHADER, SHADER)
}