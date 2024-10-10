mod examples_dropdown;
mod program_tab;
mod run_button;
mod share_button;
mod tools_dropdown;

use leptos::{component, create_rw_signal, view, IntoView};

use self::examples_dropdown::ExamplesDropdown;
use self::program_tab::ProgramTab;
use self::run_button::RunButton;
use self::share_button::ShareButton;
use crate::components::navbar::{Button, Navbar, Tab};

pub use self::program_tab::ProgramText;

#[component]
pub fn ProgramWindow() -> impl IntoView {
    let active_tab = create_rw_signal("");

    view! {
        <Navbar default_tab="Program" active_tab=active_tab>
            <Button>
                <RunButton />
            </Button>
            <Tab name="Program">
                <ProgramTab />
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
