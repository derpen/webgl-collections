use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, WebGlProgram, WebGlShader, WebGl2RenderingContext};

#[wasm_bindgen]
pub async fn read_shader(url_path: String) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let url = format!("http://localhost:8080/{}", url_path);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    //request
    //    .headers()
    //    .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.text()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct GlShader {
    context: WebGl2RenderingContext,
    shader_program: Option<WebGlProgram>,
}

impl GlShader {

    pub fn new(
        this_context: &WebGl2RenderingContext
    ) -> Self {
        GlShader {
            context: this_context.clone(),
            shader_program: None,
        }
    }

    pub fn link_program(
        &mut self,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = self.context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        self.context.attach_shader(&program, vert_shader);
        self.context.attach_shader(&program, frag_shader);
        self.context.link_program(&program);

        if self.context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            self.shader_program = Some(program.clone());
            Ok(program)
        } else {
            Err(self.context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    pub fn set_float(
        &self,
        name: String,
        value: f64,
    ) -> Result<(), String> {
        let program = self.shader_program.as_ref().ok_or("Program is not set")?;
        let shader_location = self.context.get_uniform_location(program, &name);
        self.context.uniform1f(shader_location.as_ref(), value as f32);
        Ok(())
    }

    pub fn get_shader_program(&self) -> Option<WebGlProgram> {
        self.shader_program.clone()
    }
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub async fn create_shader_program(
    context: &WebGl2RenderingContext,
    fragment_location: String,
    vertex_location: String,
) -> Result<GlShader, JsValue>  {
    let frag_shader_location = fragment_location;
    let frag_shader = read_shader(frag_shader_location).await.unwrap();

    let vert_shader_location = vertex_location;
    let vert_shader = read_shader(vert_shader_location).await.unwrap();

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        &vert_shader.as_string().unwrap(),
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        &frag_shader.as_string().unwrap(),
    )?;

    let mut gl_shader = GlShader::new(&context);
    let program = gl_shader.link_program(&vert_shader, &frag_shader).unwrap();
    gl_shader.shader_program = Some(program.clone());

    Ok(gl_shader)
}
