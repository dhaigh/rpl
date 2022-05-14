use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gday(name: &str) -> String {
    return format!("gday, {}!", name);
}

