use std::collections::HashMap;
use wasmer::{Instance, NativeFunc};
use serde::{Serialize, Deserialize};
use anyhow::Error;
use crate::env::{get_utf8_string, mem_store, MemPtr};

pub struct Dove {
    tx: NativeFunc<(i32, i32), i64>,
}

impl Dove {
    pub fn new(instance: &Instance) -> Result<Dove, Box<dyn std::error::Error>> {
        let tx = instance
            .exports
            .get_function("tx")?
            .native::<(i32, i32), i64>()?;

        Ok(Dove { tx })
    }

    pub fn tx(
        &self,
        // Node address. http://localhost:9933/
        chain_api: String,
        // Project code. Scripts and modules
        source_map: SourceMap,
        // Dialect of the project. diem, dfinance, pont
        dialect: String,
        // Call String. NAME_SCRIPT<U8, BOOL>(1,[2,3])
        call: String,
    ) -> Result<Unit, Error> {
        let req = TxRequest {
            chain_api,
            source_map,
            dialect,
            call,
        };
        let req = serde_json::to_string(&req)?;
        let ptr = mem_store(req.as_bytes())?;
        let res = MemPtr::from(self.tx.call(ptr.ptr as i32, ptr.len as i32)?);
        serde_json::from_str::<Result<Unit, String>>(&get_utf8_string(res.wasm_ptr(), res.len))?
            .map_err(Error::msg)
    }
}

/// Creat a transaction request.
#[derive(Serialize, Deserialize)]
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

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SourceMap {
    pub source_map: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub bytecode: Vec<u8>,
}
