mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello from rust, {name}"));
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
