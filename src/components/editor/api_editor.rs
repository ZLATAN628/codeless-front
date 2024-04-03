use yew::prelude::*;

use crate::{
    // common::highlight::highlight_code,
    components::editor::code_tab::CodeTabComponent,
    context::code_context::use_codes,
};

#[derive(Properties, PartialEq)]
pub struct ApiEditorProps {
    pub width: f64,
}

#[function_component(ApiEditor)]
pub fn api_editor(props: &ApiEditorProps) -> Html {
    let style = format!("width: {}%;", &props.width);
    let code_context = use_codes();
    let code_editor = code_context.code_editors();
    // 自己写的代码编辑器 需要的 代码高亮功能
    // let text = highlight_code(&text);
    // Vscode 编辑器的
    html! {
        <div
            class="flex flex-col text-sm border border-neutral border-collapse font-mono" style={style}>
            // 代码页标签
            <CodeTabComponent />
            // 自己写的 代码编辑器
            // <ApiTextArea text={text}/>
            // Vscode 的代码编辑器
            {
                {
                    if let Some(code_editor) = code_editor {
                        code_editor.clone()
                    } else {
                        html! {
                            <div class="flex items-center justify-center w-full h-full">
                                <div class="text-left text-lg leading-10" >
                                    <p >{"保存"}<em class="ml-4 text-info">{"Ctrl + S"}</em><br /> {"运行"}<em class="ml-4 text-info">{"Ctrl + Q"}</em><br /> {"代码格式化"}<em class="ml-4 text-info">{"Ctrl + Alt + L"}</em><br /> {"最近打开"}<em class="ml-4 text-info">{"Ctrl + E"}</em></p>
                                </div>
                            </div>
                        }
                    }
                }
            }
        </div>
    }
}
