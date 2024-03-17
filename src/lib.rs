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

use std::rc::Rc;
use std::cell::RefCell;

struct MyType {
    value: i32,
}

impl MyType {
    fn new(value: i32) -> Self {
        MyType { value }
    }

    fn update(&mut self, new_value: i32) {
        self.value = new_value;
    }
}

fn main() {
    let my_type = Rc::new(RefCell::new(MyType::new(5)));

    // Clone the Rc to create a new reference
    let my_type_clone = my_type.clone();

    // Borrow the RefCell mutably
    let mut my_type_instance = my_type.borrow_mut();

    // Call a method on the struct
    my_type_instance.update(10);

    // Access the value inside the RefCell using dereference
    println!("{}", my_type_clone.borrow().value); // Output: 10
}