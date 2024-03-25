use gloo::console::log;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    backend,
    components::{title::main_title::BtnProps, tooltip::Tooltip},
    context::code_context::{use_codes, CodesStateMsg},
    image::PlaySvg,
};

#[function_component(PlayBtn)]
pub fn play_btn(props: &BtnProps) -> Html {
    let code_context = use_codes();
    let callback = Callback::from(move |_e: MouseEvent| {
        let code = code_context.current_code();
        let final_url = code_context.get_final_path(
            code.path.as_ref().unwrap_or(&"".to_string()),
            code.group_id.as_ref().unwrap(),
        );
        log!("final_url: {}", &final_url);
        // TODO:
        // save file

        // call backend api
        let code = code.clone();
        let code_context = code_context.clone();
        spawn_local(async move {
            let result = backend::send_code(code, final_url).await.unwrap();
            // update response editor
            // code_context.update_response(result);
            code_context.dispatch(CodesStateMsg::UpdateResp(result));
        });
    });
    html! {
        <Tooltip tip="运行">
            <btn onclick={callback} class={props.btn_classes}>
                <PlaySvg />
            </btn>
        </Tooltip>
    }
}
