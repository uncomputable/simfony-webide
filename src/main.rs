mod components;
mod env;
mod examples;
mod function;
mod jet;
mod util;
mod value;

pub use simfony::simplicity;

use components::App;
use leptos::*;

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}
