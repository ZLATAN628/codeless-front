use gloo::console::log;
use yew::{prelude::*, virtual_dom::VNode};

use crate::{
    backend::{self, NodeInfo},
    components::api_list::headers::add_btn::{AddApiBtn, AddBtn},
    context::code_context::{Code, CodesContext, CodesStateMsg},
    image::{AddSvg, DirSvg, FileSvg, SubSvg},
};

pub const DESKTOP_HEADER_BTN_CLASSES: &'static str = "btn btn-ghost btn-xs 2xl:btn-sm";

pub enum FileSelectMsg {
    FetchBegin,
    FetchEnd(anyhow::Result<NodeInfo>),
    FetchCode(anyhow::Result<Code>, bool),
    CodesContextMsg(CodesContext),
}

pub struct ApiSelect {
    code_list: Option<NodeInfo>,
    is_fetching: bool,
    code_context: CodesContext,
    _code_context_listener: ContextHandle<CodesContext>,
}

impl Component for ApiSelect {
    type Message = FileSelectMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(FileSelectMsg::FetchBegin);
        let (code_context, listener) = ctx
            .link()
            .context::<CodesContext>(ctx.link().callback(FileSelectMsg::CodesContextMsg))
            .unwrap();
        ApiSelect {
            code_list: None,
            is_fetching: false,
            code_context,
            _code_context_listener: listener,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col w-[320px] overflow-hidden bg-base-300">
                // modal
                <AddApiBtn />
                if self.is_fetching {
                    // Loading...
                } else {
                    <ul
                        class="flex flex-row-reverse menu menu-horizontal bg-base-300">
                        <li>
                            <AddBtn btn_classes={DESKTOP_HEADER_BTN_CLASSES}/>
                        </li>
                        <li>
                            <a>
                                <SubSvg />
                            </a>
                        </li>
                        <li>
                            <a>
                                <AddSvg />
                            </a>
                        </li>
                    </ul>
                    // 文件列表
                    <ul
                        class="menu menu-md bg-base-300  max-w-xs w-full text-sm h-screen">
                        {
                            if let Some(node_info) = &self.code_list {
                                let mut result: Vec<VNode> = Vec::new();
                                for i in 0..node_info.children.len() {
                                    result.append(&mut generate_node_info(&node_info.children[i], ctx));
                                }
                                result.into_iter().collect::<Html>()
                            } else {
                                html! {<></>}
                            }
                        }
                    </ul>
                }
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FileSelectMsg::FetchBegin => {
                self.is_fetching = true;
                ctx.link().send_future(async {
                    let result = backend::get_api_info().await;
                    FileSelectMsg::FetchEnd(result)
                });
                true
            }
            FileSelectMsg::FetchEnd(result) => {
                match result {
                    Ok(data) => {
                        self.is_fetching = false;
                        // self.code_context.save_group_info(&data);
                        self.code_context
                            .dispatch(CodesStateMsg::SaveGroup(data.clone()));
                        self.code_context.clear_storage();
                        self.code_list = Some(data);
                    }
                    Err(e) => {
                        self.is_fetching = false;
                        // TODO: 错误提示
                        log!("get api info error", format!("{}", e));
                    }
                }
                true
            }
            FileSelectMsg::FetchCode(code, save) => {
                match code {
                    Ok(data) => {
                        // self.code_context.put_code(data, save);
                        self.code_context
                            .dispatch(CodesStateMsg::UpdateCode(data, save))
                    }
                    Err(e) => {
                        log!("get code info error", format!("{}", e));
                    }
                }
                true
            }
            FileSelectMsg::CodesContextMsg(code_context) => {
                self.code_context = code_context;
                true
            }
        }
    }
}

fn generate_node_info(node_info: &NodeInfo, ctx: &Context<ApiSelect>) -> Vec<VNode> {
    let node = &node_info.node;
    let mut result: Vec<VNode> = vec![];
    if node.group_id.is_none() {
        let h = html! {
            <li>
                <details open={false}>
                    <summary>
                        <DirSvg />
                        {node.name.as_ref().unwrap()}
                    </summary>
                    <ul>
                        if !node_info.children.is_empty() {
                            {
                                {
                                    let mut result: Vec<VNode> = Vec::new();
                                    for i in 0..node_info.children.len() {
                                        result.append(&mut generate_node_info(&node_info.children[i], ctx));
                                    }
                                    result.into_iter().collect::<Html>()
                                }
                            }
                        }
                    </ul>
                </details>
            </li>
        };
        result.push(h);
    } else {
        let id = node.id.as_ref().unwrap().to_string();
        let (code_context, _) = ctx
            .link()
            .context::<CodesContext>(Callback::noop())
            .unwrap();
        let onclick = ctx.link().callback_future(move |e: MouseEvent| {
            e.prevent_default();
            let id = id.clone();
            let code_context = code_context.clone();
            async move {
                if let Some(code) = code_context.load_code(&id) {
                    FileSelectMsg::FetchCode(Ok(code), false)
                } else {
                    let code = backend::get_code_by_id(&id).await;
                    FileSelectMsg::FetchCode(code, true)
                }
            }
        });

        let h = html! {
            <li onclick={onclick}>
                <a>
                    <FileSvg />
                    {node.name.as_ref().unwrap()}
                </a>
            </li>
        };
        result.push(h);
    }
    result
}
