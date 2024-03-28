use crate::components::info::parameters::ApiParameters;
use yew::{prelude::*, virtual_dom::VNode};

use crate::context::code_context::use_codes;

#[derive(PartialEq, Properties)]
pub struct ApiInfoProps {
    pub height: f64,
}

const METHODS: &[&str] = &["GET", "POST"];

#[function_component(ApiInfo)]
pub fn api_info(props: &ApiInfoProps) -> Html {
    let style = format!("height: {}%", &props.height);
    let code_context = use_codes();
    let code = code_context.current_code();
    let params = code.parameters.clone();
    // let parameters = code.parameters.clone();
    // let parameters: UseStateHandle<Vec<CodeParameter>> = use_state(move || parameters);
    html! {
        <div
            class="flex flex-col border border-neutral bg-base-200 text-sm border-collapse" style={style}>
            <div
                class="flex flex-row justify-between w-full border-b border-neutral  border-solid h-[55px] items-center border-collapse">
                <div class="flex w-[10%] justify-around">
                    <div class="w-[65px]">{"请求方法\u{00a0}"}</div>
                    <select
                        class="select select-bordered select-xs w-[calc(100%-80px)]">
                        {
                            {
                                let mut result: Vec<VNode> = Vec::new();
                                for method in METHODS.iter() {
                                    if let Some(m) = code.method.as_ref() {
                                        if method == m {
                                            result.push(html! {
                                                <option selected={true}>{method}</option>
                                            });
                                            continue;
                                        }
                                    }
                                    result.push(html! {
                                        <option>{method}</option>
                                    });
                                }
                                result.into_iter().collect::<Html>()
                            }
                        }
                    </select>
                </div>
                <div class="flex w-[19%]">
                    <div class="w-[80px]">{"接口名称"}</div>
                    <input type="text"
                        class="input input-bordered input-xs w-[calc(100%-80px)]" value={code.name.as_ref().unwrap_or(&"".to_string()).to_string()} />
                </div>
                <div class="flex w-[70%]">
                    <div class="w-[70px]">{"接口路径"}</div>
                    <input type="text"
                        class="input input-bordered input-xs w-[calc(100%-70px)]" value={code.path.as_ref().unwrap_or(&"".to_string()).to_string()}/>
                </div>
            </div>
            <div role="tablist"
                class="tabs tabs-md tabs-bordered h-full grid-rows-[46px_1fr] w-full overflow-hidden">
                <input type="radio" name="my_tabs_3" role="tab"
                    class="tab" aria-label="请求头" />
            <div role="tabpanel"
                class="tab-content bg-base-100 border-base-300 p-6 h-full w-full">{"Tab content 1"}</div>
            <input type="radio" name="my_tabs_3" role="tab"
                class="tab" aria-label="请求参数"
                checked={true} />
            <div
                class="tab-content bg-base-100 border-base-300 h-full w-full">
                <ApiParameters is_new={false} params={Some(params)}/>
            </div>
            <input type="radio" name="my_tabs_3" role="tab"
                class="tab" aria-label="响应模板" />
            <div role="tabpanel"
                class="tab-content bg-base-100 border-base-300 p-6 h-full ">{"Tab content 3"}</div>
            </div>
        </div>
    }
}
