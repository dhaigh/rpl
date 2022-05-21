use wasm_bindgen::prelude::*;
mod scanner;

#[wasm_bindgen]
pub fn evaluate(_source: &str) -> Vec<i32> {
    return vec![3];
}
