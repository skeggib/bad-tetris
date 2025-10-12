use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

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
    let vertex_shader = compile_shader(
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
    let fragment_shader = compile_shader(
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

    let program = link_program(&gl, &vertex_shader, &fragment_shader)?;
    gl.use_program(Some(&program));

    let buffer = gl.create_buffer().ok_or("cannot create buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    let position = gl.get_attrib_location(&program, "position") as u32;
    gl.vertex_attrib_pointer_with_i32(position, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position);

    // --- rendering loop ---

    let grid_dimensions = GridDimensions {
        x: -0.5,
        y: -0.5,
        width: 1.0,
        height: 1.0,
        horizontal_cells_count: 10,
        vertical_cells_count: 10,
    };

    clear(&gl);
    draw_grid(&gl, &grid_dimensions);
    draw_block(&gl, 7, 7, &grid_dimensions);
    draw_block(&gl, 3, 4, &grid_dimensions);
    draw_block(&gl, 0, 0, &grid_dimensions);

    Ok(())
}

struct GridDimensions {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    horizontal_cells_count: usize,
    vertical_cells_count: usize,
}

fn create_grid(dimensions: &GridDimensions) -> Vec<f32> {
    let x = dimensions.x;
    let y = dimensions.y;
    let width = dimensions.width;
    let height = dimensions.height;
    let horizontal_cells_count = dimensions.horizontal_cells_count;
    let vertical_cells_count = dimensions.vertical_cells_count;

    assert!(horizontal_cells_count > 0);
    assert!(vertical_cells_count > 0);

    let mut vertices: Vec<f32> = vec![];
    vertices.reserve_exact((horizontal_cells_count + 1) * (vertical_cells_count + 1));

    let cell_width = width / horizontal_cells_count as f32;
    let cell_height = height / vertical_cells_count as f32;

    // vertical lines
    for i in 0..(horizontal_cells_count + 1) {
        vertices.push(x + i as f32 * cell_width);
        vertices.push(y);
        vertices.push(x + i as f32 * cell_width);
        vertices.push(y + height);
    }

    // horizontal lines
    for i in 0..(vertical_cells_count + 1) {
        vertices.push(x);
        vertices.push(y + i as f32 * cell_height);
        vertices.push(x + width);
        vertices.push(y + i as f32 * cell_height);
    }

    return vertices;
}

fn draw_grid(gl: &WebGl2RenderingContext, grid_dimensions: &GridDimensions) {
    let vertices = create_grid(grid_dimensions);
    buffer_data(&gl, &vertices);
    gl.draw_arrays(WebGl2RenderingContext::LINES, 0, vertices.len() as i32 / 2);
}

fn create_block(x: i32, y: i32, grid: &GridDimensions) -> Vec<f32> {
    let cell_width = grid.width / grid.horizontal_cells_count as f32;
    let cell_height = grid.height / grid.vertical_cells_count as f32;

    let x_drawing = grid.x + (x as f32 * cell_width);
    let y_drawing = grid.y + (y as f32 * cell_height);

    let vertices = vec![
        // lower triangle
        x_drawing,
        y_drawing,
        x_drawing + cell_width,
        y_drawing,
        x_drawing,
        y_drawing + cell_height,
        // upper triangle
        x_drawing + cell_width,
        y_drawing + cell_height,
        x_drawing + cell_width,
        y_drawing,
        x_drawing,
        y_drawing + cell_height,
    ];

    console::log_1(&format!("{:?}", vertices).into());

    return vertices;
}

fn draw_block(gl: &WebGl2RenderingContext, x: i32, y: i32, grid_dimensions: &GridDimensions) {
    let vertices = create_block(x, y, grid_dimensions);
    buffer_data(&gl, &vertices);
    gl.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        vertices.len() as i32 / 2,
    );
}

fn clear(gl: &WebGl2RenderingContext) {
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}

fn buffer_data(gl: &WebGl2RenderingContext, vertices: &Vec<f32>) {
    unsafe {
        let vertices_array = web_sys::js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertices_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or("cannot create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or_default())
    }
}

fn link_program(
    gl: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("cannot create program")?;
    gl.attach_shader(&program, vertex_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_default())
    }
}
