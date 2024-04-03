use monaco::api::TextModel;
use yew::prelude::*;

use crate::{
    components::response::{
        api_response_header::ApiResponseHeader, response_editor::ResponseEditor,
    },
    context::code_context::use_codes,
};

#[derive(Properties, PartialEq)]
pub struct ApiResponseProps {
    pub width: f64,
}

#[function_component(ApiResponse)]
pub fn api_response(props: &ApiResponseProps) -> Html {
    let style = format!("width: {}%;", &props.width);
    let code_context = use_codes();
    let code = code_context.current_code();

    let model = use_state_eq(|| TextModel::create("", Some("json"), None).unwrap());

    model.set_value(code.response_body.as_ref().unwrap_or(&"".to_string()));

    html! {
        <div
            class="flex flex-col justify-start h-full border-collapse border border-neutral" style={style}>
            <div role="tablist"
                class="tabs tabs-md tabs-bordered text-sm bg-base-100  grid-rows-[48px_1fr] h-full w-full">
                <input type="radio" name="my_tabs_5"
                    role="tab"
                    class="tab" aria-label="响应体"
                    checked={true} />
                <ResponseEditor text_model={(*model).clone()} />

                <input type="radio" name="my_tabs_5" role="tab"
                    class="tab" aria-label="响应头" />
                <ApiResponseHeader />
            </div>
        </div>
    }
}
