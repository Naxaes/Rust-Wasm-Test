use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use glm::{Mat4, Vec3, value_ptr};

use super::link;
use super::super::log;
use super::super::model::Model;
use super::super::shaders::{vertex, fragment};
use super::super::math;
use super::super::camera::Camera;
use super::super::mesh::Mesh;


pub struct Grid {
    program: WebGlProgram,
    grid: Mesh,

    // Uniforms.
    // view: WebGlUniformLocation,
    // projection: WebGlUniformLocation,
}

impl Grid {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, String> {
        let program = link(
            gl,
            &vertex::grid::new(gl)?,
            &fragment::grid::new(gl)?,
        )?;

        let mut vertices = Vec::with_capacity(20 * 20 * 3);
        for x in -10..10 {
            for z in -10..10 {
                vertices.push(x as f32 + 0.5);
                vertices.push(0.0);
                vertices.push(z as f32 + 0.5);
            }
        }

        let mut indices = Vec::with_capacity(10 * 3);
        for z in 0..20 {
            for x in 0..20 {
                let index = z + x * 20;

                if z < 19 {
                    indices.push(index as u16);
                    indices.push((index + 1) as u16);
                }
                if x < 19 {
                    indices.push(index as u16);
                    indices.push((index + 20) as u16);
                }
            }
        }

        let grid = Mesh::from_f32_array_with_indices_3d(gl, &vertices, &indices).unwrap();

        Ok(Self {
            program,
            grid
        })
    }

    pub fn render(&self, gl: &GL, camera: &Camera) -> Result<(), String> {
        gl.use_program(Some(&self.program));

        let view_location = gl.
            get_uniform_location(&self.program, "view").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'view'.")?;
        let proj_location = gl.
            get_uniform_location(&self.program, "projection").
            ok_or("[WEBGL2 - UNIFORM ERROR]: Couldn't get uniform 'projection'.")?;

        let camera_position = -&camera.position;
        gl.uniform_matrix4fv_with_f32_array(Some(&view_location), false, &value_ptr(&glm::translation(&camera_position)));

        gl.uniform_matrix4fv_with_f32_array(Some(&proj_location), false, &value_ptr(&glm::perspective(16.0 / 9.0, 3.14 / 2.0, 1.0, 1000.0)));

        self.grid.enable(gl);
        gl.draw_elements_with_i32(GL::LINES, self.grid.count, GL::UNSIGNED_SHORT, 0);

        Ok(())
    }
}