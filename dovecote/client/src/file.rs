use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;
use crate::context::*;
use crate::{api, js_err};
use proto::project::ID;
use proto::Request;
use proto::file::{GetFile, File};
use crate::console_log;
use anyhow::Error;
use anyhow::anyhow;
use web_sys::{Document, Element};
use std::convert::TryInto;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use crate::html::element;

#[wasm_bindgen]
pub async fn open_file(
    project_id: ID,
    file_id: ID,
    container_id: String,
    config: JsValue,
) -> Result<(), JsValue> {
    let config: RenderConfig = config.into_serde().map_err(js_err)?;
    console_log!(
        "open_file:{}-{}=>{} cfg:{:?}",
        project_id,
        file_id,
        container_id,
        config
    );
    let get_file = GetFile {
        project_id,
        file_id,
    };

    let file = proto::get_file(&api_url(), get_file)
        .await
        .map_err(js_err)?;
    render(&container_id, config, file)
}

fn render(container_id: &str, config: RenderConfig, file: File) -> Result<(), JsValue> {
    let win = window().map_err(js_err)?;
    let doc = document(&win).map_err(js_err)?;
    let container = doc
        .get_element_by_id(&container_id)
        .ok_or_else(|| anyhow!("Element with id '{}' was not fount", container_id))
        .map_err(js_err)?;
    container.set_text_content(None);

    container.append_child(render_lines(&doc, &config, 10)?.as_ref())?;
    console_log!("{:?}", container);
    console_log!("open_file:{:?}", file);
    Ok(())
}

fn render_lines(doc: &Document, config: &RenderConfig, count: u32) -> Result<Element, JsValue> {
    let container = element(
        doc,
        "div",
        Some("line-numbers-container"),
        &[],
        &[
            ("position", "absolute"),
            ("transform", "translate3d(0px, 0px, 0px)"),
            ("height", "100%"),
            ("width", "66px"),
        ],
    )?;

    for i in 0..count {
        let line = element(
            doc,
            "div",
            Some(&format!("line-numbers_{}", i)),
            &["line-numbers"],
            &[
                ("left", "18px"),
                ("width", "22px"),
            ],
        )?;
        line.set_text_content(Some(&(i + 1).to_string()));
        let line_container = element(
            doc,
            "div",
            Some(&format!("line-numbers-container_{}", i)),
            &["line-numbers-container"],
            &[
                ("position", "absolute"),
                ("top", &format!("{}px", config.line_height * i)),
                ("width", "100%"),
                ("height", &format!("{}px", config.line_height)),
            ],
        )?;

        line_container.append_child(line.as_ref())?;
        container.append_child(line_container.as_ref())?;
    }

    // return Ok(Node::from_html(r#"<div style="position: absolute; transform: translate3d(0px, 0px, 0px); top: 0; height: 100%; width: 66px;">
    //             <div style="position:absolute;top:0px;width:100%;height:18px;">
    //                 <div class="line-numbers" style="left:18px;width:22px;">1</div>
    //             </div>
    //             <div style="position:absolute;top:18px;width:100%;height:18px;">
    //                 <div class="line-numbers" style="left:18px;width:22px;">2</div>
    //             </div>
    //             <div style="position:absolute;top:36px;width:100%;height:18px;">
    //                 <div class="line-numbers" style="left:18px;width:22px;">3</div>
    //             </div>
    //         </div>"#).unwrap());

    /*
    <div style="position: absolute; transform: translate3d(0px, 0px, 0px); top: 0; height: 100%; width: 66px;">
               <div style="position:absolute;top:0px;width:100%;height:18px;">
                   <div class="line-numbers" style="left:18px;width:22px;">1</div>
               </div>
               <div style="position:absolute;top:18px;width:100%;height:18px;">
                   <div class="line-numbers" style="left:18px;width:22px;">2</div>
               </div>
               <div style="position:absolute;top:36px;width:100%;height:18px;">
                   <div class="line-numbers" style="left:18px;width:22px;">3</div>
               </div>
           </div>
    */
    Ok(container)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenderConfig {
    #[serde(default = "line_height")]
    pub line_height: u32,
}

fn line_height() -> u32 {
    18
}
