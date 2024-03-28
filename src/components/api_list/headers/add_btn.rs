use gloo::console::log;
use yew::prelude::*;

use crate::components::info::parameters::ApiParameters;
use crate::{components::tooltip::Tooltip, image::AddSvg};

macro_rules! form_label {
    ($name: expr, $id: expr) => {
        html! {
            <label class="form-control w-full">
                <div class="label">
                    <span class="label-text">{$name}</span>
                </div>
                <input type="text" placeholder="Type here" class="input input-bordered w-full" />
            </label>
        }
    };
}

const ADD_API_MODAL_ID: AttrValue = AttrValue::Static("add_api_modal");

#[derive(Properties, Clone, PartialEq)]
pub struct BtnProps {
    pub btn_classes: &'static str,
}

#[function_component(AddBtn)]
pub fn add_btn(props: &BtnProps) -> Html {
    html! {
        <Tooltip tip="新增接口">
            <label for={ADD_API_MODAL_ID} class={props.btn_classes}>
                <AddSvg />
            </label>
        </Tooltip>
    }
}

#[function_component(AddApiBtn)]
pub fn add_api_btn() -> Html {
    let confirm_click = Callback::from(|e: MouseEvent| {
        log!("confirm_click");
    });
    html! {
        <>
            <input type="checkbox" id={ADD_API_MODAL_ID} class="modal-toggle" />
            <div class="modal">
                <div class="max-w-5xl modal-box">
                    <h3 class="font-bold text-2xl text-center">{"新增接口"}</h3>
                    <div class="flex flex-col space-y-4 mt-4 justify-center	content-center">
                        {form_label!{"接口名称", 12}}
                        <label class="form-control w-full">
                            <div class="label">
                                <span class="label-text">{"接口路径"}</span>
                            </div>
                            <div class="flex w-full justify-between">
                                <input type="text" readonly={true} placeholder="http://127.0.0.1:9999" class="input input-bordered w-[30%]" />
                                <input type="text" placeholder="Type here" class="input input-bordered w-[69%]" />
                            </div>
                        </label>

                        <div class="flex justify-between">
                            <label class="form-control w-[15%]">
                                <div class="label">
                                    <span class="label-text">{"请求方法"}</span>
                                </div>
                                <select class="select select-bordered">
                                    <option selected={true}>{"GET"}</option>
                                    <option>{"POST"}</option>
                                </select>
                            </label>
                            <label class="form-control w-[84%]">
                                <div class="label">
                                    <span class="label-text">{"接口分组"}</span>
                                </div>
                                <select class="select select-bordered">
                                    <option selected={true}>{"test"}</option>
                                    <option>{"\u{00a0}\u{00a0}new-test"}</option>
                                </select>
                            </label>
                        </div>

                        <label class="form-control w-full">
                            <div class="label">
                                <span class="label-text">{"接口描述"}</span>
                            </div>
                            <textarea class="textarea textarea-bordered h-24" placeholder="描述下该接口的备注信息"></textarea>
                        </label>

                        <label class="form-control w-full">
                            <div class="label">
                                <span class="label-text">{"接口参数"}</span>
                            </div>
                            <ApiParameters is_new={true} params={None}/>
                        </label>
                    </div>

                    <div class="modal-action">
                        <label for={ADD_API_MODAL_ID} class="btn btn-ghost">{"取消"}</label>
                        <button onclick={confirm_click} class="btn">{"确认"}</button>
                    </div>
                </div>
            </div>
        </>
    }
}
