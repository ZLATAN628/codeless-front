use yew::prelude::*;

use crate::{
    components::{title::main_title::BtnProps, tooltip::Tooltip},
    image::SaveSvg,
};

#[function_component(SaveBtn)]
pub fn play_btn(props: &BtnProps) -> Html {
    let callback = Callback::from(move |_e: MouseEvent| {});
    html! {
        <Tooltip tip="保存">
            <btn onclick={callback} class={props.btn_classes}>
                <SaveSvg />
            </btn>
        </Tooltip>
    }
}
