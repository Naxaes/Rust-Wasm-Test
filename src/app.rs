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
const KEY_TOGGLE_MOUSE_LOCK : &str = "Escape";

pub const KEY_FORWARD_INDEX   : usize = 1;
pub const KEY_LEFT_INDEX      : usize = 2;
pub const KEY_BACKWARDS_INDEX : usize = 3;
pub const KEY_RIGHT_INDEX     : usize = 4;
pub const KEY_UP_INDEX        : usize = 5;
pub const KEY_DOWN_INDEX      : usize = 6;
pub const KEY_ROTATE_LEFT_INDEX  : usize = 7;
pub const KEY_ROTATE_RIGHT_INDEX : usize = 8;


lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn get_current_state() -> AppState {
    *APP_STATE.lock().unwrap().clone()
}
pub fn set_current_state(state: AppState) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(state);
}
pub fn get_state_of_frame_start(dt: f32, width: f32, height: f32) -> AppState {
    let mut data  = APP_STATE.lock().unwrap();
    let new_state = AppState {
        canvas_height: height,
        canvas_width: width,
        time: data.time + dt,
        ..*data.clone()
    };

    *data = Arc::new(new_state.clone());
    new_state
}



#[derive(Debug, Copy, Clone)]
pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub mouse_locked: bool,
    pub mouse_down: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub delta_mouse_x: f32,
    pub delta_mouse_y: f32,
    pub time: f32,

    pub key_pressed: [bool; 9]
}

impl AppState {
    pub fn new() -> Self {
        Self {
            canvas_height: 0.,
            canvas_width: 0.,
            mouse_locked: true,
            mouse_down: false,
            mouse_x: -1.,
            mouse_y: -1.,
            delta_mouse_x: 0.,
            delta_mouse_y: 0.,
            time: 0.,
            key_pressed: [false, false, false, false, false, false, false, false, false],
        }
    }
}

pub fn update_mouse_down(event: web_sys::MouseEvent) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        mouse_down: true,
        mouse_x: event.client_x() as f32,
        mouse_y: data.canvas_height - event.client_y() as f32,
        ..*data.clone()
    });
}

pub fn update_mouse_up(event: web_sys::MouseEvent) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        mouse_down: false,
        mouse_x: event.client_x() as f32,
        mouse_y: data.canvas_height - event.client_y() as f32,
        ..*data.clone()
    });
}

pub fn update_mouse_move(event: web_sys::MouseEvent) {
    let mut data = APP_STATE.lock().unwrap();
    let inverted_y = data.canvas_height - event.client_y() as f32;

    *data = Arc::new(AppState {
        mouse_x: event.client_x() as f32,
        mouse_y: inverted_y,
        delta_mouse_x: data.mouse_x - event.client_y() as f32,
        delta_mouse_y: data.mouse_y - inverted_y,
        ..*data.clone()
    });
}

fn get_key_index(code: &str) -> usize {
    if code == KEY_FORWARD {
        KEY_FORWARD_INDEX
    } else if code == KEY_LEFT {
        KEY_LEFT_INDEX
    } else if code == KEY_BACKWARDS {
        KEY_BACKWARDS_INDEX
    } else if code == KEY_RIGHT {
        KEY_RIGHT_INDEX
    } else if code == KEY_UP {
        KEY_UP_INDEX
    } else if code == KEY_DOWN {
        KEY_DOWN_INDEX
    } else if code == KEY_ROTATE_LEFT {
        KEY_ROTATE_LEFT_INDEX
    } else if code == KEY_ROTATE_RIGHT {
        KEY_ROTATE_RIGHT_INDEX
    } else {
        0
    }
}

pub fn update_key_down(event: web_sys::KeyboardEvent) {
    let index = get_key_index(&event.code());
    let mut data = APP_STATE.lock().unwrap();
    let mut key_pressed = data.key_pressed;
    key_pressed[index] = true;

    *data = Arc::new(AppState {
        key_pressed,
        ..*data.clone()
    });
}

pub fn update_key_up(event: web_sys::KeyboardEvent) {
    let code = event.code();

    if code == KEY_TOGGLE_MOUSE_LOCK {
        let mut data = APP_STATE.lock().unwrap();
        let mouse_locked = !data.mouse_locked;
        *data = Arc::new(AppState {
            mouse_locked,
            ..*data.clone()
        });
        let window   = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body     = document.body().expect("document should have a body");
        if mouse_locked {
            body.request_pointer_lock();
        } else {
            document.exit_pointer_lock();
        }
        return;
    }

    let index = get_key_index(&event.code());
    let mut data = APP_STATE.lock().unwrap();
    let mut key_pressed = data.key_pressed;
    key_pressed[index] = false;

    *data = Arc::new(AppState {
        key_pressed,
        ..*data.clone()
    });
}


// ---- ATTACHMENTS ----
pub fn attach_mouse_down_callback(canvas: &HtmlCanvasElement, callback: fn(web_sys::MouseEvent)) -> Result<(), JsValue> {
    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(())
}

pub fn attach_mouse_up_callback(canvas: &HtmlCanvasElement, callback: fn(event: web_sys::MouseEvent)) -> Result<(), JsValue> {
    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(())
}

pub fn attach_mouse_move_callback(canvas: &HtmlCanvasElement, callback: fn(event: web_sys::MouseEvent)) -> Result<(), JsValue> {
    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(())
}

// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html
pub fn attach_key_down_callback(canvas: &HtmlCanvasElement, callback: fn(event: web_sys::KeyboardEvent)) -> Result<(), JsValue> {
    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(())
}

pub fn attach_key_up_callback(canvas: &HtmlCanvasElement, callback: fn(event: web_sys::KeyboardEvent)) -> Result<(), JsValue> {
    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("keyup", callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(())
}




//
// pub fn attach_mouse_up_callback(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
//     let callback = move |event: web_sys::MouseEvent| {
//         update_mouse_pressed(event.client_x() as f32, event.client_y() as f32, false);
//     };
//
//     let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
//     canvas.add_event_listener_with_callback("mouseup", callback.as_ref().unchecked_ref())?;
//     callback.forget();
//
//     Ok(())
// }
//
// pub fn attach_mouse_move_callback(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
//     let callback = move |event: web_sys::MouseEvent| {
//         update_mouse_position(event.client_x() as f32, event.client_y() as f32);
//     };
//
//     let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
//     canvas.add_event_listener_with_callback("mousemove", callback.as_ref().unchecked_ref())?;
//     callback.forget();
//
//     Ok(())
// }
//
// // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html
// pub fn attach_key_down_callback(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
//     let callback = move |event: web_sys::KeyboardEvent| {
//         update_key_press(&event.code(), true);
//     };
//
//     let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
//     canvas.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())?;
//     callback.forget();
//
//     log("Attached key down callback.");
//     Ok(())
// }
//
//
// pub fn attach_key_up_callback(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
//     let callback = move |event: web_sys::KeyboardEvent| {
//         update_key_press(&event.code(), false);
//     };
//
//     let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
//     canvas.add_event_listener_with_callback("keyup", callback.as_ref().unchecked_ref())?;
//     callback.forget();
//
//     log("Attached key up callback.");
//     Ok(())
// }
//
//
