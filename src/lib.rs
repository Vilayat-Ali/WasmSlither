pub mod utils;
pub mod snake;
pub mod game;
pub mod food;

use utils::generate_random_points;
use wasm_bindgen::prelude::*;

pub static ROW_BOUND: u32 = 33;
pub static COL_BOUND: u32 = 56;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Coord {
   pub x: u32,
   pub y: u32
}

impl std::marker::Copy for Coord {}

impl Coord {
   pub fn new(x: u32, y: u32) -> Self {
        Self { 
            x,
            y
        }
   }

   pub fn random() -> Self {
        let (x, y) = generate_random_points();
        Self {
                x,
                y
        }
   }
}
