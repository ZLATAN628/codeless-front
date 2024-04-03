use monaco::{
    api::{CodeEditorOptions, DisposableClosure, TextModel},
    sys::editor::{IModelContentChangedEvent, IStandaloneEditorConstructionOptions},
    yew::{CodeEditor, CodeEditorLink},
};
use yew::prelude::*;

use crate::context::{
    code_context::{CodesContext, CodesStateMsg},
    theme_context::ThemeContext,
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
    #[allow(dead_code)]
    theme_context: UseStateHandle<ThemeContext>,
    _theme_context_listener: ContextHandle<UseStateHandle<ThemeContext>>,
    // listener drop 的时候 会 取消监听器的注册
    _content_change: DisposableClosure<dyn FnMut(IModelContentChangedEvent)>,
    code_editor_link: CodeEditorLink,
}

pub enum MonacoEditorMessage {
    Update,
    Init(CodeEditorLink),
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
        let (theme_context, theme_context_listener) = ctx
            .link()
            .context::<UseStateHandle<ThemeContext>>(Callback::noop())
            .unwrap();
        let options = get_options().to_sys_options();
        Self {
            id: id.to_string(),
            options,
            model: text_model.clone(),
            is_updated: false,
            theme_context,
            _theme_context_listener: theme_context_listener,
            _content_change: content_change,
            code_editor_link: CodeEditorLink::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MonacoEditorMessage::Update => {
                if !self.is_updated {
                    self.is_updated = true;
                    let (code_context, _) = ctx
                        .link()
                        .context::<CodesContext>(Callback::noop())
                        .unwrap();
                    // code_context.did_change_editor(self.id.clone());
                    code_context.dispatch(CodesStateMsg::EditorChanged(self.id.clone()));
                }
            }
            MonacoEditorMessage::Init(link) => {
                self.code_editor_link = link;
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
