use gloo::console::log;
use monaco::{
    api::{CodeEditorOptions, DisposableClosure, TextModel},
    sys::editor::{
        IEditorMinimapOptions, IModelContentChangedEvent, IStandaloneEditorConstructionOptions,
    },
    yew::{CodeEditor, CodeEditorLink},
};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

fn get_options() -> CodeEditorOptions {
    CodeEditorOptions::default()
        .with_language("json".to_owned())
        .with_value("{}".to_owned())
        .with_automatic_layout(true)
}

#[derive(PartialEq, Properties)]
pub struct ResponseEditorProps {
    pub text_model: TextModel,
}

pub struct ResponseEditor {
    options: IStandaloneEditorConstructionOptions,
    model: TextModel,
    editor_link: CodeEditorLink,
    _content_change: DisposableClosure<dyn FnMut(IModelContentChangedEvent)>,
}

pub enum ResponseEditorMessage {
    FormatCode,
    InitEditor(CodeEditorLink),
    SetReadOnly,
}

impl Component for ResponseEditor {
    type Message = ResponseEditorMessage;
    type Properties = ResponseEditorProps;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        let ResponseEditorProps { text_model } = props;

        let link = ctx.link().clone();
        let content_change =
            text_model.on_did_change_content(move |_e: IModelContentChangedEvent| {
                link.send_message(ResponseEditorMessage::FormatCode);
            });

        // 事件监听
        let options = get_options().to_sys_options();
        options.set_read_only(Some(true));
        let minmap = IEditorMinimapOptions::default();
        minmap.set_enabled(Some(false));
        options.set_minimap(Some(&minmap));
        Self {
            options,
            model: text_model.clone(),
            editor_link: CodeEditorLink::new(),
            _content_change: content_change,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ResponseEditorMessage::FormatCode => {
                match self.editor_link.with_editor(|editor| {
                    let raw_editor = editor.as_ref();
                    self.options.set_read_only(Some(false));
                    raw_editor.update_options(&self.options);
                    let promise = raw_editor.get_action("editor.action.formatDocument").run();
                    let link = ctx.link().clone();
                    let f = JsFuture::from(promise);
                    spawn_local(async move {
                        f.await.unwrap();
                        link.send_message(ResponseEditorMessage::SetReadOnly);
                    })
                }) {
                    Some(_) => (),
                    None => {
                        log!("none")
                    }
                }
            }
            ResponseEditorMessage::InitEditor(link) => {
                self.editor_link = link;
            }
            ResponseEditorMessage::SetReadOnly => {
                self.editor_link.with_editor(|editor| {
                    let raw_editor = editor.as_ref();
                    self.options.set_read_only(Some(true));
                    raw_editor.update_options(&self.options);
                });
            }
        }
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_editor_created = {
            ctx.link()
                .callback(|e: CodeEditorLink| ResponseEditorMessage::InitEditor(e))
        };
        html! {
            <>
                <CodeEditor classes={"w-full h-full tab-content text-sm overflow-auto scroll-smooth"}
                    options={self.options.clone()} model={self.model.clone()} {on_editor_created} />
            </>
        }
    }
}
