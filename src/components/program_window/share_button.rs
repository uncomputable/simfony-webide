use leptos::{component, view, IntoView};

use crate::components::copy_to_clipboard::CopyToClipboard;

#[component]
pub fn ShareButton() -> impl IntoView {
    let url = move || "Sharing is temporarily disabled".to_string();
    view! {
        <CopyToClipboard content=url class="button" tooltip_below=true>
            <i class="fa-solid fa-share-nodes"></i>
            " Share"
        </CopyToClipboard>
    }
}
