use wasm_bindgen::prelude::*;
use crate::{COL_BOUND, ROW_BOUND};

use super::Coord;

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
            coord: Coord::new(x, y),
            direction,
            next: None
        }
    }

    #[wasm_bindgen]
    pub fn random(direction: Direction) -> Self {
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
        Self {
            size: 1,
            body: Some(Box::new(Node::random(Direction::UP)))
        }
    }

    #[wasm_bindgen]
    pub fn grow_snake(&mut self) {
        let mut start: &mut Option<Box<Node>> = &mut self.body;
        while start.is_some() {
            if let Some(ref mut node) = start {
                if node.next.is_none() {
                    let new_x: u32 = {
                        match node.direction {
                            Direction::LEFT => {
                                node.coord.x + 1
                            },
                            Direction::RIGHT => {
                                if (node.coord.x + 1) <= ROW_BOUND {
                                    node.coord.x + 1
                                } else {
                                    node.coord.x
                                }
                            },
                            _ => node.coord.x
                        }
                    };
                    let new_y: u32 = {
                        match node.direction {
                            Direction::UP => {
                                node.coord.y - 1
                            },
                            Direction::DOWN => {
                                if (node.coord.y + 1) <= COL_BOUND {
                                    node.coord.y + 1
                                } else {
                                    node.coord.y
                                }
                            },
                            _ => node.coord.y
                        }
                    };
                    let new_node = Node::new(new_x, new_y, node.direction);
                    node.next = Some(Box::new(new_node));
                    self.size += 1;
                    break;
                }
                start = &mut node.next;
            }
        }
    }

    #[wasm_bindgen]
    pub fn get_snake_body(&self) -> Vec<Node> {
       let mut start: &Option<Box<Node>> = &self.body;
       let mut vec: Vec<Node> = Vec::with_capacity(self.size);

       while start.is_some() {
            if let Some(ref node) = start {
                vec.push(*node.clone());
                start = &node.next;
            }
       }

       vec
    }
}

