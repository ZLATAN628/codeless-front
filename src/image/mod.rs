use yew::prelude::*;

const STROKE: AttrValue = AttrValue::Static("currentColor");
const STROKE_WIDTH: AttrValue = AttrValue::Static("1.5");
const BOX: AttrValue = AttrValue::Static("0 0 24 24");
const FILL: AttrValue = AttrValue::Static("none");
const LCAP: AttrValue = AttrValue::Static("round");
const LJOIN: AttrValue = AttrValue::Static("round");

pub static RESPONSIVE_ICON: &'static str = "h-4 w-4 lg:h-5 lg:w-5 2xl:h-6 2xl:w-6";

#[derive(Debug, PartialEq, Properties)]
pub struct SvgProps {
    #[prop_or(RESPONSIVE_ICON)]
    pub classes: &'static str,
    pub children: Children,
}

#[derive(Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
}

#[function_component(Svg)]
pub fn svg(props: &SvgProps) -> Html {
    html! {
        <svg
            class={props.classes}
            viewBox={BOX}
            fill={FILL}
            stroke={STROKE}
            stroke-width={STROKE_WIDTH}
            stroke-linecap={LCAP}
            stroke-linejoin={LJOIN}>
            { props.children.clone() }
        </svg>
    }
}

#[function_component(DirSvg)]
pub fn dir_svg() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-4 h-4">
            <path stroke-linecap="round"
                stroke-linejoin="round"
                d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
        </svg>
    }
}

#[function_component(FileSvg)]
pub fn file_svg() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24" stroke-width="1.5"
            stroke="currentColor" class="w-4 h-4">
            <path stroke-linecap="round"
                stroke-linejoin="round"
                d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
        </svg>
    }
}

#[function_component(CloseSvg)]
pub fn close_svg(props: &IconProps) -> Html {
    html! {
        <svg onclick={props.on_click.clone()} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-4 h-4 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
    }
}

#[function_component(PlaySvg)]
pub fn play_svg(_props: &IconProps) -> Html {
    html! {
        <Svg >
            <polygon points="5 3 19 12 5 21 5 3"/>
        </Svg>
    }
}

#[function_component(SaveSvg)]
pub fn save_svg(_props: &IconProps) -> Html {
    html! {
        <Svg >
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/>
        </Svg>
    }
}

#[function_component(AddSvg)]
pub fn add_svg(_props: &IconProps) -> Html {
    html! {
        <Svg >
            <circle cx="12" cy="12" r="10"/><path d="M8 12h8"/><path d="M12 8v8"/>
        </Svg>
    }
}

#[function_component(SubSvg)]
pub fn sub_svg(_props: &IconProps) -> Html {
    html! {
        <Svg >
            <circle cx="12" cy="12" r="10"/><path d="M8 12h8"/>
        </Svg>
    }
}
