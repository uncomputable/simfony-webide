use leptos::{component, create_rw_signal, ev, view, Children, IntoView, SignalSet};

#[component]
pub fn CopyToClipboard(content: String, children: Children) -> impl IntoView {
    let maybe_clipboard = web_sys::window()
        .as_ref()
        .map(web_sys::Window::navigator)
        .as_ref()
        .map(web_sys::Navigator::clipboard);

    match maybe_clipboard {
        Some(clipboard) => {
            let tooltip_text = create_rw_signal("Copy");

            let button_click = move |_event: ev::MouseEvent| {
                let _promise = clipboard.write_text(content.as_str());
                tooltip_text.set("Copied!");
            };
            let button_mouseout = move |_event: ev::MouseEvent| {
                tooltip_text.set("Copy");
            };

            view! {
                <div class="tooltip">
                    <button
                        class="copy-button"
                        on:click=button_click
                        on:mouseout=button_mouseout
                    >
                        <span class="tooltip-text">{tooltip_text}</span>
                        {children()}
                    </button>
                </div>
            }
            .into_any()
        }
        None => view! {
            <p>Clipboard not supported</p>
        }
        .into_any(),
    }
}
