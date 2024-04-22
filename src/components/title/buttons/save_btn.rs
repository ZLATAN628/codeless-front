use yew::prelude::*;

use crate::{
    components::{title::main_title::BtnProps, tooltip::Tooltip},
    context::code_context::{use_codes, CodesStateMsg},
    image::SaveSvg,
};

#[function_component(SaveBtn)]
pub fn save_btn(props: &BtnProps) -> Html {
    let code_context = use_codes();
    let callback = Callback::from(move |_e: MouseEvent| {
        code_context.dispatch(CodesStateMsg::SaveCode(code_context.dispatcher()));
    });
    html! {
        <Tooltip tip="保存">
            <btn onclick={callback} class={props.btn_classes}>
                <SaveSvg />
            </btn>
        </Tooltip>
    }
}
