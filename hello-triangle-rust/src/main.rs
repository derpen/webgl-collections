use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlShader};

#[wasm_bindgen]
pub fn main() {
    // Get the canvas element
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

    // Get the WebGL context
    let gl = canvas.get_context("webgl").unwrap().unwrap();
    let gl: WebGlRenderingContext = gl.dyn_into().unwrap();

    // Create a vertex shader
    let vertex_shader = gl.create_shader(WebGlRenderingContext::VERTEX_SHADER).unwrap();
    gl.shader_source(vertex_shader, r#"
        void main() {
            gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
        }
    "#);
    gl.compile_shader(vertex_shader);

    // Create a fragment shader
    let fragment_shader = gl.create_shader(WebGlRenderingContext::FRAGMENT_SHADER).unwrap();
    gl.shader_source(fragment_shader, r#"
        void main() {
            gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#);
    gl.compile_shader(fragment_shader);

    // Create a program and link the shaders
    let program = gl.create_program().unwrap();
    gl.attach_shader(program, vertex_shader);
    gl.attach_shader(program, fragment_shader);
    gl.link_program(program);

    // Draw a point
    gl.use_program(program);
    gl.draw_arrays(WebGlRenderingContext::POINTS, 0, 1);
}
