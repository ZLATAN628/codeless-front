use gloo::utils::{document, window};

use crate::components::editor::keymap::get_parent_div_by_range;

use super::{add_line_number, KeyMapHandler};

#[derive(Debug, Default)]
pub struct EnterHandler();

impl KeyMapHandler for EnterHandler {
    fn on_key_down(&self, _event: &yew::prelude::KeyboardEvent) -> bool {
        let new_div = document().create_element("div").unwrap();
        new_div.set_class_name("w-full focus:bg-neutral outline-none min-h-5");
        new_div.set_text_content(Some(""));
        new_div.set_attribute("contenteditable", "true").unwrap();

        if let Some(selection) = window().get_selection().unwrap() {
            // add edit line div
            let range = selection.get_range_at(0).unwrap();
            let cur_div = get_parent_div_by_range(&range);
            cur_div
                .parent_node()
                .unwrap()
                .insert_before(&new_div, cur_div.next_sibling().as_ref())
                .unwrap();

            range.set_start(&new_div, 0).unwrap();
            range.set_end(&new_div, 0).unwrap();
            selection.remove_all_ranges().unwrap();
            selection.add_range(&range).unwrap();
            // add line number div
            add_line_number();
        }
        true
    }
}
