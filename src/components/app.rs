use leptos::{component, provide_context, view, IntoView, RwSignal};

use super::program_window::{Program, ProgramWindow, Runtime};
use crate::components::footer::Footer;
use crate::components::run_window::{HashedData, RunWindow, SignedData, SigningKeys, TxEnv};
use crate::components::state::LocalStorage;
use crate::transaction::TxParams;

#[derive(Copy, Clone, Debug, Default)]
pub struct ActiveRunTab(pub RwSignal<&'static str>);

#[component]
pub fn App() -> impl IntoView {
    let signing_keys = SigningKeys::load_from_storage().unwrap_or_default();
    provide_context(signing_keys);
    let program = Program::load_from_storage()
        .unwrap_or_else(|| Program::new_p2pk(signing_keys.public_keys[0]));
    provide_context(program);
    let tx_params = TxParams::load_from_storage().unwrap_or_default();
    let tx_env = TxEnv::new(program, tx_params);
    provide_context(tx_env);
    provide_context(SignedData::new(tx_env.lazy_env));
    provide_context(HashedData::load_from_storage().unwrap_or_default());
    provide_context(Runtime::new(program, tx_env.lazy_env));
    provide_context(ActiveRunTab::default());

    view! {
        <ProgramWindow />
        <RunWindow />
        <Footer />
    }
}
