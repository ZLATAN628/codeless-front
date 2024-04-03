use components::editor::keymap;
use yew::prelude::*;

use crate::{
    components::{main_body::MainBody, title::main_title::MainTitle},
    context::{code_context::CodesProvider, theme_context::ThemeProvider},
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
            <ThemeProvider>
                <MainTitle />
                <MainBody />
            </ThemeProvider>
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
