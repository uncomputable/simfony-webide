mod components;
mod exec;
mod instruction;
mod util;

use components::App;
use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}
