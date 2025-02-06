mod utils;

use wasm_bindgen::prelude::*;
use sha2::{Sha256, Digest};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn sha256(input: &str) -> String {
    format!("{:X}", Sha256::digest(input))
}