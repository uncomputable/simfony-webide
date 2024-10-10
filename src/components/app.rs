use leptos::{component, provide_context, view, IntoView, SignalGetUntracked};
use leptos_router::use_query_map;

use super::program_window::{ProgramText, ProgramWindow, TxEnv};
use crate::components::run_window::{
    ExecutionOutput, HashedData, RunWindow, SignedData, SigningKeys,
};
use crate::components::state::FromParams;

#[component]
pub fn App() -> impl IntoView {
    let url_params = use_query_map().get_untracked();

    let program_text = ProgramText::from_map(&url_params).unwrap_or_default();
    provide_context(program_text);
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
