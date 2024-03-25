use components::editor::keymap;
use yew::prelude::*;

use crate::{
    components::{main_body::MainBody, theme::Theme, title::main_title::MainTitle},
    context::code_context::CodesProvider,
};
mod backend;
mod common;
mod components;
mod context;
mod image;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <CodesProvider>
            <Theme>
                <MainTitle />
                <MainBody />
            </Theme>
        </CodesProvider>
    }
}

pub fn main() {
    init();
    yew::Renderer::<App>::new().render();
}

fn init() {
    keymap::init();
}
