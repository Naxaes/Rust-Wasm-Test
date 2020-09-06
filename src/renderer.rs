use web_sys::WebGl2RenderingContext as GL;
use glm::{Mat4, Vec3, value_ptr};

use crate::Camera;
use crate::mesh::Model;

pub struct Renderer {}

impl Renderer {
    pub fn draw(gl: &GL, models: &[Model], camera: &Camera) -> Result<(), String> {
        // Fastest if meshes is sorted by (material id, mesh id, draw_mode).
        // let mut bound_material = 0;
        // let mut bound_mesh     = 0;

        for model in models.iter() {
            let material = &model.draw_config.material;

            // if bound_material != config.material {
            material.enable(gl, camera);
            // bound_material = config.material;
            // }
            // if bound_mesh != mesh.id {
            model.enable(gl);
            // bound_mesh = mesh.id;
            // }

            material.upload(gl, model);
            model.enable(gl);
            model.draw(gl);
        }

        Ok(())
    }

}