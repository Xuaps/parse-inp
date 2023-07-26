mod utils;

use wasm_bindgen::prelude::*;
use parser::INP;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn deserialize_inp(content: String) -> JsValue {
    serde_wasm_bindgen::to_value(&INP::read(content)).unwrap()
}

