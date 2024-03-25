use gloo::utils::window;

use super::{calculate_node_offset, calculate_offset, get_parent_div_by_range, KeyMapHandler};

#[derive(Default)]
pub struct UpArrowHandler();
#[derive(Default)]
pub struct DownArrowHandler();

impl KeyMapHandler for UpArrowHandler {
    fn on_key_down(&self, _event: &yew::prelude::KeyboardEvent) -> bool {
        if let Some(selection) = window().get_selection().unwrap() {
            let range = selection.get_range_at(0).unwrap();
            let cur_div = get_parent_div_by_range(&range);
            if let Some(pre_div) = cur_div.previous_sibling() {
                let mut start_offset = range.start_offset().unwrap();
                if let Ok(start_node) = range.start_container() {
                    // 获取当前节点的偏移量 再加上 相对于该 节点的 偏移量
                    start_offset = calculate_node_offset(&start_node, &cur_div) + start_offset;
                }
                let child_list = pre_div.child_nodes();
                // 因为 div 中有可能包含 span 标签，需要遍历所有子节点 计算获得对应节点与偏移量
                let (node, offset) = calculate_offset(&child_list, start_offset);
                if let Some(node) = node {
                    range.set_start(&node, offset).unwrap();
                    range.set_end(&node, offset).unwrap();
                } else {
                    range.set_start(&pre_div, 0).unwrap();
                    range.set_end(&pre_div, 0).unwrap();
                }
                selection.remove_all_ranges().unwrap();
                selection.add_range(&range).unwrap();
            }
        }
        true
    }
}

impl KeyMapHandler for DownArrowHandler {
    fn on_key_down(&self, _event: &yew::prelude::KeyboardEvent) -> bool {
        if let Some(selection) = window().get_selection().unwrap() {
            let range = selection.get_range_at(0).unwrap();
            let cur_div = get_parent_div_by_range(&range);
            if let Some(next_div) = cur_div.next_sibling() {
                let mut start_offset = range.start_offset().unwrap();
                let child_list = next_div.child_nodes();
                if let Ok(start_node) = range.start_container() {
                    // 获取当前节点的偏移量 再加上 相对于该 节点的 偏移量
                    start_offset = calculate_node_offset(&start_node, &cur_div) + start_offset;
                }
                // 因为 div 中有可能包含 span 标签，需要遍历所有子节点 计算获得对应节点与偏移量
                let (node, offset) = calculate_offset(&child_list, start_offset);
                if let Some(node) = node {
                    range.set_start(&node, offset).unwrap();
                    range.set_end(&node, offset).unwrap();
                } else {
                    range.set_start(&next_div, 0).unwrap();
                    range.set_end(&next_div, 0).unwrap();
                }
                selection.remove_all_ranges().unwrap();
                selection.add_range(&range).unwrap();
            }
        }
        true
    }
}
