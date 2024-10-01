use std::rc::Rc;

use leptos::leptos_dom::Transparent;
use leptos::{
    component, create_rw_signal, ev, view, Callable, Callback, Children, ChildrenFn, CollectView,
    IntoView, RwSignal, SignalGet, SignalSet, View,
};

#[component]
pub fn Navbar(default_tab: &'static str, children: Children) -> impl IntoView {
    let active_tab = create_rw_signal(default_tab);
    let mut tabs_content = Vec::new();
    let mut button_bar = Vec::new();

    for child in children()
        .as_children()
        .iter()
        .filter_map(View::as_transparent)
        .filter_map(Transparent::downcast_ref::<TabView>)
        .cloned()
    {
        match child {
            TabView::Navigation { name, children } => {
                tabs_content.push((name, children));
                button_bar.push(view! {<RenderedNavigation tab_name=name active_tab=active_tab />})
            }
            TabView::Dropdown { name, options, select_option } => {
                button_bar.push(view! {<RenderedDropdown button_name=name options=options select_option=select_option />})
            }
            TabView::Action { action, children } => {
                button_bar.push(view! {
                    <RenderedAction action=action>
                        {children()}
                    </RenderedAction>
                })
            }
        }
    }
    let active_tab_content = move || -> ChildrenFn {
        tabs_content
            .iter()
            .find(|(tab_name, _content)| tab_name == &active_tab.get())
            .expect("Tab not found")
            .1
            .clone()
    };

    view! {
        <div class="navbar">
            {button_bar}
        </div>
        {active_tab_content}
    }
}

#[component(transparent)]
pub fn Navigation(name: &'static str, children: ChildrenFn) -> impl IntoView {
    TabView::Navigation { name, children }
}

#[component(transparent)]
pub fn Dropdown(
    name: &'static str,
    #[prop(into)] options: Rc<[&'static str]>,
    #[prop(into)] select_option: Callback<&'static str>,
) -> impl IntoView {
    TabView::Dropdown {
        name,
        options,
        select_option,
    }
}

#[component(transparent)]
pub fn Action(#[prop(into)] action: Callback<()>, children: ChildrenFn) -> impl IntoView {
    TabView::Action { action, children }
}

#[derive(Clone)]
enum TabView {
    Navigation {
        name: &'static str,
        children: ChildrenFn,
    },
    Dropdown {
        name: &'static str,
        options: Rc<[&'static str]>,
        select_option: Callback<&'static str>,
    },
    Action {
        action: Callback<()>,
        children: ChildrenFn,
    },
}

impl IntoView for TabView {
    fn into_view(self) -> View {
        Transparent::new(self).into_view()
    }
}

#[component]
fn RenderedNavigation(tab_name: &'static str, active_tab: RwSignal<&'static str>) -> impl IntoView {
    let button_click = move |_event: ev::MouseEvent| active_tab.set(tab_name);
    view! {
        <button
            class="navigation-button"
            on:click=button_click
        >
            {tab_name}
        </button>
    }
}

#[component]
fn RenderedDropdown(
    button_name: &'static str,
    options: Rc<[&'static str]>,
    select_option: Callback<&'static str>,
) -> impl IntoView {
    let options_view = move || -> View {
        options
            .iter()
            .cloned()
            .map(|option| {
                let button_click = move |_event: ev::MouseEvent| select_option.call(option);
                view! {
                     <button
                        class="action-button"
                        on:click=button_click
                    >
                        {option}
                    </button>
                }
            })
            .collect_view()
    };

    view! {
        <div class="dropdown">
            <button class="dropdown-button">
                {button_name}" "
                <i class="fa fa-caret-down"></i>
            </button>
            <div class="dropdown-content">
                {options_view}
            </div>
        </div>
    }
}

#[component]
fn RenderedAction(action: Callback<()>, children: Children) -> impl IntoView {
    let button_click = move |_event: ev::MouseEvent| action.call(());
    view! {
        <button
            class="action-button"
            on:click=button_click
        >
            {children()}
        </button>
    }
}
