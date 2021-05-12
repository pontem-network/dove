//! CFFI interface for Move Decompiler.
//!
//! Build: `cargo build -p decompiler --lib --features cffi`
//!
//! Usage exapmle:
//!
//! C-bindings for this can be generated with cbindgen.
//! - `cargo install cbindgen`
//! - `cbindgen -l=c --crate=decompiler -o ./include/decompiler.h ./src/ext/cffi.rs`
//!
//! ```c
//! #include <stdio.h>
//! #include <stdint.h>
//! #include <inttypes.h>
//! #include "decompiler.h"
//! int main(void)
//! {
//!     uint8_t bytes[] = {161,28,235,11,1,0,0,0,10,1,0,4,2,4,4,3,8,15,5,23,27,7,50,84,8,134,1,16,6,150,1,20,10,170,1,6,12,176,1,61,13,237,1,2,0,0,0,1,0,0,1,0,1,2,0,1,0,0,3,2,3,0,0,4,4,5,0,1,6,12,1,1,1,5,1,10,2,3,6,12,6,12,10,2,0,3,6,8,0,1,3,2,1,3,23,86,97,108,105,100,97,116,111,114,79,112,101,114,97,116,111,114,67,111,110,102,105,103,5,82,111,108,101,115,19,104,97,115,95,108,105,98,114,97,95,114,111,111,116,95,114,111,108,101,14,103,101,116,95,104,117,109,97,110,95,110,97,109,101,7,112,117,98,108,105,115,104,10,104,117,109,97,110,95,110,97,109,101,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,3,8,0,0,0,0,0,0,0,0,3,8,1,0,0,0,0,0,0,0,0,2,1,5,10,2,1,1,1,0,6,14,10,0,41,0,12,2,11,2,3,7,7,1,39,10,0,43,0,12,1,11,1,16,0,20,2,2,1,0,7,14,11,1,17,0,12,3,11,3,3,9,11,0,1,7,0,39,11,0,11,2,18,0,45,0,2,0,0,0};
//!     size_t length = sizeof bytes / sizeof *bytes;
//!     Result result = move_decompile(bytes, length, Diem);
//!     char *src = result.result;
//!     printf("%s\n", src);
//! }
//! ```

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;

use crate::{Config, decompile_str, VERSION};

/// Returns C-compatible, nul-terminated string with no nul bytes in the
/// middle. __Ownership__ of result is yours, feel free to free.
#[no_mangle]
#[export_name = "move_decompiler_ver"]
pub extern "C" fn version() -> *mut c_char {
    to_cstring(VERSION).into_raw()
}

#[repr(C)]
pub enum SourceType {
    Diem,
    Pont,
    Dfinance,
}

#[repr(C)]
pub struct Result {
    result: *mut c_char,
    error: *mut c_char,
}

impl Default for Result {
    fn default() -> Self {
        Self {
            result: null_mut(),
            error: null_mut(),
        }
    }
}

/// Requires byte-array *uint8[] as two args: ptr and length.
/// Requires compat_mode boolean as true to adapt bc to dfi format.
/// Returns C-compatible, nul-terminated string with no nul bytes in the
/// middle. __Ownership__ of result is yours, feel free to free.
#[no_mangle]
#[export_name = "move_decompile"]
pub unsafe extern "C" fn decompile(
    bytes: *const u8,
    len: usize,
    source_type: SourceType,
) -> Result {
    let mut bytes = {
        if !bytes.is_null() {
            let borrowed = std::slice::from_raw_parts(bytes, len);
            borrowed.to_owned()
        } else {
            return Result {
                error: to_cstring("'bytes' ptr is null").into_raw(),
                ..Default::default()
            };
        }
    };

    if let Err(err) = match source_type {
        SourceType::Diem => compat::adapt_to_basis(&mut bytes, compat::AddressType::Diem),
        SourceType::Pont => Ok(()),
        SourceType::Dfinance => {
            compat::adapt_to_basis(&mut bytes, compat::AddressType::Dfninance)
        }
    } {
        return Result {
            error: to_cstring(err.to_string()).into_raw(),
            ..Default::default()
        };
    }

    let cfg = Config {
        light_version: false,
    };

    let out = match decompile_str(&bytes, cfg).map_err(map_err) {
        Ok(v) => v,
        Err(err) => {
            return Result {
                error: err.into_raw(),
                ..Default::default()
            };
        }
    };

    Result {
        result: to_cstring(out).into_raw(),
        ..Default::default()
    }
}

fn map_err<Err: std::fmt::Display>(err: Err) -> CString {
    to_cstring(err.to_string())
}

fn to_cstring<T: AsRef<str>>(s: T) -> CString {
    CString::new(s.as_ref()).expect("Cannot make C-string from Rust-string")
}
