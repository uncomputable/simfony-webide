mod examples_dropdown;
mod examples_tab;
mod program_tab;
mod run_button;
mod tools_dropdown;
mod transaction_tab;
mod witness_tab;

use leptos::{component, view, IntoView};

use self::program_tab::ProgramTab;
use self::transaction_tab::TransactionTab;
use self::witness_tab::WitnessTab;
use crate::components::navbar::{Button, Navbar, Tab};
use crate::components::program_window::examples_dropdown::ExamplesDropdown;
use crate::components::program_window::run_button::RunButton;
use crate::components::program_window::tools_dropdown::ToolsDropdown;
use crate::components::state::ShareButton;

pub use self::program_tab::Program;
pub use self::transaction_tab::TxEnv;

#[component]
pub fn ProgramWindow() -> impl IntoView {
    view! {
        <Navbar default_tab="Program">
            <Button>
                <RunButton />
            </Button>
            <Tab name="Program">
                <ProgramTab />
            </Tab>
            <Tab name="Witness">
                <WitnessTab />
            </Tab>
            <Tab name="Transaction">
                <TransactionTab />
            </Tab>
            <Button>
                <ExamplesDropdown />
            </Button>
            <Button>
                <ToolsDropdown />
            </Button>
            <Button>
                <ShareButton />
            </Button>
        </Navbar>
    }
}
