use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlElement};

use crate::Coord;
use crate::{snake::Snake, food::Food, ROW_BOUND, COL_BOUND};

const TICK_RATE: i32 = 500;

#[wasm_bindgen]
pub struct Game {
    is_running: bool,
    score: u32,
    bounds: (u32, u32),
    snake: Rc<RefCell<Snake>>,
    food_vec: Vec<Food>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let grid_container = document.get_element_by_id("game-area").unwrap().dyn_into::<HtmlElement>().unwrap();

        for row_id in 0..ROW_BOUND {
            for col_id in 0..COL_BOUND {
                let cell = document.create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
                cell.set_class_name("h-6 w-6 bg-gray-700 border border-gray-800 rounded-sm shadow");
                cell.set_id(&format!("cell-{}-{}", row_id, col_id));
                grid_container.append_child(&cell).unwrap();
            }
        }

        console::info_1(&"Game arena generated!".into());

        Self {
            is_running: true,
            score: 0,
            bounds: (ROW_BOUND, COL_BOUND),
            snake: Rc::new(RefCell::new(Snake::new())),
            food_vec: Vec::with_capacity(5)
        }
    }

    #[wasm_bindgen]
    pub fn play(&mut self) {
        let snake = self.snake.clone();
        let window = web_sys::window().expect("Failed to load WINDOW");
        let document = window.document().expect("Failed to fetch Document");

        let game_loop = Closure::wrap(Box::new(move || {
            let snake_instance = snake.borrow();
            let snake_vec = snake_instance.clone().get_snake_body().clone();

            for node in snake_vec {
                let Coord {x, y} = node.coord;
                console::log_1(&format!("{:#?}", node.coord).into());
                let body_cell = document.clone().get_element_by_id(&format!("cell-{}-{}", x, y)).unwrap().dyn_into::<HtmlElement>().unwrap();
                body_cell.set_class_name("bg-green-700");
            }
        }) as Box<dyn FnMut()>);

        // Run game loop
        let window = web_sys::window().expect("Window Object not found!");
        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                game_loop.as_ref().unchecked_ref(),
                TICK_RATE,
            )
            .expect("Failed to set gameloop");
        game_loop.forget();
    }

}