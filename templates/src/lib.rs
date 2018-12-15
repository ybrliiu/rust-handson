extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use sha2::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// jsの関数を上書きするよ
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// jsからアクセスできるようにするためのアトリビュート
#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut hasher = DefaultHasher::new();
    name.as_bytes().hash(&mut hasher);
    alert(&format!("Hello, {}!", hasher.finish()));
}

#[wasm_bindgen]
pub fn add(a: u32, b:u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn minus(a: u32, b:u32) -> u32 {
    a - b
}

#[wasm_bindgen]
pub fn hash(s: &str) -> u32 {
    let mut hasher = DefaultHasher::new();
    s.as_bytes().hash(&mut hasher);
    hasher.finish() as u32
}

#[wasm_bindgen]
pub fn sha256(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(s.as_bytes());
    let strs: Vec<String> = hasher.result().as_slice().iter().map(|&num| { num.to_string() }).collect();
    strs.join(" ")
}

