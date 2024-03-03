use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Food {
    pub x: u32,
    pub y: u32,
    color: String
}

#[wasm_bindgen]
impl Food {
    #[wasm_bindgen(getter)] 
    pub fn get_color(&self) -> String {
        self.color.clone()
    }
}