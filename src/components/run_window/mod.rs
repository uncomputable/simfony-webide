mod execution_tab;
mod hash_store_tab;
mod key_store_tab;

use leptos::{component, view, IntoView};

use self::execution_tab::ExecutionTab;
use self::hash_store_tab::HashStoreTab;
use self::key_store_tab::KeyStoreTab;
use crate::components::navbar::{Navbar, Tab};

pub use self::execution_tab::ExecutionOutput;
pub use self::hash_store_tab::HashedData;
pub use self::key_store_tab::{SignedData, SigningKeys};

#[component]
pub fn RunWindow() -> impl IntoView {
    view! {
        <Navbar default_tab="Execution">
            <Tab name="Execution">
                <ExecutionTab />
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
