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

    let state = Rc::new(RefCell::new(App {
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
    }));

    {
        // use gl in a scope to release the state's RefCell
        let ref gl = &state.borrow().gl;

        // https://webglfundamentals.org/webgl/lessons/webgl-fundamentals.html
        // the vertex shader computes vertex positions
        // webgl uses its output to rasterize primitives (point, line, triangle)
        let vertex_shader = webgl::compile_shader(
            &gl,
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
            &gl,
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

        let program = webgl::link_program(&gl, &vertex_shader, &fragment_shader)?;
        gl.use_program(Some(&program));

        let buffer = gl.create_buffer().ok_or("cannot create buffer")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let position = gl.get_attrib_location(&program, "position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);
    }

    render(0, &mut *state.borrow_mut());

    Ok(())
}

fn render(time_ms: u64, app: &mut App) {
    app.board.advance();
    drawing::clear(&app.gl);
    drawing::draw_board(&app.board, &app.gl);
}
