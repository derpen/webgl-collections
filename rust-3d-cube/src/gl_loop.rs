use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::*;
use web_sys::console;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("performance should be available")
}

pub fn animate() {
    let start_time = performance().now();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        // Loop shit here
        let elapsed_time = performance().now() - start_time;
        console::log_1(&JsValue::from_str(&format!("Elapsed Time: {:2} seconds", elapsed_time / 1000.0)));


        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
