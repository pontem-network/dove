//! WASM interface for Move Decompiler.
//!
//! Usage example:
//!
//! ```html
//! <html>
//! <head>
//!     <title>Move Decompiler in WASM</title>
//!     <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
//!     <script src="Decompiler.js"></script>
//!     <script>
//!         var test_mod_bytes = new Uint8Array([161,28,235,11,1,0,0,0,10,1,0,4,2,4,4,3,8,15,5,23,27,7,50,84,8,134,1,16,6,150,1,20,10,170,1,6,12,176,1,61,13,237,1,2,0,0,0,1,0,0,1,0,1,2,0,1,0,0,3,2,3,0,0,4,4,5,0,1,6,12,1,1,1,5,1,10,2,3,6,12,6,12,10,2,0,3,6,8,0,1,3,2,1,3,23,86,97,108,105,100,97,116,111,114,79,112,101,114,97,116,111,114,67,111,110,102,105,103,5,82,111,108,101,115,19,104,97,115,95,108,105,98,114,97,95,114,111,111,116,95,114,111,108,101,14,103,101,116,95,104,117,109,97,110,95,110,97,109,101,7,112,117,98,108,105,115,104,10,104,117,109,97,110,95,110,97,109,101,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,3,8,0,0,0,0,0,0,0,0,3,8,1,0,0,0,0,0,0,0,0,2,1,5,10,2,1,1,1,0,6,14,10,0,41,0,12,2,11,2,3,7,7,1,39,10,0,43,0,12,1,11,1,16,0,20,2,2,1,0,7,14,11,1,17,0,12,3,11,3,3,9,11,0,1,7,0,39,11,0,11,2,18,0,45,0,2,0,0,0]);
//!         const { version, decompile, SourceType } = decompiler;
//!         async function run() {
//!             await decompiler('decompiler_bg.wasm');
//!             console.log(`version = ${version()}`);
//!             const result = decompile(test_mod_bytes, SourceType.Diem);
//!             console.log(`result = ${result}`);
//!         }
//!         run();
//!     </script>
//! </head>
//! <body></body>
//! </html>
//! ```
//!
//! Build wasm and gen JS/TS-bindings:
//! - `cargo build -p decompiler --lib --target wasm32-unknown-unknown --release`
//! - `cargo install wasm-bindgen-cli`
//! - (optional) `cargo install wasm-snip`
//! - `wasm-bindgen target/wasm32-unknown-unknown/release/decompiler.wasm --out-dir ./dist --no-modules --no-modules-global decompiler`
//! - (optional) `wasm-snip --snip-rust-panicking-code --skip-producers-section ./dist/decompiler_bg.wasm -o ./dist/decompiler_bg_snip.wasm`
//! - (optional) `wasm-opt ./dist/decompiler_bg_snip.wasm --dce -o dist/decompiler_bg.wasm`

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use crate::{Config, decompile_str, VERSION};

#[wasm_bindgen]
pub fn version() -> JsValue {
    JsValue::from_str(VERSION)
}

#[wasm_bindgen]
pub enum SourceType {
    Diem,
    Pont,
    Dfinance,
}

#[wasm_bindgen]
pub fn decompile(bytes: &[u8], source_type: SourceType) -> Result<Option<String>, JsValue> {
    let mut bytes = bytes.to_owned();

    match source_type {
        SourceType::Diem => {
            compat::adapt_to_basis(&mut bytes, compat::AddressType::Diem)
                .map_err(|err| err.to_string())?;
        }
        SourceType::Pont => {
            // no-op
        }
        SourceType::Dfinance => {
            compat::adapt_to_basis(&mut bytes, compat::AddressType::Dfninance)
                .map_err(|err| err.to_string())?;
        }
    }

    let cfg = Config {
        light_version: false,
    };

    let out = decompile_str(&bytes, cfg).map_err(|err| err.to_string())?;

    Ok(Some(out))
}
