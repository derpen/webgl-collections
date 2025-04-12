use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, console};
use js_sys;
use crate::shader_utils;

#[wasm_bindgen]
pub async fn triangle_init(context: &WebGl2RenderingContext) -> Result<shader_utils::GlShader, JsValue> {
    let frag_shader_location = String::from("shaders/cube/cube.frag");
    let frag_shader = shader_utils::read_shader(frag_shader_location).await.unwrap();

    let vert_shader_location = String::from("shaders/cube/cube.vert");
    let vert_shader = shader_utils::read_shader(vert_shader_location).await.unwrap();

    let vert_shader = shader_utils::compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        &vert_shader.as_string().unwrap(),
    )?;

    let frag_shader = shader_utils::compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        &frag_shader.as_string().unwrap(),
    )?;

    let mut gl_shader = shader_utils::GlShader::new(&context);

    let program = gl_shader.link_program(&vert_shader, &frag_shader).unwrap();

    context.use_program(Some(&program));

    let vertices: [f32; 9] = [
        -0.7, -0.7, 0.0, 
        0.7, -0.7, 0.0, 
        0.0, 0.7, 0.0];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    //let vert_count = (vertices.len() / 3) as i32;
    //draw(&context, vert_count);

    Ok(gl_shader)
}

pub fn draw(
    context: &WebGl2RenderingContext, 
    vert_count: i32,
    delta: f64,
    shader: shader_utils::GlShader
) {
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    //console::log_1(&JsValue::from_str("Is this looping"));

    shader.set_float("u_time".to_string(), delta);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

