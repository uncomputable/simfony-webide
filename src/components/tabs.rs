use std::rc::Rc;

use leptos::leptos_dom::Transparent;
use leptos::{
    component, create_signal, view, Children, ChildrenFn, CollectView, IntoView, SignalGet,
    SignalSet, View,
};

#[component]
pub fn Tabs(#[prop(into)] default_tab: Rc<str>, children: Children) -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal(default_tab);

    let tabs_map = children()
        .as_children()
        .iter()
        .filter_map(|child| {
            child
                .as_transparent()
                .and_then(Transparent::downcast_ref::<TabView>)
                .cloned()
                .map(|view| (view.name, view.children))
        })
        .collect::<Vec<(Rc<str>, ChildrenFn)>>(); // Vec instead of HashMap to preserve order

    let available_tabs_bar = tabs_map
        .iter()
        .map(|(name, _)| {
            let tab_name0 = Rc::clone(name);
            let tab_name1 = Rc::clone(name);
            let tab_name2 = Rc::clone(name);
            let button_class = move || match tab_name0 == active_tab.get() {
                true => "active",
                false => "inactive",
            };
            let button_click = move |_event| set_active_tab.set(Rc::clone(&tab_name1));

            view! {
                <button
                    class=button_class
                    on:click=button_click
                >
                    {tab_name2}
                </button>
            }
        })
        .collect_view();

    let active_tab_content = move || {
        tabs_map
            .iter()
            .find(|(other_name, _content)| other_name.as_ref() == active_tab.get().as_ref())
            .expect("Tab not found")
            .1
            .clone()
    };

    view! {
        <div class="tabs">
            {available_tabs_bar}
        </div>
        {active_tab_content}
    }
}

#[component(transparent)]
pub fn Tab(#[prop(into)] name: Rc<str>, children: ChildrenFn) -> impl IntoView {
    TabView { name, children }
}

#[derive(Clone)]
struct TabView {
    name: Rc<str>,
    children: ChildrenFn,
}

impl IntoView for TabView {
    fn into_view(self) -> View {
        Transparent::new(self).into_view()
    }
}
