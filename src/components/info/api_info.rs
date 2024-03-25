use yew::{prelude::*, virtual_dom::VNode};

use crate::context::code_context::{use_codes, CodeParameter};

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
    // let parameters = code.parameters.clone();
    // let parameters: UseStateHandle<Vec<CodeParameter>> = use_state(move || parameters);
    html! {
        <div
            class="flex flex-col border border-neutral bg-base-200 text-sm" style={style}>
            <div
                class="flex flex-row justify-between w-full border border-neutral  border-solid h-[55px] items-center">
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
                <div class="flex">
                    <div class="flex-col w-[40px]">
                        <div>
                           {"➕"}
                        </div>
                        <div>
                            {"➖"}
                        </div>
                    </div>
                    <div
                        class="w-[calc(100%-40px)] flex flex-col">
                        <div role="title"
                            class="flex flex-row w-full">
                            <div
                                class="w-[80px] flex-[0_0_auto] border-collapse border border-base-300 text-center"><span>{"必填"}</span></div>
                            <div
                                class="flex-1 border-collapse border border-base-300 text-center"><span>{"Key"}</span></div>
                            <div
                                class="flex-1 border-collapse border border-base-300 text-center"><span>{"Value"}</span></div>
                            <div
                                class="w-[140px] flex-[0_0_auto] border-collapse border border-base-300 text-center"><span>{"参数类型"}</span></div>
                            <div
                                class="flex-1 border-collapse border border-base-300 text-center"><span>{"默认值"}</span></div>
                            <div
                                class="w-[140px] flex-[0_0_auto] border-collapse border border-base-300 text-center"><span>{"验证方式"}</span></div>
                            <div
                                class="flex-1 border-collapse border border-base-300 text-center"><span>{"表达式或正则表达式"}</span></div>
                            <div
                                class="flex-1 border-collapse border border-base-300 text-center"><span>{"验证说明"}</span></div>
                            <div
                                class="flex-[2_1_0%] border-collapse border border-base-300 text-center"><span>{"描述"}</span></div>
                        </div>
                        {
                            {
                                // TODO: 待完善
                                generate_parameters(&code.parameters)
                            }
                        }
                    </div>
                </div>
            </div>
            <input type="radio" name="my_tabs_3" role="tab"
                class="tab" aria-label="响应模板" />
            <div role="tabpanel"
                class="tab-content bg-base-100 border-base-300 p-6 h-full ">{"Tab content 3"}</div>
            </div>
        </div>
    }
}

fn generate_parameters(parameters: &Vec<CodeParameter>) -> Html {
    let mut result: Vec<VNode> = Vec::new();
    for param in parameters.iter() {
        let node = html! {
            <div class="flex flex-row w-full">
                <div
                    class="w-[80px] flex-[0_0_auto] border-collapse border border-base-300 text-center">
                    <input type="checkbox"
                        checked={param.required}
                        class="checkbox checkbox-xs	" />
                </div>
                <div
                    class="flex-1 border-collapse border border-base-300 text-left"
                    contenteditable="true"><span>{param.name.as_ref().unwrap_or(&"".to_owned())}</span>
                </div>
                <div
                    class="flex-1 border-collapse border border-base-300 text-left"
                    contenteditable="true"><span>{param.value.as_ref().unwrap_or(&"".to_owned())}</span>
                </div>
                <div
                    class="w-[140px] flex-[0_0_auto] border-collapse border border-base-300 text-left">
                    <select
                        class="select select-xs w-full">
                        // TODO: 文件类型 等等
                        <option
                            selected={true}>{"String"}</option>
                            <option>{"Integer"}</option>
                    </select>
                </div>
                <div
                    class="flex-1 border-collapse border border-base-300 text-left"
                    contenteditable="true">
                    {param.default_value.as_ref().unwrap_or(&"".to_owned())}
                </div>
                <div
                    class="w-[140px] flex-[0_0_auto] border-collapse border border-base-300 text-left">
                    <select
                        class="select select-xs w-full">
                            <option selected={param.validate_type.is_none() || param.validate_type.as_ref().unwrap() == "pass"}>{"不验证"}</option>
                            <option selected={param.validate_type.is_some() && param.validate_type.as_ref().unwrap() == "expression"}>{"表达式验证"}</option>
                            <option selected={param.validate_type.is_some() && param.validate_type.as_ref().unwrap() == "pattern"}>{"正则验证"}</option>
                    </select>
                </div>
                <div
                    class="flex-1 border-collapse border border-base-300 text-left"
                    contenteditable="true"><span>
                        {param.expression.as_ref().unwrap_or(&"".to_owned())}
                    </span>
                </div>
                <div
                    class="flex-1 border-collapse border border-base-300 text-left"
                    contenteditable="true"><span>{param.error.as_ref().unwrap_or(&"".to_owned())}</span>
                </div>
                <div
                    class="flex-[2_1_0%] border-collapse border border-base-300 text-left"
                    contenteditable="true"><span>{param.description.as_ref().unwrap_or(&"".to_owned())}</span>
                </div>
            </div>
        };

        result.push(node);
    }
    result.into_iter().collect::<Html>()
}
