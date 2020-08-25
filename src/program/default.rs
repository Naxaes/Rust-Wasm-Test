use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use glm::{Mat4, Vec3, value_ptr};

use super::link;
use super::super::log;
use super::super::model::Model;
use super::super::shaders::{vertex, fragment};
use super::super::math;
use super::super::camera::Camera;


pub fn print_matrix(m: &Mat4) {
    log(format!("\
Matrix(\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
)", m.m11, m.m12, m.m13, m.m14,
    m.m21, m.m22, m.m23, m.m24,
    m.m31, m.m32, m.m33, m.m34,
    m.m41, m.m42, m.m43, m.m44
    ).as_str());
}


pub struct Default {
    program: WebGlProgram,

    // Uniforms.
    // color: WebGlUniformLocation,
    // opacity: WebGlUniformLocation,
    // transform: WebGlUniformLocation,
}

impl Default {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, String> {
        let program = link(
            gl,
            &vertex::default::new(gl)?,
            &fragment::default::new(gl)?,
        )?;

        Ok(Self {
            // color: gl.get_uniform_location(&program, "color").unwrap_or("Couldn't get uniform 'color'")?,
            // opacity: gl.get_uniform_location(&program, "opacity").unwrap_or("Couldn't get uniform 'opacity'")?,
            program: program,
        })
    }

    pub fn render(&self, gl: &GL, models: &[Model], camera: &Camera) -> Result<(), String> {
        gl.use_program(Some(&self.program));

        let model_location = gl.
            get_uniform_location(&self.program, "model").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'model'.")?;
        let view_location = gl.
            get_uniform_location(&self.program, "view").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'view'.")?;
        let proj_location = gl.
            get_uniform_location(&self.program, "projection").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'projection'.")?;

        let camera_position = -&camera.position;
        gl.uniform_matrix4fv_with_f32_array(Some(&view_location), false, &value_ptr(&glm::translation(&camera_position)));

        gl.uniform_matrix4fv_with_f32_array(Some(&proj_location), false, &value_ptr(&glm::perspective(16.0 / 9.0, 3.14 / 2.0, 1.0, 1000.0)));

        log(format!("Camera: {}, {}, {}", camera.position.x, camera.position.y, camera.position.z).as_str());

        for model in models.iter() {
            let transform = glm::translation(&model.position);
            // let transform = glm::rotate_x(&transform, model.rotation.x);
            // let transform = glm::rotate_y(&transform, model.rotation.y);
            // let transform = glm::rotate_z(&transform, model.rotation.z);

            gl.uniform_matrix4fv_with_f32_array(Some(&model_location), false, &value_ptr(&transform));

            model.mesh.enable(&gl);
            model.mesh.draw(&gl);
        }


        Ok(())
    }
}