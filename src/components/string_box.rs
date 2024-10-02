use leptos::{component, view, IntoView, Show, Signal, SignalWith};

#[component]
pub fn SuccessBox(#[prop(into)] success: Signal<String>) -> impl IntoView {
    view! {
        <StringBox string=success box_class="success-box" />
    }
}

#[component]
pub fn NeutralBox(#[prop(into)] neutral: Signal<String>) -> impl IntoView {
    view! {
        <StringBox string=neutral box_class="neutral-box" />
    }
}

#[component]
pub fn ErrorBox(#[prop(into)] error: Signal<String>) -> impl IntoView {
    view! {
        <StringBox string=error box_class="error-box" />
    }
}

#[component]
fn StringBox(#[prop(into)] string: Signal<String>, box_class: &'static str) -> impl IntoView {
    let string_is_nonempty = move || !string.with(String::is_empty);

    view! {
        <Show
            when=string_is_nonempty
        >
            <div class=box_class>
                <pre>
                    {string}
                </pre>
            </div>
        </Show>
    }
}
