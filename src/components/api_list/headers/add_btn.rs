use gloo::console::log;
use yew::prelude::*;

use crate::{components::tooltip::Tooltip, image::AddSvg};

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
                <div class="modal-box">
                    <h3 class="font-bold text-2xl text-center">{"新增接口"}</h3>
                    <div class="flex flex-col space-y-4 mt-4 justify-center	content-center	">
                        <div class="flex w-full">
                            <label class="label w-16" for="t1">
                                <span class="label-text">{"名称"}</span>
                            </label>
                            <input id={"t1"} type="text" placeholder={""}
                                class="input input-bordered w-full" />
                        </div>

                        <div class="flex w-full ">
                            <label class="label w-16" for="t2">
                                <span class="label-text">{"路径"}</span>
                            </label>
                            <input id={"t2"} placeholder={""}
                                class="input input-bordered w-full " />
                        </div>
                        <div class="flex w-full ">
                            <label class="label w-16" for="t3">
                                <span class="label-text">{"接口分组"}</span>
                            </label>
                            <input id={"t3"} placeholder={""}
                                class="input input-bordered w-full " />
                        </div>
                        <div class="flex w-full ">
                            <label class="label w-16" for="t4">
                                <span class="label-text">{"描述"}</span>
                            </label>
                            <input id={"t4"} placeholder={""}
                                class="input input-bordered w-full " />
                        </div>

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
