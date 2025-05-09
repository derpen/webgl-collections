use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext, Window};
use wasm_bindgen::JsValue;

mod shader_utils;
mod gl_loop;
mod scene;

#[wasm_bindgen(start)]
async fn start() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    //console::log_1(&JsValue::from_str("Hello triangle!"));

    let window : Window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    // size
    let width_size = window.inner_width().unwrap();
    let height_size = window.inner_height().unwrap();
    context.viewport(0, 0, width_size.as_f64().unwrap() as i32, height_size.as_f64().unwrap() as i32);

    let vert_shader = String::from("shaders/cube/cube.frag");
    let frag_shader = String::from("shaders/cube/cube.vert");

    let triangle_shader = shader_utils::create_shader_program(
        &context,
        vert_shader,
        frag_shader
        ).await?; // TODO: can i not do this ignore bullshit

    let vao = scene::triangle_init(&context, triangle_shader.clone()).await?;
    context.bind_vertex_array(Some(&vao));

    context.use_program(Some(&triangle_shader.get_shader_program().unwrap()));
    gl_loop::animate(&context, triangle_shader);

    Ok(())
}
