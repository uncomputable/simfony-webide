use leptos::{component, create_rw_signal, provide_context, view, IntoView, SignalGetUntracked};
use leptos_router::use_query_map;
use simfony::witness::WitnessValues;

use super::program_window::{Program, ProgramWindow, TxEnv};
use crate::components::run_window::{
    ExecutionOutput, HashedData, RunWindow, SignedData, SigningKeys,
};
use crate::components::state::FromParams;

#[component]
pub fn App() -> impl IntoView {
    let url_params = use_query_map().get_untracked();

    let program = create_rw_signal(Program::from_map(&url_params).unwrap_or_default());
    provide_context(program);
    let witness_values = create_rw_signal(WitnessValues::from_map(&url_params).unwrap_or_default());
    provide_context(witness_values);
    let tx_env = TxEnv::from_map(&url_params).unwrap_or_default();
    provide_context(tx_env);
    let signing_keys = SigningKeys::from_map(&url_params).unwrap_or_default();
    provide_context(signing_keys);
    let signed_data = SignedData::new(tx_env.environment());
    provide_context(signed_data);
    let hashed_data = HashedData::from_map(&url_params).unwrap_or_default();
    provide_context(hashed_data);
    provide_context(ExecutionOutput::default());

    view! {
        <ProgramWindow />
        <RunWindow />
    }
}
