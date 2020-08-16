use web_sys::*;

use super::link;
use super::super::model::Model;
use super::super::shaders::{vertex, fragment};
use super::super::math;

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

    pub fn render(&self, gl: &WebGl2RenderingContext, models: &[Model]) -> Result<(), String> {
        gl.use_program(Some(&self.program));

        let model_location = gl.
            get_uniform_location(&self.program, "model").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'model'.")?;
        let view_location = gl.
            get_uniform_location(&self.program, "view").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'model'.")?;

        let view = math::matrix::scale(1.0, 1.0, 1.0);
        gl.uniform_matrix4fv_with_f32_array(Some(&view_location),  false, &view);

        for model in models {
            gl.uniform_matrix4fv_with_f32_array(Some(&model_location), false, &model.transform);

            model.mesh.enable(&gl);
            model.mesh.draw(&gl);
        }


        Ok(())
    }
}