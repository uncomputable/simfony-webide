use leptos::{component, view, IntoView};

use crate::components::dropdown::Dropdown;

#[component]
pub fn ToolsDropdown() -> impl IntoView {
    let tools = ["🔑️ Key Store", "#️⃣ Hash Store"];
    let select_tool = move |selected| leptos::logging::log!("You selected {selected}!");

    view! {
        <Dropdown name="Tools" options=tools select_option=select_tool />
    }
}
