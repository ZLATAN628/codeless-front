use monaco::{
    api::{CodeEditorOptions, DisposableClosure, TextModel},
    sys::{
        editor::{IModelContentChangedEvent, IStandaloneEditorConstructionOptions},
        IDisposable,
    },
    yew::{CodeEditor, CodeEditorLink},
};
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

use crate::{
    context::code_context::{CodeSetProps, CodesContext, CodesStateMsg},
    set_code,
};

fn get_options() -> CodeEditorOptions {
    CodeEditorOptions::default()
        .with_language("sql".to_owned())
        .with_value("select * from users;".to_owned())
        .with_automatic_layout(true)
}

#[derive(PartialEq, Properties)]
pub struct MonacoEditorProps {
    pub text_model: TextModel,
    pub id: String,
}

//需要保留 事件监听函数 返回的 取消监听器对象 没法用 函数组件
// #[function_component(MonacoEditor)]
// pub fn monaco_editor(props: &MonacoEditorProps) -> Html {
//     let MonacoEditorProps { text_model, id } = props;
//     html! {
//         <CodeEditor classes={"h-full text-sm bg-base-100 overflow-auto scroll-smooth"}
//             options={ get_options().to_sys_options() } model={text_model.clone()} />
//     }
// }

pub struct MonacoEditor {
    // code id
    id: String,
    options: IStandaloneEditorConstructionOptions,
    model: TextModel,
    is_updated: bool,
    code_context: CodesContext,
    _code_context_handle: ContextHandle<CodesContext>,
    // listener drop 的时候 会 取消监听器的注册
    _content_change: DisposableClosure<dyn FnMut(IModelContentChangedEvent)>,
    code_editor_link: CodeEditorLink,
    _onblue: Option<IDisposable>,
    _blue_closure: Option<Closure<dyn FnMut()>>,
}

pub enum MonacoEditorMessage {
    Update,
    Init(CodeEditorLink),
    SetScript(String),
}

impl Component for MonacoEditor {
    type Message = MonacoEditorMessage;
    type Properties = MonacoEditorProps;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let MonacoEditorProps { text_model, id } = props;
        // 事件监听
        let link = ctx.link().clone();
        let content_change =
            text_model.on_did_change_content(move |_e: IModelContentChangedEvent| {
                link.send_message(MonacoEditorMessage::Update);
            });
        let (code_context, code_context_handle) = ctx
            .link()
            .context::<CodesContext>(Callback::noop())
            .unwrap();
        let options = get_options().to_sys_options();
        Self {
            id: id.to_string(),
            options,
            model: text_model.clone(),
            is_updated: false,
            code_context,
            _code_context_handle: code_context_handle,
            _content_change: content_change,
            code_editor_link: CodeEditorLink::default(),
            _blue_closure: None,
            _onblue: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MonacoEditorMessage::Update => {
                if !self.is_updated {
                    self.is_updated = true;
                    // code_context.did_change_editor(self.id.clone());
                    self.code_context
                        .dispatch(CodesStateMsg::EditorChanged(self.id.clone()));
                }
            }
            MonacoEditorMessage::Init(link) => {
                self.code_editor_link = link;
                self.code_editor_link.with_editor(|editor| {
                    let raw_editor = editor.as_ref();
                    let editor_link = self.code_editor_link.clone();
                    let link = ctx.link().clone();
                    let closure = Closure::wrap(Box::new(move || {
                        editor_link.with_editor(|editor| {
                            let value = editor.as_ref().get_value(None);
                            link.send_message(MonacoEditorMessage::SetScript(value));
                        });
                    }) as Box<dyn FnMut()>);
                    let onblue = raw_editor
                        .on_did_blur_editor_widget(JsCast::unchecked_ref(closure.as_ref()));
                    self._onblue = Some(onblue);
                    self._blue_closure = Some(closure);
                });
            }
            MonacoEditorMessage::SetScript(script) => {
                let code_context = &self.code_context;
                set_code!(code_context => Script(script));
            }
        };
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_editor_created = ctx
            .link()
            .callback(|link: CodeEditorLink| MonacoEditorMessage::Init(link));
        html! {
            <CodeEditor classes={"h-full text-sm bg-base-100 overflow-auto scroll-smooth"}
                options={self.options.clone()} model={ self.model.clone() } on_editor_created={on_editor_created} />
        }
    }
}
