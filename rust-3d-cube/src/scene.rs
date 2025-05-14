use wasm_bindgen::prelude::*;
use web_sys::{
    WebGl2RenderingContext, 
    WebGlProgram, 
    WebGlVertexArrayObject, 
    HtmlImageElement, 
    console,
    KeyboardEvent
};
use js_sys;
use crate::shader_utils;
use crate::camera;
use crate::gl_loop;
use glm::{Mat4, Vector3, Vector4, radians};
use glm::ext::{rotate, translate};

#[wasm_bindgen]
pub async fn triangle_init(
    context: &WebGl2RenderingContext,
    program: shader_utils::GlShader,
    ) -> Result<WebGlVertexArrayObject, JsValue> {
    let vertices: [f32; 9] = [
        -0.7, -0.7, 0.0, 
        0.7, -0.7, 0.0, 
        0.0, 0.7, 0.0];

    let position_attribute_location = context.get_attrib_location(&program.get_shader_program().unwrap(), "position"); // Why do
                                                                                         // u need
                                                                                         // program?
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

    //let vert_count = (vertices.len() / 3) as i32;
    //draw(&context, vert_count);

    Ok(vao)
}

#[wasm_bindgen]
pub async fn cube_init(
    context: &WebGl2RenderingContext,
    program: shader_utils::GlShader,
    ) -> Result<WebGlVertexArrayObject, JsValue> {
    let vertices: [f32; 6 * 6 * 8] = [ // holy shit I can just do that?
        // positions          // normals           // texture coords
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
    ];

    let position_attribute_location = context.get_attrib_location(&program.get_shader_program().unwrap(), "position"); // Why do
                                                                                         // u need
                                                                                         // program?
    let normal_attribute_location = context.get_attrib_location(&program.get_shader_program().unwrap(), "normal"); 
    let text_coord_attribute_location = context.get_attrib_location(&program.get_shader_program().unwrap(), "texcoord"); 

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
        // TODO
        // Might wanna rename this?
        // Since now it also includes normal and texcoords
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
        32,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.vertex_attrib_pointer_with_i32(
        normal_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        32,
        12,
    );
    context.enable_vertex_attrib_array(normal_attribute_location as u32);

    context.vertex_attrib_pointer_with_i32(
        text_coord_attribute_location as u32,
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        32,
        24,
    );
    context.enable_vertex_attrib_array(text_coord_attribute_location as u32);

    // This probably should be moved somewhere else for better visibility
    let _ = read_texture(&context, String::from("images/image.jpg")).await;

    Ok(vao)
}

#[wasm_bindgen]
pub async fn read_texture(
    context: &WebGl2RenderingContext,
    url_path: String
    ) -> Result<(), JsValue> {
    // Setting image buffers and shits
    let image_buffer = context
        .create_buffer()
        .ok_or("Could not create vertex array object")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&image_buffer));
    let texture = context.create_texture().ok_or("Could not create texture")?;
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D, 
        WebGl2RenderingContext::TEXTURE_WRAP_S, 
        WebGl2RenderingContext::REPEAT as i32
        );
    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D, 
        WebGl2RenderingContext::TEXTURE_WRAP_T, 
        WebGl2RenderingContext::REPEAT as i32
        );
    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D, 
        WebGl2RenderingContext::TEXTURE_MIN_FILTER, 
        WebGl2RenderingContext::LINEAR as i32
        );
    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D, 
        WebGl2RenderingContext::TEXTURE_MAG_FILTER, 
        WebGl2RenderingContext::LINEAR as i32
        );

    // Actually handling the texture reading
    let image = HtmlImageElement::new().unwrap();
    image.set_src(&url_path);

    let image_clone = image.clone();
    let context_clone = context.clone();

    let onload_callback = Closure::wrap(Box::new(move || {
        let _ = context_clone.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            &image_clone
        );
    }) as Box<dyn FnMut()>);

    image.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
    onload_callback.forget();
    //context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
    context.active_texture(WebGl2RenderingContext::TEXTURE0);

    Ok(())
}

pub fn draw(
    context: &WebGl2RenderingContext, 
    vert_count: i32,
    delta: f64,
    shader: shader_utils::GlShader,
    config: gl_loop::GlConfig
) {
    resize_canvas(&config, &context);

    context.clear_color(0.05, 0.05, 0.05, 1.0);

    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    //console::log_1(&JsValue::from_str("Is this looping"));

    let _ = shader.set_float("u_time".to_string(), delta);

    let mut model: Mat4 = Mat4::new(
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    );

    model = rotate(&model, radians(45.0) + delta as f32, Vector3::new(1.0, 1.0, 0.0)); // Why does
                                                                                       // plus
                                                                                       // works
    model = translate(&model, Vector3::new(0.0, -0.5, 0.0));

    let _ = shader.set_mat4("model".to_string(), model);

    // Cringe temporary hack to use camera
    // TODO: Please make it so that it doesn't have to reinitialize each time
    let camera = camera::Camera::new( // Wow this naming suck
        Vector3::new(0.0, 0.0, 5.0),
        config.get_canvas().client_width() as f32,
        config.get_canvas().client_height() as f32
        ); 

    //handle_input();

    let view_matrix = camera.get_view_matrix();
    let projection_matrix = camera.get_projection_matrix();

    let _ = shader.set_mat4("view".to_string(), view_matrix);
    let _ = shader.set_mat4("projection".to_string(), projection_matrix);

    let _ = shader.set_int("image".to_string(), 0);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

fn resize_canvas(
    config: &gl_loop::GlConfig,
    context: &WebGl2RenderingContext
    ) {
    let current_canvas = config.get_canvas();
    let previous_width = config.get_width();
    let previous_height = config.get_height();
    if previous_width != current_canvas.client_width() || previous_height != current_canvas.client_height() {
        //console::log_1(&JsValue::from_str("We set a new canvas size one here"));
        current_canvas.set_width(current_canvas.client_width() as u32);
        current_canvas.set_height(current_canvas.client_height() as u32);
        context.viewport(0, 0, current_canvas.client_width(), current_canvas.client_height());

        // TODO: need this to work somehow
        // As of now, its working, but this keep firing since I didn't update the real thing
        //config.set_new_canvas_size(current_canvas.client_width(), current_canvas.client_height());
    }
}

pub fn handle_input(event: KeyboardEvent){
    console::log_1(&format!("Key pressed: {}", event.key()).into());
}
