use leptos::{component, use_context, view, IntoView};

use crate::components::copy_to_clipboard::CopyToClipboard;
use crate::components::program_window::Program;
use crate::util;

#[component]
pub fn AddressButton() -> impl IntoView {
    let program = use_context::<Program>().expect("program should exist in context");

    let address = move || -> String {
        program
            .cmr()
            .ok()
            .map(util::liquid_testnet_address)
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "Invalid program".to_string())
    };
    view! {
        <CopyToClipboard content=address class="button" tooltip_below=true>
            <i class="fa-solid fa-inbox"></i>
            " Address"
        </CopyToClipboard>
    }
}
