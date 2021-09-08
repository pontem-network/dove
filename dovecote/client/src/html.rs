use wasm_bindgen::JsValue;
use web_sys::{Document, Element};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use crate::js_err;
use anyhow::anyhow;

pub fn element(
    doc: &Document,
    tp: &str,
    id: Option<&str>,
    classes: &[&str],
    styles: &[(&str, &str)],
) -> Result<Element, JsValue> {
    let container = doc.create_element(tp)?;
    if let Some(id) = id {
        container.set_id(id);
    }

    let html = container
        .dyn_ref::<HtmlElement>()
        .ok_or_else(|| anyhow!("#script should be an `HtmlElement`"))
        .map_err(js_err)?;

    let class_list = container.class_list();
    for class in classes {
        class_list.add_1(class)?;
    }

    let style = html.style();
    for (name, val) in styles {
        style.set_property(name, val)?;
    }

    Ok(container)
}
