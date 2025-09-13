use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext as GL, WebGlShader};

extern crate console_error_panic_hook;
mod lisa;
mod math;
mod polygon;

use lisa::Lissajou3D;

// Simple global state
static SPEED: Mutex<f32> = Mutex::new(0.1);
static TIME: Mutex<f32> = Mutex::new(0.0);
static SHOW_LONGITUDE: Mutex<bool> = Mutex::new(true);
static SHOW_LATITUDE: Mutex<bool> = Mutex::new(true);
static SHOW_TUNNEL: Mutex<bool> = Mutex::new(true);
static NUM_POLYGONS: Mutex<usize> = Mutex::new(200);
static IS_OUTSIDE_VIEW: Mutex<bool> = Mutex::new(false);

// Simple matrix struct
pub struct Mat4 {
    data: [f32; 16],
}

impl Mat4 {
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();
        let nf = 1.0 / (near - far);

        Mat4 {
            data: [
                f / aspect,
                0.0,
                0.0,
                0.0,
                0.0,
                f,
                0.0,
                0.0,
                0.0,
                0.0,
                (far + near) * nf,
                -1.0,
                0.0,
                0.0,
                2.0 * far * near * nf,
                0.0,
            ],
        }
    }

    pub fn look_at(eye: [f32; 3], center: [f32; 3], up: [f32; 3]) -> Self {
        let f = [center[0] - eye[0], center[1] - eye[1], center[2] - eye[2]];
        let f_len = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt();
        let f = [f[0] / f_len, f[1] / f_len, f[2] / f_len];

        let up_len = (up[0] * up[0] + up[1] * up[1] + up[2] * up[2]).sqrt();
        let up = [up[0] / up_len, up[1] / up_len, up[2] / up_len];

        let s = [
            f[1] * up[2] - f[2] * up[1],
            f[2] * up[0] - f[0] * up[2],
            f[0] * up[1] - f[1] * up[0],
        ];
        let s_len = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt();
        let s = [s[0] / s_len, s[1] / s_len, s[2] / s_len];

        let u = [
            s[1] * f[2] - s[2] * f[1],
            s[2] * f[0] - s[0] * f[2],
            s[0] * f[1] - s[1] * f[0],
        ];

        Mat4 {
            data: [
                s[0],
                u[0],
                -f[0],
                0.0,
                s[1],
                u[1],
                -f[1],
                0.0,
                s[2],
                u[2],
                -f[2],
                0.0,
                -(s[0] * eye[0] + s[1] * eye[1] + s[2] * eye[2]),
                -(u[0] * eye[0] + u[1] * eye[1] + u[2] * eye[2]),
                f[0] * eye[0] + f[1] * eye[1] + f[2] * eye[2],
                1.0,
            ],
        }
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.data
    }
}

#[wasm_bindgen]
pub fn set_speed(speed: f32) {
    *SPEED.lock().unwrap() = speed;
}

#[wasm_bindgen]
pub fn set_show_longitude(show: bool) {
    *SHOW_LONGITUDE.lock().unwrap() = show;
}

#[wasm_bindgen]
pub fn set_show_latitude(show: bool) {
    *SHOW_LATITUDE.lock().unwrap() = show;
}

#[wasm_bindgen]
pub fn set_show_tunnel(show: bool) {
    *SHOW_TUNNEL.lock().unwrap() = show;
}

#[wasm_bindgen]
pub fn set_num_polygons(num: usize) {
    *NUM_POLYGONS.lock().unwrap() = num.max(10).min(1000);
}

