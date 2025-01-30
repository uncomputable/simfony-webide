use elements::pset::serialize::Serialize;
use hex_conservative::DisplayHex;
use leptos::{component, use_context, view, with, IntoView};
use simfony::elements;

use crate::components::copy_to_clipboard::CopyToClipboard;
use crate::components::program_window::Program;
use crate::components::run_window::TxEnv;

#[component]
pub fn TransactionButton() -> impl IntoView {
    let program = use_context::<Program>().expect("program should exist in context");
    let tx_env = use_context::<TxEnv>().expect("transaction environment should exist in context");

    let transaction = move || {
        let params = tx_env.params;
        let env = tx_env.lazy_env;
        with!(|params, env| {
            let satisfied = match program.satisfied() {
                Ok(x) => x,
                Err(..) => return "Invalid program".to_string(),
            };
            let pruned = match satisfied.redeem().prune(env) {
                Ok(x) => x,
                Err(..) => return "Execution fails".to_string(),
            };
            params
                .transaction(&pruned)
                .serialize()
                .to_lower_hex_string()
        })
    };
    view! {
        <CopyToClipboard content=transaction class="button" tooltip_below=true>
            <i class="fa-solid fa-right-left"></i>
            " Transaction"
        </CopyToClipboard>
    }
}
