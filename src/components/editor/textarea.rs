use gloo::{console::log, utils::window};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{
    common::highlight::highlight_code,
    components::editor::keymap::{
        calculate_node_offset, calculate_offset, get_parent_div_by_range, KEY_MAP,
    },
};

#[derive(PartialEq, Properties)]
pub struct ApiTextAreaProps {
    // 代码
    pub text: String,
}

#[function_component(ApiTextArea)]
pub fn api_textarea(props: &ApiTextAreaProps) -> Html {
    let lines = props
        .text
        .split("\n")
        .map(|e| e.to_string())
        .collect::<Vec<_>>();
    let line_numbers = (1..=lines.len()).collect::<Vec<_>>();

    // 事件监听
    let key_down = Callback::from(|e: KeyboardEvent| {
        let key_code = e.key_code();
        let key_map = KEY_MAP.read().unwrap();
        log!(key_code);
        // 处理快捷键
        if let Some(handler) = key_map[key_code as usize].clone() {
            if handler.on_key_down(&e) {
                // 返回 true 阻止默认处理事件
                e.prevent_default();
                return;
            }
        }
    });

    let input = Callback::from(|e: InputEvent| {
        if e.is_composing() {
            // 输入拼音的时候 不处理
            return;
        }
        // 处理文本输入
        if let Some(selection) = window().get_selection().unwrap() {
            let range = selection.get_range_at(0).unwrap();

            let cur_div = get_parent_div_by_range(&range);
            let mut start_offset = range.start_offset().unwrap();
            if let Ok(start_node) = range.start_container() {
                // 获取当前节点的偏移量 再加上 相对于该 节点的 偏移量
                start_offset = calculate_node_offset(&start_node, &cur_div) + start_offset;
            }

            if let Some(text) = cur_div.text_content() {
                let code = highlight_code(&text);
                let cur_div = cur_div.dyn_into::<HtmlElement>().unwrap();
                log!(&code);
                cur_div.set_inner_html(&code);
                let child_list = cur_div.child_nodes();
                let (node, offset) = calculate_offset(&child_list, start_offset);
                if let Some(node) = node {
                    range.set_start(&node, offset).unwrap();
                    range.set_end(&node, offset).unwrap();
                } else {
                    range.set_start(&cur_div, 0).unwrap();
                    range.set_end(&cur_div, 0).unwrap();
                }
                selection.remove_all_ranges().unwrap();
                selection.add_range(&range).unwrap();
            } else {
                log!("empty")
            }
        }
    });

    html! {
        <div onkeydown={key_down}
            class="h-full text-sm bg-base-100 overflow-auto scroll-smooth">
            <div class="flex">
                <div id="line-numbers"
                    class="flex-col w-[50px] h-screen bg-base-200 text-center">
                    {
                        line_numbers.into_iter().map(|number| {
                            html! {
                                <div class="w-full min-h-5">
                                    {number}
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="flex-col w-full h-full" oninput={input.clone()} >
                    {
                        lines.into_iter().map(|line| {
                            let inner = Html::from_html_unchecked(AttrValue::from(line.clone()));
                            html! {
                                <div class="w-full focus:bg-neutral outline-none min-h-5 whitespace-pre" contenteditable="true" >
                                    {inner}
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </div>
    }
}