#[wasm_bindgen]
pub fn set_outside_view(outside: bool) {
    *IS_OUTSIDE_VIEW.lock().unwrap() = outside;
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

#[wasm_bindgen]
pub fn start_simple_tunnel(
    canvas_id: &str,
    a: f64,
    b: f64,
    r: f64,
    polygon_radius: f64,
    polygon_sides: usize,
    num_polygons: usize,
) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.get_element_by_id(canvas_id).unwrap().dyn_into()?;
    let gl: GL = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.enable(GL::DEPTH_TEST);
    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

    // Enable culling with proper triangle winding
    gl.disable(GL::CULL_FACE);
    // gl.cull_face(GL::BACK);

    // Simple shaders
    let vert_code = r#"
        attribute vec3 position;
        uniform mat4 u_projection;
        uniform mat4 u_view;
        uniform vec4 u_color;
        varying vec4 v_color;
        void main() {
            gl_Position = u_projection * u_view * vec4(position, 1.0);
            v_color = u_color;
        }
    "#;

    let frag_code = r#"
        precision mediump float;
        varying vec4 v_color;
        void main() {
            gl_FragColor = v_color;
        }
    "#;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_code)?;
    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_code)?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    // Store initial parameters
    *NUM_POLYGONS.lock().unwrap() = num_polygons;

    // Create Lissajou - vertex generation will be dynamic
    let lisa = Lissajou3D::new(a, b, r);

    // Create buffers and initialize with data
    let (longitude_verts, latitude_verts, tunnel_verts) = lisa.generate_tunnel_vertices(
        polygon_radius,
        polygon_sides,
        num_polygons,
        true,
        true,
        true,
    );

    let longitude_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&longitude_buffer));
    unsafe {
        let array = js_sys::Float32Array::view(&longitude_verts);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    let latitude_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&latitude_buffer));
    unsafe {
        let array = js_sys::Float32Array::view(&latitude_verts);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    let tunnel_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&tunnel_buffer));
    unsafe {
        let array = js_sys::Float32Array::view(&tunnel_verts);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
    }

    // Get attribute/uniform locations
    let pos_attrib = gl.get_attrib_location(&program, "position") as u32;
    let projection_uniform = gl.get_uniform_location(&program, "u_projection").unwrap();
    let view_uniform = gl.get_uniform_location(&program, "u_view").unwrap();
    let color_uniform = gl.get_uniform_location(&program, "u_color").unwrap();

    // Setup projection
    let aspect = canvas.width() as f32 / canvas.height() as f32;
    let projection = Mat4::perspective(std::f32::consts::PI / 4.0, aspect, 0.1, 1000.0);
    gl.uniform_matrix4fv_with_f32_array(Some(&projection_uniform), false, projection.as_slice());

    let gl = Rc::new(gl);

    // Track previous polygon count and initial vertex counts for dynamic updates
    let mut last_polygon_count = num_polygons;
    let mut cached_longitude_count = longitude_verts.len() / 3;
    let mut cached_latitude_count = latitude_verts.len() / 3;
    let mut cached_tunnel_count = tunnel_verts.len() / 3;

    // Animation loop
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // Update time
        let speed = *SPEED.lock().unwrap();
        let mut time = TIME.lock().unwrap();
        *time += speed * 0.016; // ~60fps
        let t = *time as f64;

        // Check if polygon count changed
        let current_polygon_count = *NUM_POLYGONS.lock().unwrap();

        if current_polygon_count != last_polygon_count {
            // Regenerate vertices
            let (longitude_verts, latitude_verts, tunnel_verts) = lisa.generate_tunnel_vertices(
                polygon_radius,
                polygon_sides,
                current_polygon_count,
                true,
                true,
                true,
            );

            // Update buffers
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&longitude_buffer));
            unsafe {
                let array = js_sys::Float32Array::view(&longitude_verts);
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
            }
            cached_longitude_count = longitude_verts.len() / 3;

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&latitude_buffer));
            unsafe {
                let array = js_sys::Float32Array::view(&latitude_verts);
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
            }
            cached_latitude_count = latitude_verts.len() / 3;

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&tunnel_buffer));
            unsafe {
                let array = js_sys::Float32Array::view(&tunnel_verts);
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
            }
            cached_tunnel_count = tunnel_verts.len() / 3;

            last_polygon_count = current_polygon_count;
        }

        gl.clear_color(0.02, 0.02, 0.05, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // Camera mode
        let is_outside = *IS_OUTSIDE_VIEW.lock().unwrap();
        let view = if is_outside {
            // Outside view: orbit around the curve
            let orbit_radius = 15.0;
            let orbit_angle = t * 0.3;
            let eye = [
                (orbit_angle.cos() * orbit_radius) as f32,
                5.0,
                (orbit_angle.sin() * orbit_radius) as f32,
            ];
            let look_target = [0.0, 0.0, 0.0];
            let up = [0.0, 1.0, 0.0];
            Mat4::look_at(eye, look_target, up)
        } else {
            // Inside view: camera follows curve
            let pos = lisa.position(t);
            let d1 = lisa.d1(2.0 * t);
            let d2 = lisa.d2(2.0 * t);
            let eye = [pos.x as f32, pos.y as f32, pos.z as f32];

            let look_target = [
                (pos.x + d1.x) as f32,
                (pos.y + d1.y) as f32,
                (pos.z + d1.z) as f32,
            ];
            let up = [d2.x as f32, d2.y as f32, d2.z as f32];
            Mat4::look_at(eye, look_target, up)
        };

        gl.uniform_matrix4fv_with_f32_array(Some(&view_uniform), false, view.as_slice());
        gl.enable_vertex_attrib_array(pos_attrib);

        // Draw longitude (red)
        if *SHOW_LONGITUDE.lock().unwrap() {
            gl.uniform4f(Some(&color_uniform), 1.0, 0.3, 0.3, 0.8);
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&longitude_buffer));
            gl.vertex_attrib_pointer_with_i32(pos_attrib, 3, GL::FLOAT, false, 0, 0);
            gl.depth_mask(false);
            gl.draw_arrays(GL::LINES, 0, cached_longitude_count as i32);
            gl.depth_mask(true);
        }

        // Draw latitude (green)
        if *SHOW_LATITUDE.lock().unwrap() {
            gl.uniform4f(Some(&color_uniform), 0.3, 1.0, 0.3, 0.8);
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&latitude_buffer));
            gl.vertex_attrib_pointer_with_i32(pos_attrib, 3, GL::FLOAT, false, 0, 0);
            gl.depth_mask(false);
            gl.draw_arrays(GL::LINES, 0, cached_latitude_count as i32);
            gl.depth_mask(true);
        }

        // Draw tunnel (blue, transparent)
        if *SHOW_TUNNEL.lock().unwrap() {
            gl.uniform4f(Some(&color_uniform), 0.3, 0.3, 1.0, 0.3);
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&tunnel_buffer));
            gl.vertex_attrib_pointer_with_i32(pos_attrib, 3, GL::FLOAT, false, 0, 0);

            gl.enable(GL::CULL_FACE);
            gl.depth_mask(false); // transparent: test depth but don't write

            // Pass 1: back faces first (cull front)
            gl.cull_face(GL::FRONT);
            gl.draw_arrays(GL::TRIANGLES, 0, cached_tunnel_count as i32);

            // Pass 2: front faces
            gl.cull_face(GL::BACK);
            gl.draw_arrays(GL::TRIANGLES, 0, cached_tunnel_count as i32);

            gl.depth_mask(true); // restore
            gl.disable(GL::CULL_FACE); // optional restore
        }

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
