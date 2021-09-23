use wasm_bindgen::prelude::*;
use crate::compiler::build;
use crate::compiler::source_map::SourceMap;

mod compiler;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello,{:?}!", name));
    let mut source_map = SourceMap::default();
    source_map.insert("module.move".to_string(), "module 0x1::T {}".to_string());
    build(source_map, "pont", "0x1");
}
