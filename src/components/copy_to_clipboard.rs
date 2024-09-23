use leptos::{
    component, create_rw_signal, ev, spawn_local, view, IntoView, Show, SignalGet, SignalSet,
};

#[component]
pub fn CopyToClipboard(label: String, content: String) -> impl IntoView {
    let maybe_clipboard = web_sys::window()
        .as_ref()
        .map(web_sys::Window::navigator)
        .as_ref()
        .map(web_sys::Navigator::clipboard);
    let copied = create_rw_signal(false);

    match maybe_clipboard {
        Some(clipboard) => {
            let button_click = move |_event: ev::MouseEvent| {
                let promise = clipboard.write_text(content.as_str());
                let future = wasm_bindgen_futures::JsFuture::from(promise);
                spawn_local(async move {
                    if future.await.is_ok() {
                        copied.set(true);
                        gloo_timers::future::TimeoutFuture::new(500).await;
                        copied.set(false);
                    }
                });
            };

            view! {
                <button
                    class="copy-button"
                    on:click=button_click
                >
                    <Show
                        when=move || copied.get()
                        fallback=move || label.clone()
                    >
                        "Copied!"
                    </Show>
                    <i class="far fa-copy"></i>
                </button>
            }
            .into_any()
        }
        None => view! {
            <p>Clipboard not supported</p>
        }
        .into_any(),
    }
}
