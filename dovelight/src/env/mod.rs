#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod js;
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
mod wasi;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use crate::env::js::log::{log};
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
pub use crate::env::wasi::log::{log};
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
pub use crate::env::wasi::{make_buffer, drop_buffer, MemPtr};

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::env::log(&format_args!($($t)*).to_string()))
}

pub mod http {
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub use crate::env::js::http::http_request;
    #[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
    pub use crate::env::wasi::http::http_request;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Request {
        pub method: String,
        pub url: String,
        pub headers: Vec<(String, String)>,
        pub body: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Response {
        pub status: u16,
        pub response: String,
    }
}

pub mod store {
    use anyhow::Error;
    use serde::ser::Serialize;
    use serde::de::DeserializeOwned;

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub use crate::env::js::store::*;
    #[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
    pub use crate::env::wasi::store::*;

    pub fn set<V: Serialize>(key: String, val: &V) -> Result<(), Error> {
        let val = bcs::to_bytes(val)?;
        store(key, val)
    }

    pub fn set_string(key: String, val: &str) -> Result<(), Error> {
        store(key, val.as_bytes().to_vec())
    }

    pub fn get<'a, V: DeserializeOwned>(key: String) -> Result<Option<V>, Error> {
        Ok(if let Some(buff) = load(key)? {
            Some(bcs::from_bytes(&buff)?)
        } else {
            None
        })
    }

    pub fn get_string(key: String) -> Result<Option<String>, Error> {
        load(key).map(|buf| buf.map(|buf| String::from_utf8_lossy(&buf).to_string()))
    }

    pub fn delete(key: String) -> Result<(), Error> {
        drop(key)
    }
}
