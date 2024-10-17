use leptos::{component, view, IntoView};

use crate::components::copy_to_clipboard::CopyToClipboard;
use crate::components::state::stateful_url;

#[component]
pub fn ShareButton() -> impl IntoView {
    let url = move || stateful_url().unwrap_or_default();
    view! {
        <CopyToClipboard content=url class="button">
            <i class="fa-solid fa-share-nodes"></i>
            " Share"
        </CopyToClipboard>
    }
}
