use core::mem;
use anyhow::Error;
use crate::lang::compiler::source_map::SourceMap;
use crate::api;
use serde::{Serialize, Deserialize};
use crate::api::Unit;
use crate::env::make_buffer;
use std::slice::from_raw_parts_mut;

/// Creat a transaction request.
#[derive(Serialize, Deserialize, Debug)]
struct TxRequest {
    // Node address. http://localhost:9933/
    chain_api: String,
    // Project code. Scripts and modules
    source_map: SourceMap,
    // Dialect of the project. diem, dfinance, pont
    dialect: String,
    // Call String. NAME_SCRIPT<U8, BOOL>(1,[2,3])
    call: String,
}

/// Creating a transaction
#[no_mangle]
pub unsafe extern "C" fn tx(req_ptr: i32, req_len: i32) -> i64 {
    let req = String::from_utf8_lossy(&*std::slice::from_raw_parts(
        req_ptr as *mut u8,
        req_len as usize,
    ));
    let res = _tx(&req).map_err(|err| err.to_string());

    let result_buffer = serde_json::to_vec(&res)
        .unwrap_or_else(|err| format!("{{\"Err\": \"{}\"}}", err).as_bytes().to_vec());
    let ptr = make_buffer(result_buffer.len());
    let slice = from_raw_parts_mut(ptr as *mut u8, result_buffer.len());
    (&mut *slice).copy_from_slice(&result_buffer);
    mem::transmute((ptr as i32, result_buffer.len() as i32))
}

fn _tx(req: &str) -> Result<Unit, Error> {
    let req: TxRequest = serde_json::from_str(&req)?;
    api::tx(req.chain_api, req.source_map, req.dialect, req.call)
}
