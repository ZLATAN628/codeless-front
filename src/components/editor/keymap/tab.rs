use gloo::{console::log, utils::window};
use web_sys::Node;

use super::KeyMapHandler;

#[derive(Default)]
pub struct TabHandler();

impl KeyMapHandler for TabHandler {
    fn on_key_down(&self, _event: &yew::prelude::KeyboardEvent) -> bool {
        if let Some(selection) = window().get_selection().unwrap() {
            let range = selection.get_range_at(0).unwrap();
            let mut cur = range.start_container().unwrap();
            if let Some(text_content) = cur.text_content() {
                let mut start = range.start_offset().unwrap();
                log!(&cur);
                let mut chars = text_content.chars().collect::<Vec<char>>();
                for _ in 0..4 {
                    chars.insert(start as usize, ' ');
                }
                let text_content = chars
                    .into_iter()
                    .collect::<String>()
                    .replace(' ', "\u{00a0}");
                cur.set_text_content(Some(&text_content));
                while cur.node_type() != Node::TEXT_NODE {
                    if let Some(child) = cur.child_nodes().item(0) {
                        cur = child;
                    } else {
                        break;
                    }
                }
                start += 4;
                range.set_start(&cur, start).unwrap();
                range.set_end(&cur, start).unwrap();
                selection.remove_all_ranges().unwrap();
                selection.add_range(&range).unwrap();
            }
        }
        true
    }
}
