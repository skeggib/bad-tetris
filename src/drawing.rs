use crate::board;
use crate::webgl;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;

pub fn create_program(gl: &WebGl2RenderingContext) -> Result<self::WebGlProgram, String> {
    // https://webglfundamentals.org/webgl/lessons/webgl-fundamentals.html
    // the vertex shader computes vertex positions
    // webgl uses its output to rasterize primitives (point, line, triangle)
    let vertex_shader = webgl::compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"
            // attributes receive data from the buffer
            attribute vec4 position;
            attribute vec4 color;

            // varyings send data to the fragment buffer (the fragment buffer cannot have
            // attributes)
            varying vec4 v_color;

            void main() {
                // gl_Position is the output of the shader
                gl_Position = position;
                v_color = color;
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

            // receive data from the vertex shader
            varying vec4 v_color;

            void main() {
                // gl_FragColor is the output of the shader
                gl_FragColor = v_color;
            }
        "#,
    )?;

    // providing data to the gpu:
    // - buffers contains data that attributes to extract
    // - uniforms are global variables set before executing the shader
    // - textures
    // - varying are used by the vertex shader to pass data to the fragment shader

    webgl::link_program(&gl, &vertex_shader, &fragment_shader)
}

pub struct GridDimensions {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub horizontal_cells_count: usize,
    pub vertical_cells_count: usize,
}

pub fn draw_board<const WIDTH: usize, const HEIGHT: usize>(
    board: &board::Board<WIDTH, HEIGHT>,
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
) where
    [(); WIDTH * HEIGHT]:,
{
    let grid_dimensions = GridDimensions {
        x: -0.45,
        y: -0.9,
        width: 0.9,
        height: 1.8,
        horizontal_cells_count: WIDTH,
        vertical_cells_count: HEIGHT,
    };

    let colors = hash_map! {
        // https://coolors.co/54f8d7-5474f8-d754f8-f8d756-ff9966-abf854-f86a54
        board::Color::Cyan => [84./255., 248./255., 215./255., 1.],
        board::Color::Blue => [84./255., 116./255., 248./255., 1.],
        board::Color::Magenta => [215./255., 84./255., 248./255., 1.],
        board::Color::Yellow => [248./255., 215./255., 86./255., 1.],
        board::Color::Orange => [255./255., 153./255., 102./255., 1.],
        board::Color::Green => [171./255., 248./255., 84./255., 1.],
        board::Color::Red => [248./255., 106./255., 84./255., 1.],
    };

    let cells = board.cells();
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let cell = cells[row][col];
            if cell != None {
                draw_block(
                    &gl,
                    &program,
                    col,
                    row,
                    colors[&cell.unwrap()],
                    &grid_dimensions,
                );
            }
        }
    }

    draw_grid(&gl, &program, &grid_dimensions);
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

pub fn draw_grid(
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
    grid_dimensions: &GridDimensions,
) {
    let vertices = create_grid(grid_dimensions);
    let mut colors: Vec<f32> = vec![];
    for _ in 0..(vertices.len() / 2) {
        colors.extend_from_slice(&[1., 1., 1., 1.]);
    }
    buffer_data(&gl, &program, &vertices, &colors);
    gl.draw_arrays(WebGl2RenderingContext::LINES, 0, vertices.len() as i32 / 2);
}

fn create_block(x: usize, y: usize, grid: &GridDimensions) -> Vec<f32> {
    let cell_width = grid.width / grid.horizontal_cells_count as f32;
    let cell_height = grid.height / grid.vertical_cells_count as f32;

    let x_drawing = grid.x + (x as f32 * cell_width);
    let y_drawing = grid.y + ((grid.vertical_cells_count - y - 1) as f32 * cell_height);

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

    return vertices;
}

pub fn draw_block(
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
    x: usize,
    y: usize,
    color: [f32; 4],
    grid_dimensions: &GridDimensions,
) {
    let vertices = create_block(x, y, grid_dimensions);
    let mut colors: Vec<f32> = vec![];
    for _ in 0..(vertices.len() / 2) {
        colors.extend_from_slice(&color);
    }
    buffer_data(&gl, &program, &vertices, &colors);
    gl.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        (vertices.len() / 2) as i32,
    );
}

pub fn clear(gl: &WebGl2RenderingContext) {
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}

fn buffer_data(
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
    vertices: &Vec<f32>,
    colors: &Vec<f32>,
) {
    let buffer = gl.create_buffer().ok_or("cannot create buffer").unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    let position = gl.get_attrib_location(&program, "position") as u32;
    gl.vertex_attrib_pointer_with_i32(position, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position);

    unsafe {
        let vertices_array = web_sys::js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertices_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let buffer = gl.create_buffer().ok_or("cannot create buffer").unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    let color = gl.get_attrib_location(&program, "color") as u32;
    gl.vertex_attrib_pointer_with_i32(color, 4, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(color);

    unsafe {
        let colors_array = web_sys::js_sys::Float32Array::view(&colors);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &colors_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}
