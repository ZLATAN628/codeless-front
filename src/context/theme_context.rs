use yew::prelude::*;

#[derive(PartialEq, Properties, Debug)]
pub struct ThemeProviderProps {
    pub children: Children,
}

///
/// theme: ["light", "dark", "cupcake"]
///
#[derive(PartialEq, Clone, Properties)]
pub struct ThemeContext {
    pub theme: &'static str,
}

impl From<&'static str> for ThemeContext {
    fn from(value: &'static str) -> Self {
        ThemeContext { theme: value }
    }
}

impl ThemeContext {
    #[allow(dead_code)]
    pub fn theme_name(&self) -> &'static str {
        self.theme
    }
}

#[hook]
pub(crate) fn use_themes() -> UseStateHandle<ThemeContext> {
    use_context::<UseStateHandle<ThemeContext>>().unwrap()
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let context = use_state(|| ThemeContext { theme: "dark" });
    html! {
        <ContextProvider<UseStateHandle<ThemeContext>> context={context.clone()}>
            <div
                // data-theme={context.theme}
                class="theme-controller flex flex-col h-screen overflow-hidden">
                {props.children.clone()}
            </div>
        </ContextProvider<UseStateHandle<ThemeContext>>>
    }
}
