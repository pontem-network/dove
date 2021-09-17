mod dependency;
mod interact;

use move_lang::move_compile;
use move_lang::shared::Flags;
use wasm_bindgen::prelude::*;

mod compiler;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let path = fs::read_dir(".");
    let res = move_compile(
        &["test.move".to_string()],
        &[],
        None,
        Flags::empty(),
        &mut (),
    );
    alert(&format!("Hello, {:?} {:?}!", path, res));
}