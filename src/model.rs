use web_sys::WebGl2RenderingContext as GL;
use glm::Vec3;

use super::mesh::{Mesh, VERTICES_TEXTURE_AND_NORMAL_3D_CUBE};
use super::math;


pub struct Model {
    pub mesh: Mesh,
    pub position: Vec3,
    pub rotation: Vec3,
}

impl Model {
    pub fn new(gl: &GL) -> Result<Model, String> {
        let mesh = Mesh::from_f32_array_3d(gl, &VERTICES_TEXTURE_AND_NORMAL_3D_CUBE, true, true)?;
        let position = Vec3::new(0.0, 0.0, 0.0);
        let rotation = Vec3::new(0.0, 0.0, 0.0);

        Ok(Self {
            mesh, position, rotation
        })
    }
}