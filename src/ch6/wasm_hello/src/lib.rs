extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// JavaScript 함수를 러스트에서 사용하기 위해 --- (*1)
#[wasm_bindgen]
extern {
    // JavaScript의 alert 함수를 러스트에서 사용하기 위해
    pub fn alert(s: &str);
}

// Rust로 JavaScript에서 사용할 함수를 정의 --- (*2)
#[wasm_bindgen]
pub fn hello(name: &str) {
    let msg = format!("Hello, {}!", name);
    alert(&msg);
}
#[wasm_bindgen]
pub fn rust_mul(a:i32, b:i32) -> i32 {
    a * b
}

