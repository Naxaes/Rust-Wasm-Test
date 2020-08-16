use web_sys::WebGl2RenderingContext as GL;

use super::mesh::{Mesh, VERTICES_3D_INDEXED_RECTANGLE, INDICES_RECTANGLE};
use super::math;


pub struct Model {
    pub mesh: Mesh,
    pub transform: [f32; 16],
}

impl Model {
    pub fn new(gl: &GL) -> Result<Model, String> {
        let mesh = Mesh::from_f32_array(gl, &VERTICES_3D_INDEXED_RECTANGLE, &INDICES_RECTANGLE)?;
        let transform = math::matrix::scale(1.0, 1.0, 1.0);

        Ok(Self {
            mesh, transform
        })
    }
}