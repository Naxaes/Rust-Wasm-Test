use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use glm::{Mat4, Vec3, value_ptr};

use crate::programs::create_program;
use crate::shaders::compile;
use crate::camera::Camera;
use crate::textures::Texture;
use crate::mesh::Model;


pub struct DrawConfig {
    pub draw_mode: u32,                 // Lines, Triangles, ...
    pub start: i32,                     // Where to start.
    pub stop: i32,                      // -1 is to the end.
    pub material: Box<dyn Material>,    // Could have a bitfield describing the Material instead?
}

impl DrawConfig {
    pub fn new(draw_mode: u32, start: i32, stop: i32, material: Box<dyn Material>) -> Self {
        Self {
            draw_mode,
            start,
            stop,
            material,
        }
    }

    pub fn default(gl: &GL) -> Self {
        Self {
            draw_mode: GL::TRIANGLES,
            start: 0,
            stop: -1,
            material: Box::new(ColorMaterial::new(gl).unwrap()),
        }
    }
}


pub const COLOR_VERTEX_SHADER : &str = r#"#version 300 es
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

pub const COLOR_FRAGMENT_SHADER : &str = r#"#version 300 es
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


pub const POSITION_3D_VERTEX_SHADER : &str = r#"#version 300 es
layout (location = 0) in vec3 position;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * model * vec4(position, 1.0);
}
"#;


pub const SINGLE_COLOR_FRAGMENT_SHADER : &str = r#"#version 300 es
precision mediump float;

out vec4 FragColor;

void main()
{
    FragColor = vec4(0.4);
}
"#;


pub trait Material {
    fn enable(&self, gl: &GL, camera: &Camera);
    fn upload(&self, gl: &GL, model: &Model);
}



pub struct ColorMaterial {
    id: WebGlProgram,

    // Uniforms.
    model: WebGlUniformLocation,
    view:  WebGlUniformLocation,       // These should be global uniforms (UBO).
    projection: WebGlUniformLocation,  // These should be global uniforms (UBO).
}

impl ColorMaterial {
    pub fn new(gl: &GL) -> Result<Self, String> {
        let program = create_program(gl, COLOR_VERTEX_SHADER, COLOR_FRAGMENT_SHADER)?;

        let model = gl.
            get_uniform_location(&program, "model").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'model'.")?;
        let view = gl.
            get_uniform_location(&program, "view").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'view'.")?;
        let projection = gl.
            get_uniform_location(&program, "projection").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'projection'.")?;

        Ok(Self {
            id: program,
            model,
            view,
            projection
        })
    }
}

impl Material for ColorMaterial {
    fn enable(&self, gl: &GL, camera: &Camera) {  // Camera will disappear when camera is in UBO.
        gl.use_program(Some(&self.id));
        gl.uniform_matrix4fv_with_f32_array(Some(&self.view), false, &value_ptr(&camera.view_matrix()));
        gl.uniform_matrix4fv_with_f32_array(Some(&self.projection), false, &value_ptr(&camera.projection_matrix()));
    }
    fn upload(&self, gl: &GL, model: &Model) {
        let transform = glm::translation(&model.position);
        let transform = glm::rotate_x(&transform, model.rotation.x);
        let transform = glm::rotate_y(&transform, model.rotation.y);
        let transform = glm::rotate_z(&transform, model.rotation.z);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.model), false, &value_ptr(&transform));
    }
}


pub struct SingleColorMaterial {
    id: WebGlProgram,

    // Uniforms.
    model: WebGlUniformLocation,
    view:  WebGlUniformLocation,       // These should be global uniforms (UBO).
    projection: WebGlUniformLocation,  // These should be global uniforms (UBO).
}

impl SingleColorMaterial {
    pub fn new(gl: &GL) -> Result<Self, String> {
        let program = create_program(gl, POSITION_3D_VERTEX_SHADER, SINGLE_COLOR_FRAGMENT_SHADER)?;

        let model = gl.
            get_uniform_location(&program, "model").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'model'.")?;
        let view = gl.
            get_uniform_location(&program, "view").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'view'.")?;
        let projection = gl.
            get_uniform_location(&program, "projection").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'projection'.")?;

        Ok(Self {
            id: program,
            model,
            view,
            projection,
        })
    }
}

