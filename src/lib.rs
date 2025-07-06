use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, HtmlVideoElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL,
    WebGlShader, WebGlTexture,
};

#[wasm_bindgen]
pub struct Mirror {
    n: i32,
}

#[wasm_bindgen]
impl Mirror {
    #[wasm_bindgen(constructor)]
    pub fn new(_n: i32) -> Mirror {
        Mirror { n: _n }
    }

    #[wasm_bindgen(method)]
    pub fn talk(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Mirror {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mirroring value from Rust: {}", &self.n)
    }
}

#[wasm_bindgen]
pub fn start_webgl(canvas_id: &str) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.get_element_by_id(canvas_id).unwrap().dyn_into()?;

    let gl: GL = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    let vert_code = r#"
        attribute vec2 position;
        uniform float angle;
        void main() {
            float c = cos(angle);
            float s = sin(angle);
            gl_Position = vec4(
                c * position.x - s * position.y,
                s * position.x + c * position.y,
                0.0, 1.0
            );
        }
    "#;

    let frag_code = r#"
        void main() {
            gl_FragColor = vec4(0.3, 0.6, 1.0, 1.0);
        }
    "#;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_code)?;
    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_code)?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    let vertices: [f32; 6] = [0.0, 0.5, -0.5, -0.5, 0.5, -0.5];
    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
    }

    let pos_attrib = gl.get_attrib_location(&program, "position") as u32;
    gl.enable_vertex_attrib_array(pos_attrib);
    gl.vertex_attrib_pointer_with_i32(pos_attrib, 2, GL::FLOAT, false, 0, 0);

    let angle_uniform = gl.get_uniform_location(&program, "angle").unwrap();
    let gl = Rc::new(gl);

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    let start_time = js_sys::Date::now();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let now = js_sys::Date::now();
        let elapsed = (now - start_time) as f32 / 1000.0;

        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        gl.uniform1f(Some(&angle_uniform), elapsed);
        gl.draw_arrays(GL::TRIANGLES, 0, 3);

        web_sys::window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));

    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}

#[wasm_bindgen]
pub fn start_webgl_with_video(canvas_id: &str, video: HtmlVideoElement) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let gl: GL = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    // === Shaders ===
    let vert_code = r#"
        attribute vec2 position;
        attribute vec2 texcoord;
        varying vec2 v_texcoord;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            v_texcoord = texcoord;
        }
    "#;

    let frag_code = r#"
        precision mediump float;
        varying vec2 v_texcoord;
        uniform sampler2D u_texture;
        void main() {
            gl_FragColor = texture2D(u_texture, v_texcoord);
        }
    "#;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_code)?;
    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_code)?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    // === Triangle vertices with UVs ===
    let vertices: [f32; 12] = [
        //   x,     y,    u,  v
        0.0, 0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 1.0, 1.0,
    ];

    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
    }

    let position_attrib = gl.get_attrib_location(&program, "position") as u32;
    gl.enable_vertex_attrib_array(position_attrib);
    gl.vertex_attrib_pointer_with_i32(position_attrib, 2, GL::FLOAT, false, 4 * 4, 0);

    let texcoord_attrib = gl.get_attrib_location(&program, "texcoord") as u32;
    gl.enable_vertex_attrib_array(texcoord_attrib);
    gl.vertex_attrib_pointer_with_i32(texcoord_attrib, 2, GL::FLOAT, false, 4 * 4, 2 * 4);

    // === Texture ===
    let texture = gl.create_texture().ok_or("Unable to create texture")?;
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);

    // === Animation loop ===
    let gl = Rc::new(gl);
    let video = Rc::new(video);

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // Upload the current video frame to the texture
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
        let _ = gl.tex_image_2d_with_u32_and_u32_and_video(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            &*video,
        );

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.draw_arrays(GL::TRIANGLES, 0, 3);

        web_sys::window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));

    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or("unable to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error".into()))
    }
}

fn link_program(gl: &GL, vert: &WebGlShader, frag: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("unable to create program")?;
    gl.attach_shader(&program, vert);
    gl.attach_shader(&program, frag);
    gl.link_program(&program);
    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error".into()))
    }
}
