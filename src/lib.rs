extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(message: &str);
}

#[wasm_bindgen]
pub fn hello() {
    log("Hello from Rust!");
}


#[wasm_bindgen]
pub struct Client {

}


#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Client {

        }
    }

    pub fn update(&mut self, _dt: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
    }
}