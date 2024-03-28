use gloo::{
    events::EventListener,
    utils::{document, window},
};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::components::{editor::api_editor::ApiEditor, response::api_response::ApiResponse};

#[derive(PartialEq, Properties)]
pub struct ApiEditorRespProps {
    pub height: f64,
}

#[function_component(ApiEditorResp)]
pub fn api_editor_resp(props: &ApiEditorRespProps) -> Html {
    let style = format!("height: {}%", &props.height);
    let left_width = use_state(|| 60.0);
    let right_width = use_state(|| 40.0);
    let is_dragging = use_state(|| false);
    let div_node = use_node_ref();

    let is_dragging_clone = is_dragging.clone();
    let mouse_down = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        is_dragging_clone.set(true);
    });

    let left_width_clone = left_width.clone();
    let right_width_clone = right_width.clone();
    let is_dragging_clone = is_dragging.clone();

    let div_node_clone = div_node.clone();
    use_effect(move || {
        let document = document();
        let is_dragging_clone2 = is_dragging_clone.clone();
        let mouseup_listener = EventListener::new(&document, "mouseup", move |_| {
            if *is_dragging_clone {
                is_dragging_clone.set(false);
            }
        });
        let mut file_select_width = 0.0;
        if let Some(div_node) = div_node_clone.cast::<HtmlElement>() {
            let rect = div_node.get_bounding_client_rect();
            file_select_width = rect.x();
        }

        let mousemove_listener = EventListener::new(&document, "mousemove", move |event| {
            let event = event.dyn_ref::<MouseEvent>().unwrap();
            if *is_dragging_clone2 {
                let left = (event.client_x() as f64 - file_select_width)
                    / (window().inner_width().unwrap().as_f64().unwrap() - file_select_width)
                    * 100.0;
                if left > 10.0 && left < 90.0 {
                    left_width_clone.set(left);
                    let right = 100.0 - left;
                    right_width_clone.set(right);
                }
            }
        });

        move || {
            drop(mouseup_listener);
            drop(mousemove_listener);
        }
    });
    html! {
        <div ref={div_node} class="flex flex-row w-full justify-between border-r border-neutral border-collapse" style={style}>
            <ApiEditor width={*left_width} />
            <div onmousedown={mouse_down} class="w-[10px] cursor-col-resize">{"\u{00a0}"}</div>
            <ApiResponse width={*right_width} />
        </div>
    }
}
