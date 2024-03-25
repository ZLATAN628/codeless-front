use web_sys::MouseEvent;
use yew::{classes, function_component, html, AttrValue, Callback, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub tip: AttrValue,
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub position: TooltipPosition,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TooltipPosition {
    #[allow(dead_code)]
    Top,
    Bottom,
    #[allow(dead_code)]
    Left,
    #[allow(dead_code)]
    Right,
}

impl Default for TooltipPosition {
    fn default() -> Self {
        TooltipPosition::Bottom
    }
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let mut classes = classes!("overflow-visible", "tooltip", "tooltip-info", "block");

    match &props.position {
        TooltipPosition::Top => classes.push("tooltip-top"),
        TooltipPosition::Bottom => classes.push("tooltip-bottom"),
        TooltipPosition::Left => classes.push("tooltip-left"),
        TooltipPosition::Right => classes.push("tooltip-right"),
    };

    html! {
        <div data-tip={&props.tip} class={classes} onclick={&props.onclick}>
            {for props.children.clone()}
        </div>
    }
}
