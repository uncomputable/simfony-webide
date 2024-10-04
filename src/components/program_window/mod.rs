mod examples_dropdown;
mod program_tab;
mod run_button;
mod share_button;
mod tools_dropdown;
mod transaction_tab;
mod witness_tab;

use leptos::{component, view, IntoView};

use self::examples_dropdown::ExamplesDropdown;
use self::program_tab::ProgramTab;
use self::run_button::RunButton;
use self::share_button::ShareButton;
use self::tools_dropdown::ToolsDropdown;
use self::transaction_tab::TransactionTab;
use self::witness_tab::WitnessTab;
use crate::components::navbar::{Button, Navbar, Tab};

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

            <Button>
                <div class="beta-tag">beta</div>
            </Button>
        </Navbar>
    }
}
