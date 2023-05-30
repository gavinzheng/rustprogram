// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate. 
use wasm_bindgen::prelude::*;

// Our Mul function
// wasm-pack requires "exported" functions 
// to include #[wasm_bindgen]
#[wasm_bindgen]
pub fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}
