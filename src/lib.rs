mod utils;
mod snake;
mod food;

use wasm_bindgen::prelude::*;
use getrandom::getrandom;

#[wasm_bindgen]
pub struct Coord {
    pub x: u32,
    pub y: u32
}

#[wasm_bindgen]
impl Coord {
    #[wasm_bindgen(getter)]
    pub fn get_x(&self) -> u32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn get_y(&self) -> u32 {
        self.y
    }
}

#[wasm_bindgen]
pub fn generate_random_points(x_bound: u32, y_bound: u32) -> Coord {
    Coord {
        x: {
                let mut buffer = [0u8; 4];
                getrandom(&mut buffer).expect("Failed to generate random number");
                let random_number = u32::from_ne_bytes(buffer) % (x_bound + 1);
                random_number
            },
        y: {
                let mut buffer = [0u8; 4];
                getrandom(&mut buffer).expect("Failed to generate random number");
                let random_number = u32::from_ne_bytes(buffer) % (y_bound + 1);
                random_number
            }
    }
} 