use chrono::Local;
use console_error_panic_hook;
use debug_cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

mod board;
mod drawing;
mod webgl;

struct App {
    gl: WebGl2RenderingContext,
    board: board::Board,
    last_update_time: i64,
    // the keydown callback is a member of app so that it lives during its lifetime
    keydown_callback: Closure<dyn Fn(&web_sys::Event)>,
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let canvas = web_sys::window()
        .ok_or("cannot get window")?
        .document()
        .ok_or("cannot get document")?
        .get_element_by_id("canvas")
        .ok_or("cannot get canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let state = Rc::new(RefCell::new(None::<App>));
    let state_copy = state.clone();
    *state.borrow_mut() = Some(App {
        gl: canvas
            .get_context("webgl2")?
            .ok_or("cannot get webgl2 context")?
            .dyn_into::<WebGl2RenderingContext>()?,
        #[rustfmt::skip]
        board: board::Board { cells: [
            false, false, false, false, false, false, false, false, false, false,
            false, false, false, false,  true, false, false, false, false, false,
            false, false, false, false, false,  true, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false,
            false,  true,  true, false, false, false, false,  true, false, false,
            false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false,
        ]},
        last_update_time: 0,
        keydown_callback: Closure::wrap(Box::new(move |event: &web_sys::Event| {
            match event.clone().dyn_into::<web_sys::KeyboardEvent>() {
                Ok(keyboard_event) => match keyboard_event.key().as_str() {
                    "ArrowUp" => web_sys::console::log_1(&"up".into()),
                    "ArrowDown" => web_sys::console::log_1(&"down".into()),
                    "ArrowLeft" => state_copy.borrow_mut().as_mut().unwrap().board.left(),
                    "ArrowRight" => state_copy.borrow_mut().as_mut().unwrap().board.right(),
                    &_ => (),
                },
                Err(_) => (),
            }
        })),
    });

    {
        // borrow the state and put it in a variable so that it survives this scope
        let binding = state.borrow();
        // get a ref to gl to avoid repeating state.borrow().as_ref().unwrap().gl
        let gl: &WebGl2RenderingContext = &binding.as_ref().unwrap().gl;

        // https://webglfundamentals.org/webgl/lessons/webgl-fundamentals.html
        // the vertex shader computes vertex positions
        // webgl uses its output to rasterize primitives (point, line, triangle)
        let vertex_shader = webgl::compile_shader(
            gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            r#"
                // receives data from the buffer
                attribute vec4 position;
                void main() {
                    // gl_Position is the output of the shader
                    gl_Position = position;
                }
            "#,
        )?;

        // the fragment shader computes the color of each pixel of the drawn primitive
        let fragment_shader = webgl::compile_shader(
            gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r#"
                // choose a precision for the fragment shader (mediump)
                precision mediump float;
                void main() {
                    // gl_FragColor is the output of the shader
                    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
                }
            "#,
        )?;

        // providing data to the gpu:
        // - buffers contains data that attributes extract
        // - uniforms are global variables set before executing the shader
        // - textures
        // - varying are used by the vertex shader to pass data to the fragment shader

        let program = webgl::link_program(gl, &vertex_shader, &fragment_shader)?;
        gl.use_program(Some(&program));

        let buffer = gl.create_buffer().ok_or("cannot create buffer")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let position = gl.get_attrib_location(&program, "position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);
    }

    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "keydown",
            state
                .borrow()
                .as_ref()
                .unwrap()
                .keydown_callback
                .as_ref()
                .as_ref()
                .unchecked_ref(),
        )
        .expect("add_event_listener failed");

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        update(
            Local::now().timestamp_millis(),
            &mut state.borrow_mut().as_mut().unwrap(),
        );
        render(&state.borrow().as_ref().unwrap());
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("request_animation_frame failed");
}

fn update(time_ms: i64, app: &mut App) {
    if time_ms - app.last_update_time >= 1000 {
        app.board.advance();
        app.last_update_time = time_ms;
    }
}

fn render(app: &App) {
    drawing::clear(&app.gl);
    drawing::draw_board(&app.board, &app.gl);
}
