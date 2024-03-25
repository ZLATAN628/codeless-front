use crate::common::utils::open_modal;
use crate::components::modal::Modal;
use crate::context::code_context::CodesStateMsg;
use crate::{
    context::code_context::{Code, CodeTabs, CodesContext},
    image::CloseSvg,
};
use yew::html::Scope;
use yew::{prelude::*, virtual_dom::VNode};

pub struct CodeTabComponent {
    code_context: CodesContext,
    _context_handle: ContextHandle<CodesContext>,
}

pub enum CodeTabMsg {
    CodesContextMsg(CodesContext),
    CodeTabClose(String, bool),
}

impl Component for CodeTabComponent {
    type Message = CodeTabMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        let (code_context, context_handle) = ctx
            .link()
            .context::<CodesContext>(ctx.link().callback(CodeTabMsg::CodesContextMsg))
            .unwrap();

        Self {
            code_context,
            _context_handle: context_handle,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CodeTabMsg::CodesContextMsg(code_context) => {
                self.code_context = code_context;
                true
            }
            CodeTabMsg::CodeTabClose(id, is_save) => {
                if is_save {
                    // TODO: save code
                }
                // self.code_context.remove_code(&id);
                self.code_context
                    .dispatch(CodesStateMsg::RemoveCode(id.clone()));
                // self.code_context.set_next_code(&id);
                self.code_context
                    .dispatch(CodesStateMsg::SetNextCode(id.clone()));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let code = self.code_context.current_code();
        let tabs = self.code_context.current_code_tabs();
        // let tabs = tabs.borrow();
        html! {
            <div role="tablist" class="tabs tabs-sm tabs-bordered grid-cols-auto-fill-min  scrollbar-hide w-full">
                {
                    generate_code_tabs(&tabs, code, self.code_context.clone(), ctx.link().clone())
                }
            </div>
        }
    }
}

fn generate_code_tabs(
    tabs: &CodeTabs,
    code: &Code,
    code_context: CodesContext,
    link: Scope<CodeTabComponent>,
) -> Html {
    let mut result: Vec<VNode> = Vec::new();
    for id in tabs.order.iter() {
        let link = link.clone();
        let tab = tabs.data.get(id).unwrap();
        let mut class = classes!("tab", "justify-between", "row-auto", "flex-nowrap");
        if id == code.id() {
            class.push("tab-active");
        }
        let code_context_clone = code_context.clone();
        let code_id = id.clone();
        let tab_click = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // code_context_clone.set_current_code_id(code_id.clone());
            code_context_clone.dispatch(CodesStateMsg::ChangeCode(code_id.clone()))
        });

        let code_id = id.clone();
        let code_context_clone = code_context.clone();
        let link_clone = link.clone();
        let close_click = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            // TODO: complete code
            // 1.Determine whether the code has been updated, let user confirm
            // 2.delete local storage code info
            // 3.clear code_editors and code_tabs
            // 4.set new code
            if code_context_clone.is_update_code(&code_id) {
                open_modal(&format!("texteditor-{}", code_id));
            } else {
                link_clone.send_message(CodeTabMsg::CodeTabClose(code_id.clone(), false));
            }
        });

        let code_id = id.clone();
        let link_clone = link.clone();
        let save_click = Callback::from(move |_e: MouseEvent| {
            link_clone.send_message(CodeTabMsg::CodeTabClose(code_id.clone(), true));
        });
        let link_clone = link.clone();
        let code_id = id.clone();
        let cancel_click = Callback::from(move |_e: MouseEvent| {
            link_clone.send_message(CodeTabMsg::CodeTabClose(code_id.clone(), false));
        });

        let is_update = code_context.is_update_code(id);

        result.push(html! {
            <>
                <Modal id={format!("texteditor-{}", id)}>
                    <form method="dialog">
                        <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">{"x"}</button>
                    </form>
                    <h3 class="font-bold text-lg">{"文件修改提示!"}</h3>
                    <p class="py-4">{"检测到文件已修改，是否保存修改"}</p>
                    <div class="justify-center  modal-action">
                        <form method="dialog" class="flex justify-between w-[60%]">
                            <button onclick={save_click} class="btn w-28" >{"是"}</button>
                            <button onclick={cancel_click} class="btn w-28">{"否"}</button>
                        </form>
                    </div>
                </Modal>
                <a role="tab" onclick={tab_click} class={class}>
                    {tab.name()}
                    if is_update {
                        {"\u{00a0}*"}
                    }
                    <CloseSvg on_click={close_click}/>
                </a>
            </>

        });
    }
    result.into_iter().collect::<Html>()
}
