use yew::prelude::*;

use crate::{
    components::{title::main_title::BtnProps, tooltip::Tooltip},
    context::code_context::{use_codes, CodesStateMsg},
    image::PlaySvg,
};

#[function_component(PlayBtn)]
pub fn play_btn(props: &BtnProps) -> Html {
    let code_context = use_codes();
    let callback = Callback::from(move |_e: MouseEvent| {
        let dispatcher = code_context.dispatcher();
        code_context.dispatch(CodesStateMsg::TestCode(dispatcher));
    });
    html! {
        <Tooltip tip="运行">
            <btn onclick={callback} class={props.btn_classes}>
                <PlaySvg />
            </btn>
        </Tooltip>
    }
}
