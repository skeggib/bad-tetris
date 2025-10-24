use console_error_panic_hook;
use debug_cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

mod drawing;
mod webgl;

struct App {
    gl: WebGl2RenderingContext,
    board: Board,
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
        board: Board { cells: [
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
    app.board.draw(&app.gl);
}

struct Board {
    cells: [bool; Board::WIDTH * Board::HEIGHT],
}

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    fn draw(&self, gl: &WebGl2RenderingContext) {
        let grid_dimensions = drawing::GridDimensions {
            x: -0.5,
            y: -0.5,
            width: 1.0,
            height: 1.0,
            horizontal_cells_count: 10,
            vertical_cells_count: 10,
        };

        drawing::draw_grid(&gl, &grid_dimensions);

        for i in 0..Board::WIDTH {
            for j in 0..Board::HEIGHT {
                if self.cells[j * Board::WIDTH + i] {
                    drawing::draw_block(&gl, i, j, &grid_dimensions);
                }
            }
        }
    }

    fn advance(&mut self) {
        // move all blocks one cell down if the cell bellow is empty
        // iterate through cells from bottom to top to avoid collisions
        let from_line = Board::HEIGHT - 2; // ignore the most bottom line: blocks on this line cannot fall further
        let to_line = 0;
        for j in from_line..=to_line {
            for i in 0..Board::WIDTH {
                let current_cell = j * Board::WIDTH + i;
                let bellow_cell = current_cell + Board::WIDTH;
                let current_cell_empty = !self.cells[current_cell];
                let bellow_cell_empty = !self.cells[bellow_cell];

                if !current_cell_empty && bellow_cell_empty {
                    self.cells[current_cell] = false;
                    self.cells[bellow_cell] = true;
                }
            }
        }
    }
}
