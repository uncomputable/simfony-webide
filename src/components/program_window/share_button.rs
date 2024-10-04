use leptos::{component, ev, view, IntoView};

use crate::components::state::stateful_url;

#[component]
pub fn ShareButton() -> impl IntoView {
    web_sys::window()
        .as_ref()
        .map(web_sys::Window::navigator)
        .as_ref()
        .map(web_sys::Navigator::clipboard)
        .map(|clipboard| {
            let button_click = move |_event: ev::MouseEvent| {
                let _promise = clipboard.write_text(stateful_url().unwrap_or_default().as_str());
            };

            view! {
                <div>
                    <button
                        class="button share-button"
                        on:click=button_click
                    >
                        <i class="fa-solid fa-share-nodes"></i>
                        " Share"
                    </button>
                </div>
            }
        })
}
