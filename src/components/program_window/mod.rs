mod examples_tab;
mod program_tab;
mod transaction_tab;
mod witness_tab;

use leptos::{component, view, IntoView};

use self::program_tab::ProgramTab;
use self::transaction_tab::TransactionTab;
use self::witness_tab::WitnessTab;
use crate::components::navbar::{Action, Dropdown, Navbar, Navigation};
use crate::components::program_window::examples_tab::ExamplesTab;

pub use self::program_tab::Program;
pub use self::transaction_tab::TxEnv;

#[component]
pub fn ProgramWindow() -> impl IntoView {
    let dummy_action = move |_: ()| leptos::logging::log!("It works!");
    let dummy_select = move |name| {
        leptos::logging::log!("You selected {name}!");
    };

    let examples = crate::examples::keys().collect::<Vec<_>>();
    let tools = ["🔑️ Key Store", "#️⃣ Hash Store"];

    view! {
        <Navbar default_tab="Program">
            <Action action=dummy_action>
                <i class="fas fa-play"></i>
                " Run (WIP)"
            </Action>
            <Navigation name="Program">
                <ProgramTab />
            </Navigation>
            <Navigation name="Witness">
                <WitnessTab />
            </Navigation>
            <Navigation name="Transaction">
                <TransactionTab />
            </Navigation>
            <Navigation name="Examples">
                <ExamplesTab />
            </Navigation>
            <Dropdown name="Examples (WIP)" options=examples select_option=dummy_select />
            <Dropdown name="Tools (WIP)" options=tools select_option=dummy_select />
            <Action action=dummy_action>
                <i class="fa-solid fa-share-nodes"></i>
                " Share (WIP)"
            </Action>
        </Navbar>
    }
}
