use web_sys::{XmlHttpRequest, Blob, BlobPropertyBag};
use wasm_bindgen::JsValue;
use anyhow::anyhow;
use anyhow::Error;
use crate::env::http::{Request, Response};

pub fn http_request(req: Request) -> Result<Response, Error> {
    let err = |err| anyhow!("Failed to send http request:{:?}", err);
    let xhr = XmlHttpRequest::new().map_err(err)?;
    xhr.open_with_async(&req.method, &req.url, false)
        .map_err(err)?;

    for header in &req.headers {
        xhr.set_request_header(&header.0, &header.1).map_err(err)?;
    }

    let blob = if let Some(body) = &req.body {
        Some(
            Blob::new_with_str_sequence_and_options(
                &JsValue::from_serde(&vec![serde_json::to_string(body)?])?,
                &BlobPropertyBag::new().type_("application/json"),
            )
            .map_err(err)?,
        )
    } else {
        None
    };

    xhr.send_with_opt_blob(blob.as_ref()).map_err(err)?;
    let status = xhr.status().map_err(err)?;

    Ok(if status != 200 {
        Response {
            status,
            response: format!(
                "Failed to get module :{:?}. Error:{}-{:?}",
                req,
                status,
                xhr.status_text().map_err(err)
            ),
        }
    } else {
        Response {
            status: 200,
            response: xhr.response_text().map_err(err)?.unwrap_or_default(),
        }
    })
}
