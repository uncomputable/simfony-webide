use leptos::{component, use_context, view, IntoView, RwSignal, SignalSet, SignalUpdate};
use simfony::witness::WitnessValues;

use crate::components::dropdown::Dropdown;
use crate::components::navbar::ActiveTab;
use crate::components::program_window::{Program, TxEnv};

#[component]
pub fn ExamplesDropdown() -> impl IntoView {
    let program = use_context::<RwSignal<Program>>().expect("program should exist in context");
    let witness_values =
        use_context::<RwSignal<WitnessValues>>().expect("witness values should exist in context");
    let tx_env = use_context::<TxEnv>().expect("transaction environment should exist in context");
    let active_tab = use_context::<ActiveTab>().expect("active tab should exist in context");

    let examples = crate::examples::keys().collect::<Vec<_>>();
    let select_example = move |selected| match crate::examples::get(selected) {
        Some(example) => {
            program.set(Program {
                compiled: example.compiled(),
                text: example.program_text().to_string(),
            });
            witness_values.set(example.witness_values());
            tx_env.lock_time.set(example.lock_time());
            tx_env.sequence.set(example.sequence());
            active_tab.0.update(|_| {}); // refresh active tab
        }
        None => {
            // do nothing
        }
    };

    view! {
        <Dropdown name="Examples" options=examples select_option=select_example />
    }
}
