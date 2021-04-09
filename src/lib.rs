#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate nalgebra;
extern crate wasm_bindgen;
extern crate console_error_panic_hook;
extern crate web_sys;
extern crate js_sys;
extern crate nalgebra_glm as glm;

#[macro_use]
mod macros;

#[macro_use]
mod utils;

mod app;
mod camera;
mod mesh;
mod math;
mod shaders;
mod programs;
mod gl_setup;
mod materials;
mod renderer;
mod textures;

use std::sync::Arc;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use nalgebra::{Matrix4, Vector3};
use js_sys::Math::abs;

use crate::app::*;
use crate::camera::Camera;
use crate::mesh::{Model, Mesh, VERTICES_TEXTURE_AND_NORMAL_3D_CUBE};
use crate::renderer::Renderer;
use crate::utils::create_grid;
use crate::materials::{SingleColorMaterial, DefaultMaterial, DrawConfig};


macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(message: &str);
}


#[wasm_bindgen]
pub struct Client {
    gl: GL,
    canvas: web_sys::HtmlCanvasElement,
    models: Vec<Model>,
    camera: Camera,
}

// https://nalgebra.org/cg_recipes/


#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let (gl, canvas) = gl_setup::initialize_webgl_context().unwrap();

        attach_mouse_down_callback(&canvas, update_mouse_down)
            .expect("Couldn't attached mouse down callback!");
        attach_mouse_up_callback(&canvas, update_mouse_up)
            .expect("Couldn't attach mouse up callback!");
        attach_mouse_move_callback(&canvas, update_mouse_move)
            .expect("Couldn't attach mouse move callback!");
        attach_key_up_callback(&canvas, update_key_up)
            .expect("Couldn't attach key up callback!");
        attach_key_down_callback(&canvas, update_key_down)
            .expect("Couldn't attach key down callback!");

        let (vertices, indices) = create_grid(20, 20);

        let models = vec![
            Model::new(
                Mesh::from_f32_array_3d(&gl, &VERTICES_TEXTURE_AND_NORMAL_3D_CUBE, true, true, true).unwrap(),
                // DrawConfig::default(&gl),
                DrawConfig::new(GL::TRIANGLES, 0, -1, Box::new(DefaultMaterial::new(&gl).unwrap()))
            ),
            Model::new(
                Mesh::from_f32_array_with_indices_3d(&gl, &vertices, &indices).unwrap(),
                DrawConfig::new(GL::LINES, 0, -1, Box::new(SingleColorMaterial::new(&gl).unwrap()))
            )
        ];
        let camera = Camera::new();

        Client {
            gl,
            canvas,
            models,
            camera,
        }
    }

    fn get_window_size(window: &web_sys::Window) -> (u32, u32) {
        let width  = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
        (width, height)
    }

    fn should_resize_canvas(canvas: &web_sys::HtmlCanvasElement, width: u32, height: u32) -> bool {
        height != canvas.height() || width != canvas.width()
    }
    
    fn resize_canvas(gl: &GL, canvas: &web_sys::HtmlCanvasElement, width: u32, height: u32) {
        // TODO(ted): Why set both CSS and attribute?
        let style = (*canvas).style();
        style.set_property("height", format!("{}", height).as_str()).unwrap();
        style.set_property("width",  format!("{}", width).as_str()).unwrap();
        canvas.set_height(height);
        canvas.set_width(width);

        gl.viewport(0, 0, width as i32, height as i32);
    }

    pub fn update(&mut self, dt: f32, width: f32, height: f32) -> Result<(), JsValue> {
        // window: web_sys::Window
        // canvas: web_sys::HtmlElement
        // element: web_sys::Element = *canvas;

        // window = web_sys::window().expect("no global `window` exists");
        // document = window.document().expect("should have a document on window");
        // body = document.body().expect("document should have a body");
        if Self::should_resize_canvas(&self.canvas, width as u32, height as u32) {
            Self::resize_canvas(&self.gl, &self.canvas, width as u32, height as u32);
        }

        let state = app::get_state_of_frame_start(dt, width, height);

        if state.mouse_down {
            self.models[0].rotation.y -= (state.delta_mouse_x / state.canvas_width)  * std::f32::consts::PI * (dt/100.0);
            self.models[0].rotation.x -= (state.delta_mouse_y / state.canvas_height) * std::f32::consts::PI * (dt/100.0);
            self.models[0].position.x = (2.0 * state.mouse_x - state.canvas_width)  / state.canvas_width;
            self.models[0].position.y = (2.0 * state.mouse_y - state.canvas_height) / state.canvas_height;
        } else if state.mouse_locked {
            let offset_from_center_x = (state.mouse_x - state.canvas_width  / 2.0) / state.canvas_width;
            let offset_from_center_y = (state.mouse_y - state.canvas_height / 2.0) / state.canvas_height;

            if offset_from_center_x.abs() >= 0.1 {
                self.camera.direction.yaw -= offset_from_center_x * (dt/1000.0);
            }
            if offset_from_center_y.abs() >= 0.1 {
                self.camera.direction.pitch -= offset_from_center_y * (dt/1000.0);
            }
        }

        let forward   = state.key_pressed[KEY_FORWARD_INDEX]      as i32;
        let backward  = state.key_pressed[KEY_BACKWARDS_INDEX]    as i32;
        let left      = state.key_pressed[KEY_LEFT_INDEX]         as i32;
        let right     = state.key_pressed[KEY_RIGHT_INDEX]        as i32;
        let up        = state.key_pressed[KEY_UP_INDEX]           as i32;
        let down      = state.key_pressed[KEY_DOWN_INDEX]         as i32;
        let rot_left  = state.key_pressed[KEY_ROTATE_LEFT_INDEX]  as i32;
        let rot_right = state.key_pressed[KEY_ROTATE_RIGHT_INDEX] as i32;

        // Should take camera direction into account, i.e. movement should be local to camera.
        let delta_x = (right   - left)     as f32 * dt / 1000.0;
        let delta_y = (up      - down)     as f32 * dt / 1000.0;
        let delta_z = (forward - backward) as f32 * dt / 1000.0;

        self.camera.move_right(delta_x);
        self.camera.move_up(delta_y);
        self.camera.move_forward(delta_z);

        let rotation = (rot_left - rot_right) as f32 * dt / 1000.0;
        self.camera.rotate(rotation, 0.0, 0.0);

        // log(format!("Key input: {}, {}, {} | {}", &delta_x, &delta_y, &delta_z, &rotation).as_str());
        // log(format!("Mouse input: {}, {}", &state.delta_mouse_x, &state.delta_mouse_y).as_str());
        // log(format!("Cam pos:     {}, {}, {}", &self.camera.position.x, &self.camera.position.y, &self.camera.position.z).as_str());
        // log(format!("Cam right:   {}, {}, {}", &self.camera.direction.right.x, &self.camera.direction.right.y, &self.camera.direction.right.z).as_str());
        // log(format!("Cam up:      {}, {}, {}", &self.camera.direction.up.x, &self.camera.direction.up.y, &self.camera.direction.up.z).as_str());
        // log(format!("Cam forward: {}, {}, {}", &self.camera.direction.forward.x, &self.camera.direction.forward.y, &self.camera.direction.forward.z).as_str());

        Ok(())
    }

    pub fn render(&self) {
        // let state = app::get_current_state();
        // log(format!("Keys: {:?} | Mouse: {}", state.key_pressed, state.mouse_down).as_str());

        self.gl.clear_color(0.1, 0.1, 0.1, 0.1);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        Renderer::draw(&self.gl, &self.models.as_slice(), &self.camera).unwrap();

        let error = self.gl.get_error();
        if error != GL::NO_ERROR {
            log(format!("GL ERROR: {}", error).as_str());
            log(format!("Test: {:?}", vectors::Vector3::default()).as_str());
        }
    }
}
