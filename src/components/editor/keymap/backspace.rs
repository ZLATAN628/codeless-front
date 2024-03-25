use gloo::utils::window;

use super::{delete_line_and_reset_range, get_parent_div_by_range, KeyMapHandler};

#[derive(Default)]
pub struct BackSpaceHandler();

impl KeyMapHandler for BackSpaceHandler {
    fn on_key_down(&self, _event: &yew::prelude::KeyboardEvent) -> bool {
        if let Some(selection) = window().get_selection().unwrap() {
            let range = selection.get_range_at(0).unwrap();
            let cur_div = get_parent_div_by_range(&range);
            // 如果只有一行 就不删除
            if let Some(text_content) = cur_div.text_content() {
                if text_content.len() == 0 {
                    delete_line_and_reset_range(&cur_div, &range);
                    selection.remove_all_ranges().unwrap();
                    selection.add_range(&range).unwrap();
                    return true;
                }
            }
        }
        false
    }
}
