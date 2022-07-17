extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // https://developer.mozilla.org/ja/docs/WebAssembly/Rust_to_wasm#lets_write_some_rust
    pub fn alert(s: &str);

    // https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
    log(&format!("Hey, {}!", name));
}
