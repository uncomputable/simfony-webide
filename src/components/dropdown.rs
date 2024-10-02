use std::rc::Rc;

use leptos::{component, ev, view, CollectView, IntoView, View};

#[component]
pub fn Dropdown(
    name: &'static str,
    #[prop(into)] options: Rc<[&'static str]>,
    select_option: impl Fn(&'static str) + Copy + 'static,
) -> impl IntoView {
    let options_view = move || -> View {
        options
            .iter()
            .map(|name| {
                view! {
                    <Option name=name select_option=select_option />
                }
            })
            .collect_view()
    };

    view! {
        <div class="dropdown">
            <button class="dropdown-button">
                {name}" "
                <i class="fa fa-caret-down"></i>
            </button>
            <div class="dropdown-content">
                {options_view}
            </div>
        </div>
    }
}

#[component]
fn Option(name: &'static str, select_option: impl Fn(&'static str) + 'static) -> impl IntoView {
    let button_click = move |_event: ev::MouseEvent| select_option(name);
    view! {
        <button
            class="action-button"
            on:click=button_click
        >
            {name}
        </button>
    }
}