impl Material for SingleColorMaterial {
    fn enable(&self, gl: &GL, camera: &Camera) {  // Camera will disappear when camera is in UBO.
        gl.use_program(Some(&self.id));
        gl.uniform_matrix4fv_with_f32_array(Some(&self.view), false, &value_ptr(&camera.view_matrix()));
        gl.uniform_matrix4fv_with_f32_array(Some(&self.projection), false, &value_ptr(&camera.projection_matrix()));
    }

    fn upload(&self, gl: &GL, model: &Model) {
        let transform = glm::translation(&model.position);
        let transform = glm::rotate_x(&transform, model.rotation.x);
        let transform = glm::rotate_y(&transform, model.rotation.y);
        let transform = glm::rotate_z(&transform, model.rotation.z);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.model), false, &value_ptr(&transform));
    }
}





pub const FRAGMENT_SHADER : &str = r#"#version 300 es
precision mediump float;

in vec4 out_position;
in vec2 out_texture_coordinate;
in vec3 out_normal;


uniform sampler2D albedo;
uniform sampler2D metallic;
uniform sampler2D normal_map;
uniform sampler2D height_map;
uniform sampler2D occlusion;

out vec4 FragColor;

void main()
{
    FragColor = texture(albedo, out_texture_coordinate);
}
"#;


pub struct DefaultMaterial {
    id: WebGlProgram,

    albedo: WebGlTexture,
    // metallic: WebGlTexture,
    // normal_map: WebGlTexture,
    // height_map: WebGlTexture,
    // occlusion: WebGlTexture,

    // Uniforms.
    model: WebGlUniformLocation,
    view:  WebGlUniformLocation,       // These should be global uniforms (UBO).
    projection: WebGlUniformLocation,  // These should be global uniforms (UBO).

    albedo_location: WebGlUniformLocation,
}

impl DefaultMaterial {
    pub fn new(gl: &GL) -> Result<Self, String> {
        let program = create_program(gl, COLOR_VERTEX_SHADER, FRAGMENT_SHADER)?;

        let model = gl.
            get_uniform_location(&program, "model").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'model'.")?;
        let view = gl.
            get_uniform_location(&program, "view").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'view'.")?;
        let projection = gl.
            get_uniform_location(&program, "projection").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'projection'.")?;
        let albedo_location = gl.
            get_uniform_location(&program, "albedo").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'albedo'.")?;

        let albedo = Texture::from_pixels(gl, 1, 1, Some(&[0u8, 255u8, 0u8, 255u8]))?;

        Ok(Self {
            id: program,
            model,
            view,
            projection,
            albedo,
            albedo_location,
        })
    }
}

impl Material for DefaultMaterial {
    fn enable(&self, gl: &GL, camera: &Camera) {  // Camera will disappear when camera is in UBO.
        gl.use_program(Some(&self.id));
        gl.uniform_matrix4fv_with_f32_array(Some(&self.view), false, &value_ptr(&camera.view_matrix()));
        gl.uniform_matrix4fv_with_f32_array(Some(&self.projection), false, &value_ptr(&camera.projection_matrix()));
    }

    fn upload(&self, gl: &GL, model: &Model) {
        let transform = glm::translation(&model.position);
        let transform = glm::rotate_x(&transform, model.rotation.x);
        let transform = glm::rotate_y(&transform, model.rotation.y);
        let transform = glm::rotate_z(&transform, model.rotation.z);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.model), false, &value_ptr(&transform));

        // let albedo = Texture::from_pixels(gl, 1, 1, Some(&[255u8]));
        // let metallic = Texture::from_pixels(gl, 1, 1, Some(&[255u8]));
        // let normal_map = Texture::from_pixels(gl, 1, 1, Some(&[255u8]));
        // let height_map = Texture::from_pixels(gl, 1, 1, Some(&[255u8]));
        // let occlusion = Texture::from_pixels(gl, 1, 1, Some(&[255u8]));

        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, Some(&self.albedo));
        gl.uniform1i(Some(&self.albedo_location), 0);
    }
}