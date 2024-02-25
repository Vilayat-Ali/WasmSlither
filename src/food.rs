use wasm_bindgen::prelude::*;

use super::{Coord, generate_random_points};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Food {
    pub x: u32,
    pub y: u32,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FoodSpawner {
    x_bounds: u32,
    y_bounds: u32,
    food_vec: Vec<Food>,
}

#[wasm_bindgen]
impl FoodSpawner {
    #[wasm_bindgen(constructor)]
    pub fn new(x_bounds: u32, y_bounds: u32) -> Self {
        Self {
            x_bounds,
            y_bounds,
            food_vec: Vec::with_capacity(10)
        }
    }

    #[wasm_bindgen]
    pub fn spawn(&mut self) {
        let Coord{x, y} = generate_random_points(self.x_bounds, self.y_bounds);
        self.food_vec.push(Food {
            x,
            y
        });
    }

    #[wasm_bindgen]
    pub fn eat_food(&mut self, x: u32, y: u32) {
        let idx: usize = self.food_vec.iter().position(|food| food.x == x && food.y == y).expect("Invalid food coordinates");
        self.food_vec.swap_remove(idx);
    } 
}