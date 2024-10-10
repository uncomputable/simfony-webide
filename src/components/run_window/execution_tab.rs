use js_sys::Date;
use leptos::{component, use_context, view, IntoView, RwSignal, SignalWith};

use crate::components::string_box::{ErrorBox, NeutralBox, SuccessBox};

#[derive(Copy, Clone, Debug, Default)]
pub struct ExecutionOutput {
    pub debug: RwSignal<String>,
    pub error: RwSignal<String>,
}

#[component]
pub fn ExecutionTab() -> impl IntoView {
    let output =
        use_context::<ExecutionOutput>().expect("execution output should exist in context");
    let success_string = move || {
        output.error.with(|error| match error.is_empty() {
            true => format!("{}: Success.", get_local_datetime()),
            false => "".to_string(),
        })
    };
    let failure_string = move || {
        output.error.with(|error| match error.is_empty() {
            true => "".to_string(),
            false => format!("{}:\n{error}", get_local_datetime()),
        })
    };

    view! {
        <div class="tab-content">
            <SuccessBox success=success_string />
            <ErrorBox error=failure_string />
            <NeutralBox neutral=output.debug />
        </div>
    }
}

fn get_local_datetime() -> String {
    let date = Date::new_0();
    date.to_iso_string().as_string().unwrap()
}
