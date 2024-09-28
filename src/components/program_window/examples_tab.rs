use leptos::{
    component, create_rw_signal, ev, event_target_value, use_context, view, CollectView, IntoView,
    RwSignal, Signal, SignalGet, SignalSet,
};
use simfony::witness::WitnessValues;

use crate::components::apply_changes::ApplyChanges;
use crate::components::program_window::{Program, TxEnv};
use crate::examples;
use crate::examples::Example;

#[component]
pub fn ExamplesTab() -> impl IntoView {
    let program = use_context::<RwSignal<Program>>().expect("program should exist in context");
    let witness_values =
        use_context::<RwSignal<WitnessValues>>().expect("witness values should exist in context");
    let tx_env = use_context::<TxEnv>().expect("transaction environment should exist in context");
    let selected_name = create_rw_signal("".to_string());
    let apply_changes = ApplyChanges::default();

    let select_example = move |event: ev::Event| {
        let name = event_target_value(&event);
        selected_name.set(name);
    };
    let submit_example = move |event: ev::SubmitEvent| {
        event.prevent_default(); // stop page from reloading
        let name = selected_name.get();
        match examples::get(&name) {
            Some(example) => {
                program.set(Program {
                    compiled: example.compiled(),
                    text: example.program_text().to_string(),
                });
                witness_values.set(example.witness_values());
                tx_env.lock_time.set(example.lock_time());
                tx_env.sequence.set(example.sequence());
                apply_changes.set_success(true);
            }
            None => {
                apply_changes.set_success(false);
            }
        }
    };
    let selectable_values = examples::keys()
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
        examples::get(&name).map(Example::description).unwrap_or(
            r#"Select an example to see what it is about.

Click Apply to write the example to the other tabs."#,
        )
    };

    view! {
        <div class="program-details">
            <h3 class="program-title">
                {selected_title}
            </h3>
            <div
                class="program-description"
            >
                {selected_description}
            </div>
        </div>
    }
}
