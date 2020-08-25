use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct ContextOptions {
    antialias: bool,
}


pub fn initialize_webgl_context() -> Result<(WebGl2RenderingContext, web_sys::HtmlCanvasElement), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl: WebGl2RenderingContext = canvas
        .get_context_with_context_options(
            "webgl2", &JsValue::from_serde(
                &ContextOptions { antialias: true }
            ).unwrap())?
        .unwrap()
        .dyn_into()?;

    // gl.enable(GL::BLEND);
    // gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    gl.enable(GL::DEPTH_TEST);
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    // gl.clear_depth(1.);


    Ok((gl, canvas))
}

