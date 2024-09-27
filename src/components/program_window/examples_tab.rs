use leptos::{
    component, create_rw_signal, ev, event_target_value, use_context, view, CollectView, IntoView,
    RwSignal, Signal, SignalGet, SignalSet,
};

use crate::components::apply_changes::ApplyChanges;
use crate::components::program_window::Program;
use crate::examples;

#[component]
pub fn ExamplesTab() -> impl IntoView {
    let program = use_context::<RwSignal<Program>>().expect("program should exist in context");
    // TODO: Set witness data
    // TODO: Set transaction data
    let selected_name = create_rw_signal("".to_string());
    let apply_changes = ApplyChanges::default();

    let select_example = move |event: ev::Event| {
        let name = event_target_value(&event);
        selected_name.set(name);
    };
    let submit_example = move |event: ev::SubmitEvent| {
        event.prevent_default(); // stop page from reloading
        let name = selected_name.get();
        match examples::get_program_str(&name) {
            Some(s) => {
                let x = Program::compile(s.to_string()).expect("example program should compile");
                program.set(x);
                apply_changes.set_success(true);
            }
            None => {
                apply_changes.set_success(false);
            }
        }
    };
    let selectable_values = examples::get_names()
        .map(|name| {
            view! {
                <option value={name}>{name}</option>
            }
        })
        .collect_view();

    view! {
        <div>
            <ExampleDescription selected_name=selected_name />
            <form on:submit=submit_example>
                <select
                    class="example-program-select"
                    on:input=select_example
                >
                    <option value="" disabled selected>Select an example!</option>
                    {selectable_values}
                </select>
                {apply_changes}
            </form>
        </div>
    }
}

#[component]
fn ExampleDescription(#[prop(into)] selected_name: Signal<String>) -> impl IntoView {
    let selected_title = move || -> String {
        let name = selected_name.get();
        match name.is_empty() {
            true => "Nothing selected".to_string(),
            false => name,
        }
    };
    let selected_description = move || -> &'static str {
        let name = selected_name.get();
        examples::get_description(&name).unwrap_or(
            r#"<p>
Select an example to see what it is about.
Click apply to write the example to the other tabs.
</p>"#,
        )
    };

    view! {
        <div>
            <h3 class="program-title">
                {selected_title}
            </h3>
            <div
                class="program-description"
                inner_html={selected_description}
            >
            </div>
        </div>
    }
}
