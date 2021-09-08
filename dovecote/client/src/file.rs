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
use crate::code::to_html;

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

    let f_id = file_id.clone();
    let get_file = GetFile {
        project_id,
        file_id,
    };

    let file = proto::get_file(&api_url(), get_file)
        .await
        .map_err(js_err)?;
    render(&container_id, &f_id, config, file)
}

fn render(
    container_id: &str,
    file_id: &str,
    config: RenderConfig,
    file: File,
) -> Result<(), JsValue> {
    let win = window().map_err(js_err)?;
    let doc = document(&win).map_err(js_err)?;
    let container = doc
        .get_element_by_id(&container_id)
        .ok_or_else(|| anyhow!("Element with id '{}' was not fount", container_id))
        .map_err(js_err)?;
    container.set_text_content(None);

    let (code_lines, element) = render_code(&doc, &config, file_id, file.content.as_ref())?;
    container.append_child(element.as_ref())?;
    container.append_child(render_lines(&doc, &config, file_id, code_lines)?.as_ref())?;
    Ok(())
}

fn render_code(
    doc: &Document,
    config: &RenderConfig,
    file_id: &str,
    code: &str,
) -> Result<(u32, Element), JsValue> {
    let view_lines = element(
        doc,
        "div",
        Some(&format!("view-lines-{}", file_id)),
        &["view-lines", "mouse-cursor-text"],
        &[
            ("position", "absolute"),
            ("overflow", "hidden"),
            ("left", "66px"),
            ("line-height", &format!("{}px", config.line_height)),
        ],
    )?;
    let lines = to_html(doc, code, config)?;
    let count = lines.len() as u32;
    for line in lines {
        view_lines.append_child(line.as_ref())?;
    }

    Ok((count, view_lines))
}

fn render_lines(
    doc: &Document,
    config: &RenderConfig,
    file_id: &str,
    count: u32,
) -> Result<Element, JsValue> {
    let container = element(
        doc,
        "div",
        Some(&format!("line-numbers-container-{}", file_id)),
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
            Some(&format!("line-numbers-{}-{}", i, file_id)),
            &["line-numbers"],
            &[("left", "18px"), ("width", "22px")],
        )?;
        line.set_text_content(Some(&(i + 1).to_string()));
        let line_container = element(
            doc,
            "div",
            Some(&format!("line-numbers-container-{}-{}", i, file_id)),
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
