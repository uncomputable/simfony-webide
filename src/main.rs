mod components;
mod examples;
mod function;
mod util;
mod value;

use components::App;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}
