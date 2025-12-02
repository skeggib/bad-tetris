#![feature(generic_const_exprs)]
#![feature(hash_map_macro)]
#![feature(stmt_expr_attributes)]

use chrono::Local;
use console_error_panic_hook;
use debug_cell::RefCell;
use rand::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;

mod board;
mod drawing;
mod tetrominos;
mod webgl;

struct App {
    gl: WebGl2RenderingContext,
    program: WebGlProgram,
    board: board::Board<10, 20>,
    last_update_time: i64,
    // the keydown callback is a member of app so that it lives during its lifetime
    keydown_callback: Closure<dyn Fn(&web_sys::Event)>,
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    // log panics to the console
    console_error_panic_hook::set_once();

    // get the canas with id 'canvas'
    let canvas = web_sys::window()
        .ok_or("cannot get window")?
        .document()
        .ok_or("cannot get document")?
        .get_element_by_id("canvas")
        .ok_or("cannot get canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    // get a webgl2 context from the canvas
    let gl = canvas
        .get_context("webgl2")?
        .ok_or("cannot get webgl2 context")?
        .dyn_into::<WebGl2RenderingContext>()?;

    let program = drawing::create_program(&gl)?;

    gl.use_program(Some(&program));

    let state = Rc::new(RefCell::new(None::<App>));
    let state_copy = state.clone();
    *state.borrow_mut() = Some(App {
        gl: gl,
        program: program,
        board: board::Board::new([[None; 10]; 20], rand::rngs::StdRng::from_os_rng()),
        last_update_time: 0,
        keydown_callback: Closure::wrap(Box::new(move |event: &web_sys::Event| {
            match event.clone().dyn_into::<web_sys::KeyboardEvent>() {
                Ok(keyboard_event) => match keyboard_event.key().as_str() {
                    "ArrowUp" => state_copy.borrow_mut().as_mut().unwrap().board.rotate(),
                    "ArrowDown" => web_sys::console::log_1(&"down".into()),
                    "ArrowLeft" => state_copy.borrow_mut().as_mut().unwrap().board.left(),
                    "ArrowRight" => state_copy.borrow_mut().as_mut().unwrap().board.right(),
                    key_name => web_sys::console::log_1(&key_name.into()),
                },
                Err(_) => (),
            }
        })),
    });

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
    drawing::draw_board(&app.board, &app.gl, &app.program);
}
