use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::*;
use web_sys::{console,WebGl2RenderingContext,WebGlShader,WebGlProgram};
use crate::scene;
use crate::shader_utils;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("performance should be available")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn animate(
    context: &WebGl2RenderingContext,
    shader: shader_utils::GlShader
){
    let start_time = performance().now();
    let current_shader = Rc::new(shader.clone());
    let context_rc = Rc::new(context.clone());

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        // Loop shit here
        let elapsed_time = performance().now() - start_time;
        render_loop(elapsed_time, &*context_rc, (*current_shader).clone());

        request_animation_frame(f.borrow().as_ref().unwrap());

    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

// TODO
// This is not delta
// It's just time elapsed
fn render_loop(
    delta: f64,
    context: &WebGl2RenderingContext,
    shader: shader_utils::GlShader
) {
    console::log_1(&JsValue::from_str(&format!("Elapsed Time: {:2} seconds", delta / 1000.0)));

    context.clear_color(0.05, 0.05, 0.05, 1.0);

    // TODO: Maybe draw shouldn't take any arguments
    scene::draw(&context, 36, delta / 1000.0, shader); // TODO: this vertex count draw needs to be
                                                       // moved somewhere
}

