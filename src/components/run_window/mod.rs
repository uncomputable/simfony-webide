mod execution_tab;
mod hash_store_tab;
mod key_store_tab;
mod transaction_tab;

use crate::components::app::ActiveRunTab;
use leptos::{component, use_context, view, IntoView};

use self::execution_tab::ExecutionTab;
use self::hash_store_tab::HashStoreTab;
use self::key_store_tab::KeyStoreTab;
use self::transaction_tab::TransactionTab;
use crate::components::navbar::{Navbar, Tab};

pub use self::hash_store_tab::HashedData;
pub use self::key_store_tab::{SignedData, SigningKeys};
pub use self::transaction_tab::TxEnv;

#[component]
pub fn RunWindow() -> impl IntoView {
    let active_run_tab =
        use_context::<ActiveRunTab>().expect("active run tab should exist in context");
    view! {
        <Navbar default_tab="Execution" active_tab=active_run_tab.0>
            <Tab name="Execution">
                <ExecutionTab />
            </Tab>
            <Tab name="Transaction">
                <TransactionTab />
            </Tab>
            <Tab name="Key Store">
                <KeyStoreTab />
            </Tab>
            <Tab name="Hash Store">
                <HashStoreTab />
            </Tab>
        </Navbar>
    }
}
