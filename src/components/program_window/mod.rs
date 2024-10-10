mod examples_dropdown;
mod program_tab;
mod run_button;
mod share_button;
mod tools_dropdown;

use leptos::{component, view, IntoView};

use self::examples_dropdown::ExamplesDropdown;
use self::program_tab::ProgramTab;
use self::run_button::RunButton;
use self::share_button::ShareButton;
use crate::components::toolbar::Toolbar;

pub use self::program_tab::ProgramText;

#[component]
pub fn ProgramWindow() -> impl IntoView {
    view! {
        <Toolbar>
            <RunButton />
            <ExamplesDropdown />
            <ShareButton />
            <div class="beta-tag">beta</div>
        </Toolbar>
        <ProgramTab />
    }
}
