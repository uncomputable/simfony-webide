use leptos::{component, ev, view, IntoView};

#[component]
pub fn RunButton() -> impl IntoView {
    let dummy_run = |_: ev::MouseEvent| leptos::logging::log!("Dummy run!");

    view! {
        <button
            class="action-button"
            on:click=dummy_run
        >
            <i class="fas fa-play"></i>
            " Run (WIP)"
        </button>
    }
}
