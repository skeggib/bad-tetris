use crate::board;
use crate::webgl;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlUniformLocation;

pub struct GridDimensions {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub cols: usize,
    pub rows: usize,
}

pub struct Display {
    gl: WebGl2RenderingContext,
    grid_cols_program: GridColsProgram,
    grid_rows_program: GridRowsProgram,
    blocks_program: BlocksProgram,
}

struct BlocksProgram {
    program: WebGlProgram,
}

impl Display {
    pub fn new(gl: WebGl2RenderingContext) -> Display {
        web_sys::console::log_1(&"create display".into());
        let grid_cols_program = GridColsProgram::new(&gl);
        let grid_rows_program = GridRowsProgram::new(&gl);
        let blocks_program = BlocksProgram::new(&gl);
        Display {
            gl: gl,
            grid_cols_program: grid_cols_program,
            grid_rows_program: grid_rows_program,
            blocks_program: blocks_program,
        }
    }

    pub fn clear(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn draw_board<const WIDTH: usize, const HEIGHT: usize>(
        &self,
        board: &board::Board<WIDTH, HEIGHT>,
    ) where
        [(); WIDTH * HEIGHT]:,
    {
        let grid_dimensions = GridDimensions {
            x: -0.45,
            y: -0.9,
            width: 0.9,
            height: 1.8,
            cols: WIDTH,
            rows: HEIGHT,
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
                    self.blocks_program.draw(
                        &self.gl,
                        col,
                        row,
                        colors[&cell.unwrap()],
                        &grid_dimensions,
                    );
                }
            }
        }

        self.grid_cols_program.draw(&self.gl, &grid_dimensions);
        self.grid_rows_program.draw(&self.gl, &grid_dimensions);
    }
}

struct GridColsProgram {
    program: WebGlProgram,
    u_x: WebGlUniformLocation,
    u_y: WebGlUniformLocation,
    u_w: WebGlUniformLocation,
    u_h: WebGlUniformLocation,
    u_cols: WebGlUniformLocation,
}

impl GridColsProgram {
    fn new(gl: &WebGl2RenderingContext) -> GridColsProgram {
        let program = GridColsProgram::create_program(gl).unwrap();
        let u_x = gl.get_uniform_location(&program, "u_x").unwrap();
        let u_y = gl.get_uniform_location(&program, "u_y").unwrap();
        let u_w = gl.get_uniform_location(&program, "u_w").unwrap();
        let u_h = gl.get_uniform_location(&program, "u_h").unwrap();
        let u_cols = gl.get_uniform_location(&program, "u_cols").unwrap();
        GridColsProgram {
            program: program,
            u_x: u_x,
            u_y: u_y,
            u_w: u_w,
            u_h: u_h,
            u_cols: u_cols,
        }
    }

    fn create_program(gl: &WebGl2RenderingContext) -> Result<self::WebGlProgram, String> {
        web_sys::console::log_1(&"create grid program".into());

        web_sys::console::log_1(&"compile vertex shared".into());
        let vertex_shader = webgl::compile_shader(
            &gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            r#"#version 300 es
            // attributes receive data from the buffer
            uniform float u_x;
            uniform float u_y;
            uniform float u_w;
            uniform float u_h;
            uniform int u_cols;
            void main() {
                float x = u_x + float(gl_VertexID / 2) * (u_w / float(u_cols));
                float y = u_y + float(gl_VertexID % 2) * u_h;
                gl_Position = vec4(x, y, 0.0, 1.0);
            }
        "#,
        )?;

        web_sys::console::log_1(&"compile fragment shared".into());
        let fragment_shader = webgl::compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r#"#version 300 es
            precision mediump float;
            out vec4 fragColor;
            void main() {
                // gl_FragColor would be the output of the shader in version 100 es
                // in version 300 es we need to declare an output ourselves
                fragColor = vec4(1.0, 1.0, 1.0, 1.0);
            }
        "#,
        )?;

        web_sys::console::log_1(&"link grid program".into());
        webgl::link_program(&gl, &vertex_shader, &fragment_shader)
    }

    fn draw(&self, gl: &WebGl2RenderingContext, grid_dimensions: &GridDimensions) {
        gl.use_program(Some(&self.program));

        gl.uniform1f(Some(&self.u_x), grid_dimensions.x);
        gl.uniform1f(Some(&self.u_y), grid_dimensions.y);
        gl.uniform1f(Some(&self.u_w), grid_dimensions.width);
        gl.uniform1f(Some(&self.u_h), grid_dimensions.height);
        gl.uniform1i(Some(&self.u_cols), grid_dimensions.cols as i32);

        gl.draw_arrays(
            WebGl2RenderingContext::LINES,
            0,
            (grid_dimensions.cols as i32 + 1) * 2,
        );
    }
}

