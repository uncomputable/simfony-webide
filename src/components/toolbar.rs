use leptos::{component, view, Children, IntoView};

#[component]
pub fn Toolbar(children: Children) -> impl IntoView {
    view! {
        <div class="navbar">
            {children()}
        </div>
    }
}
