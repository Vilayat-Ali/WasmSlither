use wasm_bindgen::prelude::*;
use super::{utils::generate_random_points, Coord};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl std::marker::Copy for Direction {}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Node {
    pub coord: Coord,
    pub direction: Direction,
    next: Option<Box<Node>>,
}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32, direction: Direction) -> Self {
        Self {
            coord: Coord::random(),
            direction,
            next: None
        }
    }
} 

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Snake {
    pub size: usize,
    body: Option<Box<Node>>
}

#[wasm_bindgen]
impl Snake {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let (x, y) = generate_random_points();
        Self {
            size: 1,
            body: Some(Box::new(Node::new(x, y, Direction::UP)))
        }
    }

    #[wasm_bindgen]
    pub fn get_snake_body(&self) -> Vec<Node> {
        let mut body_ref: &Option<Box<Node>> = &self.body;
        let mut vec: Vec<Node> = Vec::with_capacity(10);

        while body_ref.is_some() {
            if let Some(ref node) = body_ref {
                vec.push(*(node.clone()));
                body_ref = &node.next;
            };
        }

        vec.push(*(body_ref.clone().unwrap()));
        vec
    }
}