struct GridRowsProgram {
    program: WebGlProgram,
    u_x: WebGlUniformLocation,
    u_y: WebGlUniformLocation,
    u_w: WebGlUniformLocation,
    u_h: WebGlUniformLocation,
    u_rows: WebGlUniformLocation,
}

impl GridRowsProgram {
    fn new(gl: &WebGl2RenderingContext) -> GridRowsProgram {
        let program = GridRowsProgram::create_program(gl).unwrap();
        let u_x = gl.get_uniform_location(&program, "u_x").unwrap();
        let u_y = gl.get_uniform_location(&program, "u_y").unwrap();
        let u_w = gl.get_uniform_location(&program, "u_w").unwrap();
        let u_h = gl.get_uniform_location(&program, "u_h").unwrap();
        let u_rows = gl.get_uniform_location(&program, "u_rows").unwrap();
        GridRowsProgram {
            program: program,
            u_x: u_x,
            u_y: u_y,
            u_w: u_w,
            u_h: u_h,
            u_rows: u_rows,
        }
    }

    fn create_program(gl: &WebGl2RenderingContext) -> Result<self::WebGlProgram, String> {
        web_sys::console::log_1(&"create grid program".into());

        web_sys::console::log_1(&"compile vertex shared".into());
        let vertex_shader = webgl::compile_shader(
            &gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            r#"#version 300 es
            uniform float u_x;
            uniform float u_y;
            uniform float u_w;
            uniform float u_h;
            uniform int u_rows;
            void main() {
                float x = u_x + float(gl_VertexID % 2) * u_w;
                float y = u_y + float(gl_VertexID / 2) * (u_h / float(u_rows));
                gl_Position = vec4(x, y, 0.0, 1.0);
            }
        "#,
        )?;

        web_sys::console::log_1(&"compile fragment shared".into());
        let fragment_shader = webgl::compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r#"#version 300 es
            precision mediump float;
            out vec4 fragColor;
            void main() {
                // gl_FragColor would be the output of the shader in version 100 es
                // in version 300 es we need to declare an output ourselves
                fragColor = vec4(1.0, 1.0, 1.0, 1.0);
            }
        "#,
        )?;

        web_sys::console::log_1(&"link grid program".into());
        webgl::link_program(&gl, &vertex_shader, &fragment_shader)
    }

    fn draw(&self, gl: &WebGl2RenderingContext, grid_dimensions: &GridDimensions) {
        gl.use_program(Some(&self.program));

        gl.uniform1f(Some(&self.u_x), grid_dimensions.x);
        gl.uniform1f(Some(&self.u_y), grid_dimensions.y);
        gl.uniform1f(Some(&self.u_w), grid_dimensions.width);
        gl.uniform1f(Some(&self.u_h), grid_dimensions.height);
        gl.uniform1i(Some(&self.u_rows), grid_dimensions.rows as i32);

        gl.draw_arrays(
            WebGl2RenderingContext::LINES,
            0,
            (grid_dimensions.rows as i32 + 1) * 2,
        );
    }
}

impl BlocksProgram {
    fn new(gl: &WebGl2RenderingContext) -> BlocksProgram {
        BlocksProgram {
            program: BlocksProgram::create_program(gl).unwrap(),
        }
    }

    fn create_program(gl: &WebGl2RenderingContext) -> Result<self::WebGlProgram, String> {
        web_sys::console::log_1(&"create program".into());

        web_sys::console::log_1(&"compile vertex shared".into());
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

        web_sys::console::log_1(&"compile fragment shared".into());
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

        web_sys::console::log_1(&"link program".into());
        webgl::link_program(&gl, &vertex_shader, &fragment_shader)
    }

    fn create_block(&self, x: usize, y: usize, grid: &GridDimensions) -> Vec<f32> {
        let cell_width = grid.width / grid.cols as f32;
        let cell_height = grid.height / grid.rows as f32;

        let x_drawing = grid.x + (x as f32 * cell_width);
        let y_drawing = grid.y + ((grid.rows - y - 1) as f32 * cell_height);

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

    fn draw(
        &self,
        gl: &WebGl2RenderingContext,
        x: usize,
        y: usize,
        color: [f32; 4],
        grid_dimensions: &GridDimensions,
    ) {
        gl.use_program(Some(&self.program));

        let vertices = self.create_block(x, y, grid_dimensions);
        let mut colors: Vec<f32> = vec![];
        for _ in 0..(vertices.len() / 2) {
            colors.extend_from_slice(&color);
        }
        self.buffer_data(gl, &vertices, &colors);
        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            (vertices.len() / 2) as i32,
        );
    }

    fn buffer_data(&self, gl: &WebGl2RenderingContext, vertices: &Vec<f32>, colors: &Vec<f32>) {
        let buffer = gl.create_buffer().ok_or("cannot create buffer").unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let position = gl.get_attrib_location(&self.program, "position") as u32;
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

        let color = gl.get_attrib_location(&self.program, "color") as u32;
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
}
