use leptos::{component, create_rw_signal, provide_context, view, IntoView};
use simfony::witness::WitnessValues;

use super::program_window::{Program, ProgramWindow, TxEnv};
use crate::components::run_window::{HashedData, RunWindow, SignedData, SigningKeys};

#[component]
pub fn App() -> impl IntoView {
    let program = create_rw_signal(Program::default());
    provide_context(program);
    let witness_values = create_rw_signal(WitnessValues::empty());
    provide_context(witness_values);
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
