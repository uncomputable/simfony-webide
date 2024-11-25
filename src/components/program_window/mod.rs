mod address_button;
mod examples_dropdown;
mod help_button;
mod program_tab;
mod run_button;
mod share_button;
mod tools_dropdown;
mod transaction_button;

use leptos::create_signal;
use leptos::{component, view, IntoView, SignalGet, SignalSet};

use self::address_button::AddressButton;
use self::examples_dropdown::ExamplesDropdown;
use self::help_button::HelpButton;
use self::program_tab::ProgramTab;
use self::run_button::RunButton;
use self::share_button::ShareButton;
use self::transaction_button::TransactionButton;
use crate::components::toolbar::Toolbar;

pub use self::examples_dropdown::select_example;
pub use self::program_tab::{Program, Runtime};

#[component]
pub fn ProgramWindow() -> impl IntoView {
    let (mobile_open, set_mobile_open) = create_signal(false);

    view! {
        <Toolbar>
            <RunButton />
            <ExamplesDropdown />

            <div class="mobile-hidden"  class:open = move || mobile_open.get() >
                <AddressButton />
                <TransactionButton />
                <ShareButton />
                <div class="beta-tag">beta</div>
            </div>

            <HelpButton />

            {move || if !mobile_open.get() {
                view! { <i class="fa-solid fa-bars hamburger" on:click=move |_| set_mobile_open.set(true)></i>}
            } else {
                view! { <i class="fa-solid fa-xmark hamburger-close" on:click=move |_| set_mobile_open.set(false)></i> }
            }}
        </Toolbar>
        <ProgramTab />
    }
}
