mod utils;
mod snake;
mod food;

use rand::Rng;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Coord {
    pub x: u32,
    pub y: u32
}

#[wasm_bindgen]
pub fn generate_random_points(x_bound: u32, y_bound: u32) -> Coord {
    Coord {
        x: {
                let mut rand = rand::thread_rng();
                rand.gen_range(0..=x_bound)
            },
        y: {
                let mut rand = rand::thread_rng();
                rand.gen_range(0..=y_bound)
            }
    }
} 
