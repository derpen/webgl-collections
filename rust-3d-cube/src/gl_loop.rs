use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::*;
use web_sys::{console,WebGl2RenderingContext,WebGlShader,WebGlProgram, HtmlCanvasElement};
use crate::scene;
use crate::shader_utils;

// This entire file is just to set up the Loop
// Honestly, the file name is very misleading
// The entire code is messy too, will need to rewrite this at some point
// Real loop code happens in scene.rs

#[derive(Clone)]
#[wasm_bindgen]
pub struct GlConfig {
    // These are canvas height and width
    width: i32,
    height: i32,
    canvas: HtmlCanvasElement,
}

impl GlConfig {
    pub fn new(
        new_width: i32,
        new_height: i32,
        new_canvas: &HtmlCanvasElement
    ) ->Self {
        GlConfig{
            width: new_width,
            height: new_height,
            canvas: new_canvas.clone()
        }
    }

    pub fn set_new_canvas_size(
        &mut self,
        new_width: i32,
        new_height: i32
    ) {
        self.width = new_width;
        self.height = new_height;
    }

    pub fn get_width(&self) -> i32{
        self.width
    }

    pub fn get_height(&self) -> i32{
        self.height
    }

    pub fn get_canvas(&self) -> HtmlCanvasElement{
        self.canvas.clone()
    }
}

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
    shader: shader_utils::GlShader,
    config: GlConfig
){
    let start_time = performance().now();
    let current_shader = Rc::new(shader.clone());
    let context_rc = Rc::new(context.clone());
    let config_rc = Rc::new(config.clone());

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        // Loop shit here
        let elapsed_time = performance().now() - start_time;
        render_loop(elapsed_time, &*context_rc, (*current_shader).clone(),  (*config_rc).clone());

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
    shader: shader_utils::GlShader,
    config: GlConfig
) {
    //console::log_1(&JsValue::from_str(&format!("Elapsed Time: {:2} seconds", delta / 1000.0)));

    // TODO: Maybe draw shouldn't take any arguments
    scene::draw(&context, 36, delta / 1000.0, shader, config); // TODO: this vertex count draw needs to be
                                                       // moved somewhere
}

