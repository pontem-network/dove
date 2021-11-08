use core::mem;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use wasmer::{Array, Instance, Memory, NativeFunc, WasmPtr};
use anyhow::Error;

static ENV: Lazy<RwLock<Option<Env>>> = Lazy::new(RwLock::default);

static STORE: Lazy<RwLock<HashMap<String, Vec<u8>>>> = Lazy::new(RwLock::default);

struct Env {
    mem: Memory,
    make_buffer: NativeFunc<i32, WasmPtr<u8, Array>>,
    drop_buffer: NativeFunc<(i32, i32), ()>,
}

impl Env {
    pub fn new(instance: &Instance) -> Result<Env, Error> {
        Ok(Env {
            mem: instance.exports.get_memory("memory")?.to_owned(),
            make_buffer: instance
                .exports
                .get_function("make_buffer")?
                .native::<i32, WasmPtr<u8, Array>>()?,
            drop_buffer: instance
                .exports
                .get_function("drop_buffer")?
                .native::<(i32, i32), ()>()?,
        })
    }
}

pub fn mem_store(buff: &[u8]) -> Result<MemPtr, Error> {
    let env = ENV.read().unwrap();
    let env = env.as_ref().unwrap();
    let ptr = env.make_buffer.call(buff.len() as i32)?;

    let values = ptr.deref(&env.mem, 0, buff.len() as u32).unwrap();
    for i in 0..buff.len() {
        values[i].set(buff[i]);
    }
    Ok(MemPtr {
        ptr: ptr.offset() as i32,
        len: buff.len() as i32,
        auto_release: true,
    })
}

pub fn mem_drop(ptr: &MemPtr) {
    let env = ENV.read().unwrap();
    let env = env.as_ref().unwrap();
    env.drop_buffer
        .call(ptr.ptr as i32, ptr.len as i32)
        .unwrap();
}

pub fn init(instance: &Instance) -> Result<(), Error> {
    *ENV.write().unwrap() = Some(Env::new(instance)?);
    Ok(())
}

pub fn get_utf8_string(ptr: WasmPtr<u8, Array>, len: i32) -> String {
    let env = ENV.read().unwrap();
    let env = env.as_ref().unwrap();
    ptr.get_utf8_string(&env.mem, len as u32).unwrap()
}

pub fn get_buff(ptr: WasmPtr<u8, Array>, len: i32) -> Vec<u8> {
    let env = ENV.read().unwrap();
    let env = env.as_ref().unwrap();

    let memory_size = env.mem.size().bytes().0;
    if ptr.offset() as usize + len as usize > env.mem.size().bytes().0
        || ptr.offset() as usize >= memory_size
    {
        return vec![];
    }

    let view = env.mem.view::<u8>();

    let mut vec: Vec<u8> = Vec::with_capacity(len as usize);
    let base = ptr.offset() as usize;
    for i in 0..(len as usize) {
        let byte = view[base + i].get();
        vec.push(byte);
    }
    vec
}

#[derive(Debug)]
pub struct MemPtr {
    pub ptr: i32,
    pub len: i32,
    auto_release: bool,
}

impl MemPtr {
    pub fn wasm_ptr(&self) -> WasmPtr<u8, Array> {
        WasmPtr::new(self.ptr as u32)
    }

    pub fn set_auto_release(&mut self, val: bool) {
        self.auto_release = val;
    }
}

impl Drop for MemPtr {
    fn drop(&mut self) {
        if self.auto_release {
            mem_drop(self)
        }
    }
}

impl From<i64> for MemPtr {
    fn from(val: i64) -> Self {
        let (ptr, len) = unsafe { mem::transmute::<i64, (i32, i32)>(val) };
        MemPtr {
            ptr,
            len,
            auto_release: true,
        }
    }
}

pub fn log(ptr: WasmPtr<u8, Array>, len: i32) {
    println!("{}", get_utf8_string(ptr, len));
}

pub fn send_http_request(ptr: WasmPtr<u8, Array>, len: i32) -> i64 {
    let req = get_utf8_string(ptr, len);
    println!("send_http_request:{:?}", req);
    let resp = _send_http_request(req).unwrap_or_else(|err| Response {
        status: -1,
        response: err.to_string(),
    });
    let bytes = serde_json::to_vec(&resp).unwrap_or_else(|_| {
        format!("{{\"status\": -1, \"response\":\"{}\"}}", resp.response).into_bytes()
    });
    let mut ptr = mem_store(&bytes).unwrap();
    ptr.set_auto_release(false);
    unsafe { mem::transmute((ptr.ptr, ptr.len)) }
}

fn _send_http_request(req: String) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
    let req: Request = serde_json::from_str(&req)?;
    let mut builder = client.request(reqwest::Method::from_str(&req.method)?, req.url);
    if let Some(body) = req.body {
        builder = builder.body(body);
    }

    for (key, val) in req.headers {
        builder = builder.header(key, val);
    }

    let res = builder.send()?;
    Ok(Response {
        status: res.status().as_u16() as i16,
        response: res.text()?,
    })
}

pub fn store(
    key_ptr: WasmPtr<u8, Array>,
    key_len: i32,
    val_ptr: WasmPtr<u8, Array>,
    val_len: i32,
) -> i64 {
    let key = get_utf8_string(key_ptr, key_len);
    let val = get_buff(val_ptr, val_len);
    println!("store:{}", key);
    let mut store = STORE.write().unwrap();
    store.insert(key, val);
    let res = serde_json::to_vec::<Result<(), String>>(&Ok(())).unwrap();
    let mut ptr = mem_store(&res).unwrap();
    ptr.set_auto_release(false);
    unsafe { mem::transmute((ptr.ptr, ptr.len)) }
}

pub fn drop(key_ptr: WasmPtr<u8, Array>, key_len: i32) -> i64 {
    let key = get_utf8_string(key_ptr, key_len);
    println!("drop:{}", key);
    let mut store = STORE.write().unwrap();
    store.remove(&key);

    let res = serde_json::to_vec::<Result<(), String>>(&Ok(())).unwrap();
    let mut ptr = mem_store(&res).unwrap();
    ptr.set_auto_release(false);
    unsafe { mem::transmute((ptr.ptr, ptr.len)) }
}

pub fn load(key_ptr: WasmPtr<u8, Array>, key_len: i32) -> i64 {
    let key = get_utf8_string(key_ptr, key_len);
    println!("load:{}", key);
    let store = STORE.read().unwrap();
    let res = Ok(store.get(&key));

    let res = serde_json::to_vec::<Result<Option<&Vec<u8>>, String>>(&res).unwrap();
    let mut ptr = mem_store(&res).unwrap();
    ptr.set_auto_release(false);
    unsafe { mem::transmute((ptr.ptr, ptr.len)) }
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: i16,
    pub response: String,
}
