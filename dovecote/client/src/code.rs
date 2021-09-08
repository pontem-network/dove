use crate::file::RenderConfig;
use wasm_bindgen::JsValue;
use web_sys::Element;
use crate::html::element;
use web_sys::Document;
use proto::file::{File, FileType};
use crate::sources::highlight::mark_code;
use crate::sources::highlight::toml::Toml;

pub fn to_html(
    doc: &Document,
    file_id: &str,
    code: &File,
    config: &RenderConfig,
) -> Result<Vec<Element>, JsValue> {
    let mut lines = vec![];
    match code.tp {
        FileType::Toml => {
            for (i, line) in mark_code::<Toml>(file_id, code.content.as_str()).into_iter().enumerate() {
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

                for (style, content) in line.items {
                    let sp = element(
                        doc,
                        "span",
                        None,
                        &[style.as_style_name()],
                        &[],
                    )?;
                    sp.set_text_content(Some(content));
                    span.append_child(sp.as_ref())?;
                }
                view_line.append_child(span.as_ref())?;

                lines.push(view_line);
            }
        }
        FileType::Move | FileType::Uncnown => {
            let mut lines = vec![];
            for (i, line) in code.content.lines().enumerate() {
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
                            is_first = false;
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
        }
    }
    Ok(lines)
}

pub fn to_code(_lines: Vec<Element>) -> Result<String, JsValue> {
    todo!()
}
