use yew::prelude::*;

use crate::components::{api_display::ApiDisplay, api_list::api_select::ApiSelect};

#[function_component(MainBody)]
pub fn main_body() -> Html {
    html! {
        <div class="flex h-screen">
            <ApiSelect />
            <ApiDisplay />
        </div>
    }
}
