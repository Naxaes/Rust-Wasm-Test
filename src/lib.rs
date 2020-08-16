#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate nalgebra;
extern crate wasm_bindgen;
extern crate console_error_panic_hook;
extern crate web_sys;
extern crate js_sys;

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

use app::{AppState, attach_mouse_down_handler, attach_mouse_up_handler, attach_mouse_move_handler, update_canvas_and_time};
use model::Model;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(message: &str);
}


#[wasm_bindgen]
pub struct Client {
    gl: WebGl2RenderingContext,
    canvas: web_sys::HtmlCanvasElement,
    default: program::Default,
    models: [Model; 1],
    time: f32
}


#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let (gl, canvas) = gl_setup::initialize_webgl_context().unwrap();
        let default = program::Default::new(&gl).unwrap();

        attach_mouse_down_handler(&canvas).unwrap();
        attach_mouse_up_handler(&canvas).unwrap();
        attach_mouse_move_handler(&canvas).unwrap();

        let models = [Model::new(&gl).unwrap()];

        Client {
            gl,
            canvas,
            default,
            models,
            time: 1.0
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
        log(format!("{:.2} {} {}", dt, self.time, self.time.sin()).as_str());
        self.time += dt;
        self.models[0].transform[12] = self.time.sin();

        update_canvas_and_time(dt, height, width);
        self.resize_window();
        Ok(())
    }

    pub fn render(&self) {
        let current_state = app::get_current_state();

        self.gl.clear_color(
            current_state.mouse_x / current_state.canvas_width,
            current_state.mouse_y / current_state.canvas_height,
            0.0,
            1.0
        );
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.default.render(&self.gl, &self.models).unwrap();

        let error = self.gl.get_error();
        if error != GL::NO_ERROR {
            log(format!("GL ERROR: {}", error).as_str());
        }
    }
}
