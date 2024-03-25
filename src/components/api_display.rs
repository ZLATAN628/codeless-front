use gloo::{
    events::EventListener,
    utils::{document, window},
};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::components::{api_editor_resp::ApiEditorResp, info::api_info::ApiInfo};

#[function_component(ApiDisplay)]
pub fn api_display() -> Html {
    let top_height = use_state(|| 60.0);
    let bottom_height = use_state(|| 40.0);
    let is_dragging = use_state(|| false);
    let div_node = use_node_ref();

    let is_dragging_clone = is_dragging.clone();
    let mouse_down = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        is_dragging_clone.set(true);
    });

    let top_height_clone = top_height.clone();
    let bottom_height_clone = bottom_height.clone();
    let is_dragging_clone: UseStateHandle<bool> = is_dragging.clone();

    let div_node_clone = div_node.clone();
    use_effect(move || {
        let document = document();
        let is_dragging_clone2 = is_dragging_clone.clone();

        let mut title_bar_height = 0.0;
        if let Some(div_node) = div_node_clone.cast::<HtmlElement>() {
            let rect = div_node.get_bounding_client_rect();
            title_bar_height = rect.y();
        }

        let mouseup_listener = EventListener::new(&document, "mouseup", move |_| {
            if *is_dragging_clone {
                is_dragging_clone.set(false);
            }
        });

        let mousemove_listener = EventListener::new(&document, "mousemove", move |event| {
            let event = event.dyn_ref::<MouseEvent>().unwrap();
            if *is_dragging_clone2 {
                let top = (event.client_y() as f64 - title_bar_height)
                    / (window().inner_height().unwrap().as_f64().unwrap() - title_bar_height)
                    * 100.0;
                if top > 10.0 && top < 90.0 {
                    top_height_clone.set(top);
                    let bottom = 100.0 - top;
                    bottom_height_clone.set(bottom);
                }
            }
        });

        move || {
            drop(mouseup_listener);
            drop(mousemove_listener);
        }
    });

    html! {
        <div ref={div_node} class="flex flex-col h-screen w-screen">
            <ApiEditorResp height={*top_height}/>
            <div onmousedown={mouse_down} class="cursor-row-resize border border-neutral border-coll border-collapse bg-base-200"><h2>{"\u{00a0}\u{00a0}接口信息"}</h2> </div>
            <ApiInfo height={(*bottom_height).clone()}/>
        </div>
    }
}
