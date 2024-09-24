use std::collections::HashMap;

use crate::components::run_window::{HashedData, RunWindow, SignedData, SigningKeys};
use leptos::{component, create_rw_signal, provide_context, view, IntoView, RwSignal};
use simfony::str::WitnessName;

use super::program_window::{ProgramWindow, TxEnv};

#[derive(Copy, Clone)]
pub struct ProgramWrapper(pub RwSignal<String>);

#[derive(Copy, Clone)]
pub struct WitnessWrapper(pub RwSignal<HashMap<WitnessName, simfony::value::Value>>);

#[component]
pub fn App() -> impl IntoView {
    let program_text = create_rw_signal("".to_string());
    provide_context(ProgramWrapper(program_text));
    let witness_values = create_rw_signal(HashMap::new());
    provide_context(WitnessWrapper(witness_values));
    let tx_env = TxEnv::new(0, 0);
    provide_context(tx_env);
    let signing_keys = SigningKeys::new(1);
    provide_context(signing_keys);
    let signed_data = SignedData::new(tx_env.environment());
    provide_context(signed_data);
    let hashed_data = HashedData::new(1);
    provide_context(hashed_data);

    view! {
        <ProgramWindow />
        <RunWindow />
    }
}
