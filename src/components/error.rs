use leptos::{component, view, IntoView, Show, Signal, SignalGet};

#[component]
pub fn ErrorBox(#[prop(into)] error: Signal<String>) -> impl IntoView {
    let error_is_nonempty = move || !error.get().is_empty();

    view! {
        <Show
            when=error_is_nonempty
        >
            <div class="error-box">
                <pre>
                    {error}
                </pre>
            </div>
        </Show>
    }
}
