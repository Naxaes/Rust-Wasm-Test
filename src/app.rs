use std::sync::Arc;
use std::sync::Mutex;

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::{EventListener, HtmlCanvasElement};
use super::log;


const KEY_FORWARD   : &str = "KeyW";
const KEY_BACKWARDS : &str = "KeyS";
const KEY_LEFT      : &str = "KeyA";
const KEY_RIGHT     : &str = "KeyD";
const KEY_UP        : &str = "Space";
const KEY_DOWN      : &str = "ControlLeft";
const KEY_ROTATE_LEFT  : &str = "KeyQ";
const KEY_ROTATE_RIGHT : &str = "KeyE";

pub const KEY_FORWARD_INDEX   : usize = 0;
pub const KEY_LEFT_INDEX      : usize = 1;
pub const KEY_BACKWARDS_INDEX : usize = 2;
pub const KEY_RIGHT_INDEX     : usize = 3;
pub const KEY_UP_INDEX        : usize = 4;
pub const KEY_DOWN_INDEX      : usize = 5;
pub const KEY_ROTATE_LEFT_INDEX  : usize = 6;
pub const KEY_ROTATE_RIGHT_INDEX : usize = 7;


lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn get_current_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub mouse_down: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub delta_mouse_x: f32,
    pub delta_mouse_y: f32,
    pub time: f32,

    pub key_pressed: [bool; 8]
}

impl AppState {
    pub fn new() -> Self {
        Self {
            canvas_height: 0.,
            canvas_width: 0.,
            mouse_down: false,
            mouse_x: -1.,
            mouse_y: -1.,
            delta_mouse_x: 0.,
            delta_mouse_y: 0.,
            time: 0.,
            key_pressed: [false, false, false, false, false, false, false, false],
        }
    }
}

pub fn update_mouse_pressed(x: f32, y: f32, is_down: bool) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        mouse_down: is_down,
        mouse_x: x,
        mouse_y: data.canvas_height - y,
        ..*data.clone()
    });
}

pub fn update_mouse_position(x: f32, y: f32) {
    let mut data = APP_STATE.lock().unwrap();
    let inverted_y = data.canvas_height - y;

    *data = Arc::new(AppState {
        mouse_x: x,
        mouse_y: inverted_y,
        delta_mouse_x: data.mouse_x - x,
        delta_mouse_y: data.mouse_y - inverted_y,
        ..*data.clone()
    });
}


pub fn update_canvas_and_time(time: f32, canvas_height: f32, canvas_width: f32) {
    let mut data = APP_STATE.lock().unwrap();

    *data = Arc::new(AppState {
        canvas_height: canvas_height,
        canvas_width: canvas_width,
        time: time,
        ..*data.clone()
    });
}

pub fn update_key_press(code: &String, down: bool) {
    log(format!("Code: {}, down: {}", &code, down).as_str());
    let index =
        if *code == KEY_FORWARD {
            KEY_FORWARD_INDEX
        } else if *code == KEY_LEFT {
            KEY_LEFT_INDEX
        } else if *code == KEY_BACKWARDS {
            KEY_BACKWARDS_INDEX
        } else if *code == KEY_RIGHT {
            KEY_RIGHT_INDEX
        } else if *code == KEY_UP {
            KEY_UP_INDEX
        } else if *code == KEY_DOWN {
            KEY_DOWN_INDEX
        } else if *code == KEY_ROTATE_LEFT {
            KEY_ROTATE_LEFT_INDEX
        } else if *code == KEY_ROTATE_RIGHT {
            KEY_ROTATE_RIGHT_INDEX
        } else {
            return;
        };

    let mut data = APP_STATE.lock().unwrap();
    let mut key_pressed = data.key_pressed;
    key_pressed[index] = down;

    *data = Arc::new(AppState {
        key_pressed,
        ..*data.clone()
    });
}


// ---- ATTACHMENTS ----

pub fn attach_mouse_down_callback(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let callback = move |event: web_sys::MouseEvent| {
        update_mouse_pressed(event.client_x() as f32, event.client_y() as f32, true);
    };

    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", callback.as_ref().unchecked_ref())?;
    callback.forget();

    Ok(())
}

pub fn attach_mouse_up_callback(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let callback = move |event: web_sys::MouseEvent| {
        update_mouse_pressed(event.client_x() as f32, event.client_y() as f32, false);
    };

    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", callback.as_ref().unchecked_ref())?;
    callback.forget();

    Ok(())
}

pub fn attach_mouse_move_callback(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let callback = move |event: web_sys::MouseEvent| {
        update_mouse_position(event.client_x() as f32, event.client_y() as f32);
    };

    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", callback.as_ref().unchecked_ref())?;
    callback.forget();

    Ok(())
}

// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html
pub fn attach_key_down_callback(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let callback = move |event: web_sys::KeyboardEvent| {
        update_key_press(&event.code(), true);
    };

    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())?;
    callback.forget();

    log("Attached key down callback.");
    Ok(())
}


pub fn attach_key_up_callback(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let callback = move |event: web_sys::KeyboardEvent| {
        update_key_press(&event.code(), false);
    };

    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("keyup", callback.as_ref().unchecked_ref())?;
    callback.forget();

    log("Attached key up callback.");
    Ok(())
}


