use anyhow::Error;
use crate::env::{
    http::{Request, Response as HttpResponse},
    MemPtr,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: i16,
    pub response: String,
}

pub fn http_request(req: Request) -> Result<HttpResponse, Error> {
    let res = serde_json::to_vec(&req)?;
    let ptr = unsafe { send_http_request(res.as_ptr() as i32, res.len() as i32) };
    let ptr = MemPtr::from(ptr);
    let resp = serde_json::from_slice::<Response>(ptr.as_slice())?;
    if resp.status < 0 {
        Err(Error::msg(resp.response))
    } else {
        Ok(HttpResponse {
            status: resp.status as u16,
            response: resp.response,
        })
    }
}

extern "C" {
    #[link(name = "send_http_request")]
    pub fn send_http_request(ptr: i32, len: i32) -> i64;
}
