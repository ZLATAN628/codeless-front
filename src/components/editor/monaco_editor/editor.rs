use monaco::{
    api::{CodeEditorOptions, DisposableClosure, TextModel},
    sys::editor::{BuiltinTheme, IModelContentChangedEvent, IStandaloneEditorConstructionOptions},
    yew::CodeEditor,
};
use yew::prelude::*;

use crate::context::code_context::{CodesContext, CodesStateMsg};

fn get_options() -> CodeEditorOptions {
    CodeEditorOptions::default()
        .with_language("sql".to_owned())
        .with_value("select * from users;".to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
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
    // listener drop 的时候 会 取消监听器的注册
    _content_change: DisposableClosure<dyn FnMut(IModelContentChangedEvent)>,
}

pub enum MonacoEditorMessage {
    Update,
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
        let options = get_options().to_sys_options();
        Self {
            id: id.to_string(),
            options,
            model: text_model.clone(),
            is_updated: false,
            _content_change: content_change,
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
        };
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <CodeEditor classes={"h-full text-sm bg-base-100 overflow-auto scroll-smooth"}
                options={self.options.clone()} model={ self.model.clone() } />
        }
    }
}
