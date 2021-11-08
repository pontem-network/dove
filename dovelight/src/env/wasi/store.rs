use anyhow::Error;
use crate::env::MemPtr;

mod platform {
    extern "C" {
        pub fn store(key_ptr: i32, key_len: i32, val_ptr: i32, val_len: i32) -> i64;
        pub fn drop(key_ptr: i32, key_len: i32) -> i64;
        pub fn load(key_ptr: i32, key_len: i32) -> i64;
    }
}

pub fn store(key: String, val: Vec<u8>) -> Result<(), Error> {
    let ptr = unsafe {
        platform::store(
            key.as_ptr() as i32,
            key.len() as i32,
            val.as_ptr() as i32,
            val.len() as i32,
        )
    };
    let ptr = MemPtr::from(ptr);
    let res: Result<(), String> = serde_json::from_slice(ptr.as_slice())?;
    res.map_err(|err| Error::msg(err))
}

pub fn load(key: String) -> Result<Option<Vec<u8>>, Error> {
    let ptr = unsafe { platform::load(key.as_ptr() as i32, key.len() as i32) };
    let ptr = MemPtr::from(ptr);
    serde_json::from_slice::<Result<Option<Vec<u8>>, String>>(ptr.as_slice())?
        .map_err(|err| Error::msg(err))
}

pub fn drop(key: String) -> Result<(), Error> {
    let ptr = unsafe { platform::drop(key.as_ptr() as i32, key.len() as i32) };
    let ptr = MemPtr::from(ptr);

    let res: Result<(), String> = serde_json::from_slice(ptr.as_slice())?;
    res.map_err(|err| Error::msg(err))
}
