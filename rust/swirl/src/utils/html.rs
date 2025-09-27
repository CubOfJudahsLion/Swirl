//! # Module        :   _HTML_
//! - **Purpose**   :   Utilities to search for HTML elements.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Thu Mar 11 11:55:43 AM CST 2021
//! - **Modified**  :   Mon 06/13/2022 03:14 PM


use wasm_bindgen::{JsValue, JsCast};
use web_sys::{window, Document, Element, HtmlCanvasElement, HtmlElement, HtmlHeadElement, HtmlLabelElement, Window};


/// Obtains the current window.
pub fn get_window() -> Result<Window, JsValue> {
    window()
        .ok_or_else(|| JsValue::from_str("Cannot find main window."))
}


/// Obtains the Document element.
pub fn get_document() -> Result<Document, JsValue> {
    get_window()?
        .document()
        .ok_or_else(|| JsValue::from_str("Cannot find Document element."))
}


/// Gets the document's head.
pub fn get_head() -> Result<HtmlHeadElement, JsValue> {
    get_document()?
        .head()
        .ok_or_else(|| JsValue::from_str("Couldn't find head element."))
}


/// Gets the document's body.
pub fn get_body() -> Result<HtmlElement, JsValue> {
    get_document()?
        .body()
        .ok_or_else(|| JsValue::from_str("Couldn't find document's body."))
}


//  Internal use. Builds a query using all these optional elements: tag, id and classes.
fn build_element_query(tag_name: Option<&str>, id: Option<&str>, classes: &[&str]) -> String {
    let mut v: Vec<String> = match tag_name {
        Some(t) =>  vec![t.to_string()],
        None    =>  Vec::new(),
    };
    v.append(
        &mut match id {
            Some(id)    => {
                if id.starts_with('#') {
                    vec![id.to_string()]
                }
                else {
                    let mut s = "#".to_string();
                    s.push_str(id);
                    vec![s]
                }
            }
            None        => Vec::<String>::new(),
        }
    );
    v.append(
        &mut classes
            .iter()
            .map(|class| {
                if class.starts_with('.') {
                    class.to_string()
                }
                else {
                    let mut s = ".".to_string();
                    s.push_str(class);
                    s
                }
            })
            .collect::<Vec<String>>()
    );
    v.join("")
}


/// Queries the current document for an element. `tag_name`, if provided, should be an ordinary tag name (e.g., `div`
/// or `input`); likewise, `id` can be with or without the initial hash-mark (`#`). The `classes` are allowed to be
/// with or without the initial period.
///
/// Returns an element (first to match the query) or a JsValue error.
pub fn get_html_element<ElementType>(tag_name: Option<&str>, id: Option<&str>, classes: &[&str]) -> Result<ElementType, JsValue>
    where   ElementType: JsCast
{
    let elem: Element = get_document()?
        .query_selector(&build_element_query(tag_name, id, classes))
        .and_then(|opt_elem|
            opt_elem.ok_or(JsValue::from_str("Can't resolve element."))
        )?;
    <Element as Into<JsValue>>::into(elem)
        .dyn_into::<ElementType>()
        .map_err(|_| JsValue::from_str("Cannot convert element to required type"))
}


/// Obtains the canvas referred to by the provided parameter.
pub fn get_canvas(id: &str) -> Result<HtmlCanvasElement, JsValue> {
    window()
        .and_then(|wnd| wnd.document())
        .and_then(|doc| doc.get_element_by_id(id))
        .ok_or(JsValue::from_str("Canvas element not found"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Target element is not an HTMLCanvasElement."))
}


/// Obtains the common canvas. If said canvas has not been
/// created, the function takes care of building and
/// positioning it (hidden) at the end of the document's
/// body.
///
/// Several operations in this library might resort to an extra
/// canvas for intermediate operations -- and as long as they
/// request the same kind of context, they'll get the same one.
#[inline(always)]
pub fn get_common_canvas() -> Result<HtmlCanvasElement, JsValue> {
    use crate::COMMON_CANVAS_NAME;
    get_canvas(COMMON_CANVAS_NAME)
        .or_else(|_| {
            let doc     = get_document()?;
            let canvas  = doc
                .create_element("canvas")?
                .unchecked_into::<HtmlCanvasElement>();
            canvas.set_id(COMMON_CANVAS_NAME);
            canvas.style().set_property("visibility", "hidden")?;
            let body    = doc
                .body()
                .ok_or_else(|| JsValue::from_str("Couldn't find document's body"))?;
            body.append_with_node_1(&canvas)?;
            Ok(canvas)
        })
}


/// Obtains the label referred to by its id.
pub fn get_label(id: &str) -> Result<HtmlLabelElement, JsValue> {
    window()
        .and_then(|wnd| wnd.document())
        .and_then(|doc| doc.get_element_by_id(id))
        .ok_or(JsValue::from_str("Info label not found"))?
        .dyn_into::<HtmlLabelElement>()
        .map_err(|_| JsValue::from_str("Target element is not an HTMLLabelElement."))
}


#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    use super::*;

    #[wasm_bindgen_test]
    fn win_doc_body_head() {
        assert_eq!(get_window().unwrap(), window().unwrap());
        let window = window().unwrap();
        assert_eq!(get_document().unwrap(), window.document().unwrap());
        let document = window.document().unwrap();
        assert_eq!(get_body().unwrap(), document.body().unwrap());
        assert_eq!(get_head().unwrap(), document.head().unwrap());
    }

    #[wasm_bindgen_test]
    fn canvas_and_common_canvas() {
        let common_canvas = get_common_canvas().unwrap();
        assert_eq!(common_canvas, get_canvas(crate::COMMON_CANVAS_NAME).unwrap());
        assert_eq!(common_canvas, get_common_canvas().unwrap());
    }
}
