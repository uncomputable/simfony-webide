use leptos::*;

#[component]
pub fn ParseError(maybe_error: Signal<Option<String>>) -> impl IntoView {
    view! {
        {
            move || maybe_error.get().map(|error| view! {
                <div class="parsing-error-box">
                    <pre>
                        {error}
                    </pre>
                </div>
            })
        }
    }
}
