use yew::prelude::*;

#[derive(PartialEq, Properties, Debug)]
pub struct ThemeProps {
    pub children: Children,
}

#[function_component(Theme)]
pub fn theme(props: &ThemeProps) -> Html {
    html! {
        <div data-theme="dark"
            class="flex flex-col h-screen overflow-hidden">
            {props.children.clone()}
        </div>
    }
}
