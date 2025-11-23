use crate::board;
use web_sys::WebGl2RenderingContext;

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

    draw_grid(&gl, &grid_dimensions);

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if board.cells()[row][col] {
                draw_block(&gl, col, row, &grid_dimensions);
            }
        }
    }
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

pub fn draw_grid(gl: &WebGl2RenderingContext, grid_dimensions: &GridDimensions) {
    let vertices = create_grid(grid_dimensions);
    buffer_data(&gl, &vertices);
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
    x: usize,
    y: usize,
    grid_dimensions: &GridDimensions,
) {
    let vertices = create_block(x, y, grid_dimensions);
    buffer_data(&gl, &vertices);
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
