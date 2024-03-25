use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ModalProps {
    pub id: String,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    html! {
        <>
            <dialog id={props.id.clone()} class="modal">
            <div class="modal-box">
                {props.children.clone()}
            </div>
            </dialog>
        </>
    }
}
