use gloo::utils::document;
use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, JsValue};

pub fn open_modal(id: &str) {
    if let Some(modal) = document().get_element_by_id(id) {
        let show_modal = Reflect::get(&modal, &JsValue::from_str("showModal")).unwrap();
        if show_modal.is_function() {
            show_modal
                .dyn_ref::<Function>()
                .unwrap()
                .call0(&modal)
                .unwrap();
        }
    }
}
