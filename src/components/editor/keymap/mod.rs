use std::sync::{Arc, RwLock};

use gloo::utils::document;
use lazy_static::lazy_static;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, KeyboardEvent, Node, NodeList, Range};

use self::{
    arrow::{DownArrowHandler, UpArrowHandler},
    backspace::BackSpaceHandler,
    enter::EnterHandler,
    tab::TabHandler,
};

mod arrow;
mod backspace;
mod enter;
mod tab;

pub trait KeyMapHandler {
    /**
     * return true => 阻止默认事件
     */
    fn on_key_down(&self, event: &KeyboardEvent) -> bool;
}

lazy_static! {
    pub static ref KEY_MAP: RwLock<Vec<Option<Arc<dyn KeyMapHandler + Sync + Send>>>> =
        RwLock::new(vec![None; 256]);
}

#[allow(dead_code)]
pub fn init() {
    let mut key_map = KEY_MAP.write().unwrap();
    key_map[8] = Some(Arc::new(BackSpaceHandler::default()));
    key_map[9] = Some(Arc::new(TabHandler::default()));
    key_map[13] = Some(Arc::new(EnterHandler::default()));
    key_map[38] = Some(Arc::new(UpArrowHandler::default()));
    key_map[40] = Some(Arc::new(DownArrowHandler::default()));
}

pub fn get_parent_div_by_range(range: &Range) -> Node {
    let cur = range.start_container().unwrap();
    if cur.node_name() != "DIV" {
        return get_parent_div_by_node(&cur);
    }
    return cur;
}

fn get_parent_div_by_node(node: &Node) -> Node {
    let parent = node.parent_node().unwrap();
    if parent.node_name() != "DIV" {
        return get_parent_div_by_node(&parent);
    }
    return parent;
}

// 计算当前子节点 在 父节点中的偏移量
pub fn calculate_node_offset(sub_node: &Node, parent_node: &Node) -> u32 {
    let mut offset = 0;
    for i in 0..parent_node.child_nodes().length() {
        let child = parent_node.child_nodes().item(i).unwrap();
        if child.eq(sub_node) || child.contains(Some(sub_node)) || sub_node.contains(Some(&child)) {
            return offset;
        }
        if child.node_type() == Node::TEXT_NODE {
            let node_length = child.node_value().unwrap().chars().count() as u32;
            offset += node_length;
        } else if child.node_type() == Node::ELEMENT_NODE {
            let node_length = child
                .text_content()
                .unwrap_or("".to_string())
                .chars()
                .count() as u32;
            offset += node_length;
        }
    }
    offset
}

// 计算在所有子节点中 的 偏移量
pub fn calculate_offset<'a>(child_list: &'a NodeList, expect_index: u32) -> (Option<Node>, u32) {
    if expect_index == 0 {
        return (None, 0);
    }
    let mut offset = 0;
    let mut last_node = None;
    let mut last_node_length = 0;
    for i in 0..child_list.length() {
        let mut child = child_list.item(i).unwrap();
        if child.node_type() == Node::TEXT_NODE {
            let node_length = child.node_value().unwrap().chars().count() as u32;
            if expect_index <= (offset + node_length) {
                return (Some(child), expect_index - offset);
            }
            offset += node_length;
            last_node_length = node_length;
        } else if child.node_type() == Node::ELEMENT_NODE {
            let node_length = child
                .text_content()
                .unwrap_or("".to_string())
                .chars()
                .count() as u32;
            // 暂时认为不存在 span 嵌套情况，直接获取内部的 textNode
            child = child.child_nodes().item(0).unwrap();
            if expect_index <= (offset + node_length) {
                return (Some(child), expect_index - offset);
            }
            offset += node_length;
            last_node_length = node_length;
        }

        if i == child_list.length() - 1 {
            last_node = Some(child);
        }
    }

    if expect_index <= offset {
        (last_node, expect_index - (offset - last_node_length))
    } else {
        (last_node, last_node_length)
    }
}

pub fn calculate_total_size(node: &Node) -> u32 {
    let mut offset = 0;
    let child_list = node.child_nodes();
    for i in 0..child_list.length() {
        let child = child_list.item(i).unwrap();
        if child.node_type() == Node::TEXT_NODE {
            let node_length = child.node_value().unwrap().chars().count() as u32;
            offset += node_length;
        } else if child.node_type() == Node::ELEMENT_NODE {
            let node_length = child
                .text_content()
                .unwrap_or("".to_string())
                .chars()
                .count() as u32;
            offset += node_length;
        }
    }
    offset
}

// delete current div && move cursor range to previous div
pub fn delete_line_and_reset_range(cur_div: &Node, range: &Range) {
    if let Some(pre_div) = cur_div.previous_sibling() {
        if let Some(parent) = cur_div.parent_node() {
            parent.remove_child(&cur_div).unwrap();
            // update line number
            sub_line_number();
        }
        let offset = calculate_total_size(&pre_div);
        let child_nodes = pre_div.child_nodes();
        let (node, offset) = calculate_offset(&child_nodes, offset);
        if let Some(node) = node {
            range.set_start(&node, offset).unwrap();
            range.set_end(&node, offset).unwrap();
        } else {
            range.set_start(&pre_div, 0).unwrap();
            range.set_end(&pre_div, 0).unwrap();
        }
    }
}

pub fn add_line_number() {
    let lines = document().get_element_by_id("line-numbers").unwrap();
    let lines = lines.dyn_into::<HtmlDivElement>().unwrap();
    let index = lines.child_element_count() + 1;
    let new_div = document().create_element("div").unwrap();
    new_div.set_class_name("w-full min-h-5");
    new_div.set_text_content(Some(&format!("{}", index)));
    lines.append_child(&new_div).unwrap();
}

pub fn sub_line_number() {
    let lines = document().get_element_by_id("line-numbers").unwrap();
    let last_child = lines.last_child();
    if let Some(last_child) = last_child {
        lines.remove_child(&last_child).unwrap();
    }
}
