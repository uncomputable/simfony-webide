mod execution_tab;
mod hash_store_tab;
mod key_store_tab;
mod transaction_tab;

use leptos::{component, view, IntoView, RwSignal};

use self::execution_tab::ExecutionTab;
use self::hash_store_tab::HashStoreTab;
use self::key_store_tab::KeyStoreTab;
use self::transaction_tab::TransactionTab;
use crate::components::navbar::{Navbar, Tab};

pub use self::execution_tab::ExecutionOutput;
pub use self::hash_store_tab::HashedData;
pub use self::key_store_tab::{SignedData, SigningKeys};
pub use self::transaction_tab::TxEnv;

#[component]
pub fn RunWindow(active_tab: RwSignal<&'static str>) -> impl IntoView {
    view! {
        <Navbar default_tab="Execution" active_tab=active_tab>
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
