use yew::prelude::*;

use crate::{
    components::title::buttons::{play_btn::PlayBtn, save_btn::SaveBtn, theme_btn::ThemeBtn},
    context::code_context::use_codes,
};

#[derive(PartialEq, Properties)]
pub struct BtnProps {
    pub btn_classes: &'static str,
}

#[function_component(MainTitle)]
pub fn main_title() -> Html {
    let btn_classes = "btn btn-ghost btn-xs 2xl:btn-sm";
    let code_context = use_codes();
    let code = code_context.current_code();
    html! {
        <div class="border border-neutral flex justify-between h-8">
           <div>{"title bar"}</div>
           <div>{code.path.as_ref().unwrap_or(&"/".to_string())}</div>
           <div class="w-[120px] flex justify-between">
                <ThemeBtn />
                <PlayBtn {btn_classes} />
                <SaveBtn {btn_classes} />
           </div>
        </div>
    }
}
