use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


use crate::{food::Food, snake::Snake, COL_BOUND, ROW_BOUND};
use web_sys::{console, HtmlElement};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    pub is_running: bool,
    pub score: u32,
    bounds: (u32, u32),
    snake: Snake,
    food_vec: Vec<Food>
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
            snake: Snake::new(),
            food_vec: Vec::with_capacity(5)
        }
    }


    #[wasm_bindgen]
    pub fn play(&mut self) {
        let window = web_sys::window().expect("Wndow not found");

        let game_loop = Closure::wrap(Box::new(|| {  
            console::log_1(&"Snake moving...".into());
            // let window = web_sys::window().expect("Wndow not found");
            // let document = window.document().unwrap();
            // let snake_node_vec = self.snake.get_snake_body().clone();
            // for node in snake_node_vec {
            // let cell_element = document.get_element_by_id(&format!("cell-{}-{}", node.coord.x, node.coord.y)).unwrap().dyn_into::<HtmlElement>().unwrap();
            // cell_element.set_class_name("bg-white");
            // }
        }) as Box<dyn FnMut()>);

        // running game loop
        window.set_interval_with_callback_and_timeout_and_arguments_0(game_loop.as_ref().unchecked_ref(), 500).unwrap();
        game_loop.forget();
    }
}