mod examples_dropdown;
mod program_tab;
mod run_button;
mod share_button;
mod tools_dropdown;
mod transaction_tab;

use leptos::{component, view, IntoView};

use self::examples_dropdown::ExamplesDropdown;
use self::program_tab::ProgramTab;
use self::run_button::RunButton;
use self::share_button::ShareButton;
use self::transaction_tab::TransactionTab;
use crate::components::navbar::{Button, Navbar, Tab};

pub use self::program_tab::ProgramText;
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
            <Tab name="Transaction">
                <TransactionTab />
            </Tab>
            <Button>
                <ExamplesDropdown />
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
