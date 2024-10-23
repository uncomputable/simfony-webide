use leptos::{
    component, create_rw_signal, provide_context, view, IntoView, RwSignal, SignalGetUntracked,
};
use leptos_router::use_query_map;

use super::program_window::{Program, ProgramWindow};
use crate::components::run_window::{
    ExecutionOutput, HashedData, RunWindow, SignedData, SigningKeys, TxEnv,
};
use crate::components::state::FromParams;
use crate::transaction::TxParams;

#[derive(Copy, Clone, Debug)]
pub(crate) struct ActiveRunTab(pub RwSignal<&'static str>);

#[component]
pub fn App() -> impl IntoView {
    let url_params = use_query_map().get_untracked();

    let program = Program::from_map(&url_params).unwrap_or_default();
    provide_context(program);
    let tx_params = TxParams::from_map(&url_params).unwrap_or_default();
    let tx_env = TxEnv::new(program, tx_params);
    provide_context(tx_env);
    let signing_keys = SigningKeys::from_map(&url_params).unwrap_or_default();
    provide_context(signing_keys);
    let signed_data = SignedData::new(tx_env.lazy_env);
    provide_context(signed_data);
    let hashed_data = HashedData::from_map(&url_params).unwrap_or_default();
    provide_context(hashed_data);
    provide_context(ExecutionOutput::default());

    let active_run_tab = create_rw_signal("");
    provide_context(ActiveRunTab(active_run_tab));

    view! {
        <ProgramWindow />
        <RunWindow active_tab=active_run_tab />
    }
}
