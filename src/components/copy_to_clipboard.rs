use leptos::{component, create_rw_signal, ev, view, with, Children, IntoView, Signal, SignalSet};

#[component]
pub fn CopyToClipboard(
    #[prop(into)] content: Signal<String>,
    #[prop(into)] class: String,
    #[prop(default = false)] tooltip_above: bool,
    children: Children,
) -> impl IntoView {
    web_sys::window()
        .as_ref()
        .map(web_sys::Window::navigator)
        .as_ref()
        .map(web_sys::Navigator::clipboard)
        .map(|clipboard| {
            let tooltip_text = create_rw_signal("Copy");

            let button_click = move |_event: ev::MouseEvent| {
                let _promise = with!(|content| clipboard.write_text(content));
                tooltip_text.set("Copied!");
            };
            let button_mouseout = move |_event: ev::MouseEvent| {
                tooltip_text.set("Copy");
            };
            let tooltip_class = match tooltip_above {
                false => "tooltip-below",
                true => "tooltip-above",
            };

            view! {
                <div class=tooltip_class>
                    <button
                        class=class
                        on:click=button_click
                        on:mouseout=button_mouseout
                    >
                        <span class="tooltip-text">{tooltip_text}</span>
                        {children()}
                    </button>
                </div>
            }
        })
}
