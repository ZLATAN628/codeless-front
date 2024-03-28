use gloo::console::log;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsCast;

use gloo::utils::{document, window};
use wasm_bindgen::JsValue;
use web_sys::{HtmlElement, HtmlInputElement, HtmlSelectElement, Node};
use yew::prelude::*;
use yew::virtual_dom::VNode;

use crate::components::api_list::api_select::DESKTOP_HEADER_BTN_CLASSES;
use crate::context::code_context::CodeParameter;
use crate::image::{AddSvg2, SubSvg2};

#[derive(Properties, Clone, PartialEq)]
pub struct ApiParametersProps {
    /// 是否新增接口页
    pub is_new: bool,
    pub params: Option<Vec<CodeParameter>>,
}

pub struct ApiParameters {
    cur_params: Rc<RefCell<Vec<CodeParameter>>>,
    is_new: bool,
}

pub enum ApiParametersMsg {
    ADD,
    SUB(i32),
}

impl Component for ApiParameters {
    type Message = ApiParametersMsg;
    type Properties = ApiParametersProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let ApiParametersProps { is_new, params } = props;
        let params = match params.as_ref() {
            Some(params) => params.clone(),
            None => Vec::new(),
        };
        Self {
            cur_params: Rc::new(RefCell::new(params)),
            is_new: *is_new,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApiParametersMsg::ADD => {
                let mut params = self.cur_params.borrow_mut();
                save_parameters(&mut params, self.is_new);
                params.push(CodeParameter::default());
            }
            ApiParametersMsg::SUB(index) => {
                let mut params = self.cur_params.borrow_mut();
                save_parameters(&mut params, self.is_new);
                if !params.is_empty() {
                    if index == -1 {
                        params.pop();
                    } else {
                        params.remove(index as usize);
                    }
                }
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let ApiParametersProps { is_new: _, params } = ctx.props();
        let mut cp = self.cur_params.borrow_mut();
        match params.as_ref() {
            Some(params) => {
                *cp = params.clone();
            }
            None => {
                *cp = Vec::new();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes;
        if self.is_new {
            classes = "flex flex-row w-full text-xs";
        } else {
            classes = "flex flex-row w-full text-sm";
        };

        let add_click = ctx.link().callback(|_| ApiParametersMsg::ADD);

        let is_new = self.is_new;
        let sub_click = ctx.link().callback(move |_| {
            let index = match get_params_index() {
                Ok(index) => index,
                Err(e) => {
                    log!(e);
                    -1
                }
            };
            ApiParametersMsg::SUB(index)
        });
        // self.cur_params.
        let params = self.cur_params.borrow();
        let param_html = generate_parameters(params);

        let id = get_dom_id(is_new);

        html! {
            <div class="flex h-full">
                <div class="flex-col w-min">
                    <label class={DESKTOP_HEADER_BTN_CLASSES} onclick={add_click}>
                        <AddSvg2 />
                    </label>
                    <label class={DESKTOP_HEADER_BTN_CLASSES} onclick={sub_click}>
                        <SubSvg2 />
                    </label>
                </div>
                <div
                    class="w-full flex flex-col" id={id}>
                    <div role="title"
                        class={classes}>
                        <div
                            class="w-[80px] flex-[0_0_auto] border-collapse border border-base-300 text-center"><span>{"必填"}</span></div>
                        <div
                            class="flex-1 border-collapse border border-base-300 text-center"><span>{"Key"}</span></div>
                        <div
                            class="flex-1 border-collapse border border-base-300 text-center"><span>{"Value"}</span></div>
                        <div
                            class="w-[140px] flex-[0_0_auto] border-collapse border border-base-300 text-center"><span>{"参数类型"}</span></div>
                        <div
                            class="flex-1 border-collapse border border-base-300 text-center"><span>{"默认值"}</span></div>
                        <div
                            class="w-[140px] flex-[0_0_auto] border-collapse border border-base-300 text-center"><span>{"验证方式"}</span></div>
                        <div
                            class="flex-1 border-collapse border border-base-300 text-center"><span>{"表达式"}</span></div>
                        <div
                            class="flex-1 border-collapse border border-base-300 text-center"><span>{"验证说明"}</span></div>
                        <div
                            class="flex-[2_1_0%] border-collapse border border-base-300 text-center"><span>{"描述"}</span></div>
                    </div>
                    {param_html}
                </div>
            </div>
        }
    }
}

fn generate_parameters(parameters: Ref<'_, Vec<CodeParameter>>) -> Html {
    let mut result: Vec<VNode> = Vec::new();
    for param in parameters.iter() {
        let node = html! {
            <div class="flex flex-row w-full params">
                <div
                    class="w-[80px] flex-[0_0_auto] border-collapse border border-base-300 flex justify-center content-center">
                        <input type="checkbox"
                            checked={param.required}
                            class="checkbox checkbox-xs	" />
                </div>
                <div
                    class="flex-1 border-collapse border border-base-300 text-left">
                        <input type="text" class="input input-xs input-bordered w-full " value={param.name.clone().unwrap_or("".to_owned())} />
                </div>
                <div
                    class="flex-1 border-collapse border border-base-300 text-left">
                        <input type="text" class="input input-xs input-bordered w-full " value={param.value.clone().unwrap_or("".to_owned())} />
                </div>
                <div
                    class="w-[140px] flex-[0_0_auto] border-collapse border border-base-300 text-left">
                    <select
                        class="select select-xs w-full">
                        // TODO: 文件类型 等等
                        <option
                            selected={param.data_type.is_none() || param.data_type.as_ref().unwrap() == "String"}>{"String"}</option>
                            <option selected={param.data_type.is_some() && param.data_type.as_ref().unwrap() == "Integer"}>{"Integer"}</option>
                    </select>
                </div>
                <div
                    class="flex-1 border-collapse border border-base-300 text-left">
                        <input type="text" class="input input-xs input-bordered w-full " value={param.default_value.clone().unwrap_or("".to_owned())} />
                </div>
                <div
                    class="w-[140px] flex-[0_0_auto] border-collapse border border-base-300 text-left">
                    <select
                        class="select select-xs w-full">
                            // TODO: pass
                            <option selected={param.validate_type.is_none() || param.validate_type.as_ref().unwrap() == "不验证"}>{"不验证"}</option>
                            // expression
                            <option selected={param.validate_type.is_some() && param.validate_type.as_ref().unwrap() == "表达式验证"}>{"表达式验证"}</option>
                            // pattern
                            <option selected={param.validate_type.is_some() && param.validate_type.as_ref().unwrap() == "正则验证"}>{"正则验证"}</option>
                    </select>
                </div>
                <div
                    class="flex-1 border-collapse border border-base-300 text-left">
                    <input type="text" class="input input-xs input-bordered w-full " value={param.expression.clone().unwrap_or("".to_owned())} />
                </div>
                <div
                    class="flex-1 border-collapse border border-base-300 text-left">
                        <input type="text" class="input input-xs input-bordered w-full " value={param.error.clone().unwrap_or("".to_owned())} />
                </div>
                <div
                    class="flex-[2_1_0%] border-collapse border border-base-300 text-left">
                        <input type="text" class="input input-xs input-bordered w-full " value={param.description.clone().unwrap_or("".to_owned())} />
                </div>
            </div>
        };
        result.push(node);
    }
    result.into_iter().collect::<Html>()
}

fn get_params_index() -> Result<i32, JsValue> {
    if let Ok(Some(selection)) = window().get_selection() {
        if let Ok(range) = selection.get_range_at(0) {
            let node = range.start_container().unwrap();
            if let Some(div) = get_params_node(Some(node))? {
                let parent = div.parent_node().unwrap();
                let child_nodes = parent.child_nodes();
                for i in 0..child_nodes.length() {
                    if div.is_same_node(Some(&child_nodes.get(i).unwrap())) {
                        return Ok(i as i32 - 1);
                    }
                }
            }
        }
    }
    Ok(-1)
}

fn get_params_node(node: Option<Node>) -> Result<Option<HtmlElement>, JsValue> {
    if node.is_none() {
        return Ok(None);
    }
    let node = node.unwrap();
    if node.node_type() != Node::ELEMENT_NODE {
        return get_params_node(node.parent_node());
    }
    let node = node.dyn_into::<HtmlElement>()?;
    if !node.class_list().contains("params") {
        return get_params_node(node.parent_node());
    }
    Ok(Some(node))
}

fn get_dom_id(is_new: bool) -> &'static str {
    if is_new {
        "api_parameters_new"
    } else {
        "api_parameters"
    }
}

fn save_parameters(params: &mut Vec<CodeParameter>, is_new: bool) {
    let id = get_dom_id(is_new);
    let nodes = document().get_element_by_id(id).unwrap();
    let children = nodes.child_nodes();
    for i in 0..params.len() {
        let param = params.get_mut(i).unwrap();
        if let Some(child) = children.get(i as u32 + 1) {
            let children = child.child_nodes();
            // 必填
            param.required = get_checkbox_value(children.get(0)).unwrap_or(false);
            // Key
            param.name = get_input_value(children.get(1));
            // Value
            param.value = get_input_value(children.get(2));
            // 参数类型
            param.data_type = get_select_value(children.get(3));
            // 默认值
            param.default_value = get_input_value(children.get(4));
            // 验证方式
            param.validate_type = get_select_value(children.get(5));
            // 表达式
            param.expression = get_input_value(children.get(6));
            // 验证说明
            param.error = get_input_value(children.get(7));
            // 描述
            param.description = get_input_value(children.get(8));
        }
    }
}

fn get_input_value(node: Option<Node>) -> Option<String> {
    match node {
        Some(node) => match node.child_nodes().get(0) {
            Some(node) => {
                let input = node.dyn_into::<HtmlInputElement>().unwrap();
                Some(input.value())
            }
            None => None,
        },
        None => None,
    }
}

fn get_checkbox_value(node: Option<Node>) -> Option<bool> {
    match node {
        Some(node) => match node.child_nodes().get(0) {
            Some(node) => {
                let input = node.dyn_into::<HtmlInputElement>().unwrap();
                Some(input.checked())
            }
            None => None,
        },
        None => None,
    }
}

fn get_select_value(node: Option<Node>) -> Option<String> {
    match node {
        Some(node) => match node.child_nodes().get(0) {
            Some(node) => {
                let select = node.dyn_into::<HtmlSelectElement>().unwrap();
                Some(select.value())
            }
            None => None,
        },
        None => None,
    }
}
