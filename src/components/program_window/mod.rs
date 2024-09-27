mod examples_tab;
mod program_tab;
mod transaction_tab;
mod witness_tab;

use leptos::{component, view, IntoView};

use self::examples_tab::ExamplesTab;
use self::program_tab::ProgramTab;
use self::transaction_tab::TransactionTab;
use self::witness_tab::WitnessTab;
use crate::components::tabs::{Tab, Tabs};

pub use self::program_tab::Program;
pub use self::transaction_tab::TxEnv;

#[component]
pub fn ProgramWindow() -> impl IntoView {
    view! {
        <Tabs default_tab="Program">
            <Tab name="Program">
                <ProgramTab />
            </Tab>
            <Tab name="Witness">
                <WitnessTab />
            </Tab>
            <Tab name="Transaction">
                <TransactionTab />
            </Tab>
            <Tab name="Examples">
                <ExamplesTab />
            </Tab>
        </Tabs>
    }
}
