use leptos::{component, provide_context, view, IntoView, RwSignal, SignalGetUntracked};
use leptos_router::use_query_map;

use super::program_window::{Program, ProgramWindow, Runtime};
use crate::components::run_window::{HashedData, RunWindow, SignedData, SigningKeys, TxEnv};
use crate::components::state::FromParams;
use crate::transaction::TxParams;

#[derive(Copy, Clone, Debug, Default)]
pub struct ActiveRunTab(pub RwSignal<&'static str>);

#[component]
pub fn App() -> impl IntoView {
    let url_params = use_query_map().get_untracked();

    let signing_keys = SigningKeys::from_map(&url_params).unwrap_or_default();
    provide_context(signing_keys);
    let program = Program::new(signing_keys.first_public_key());
    provide_context(program);
    let tx_params = TxParams::from_map(&url_params).unwrap_or_default();
    let tx_env = TxEnv::new(program, tx_params);
    provide_context(tx_env);
    provide_context(SignedData::new(tx_env.lazy_env));
    provide_context(HashedData::from_map(&url_params).unwrap_or_default());
    provide_context(Runtime::new(program, tx_env.lazy_env));
    provide_context(ActiveRunTab::default());

    view! {
        <ProgramWindow />
        <RunWindow />
    }
}
