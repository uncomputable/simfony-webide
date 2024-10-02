use leptos::{component, use_context, view, IntoView, RwSignal};

use crate::components::error::ErrorBox;

#[derive(Copy, Clone, Debug, Default)]
pub struct ExecutionOutput {
    pub debug: RwSignal<String>,
    pub error: RwSignal<String>,
}

#[component]
pub fn ExecutionTab() -> impl IntoView {
    let output =
        use_context::<ExecutionOutput>().expect("execution output should exist in context");

    view! {
        <div>
            <ErrorBox error=output.debug />
            <ErrorBox error=output.error />
        </div>
    }
}
