mod address_button;
mod examples_dropdown;
mod program_tab;
mod run_button;
mod share_button;
mod tools_dropdown;

use leptos::{component, view, IntoView};

use self::address_button::AddressButton;
use self::examples_dropdown::ExamplesDropdown;
use self::program_tab::ProgramTab;
use self::run_button::RunButton;
use self::share_button::ShareButton;
use crate::components::toolbar::Toolbar;

pub use self::program_tab::Program;

#[component]
pub fn ProgramWindow() -> impl IntoView {
    view! {
        <Toolbar>
            <RunButton />
            <ExamplesDropdown />
            <AddressButton />
            <ShareButton />
            <div class="beta-tag">beta</div>
        </Toolbar>
        <ProgramTab />
    }
}
