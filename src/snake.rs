use wasm_bindgen::prelude::*;
use super::generate_random_points;

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
    pub x: u32,
    pub y: u32,
    pub direction: Direction,
    next: Option<Box<Node>>,
}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32, direction: Direction) -> Self {
        Self {
            x,
            y,
            direction,
            next: None
        }
    }
} 

#[wasm_bindgen]
#[derive(Debug)]
pub struct Snake {
    pub size: usize,
    body: Option<Box<Node>>
}

#[wasm_bindgen]
impl Snake {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let coord = generate_random_points(33, 56);
        Self {
            size: 1,
            body: Some(Box::new(Node::new(coord.x, coord.y, Direction::UP)))
        }
    }

    #[wasm_bindgen]
    pub fn grow_snake(&mut self) {
        let mut body_ref: &mut Option<Box<Node>> = &mut self.body;

        while body_ref.is_some() {
            if let Some(ref mut node) = body_ref {
                if node.next.is_none() {
                    let (modified_x, modified_y) = ({
                            match node.direction {
                                Direction::LEFT => node.x - 1,
                                Direction::RIGHT => node.x + 1,
                                _ => node.x
                            }
                        }, {
                            match node.direction {
                                Direction::UP => node.y - 1,
                                Direction::DOWN => node.y + 1,
                                _ => node.y
                            }
                        });

                    node.next = Some(
                        Box::new(
                            Node::new(
                                modified_x,
                                modified_y,
                                node.direction
                            )
                        )
                    )
                }

                body_ref = &mut node.next;
            }
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