use crate::file::RenderConfig;
use wasm_bindgen::JsValue;
use web_sys::Element;
use crate::html::element;
use web_sys::Document;

pub fn to_html(
    doc: &Document,
    code: &str,
    config: &RenderConfig,
) -> Result<Vec<Element>, JsValue> {
    let mut lines = vec![];
    for (i, line) in code.lines().enumerate() {
        let view_line = element(
            doc,
            "div",
            None,
            &["view-line"],
            &[
                ("top", &format!("{}px", config.line_height * i as u32)),
                ("height", &format!("{}px", config.line_height)),
            ],
        )?;

        let span = element(doc, "span", None, &[], &[])?;

        let mut is_empty = true;
        let mut is_first = true;
        for item in line.split(' ') {
            is_first = false;
            is_empty = false;

            let sp = element(
                doc,
                "span",
                None,
                &["cs1"],
                &[],
            )?;

            if item == "" {
                sp.set_text_content(Some(" "));
            } else {
                if is_first {
                    sp.set_text_content(Some(item));
                } else {
                    sp.set_text_content(Some(&format!(" {}", item)));
                }
            }
            span.append_child(sp.as_ref())?;
        }

        if is_empty {
            span.append_child(element(doc, "span", None, &[], &[])?.as_ref())?;
        }

        view_line.append_child(span.as_ref())?;
        lines.push(view_line);
    }

    //<div style="top:324px;height:18px;" class="view-line">
    // <span><span></span></span></div>
    // <div style="top:342px;height:18px;" class="view-line">
    // <span><span class="mtk5">use</span><span class="mtk1">&nbsp;</span><span class="mtk16">stdweb</span><span class="mtk1">::</span><span class="mtk16">web</span><span class="mtk1">::</span><span class="mtk16">event</span><span class="mtk1">::{</span></span></div><div style="top:360px;height:18px;" class="view-line"><span><span class="mtk1">&nbsp;&nbsp;&nbsp;&nbsp;</span><span class="mtk16">DoubleClickEvent</span><span class="mtk1">,</span></span></div><div style="top:378px;height:18px;" class="view-line"><span><span class="mtk1">&nbsp;&nbsp;&nbsp;&nbsp;</span><span class="mtk16">ClickEvent</span><span class="mtk1">,</span></span></div>

    Ok(lines)
}

pub fn to_code(lines: Vec<Element>) -> Result<String, JsValue> {
    todo!()
}

#[test]
fn test() {
    let code = r#"
use wasm_bindgen::JsValue;
use anyhow::{Error, anyhow};
use wasm_bindgen::prelude::*;
use web_sys::{Window, Document};

pub fn api_url() -> String {
    format!(
        "{}/api/rpc",
        web_sys::window()
            .and_then(|win| win.location().origin().ok())
            .unwrap_or_default()
    )
}

pub fn window() -> Result<Window, Error> {
    web_sys::window().ok_or_else(|| anyhow!("no global `window` exists"))
}

pub fn document(win: &Window) -> Result<Document, Error> {
    win.document()
        .ok_or_else(|| anyhow!("should have a document on window"))
}
    "#;

    for line in code.lines() {
        let mut first = true;
        for i in line.split(' ') {
            if first {
                first = false;
                print!("{}", i);
            } else {
                print!(" {}", i);
            }
        }
        println!();
    }
}
