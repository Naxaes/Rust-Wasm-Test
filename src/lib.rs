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
mod model;
mod app;
mod camera;
mod mesh;
mod math;
mod shaders;
mod program;
mod gl_setup;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

use app::{AppState, attach_mouse_down_callback, attach_mouse_up_callback, attach_mouse_move_callback, update_canvas_and_time, attach_key_up_callback, attach_key_down_callback};
use app::*;
use model::Model;
use camera::Camera;
use nalgebra::{Matrix4, Vector3};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(message: &str);
}


#[wasm_bindgen]
pub struct Client {
    gl: GL,
    canvas: web_sys::HtmlCanvasElement,
    default: program::Default,
    models: [Model; 1],
    time: f32,
    camera: Camera,
}

// https://nalgebra.org/cg_recipes/


#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let (gl, canvas) = gl_setup::initialize_webgl_context().unwrap();
        let default = program::Default::new(&gl).unwrap();

        attach_mouse_down_callback(&canvas).unwrap();
        attach_mouse_up_callback(&canvas).unwrap();
        attach_mouse_move_callback(&canvas).unwrap();
        attach_key_up_callback(&canvas).unwrap();
        attach_key_down_callback(&canvas).unwrap();

        let models = [Model::new(&gl).unwrap()];
        let time   = 0.0;
        let camera = Camera::new();

        Client {
            gl,
            canvas,
            default,
            models,
            time,
            camera,
        }
    }

    fn resize_window(&self) {
        // window: web_sys::Window
        // canvas: web_sys::HtmlElement
        // element: web_sys::Element = *canvas;

        let window = web_sys::window().unwrap();
        let canvas = &self.canvas;

        let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
        let width  = window.inner_width().unwrap().as_f64().unwrap() as u32;

        if height != canvas.height() || width != canvas.width() {
            // TODO(ted): Why set both CSS and attribute?
            let style = (*canvas).style();
            style.set_property("height", format!("{}", height).as_str()).unwrap();
            style.set_property("width",  format!("{}", width).as_str()).unwrap();
            canvas.set_height(height);
            canvas.set_width(width);

            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn update(&mut self, dt: f32, height: f32, width: f32) -> Result<(), JsValue> {
        self.resize_window();
        update_canvas_and_time(dt, height, width);  // TODO(ted): Cache changes and do them all at once.

        let current_state = app::get_current_state();

        if current_state.mouse_down {
            self.models[0].rotation.y -= (current_state.delta_mouse_x / current_state.canvas_width)  * 2.0 * std::f32::consts::PI;
            self.models[0].rotation.x += (current_state.delta_mouse_y / current_state.canvas_height) * 2.0 * std::f32::consts::PI;
            // self.models[0].position.x = (2.0 * current_state.mouse_x - current_state.canvas_width)  / current_state.canvas_width;
            // self.models[0].position.y = (2.0 * current_state.mouse_y - current_state.canvas_height) / current_state.canvas_height;
        }

        let forward  = current_state.key_pressed[KEY_FORWARD_INDEX]   as i32;
        let backward = current_state.key_pressed[KEY_BACKWARDS_INDEX] as i32;
        let left     = current_state.key_pressed[KEY_LEFT_INDEX]      as i32;
        let right    = current_state.key_pressed[KEY_RIGHT_INDEX]     as i32;
        let up       = current_state.key_pressed[KEY_UP_INDEX]        as i32;
        let down     = current_state.key_pressed[KEY_DOWN_INDEX]      as i32;

        // Should take camera direction into account, i.e. movement should be local to camera.
        self.camera.position.x += (right   - left)     as f32 * dt / 1000.0;
        self.camera.position.y += (down    - up)       as f32 * dt / 1000.0;
        self.camera.position.z += (forward - backward) as f32 * dt / 1000.0;

        self.time += dt;

        Ok(())
    }

    pub fn render(&self) {
        // let current_state = app::get_current_state();
        // log(format!("Keys: {:?} | Mouse: {}", current_state.key_pressed, current_state.mouse_down).as_str());

        self.gl.clear_color(0.1, 0.1, 0.1, 0.1);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.default.render(&self.gl, &self.models, &self.camera).unwrap();

        let error = self.gl.get_error();
        if error != GL::NO_ERROR {
            log(format!("GL ERROR: {}", error).as_str());
        }
    }
}
