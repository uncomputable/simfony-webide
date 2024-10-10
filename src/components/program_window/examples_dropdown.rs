use leptos::{component, use_context, view, IntoView, SignalSet, SignalUpdate};

use crate::components::dropdown::Dropdown;
use crate::components::navbar::ActiveTab;
use crate::components::program_window::{ProgramText, TxEnv};

#[component]
pub fn ExamplesDropdown() -> impl IntoView {
    let program_text = use_context::<ProgramText>().expect("program text should exist in context");
    let tx_env = use_context::<TxEnv>().expect("transaction environment should exist in context");
    let active_tab = use_context::<ActiveTab>().expect("active tab should exist in context");

    let examples = crate::examples::keys().collect::<Vec<&'static str>>();
    let select_example = move |selected| match crate::examples::get(selected) {
        Some(example) => {
            program_text.0.set(example.program_text().to_string());
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
