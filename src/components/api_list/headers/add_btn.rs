use gloo::console::log;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, Node};
use yew::prelude::*;
use yew::virtual_dom::VNode;

use crate::backend::{NodeInfo, API_ORIGIN};
use crate::components::info::parameters::ApiParameters;
use crate::context::code_context::{use_codes, CodesStateMsg};
use crate::{components::tooltip::Tooltip, image::AddSvg};

macro_rules! form_label {
    ($name: expr, $id: expr) => {
        html! {
            <label class="form-control w-full">
                <div class="label">
                    <span class="label-text">{$name}</span>
                </div>
                <input id={$id} type="text" placeholder="" class="input input-bordered w-full" />
            </label>
        }
    };
}

const ADD_API_MODAL_ID: AttrValue = AttrValue::Static("add_api_modal");

#[derive(Properties, Clone, PartialEq)]
pub struct BtnProps {
    pub btn_classes: &'static str,
}

#[function_component(AddBtn)]
pub fn add_btn(props: &BtnProps) -> Html {
    html! {
        <Tooltip tip="新增接口">
            <label for={ADD_API_MODAL_ID} class={props.btn_classes}>
                <AddSvg />
            </label>
        </Tooltip>
    }
}

#[function_component(AddApiBtn)]
pub fn add_api_btn() -> Html {
    let code_context = use_codes();
    let node_info = code_context.get_node_info();
    let code_context_clone = code_context.clone();
    let group_change = Callback::from(move |e: Event| {
        let group_element = e.target_unchecked_into::<HtmlSelectElement>();
        let options = group_element.selected_options();
        let option = options.item(0).unwrap();
        let data = option.get_attribute("data").unwrap();
        option.set_text_content(Some(&data));
        let prefix = document()
            .get_element_by_id("new-code-path-prefix")
            .unwrap();

        let pre: &Node = prefix.as_ref();
        let input = pre.dyn_ref::<HtmlInputElement>().unwrap();
        input.set_value(&format!(
            "{}{}/",
            *API_ORIGIN,
            code_context_clone.get_final_path("", &option.get_attribute("value").unwrap())
        ));
    });

    let group_click = Callback::from(move |e: MouseEvent| {
        let group_element = e.target_unchecked_into::<HtmlSelectElement>();
        let options = group_element.options();
        for i in 0..options.length() {
            let op = options.item(i).unwrap();
            let data = op.get_attribute("data-show").unwrap();
            op.set_text_content(Some(&data));
        }
    });

    let group_blur = Callback::from(move |e: FocusEvent| {
        let group_element = e.target_unchecked_into::<HtmlSelectElement>();
        let options = group_element.selected_options();
        let option = options.item(0).unwrap();
        let data = option.get_attribute("data").unwrap();
        option.set_text_content(Some(&data));
    });

    let prevent_default_click = Callback::from(|e: MouseEvent| {
        e.prevent_default();
    });
    let confirm_click = Callback::from(|_e: MouseEvent| {
        log!("confirm_click");
        // code_context.dispatch(CodesStateMsg::InsertNewCode());
    });
    html! {
        <>
            <input type="checkbox" id={ADD_API_MODAL_ID} class="modal-toggle" />
            <div class="modal">
                <div class="max-w-5xl modal-box">
                    <h3 class="font-bold text-2xl text-center">{"新增接口"}</h3>
                    <div class="flex flex-col space-y-4 mt-4 justify-center	content-center">
                        {form_label!{"接口名称", "new-code-name"}}

                        <div class="flex justify-between">
                            <label class="form-control w-[15%]">
                                <div class="label">
                                    <span class="label-text">{"请求方法"}</span>
                                </div>
                                <select id="new-code-method" class="select select-bordered">
                                    <option selected={true}>{"GET"}</option>
                                    <option>{"POST"}</option>
                                </select>
                            </label>
                            <label class="form-control w-[84%]">
                                <div class="label" onclick={prevent_default_click}>
                                    <span class="label-text">{"接口分组"}</span>
                                </div>
                                <select id="new-code-group" class="select select-bordered"
                                    onchange={group_change}
                                    onclick={group_click}
                                    onblur={group_blur}>
                                    {generate_group_list(node_info)}
                                </select>
                            </label>
                        </div>

                        <label class="form-control w-full">
                            <div class="label">
                                <span class="label-text">{"接口路径"}</span>
                            </div>
                            <div class="flex w-full justify-between">
                                <input id="new-code-path-prefix" type="text" readonly={true} class="input input-bordered w-[49%]" />
                                <input id="new-code-path" type="text" placeholder="" class="input input-bordered w-[50%]" />
                            </div>
                        </label>

                        <label class="form-control w-full">
                            <div class="label">
                                <span class="label-text">{"接口描述"}</span>
                            </div>
                            <textarea id="new-code-description" class="textarea textarea-bordered h-24" placeholder=""></textarea>
                        </label>

                        <label class="form-control w-full">
                            <div class="label">
                                <span class="label-text">{"接口参数"}</span>
                            </div>
                            <ApiParameters is_new={true} params={None} readonly={false}/>
                        </label>
                    </div>

                    <div class="modal-action">
                        <label for={ADD_API_MODAL_ID} class="btn btn-ghost">{"取消"}</label>
                        <button onclick={confirm_click} class="btn">{"确认"}</button>
                    </div>
                </div>
            </div>
        </>
    }
}

fn generate_group_list(node_info: &NodeInfo) -> Html {
    let mut result: Vec<VNode> = vec![];
    for child in node_info.children.iter() {
        generate_group_list0(child, &mut result, 0);
    }
    result.into_iter().collect::<Html>()
}

fn generate_group_list0(node_info: &NodeInfo, result: &mut Vec<VNode>, hierarchy: usize) {
    if node_info.node.group_id.is_some() {
        return;
    }
    let prefix = if hierarchy > 0 {
        format!("|{}", "-".to_string().repeat(hierarchy))
    } else {
        "".to_string()
    };
    let group_name = node_info.node.name.clone().unwrap_or("".to_string());
    let value = node_info.node.id.clone().unwrap();
    let option = html! {
        <option data={group_name.clone()} data-show={format!("{}{}", prefix, group_name)} value={value}></option>
    };
    result.push(option);
    for child in node_info.children.iter() {
        generate_group_list0(child, result, hierarchy + 1);
    }
}
