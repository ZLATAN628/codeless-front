use yew::prelude::*;

use crate::context::code_context::use_codes;

#[function_component(ApiResponseHeader)]
pub fn api_response_header() -> Html {
    let code_context = use_codes();
    let code = code_context.current_code();
    html! {
        <div class="w-full h-full tab-content">
            <table class="table table-md">
                <thead>
                    <tr class="text-center">
                        <th>{"名称"}</th>
                        <th>{"值"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        for code.resp_headers.iter().map(|(key, value)| {
                            html! {
                                <tr>
                                    <td>{key}</td>
                                    <td>{value}</td>
                                </tr>
                            }
                        })
                    }
                </tbody>
            </table>
        </div>
    }
}
