use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

mod drawing;
mod webgl;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let canvas = web_sys::window()
        .ok_or("cannot get window")?
        .document()
        .ok_or("cannot get document")?
        .get_element_by_id("canvas")
        .ok_or("cannot get canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl2")?
        .ok_or("cannot get webgl2 context")?
        .dyn_into::<WebGl2RenderingContext>()?;

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

    // --- out of loop ---

    let program = webgl::link_program(&gl, &vertex_shader, &fragment_shader)?;
    gl.use_program(Some(&program));

    let buffer = gl.create_buffer().ok_or("cannot create buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    let position = gl.get_attrib_location(&program, "position") as u32;
    gl.vertex_attrib_pointer_with_i32(position, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position);

    // --- rendering loop ---

    let grid_dimensions = drawing::GridDimensions {
        x: -0.5,
        y: -0.5,
        width: 1.0,
        height: 1.0,
        horizontal_cells_count: 10,
        vertical_cells_count: 10,
    };

    drawing::clear(&gl);
    drawing::draw_grid(&gl, &grid_dimensions);
    drawing::draw_block(&gl, 7, 7, &grid_dimensions);
    drawing::draw_block(&gl, 3, 4, &grid_dimensions);
    drawing::draw_block(&gl, 0, 0, &grid_dimensions);

    Ok(())
}
