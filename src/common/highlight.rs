use gloo::utils::window;
use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};

use super::js_utils;

// use crate::common::binding::hljs;

pub fn highlight_code(code: &str) -> String {
    let hljs = window().get("hljs").unwrap();
    let highlight = Reflect::get(&hljs, &JsValue::from_str("highlight")).unwrap();
    if highlight.is_function() {
        let option = Object::new();
        js_utils::set_value(&option, "language", "sql").unwrap();
        let res = highlight
            .dyn_ref::<Function>()
            .unwrap()
            .call2(&hljs, &JsValue::from_str(code), &option)
            .unwrap();

        return js_utils::get_str_value(&res, "value").unwrap();
    }
    "code".to_string()
}
