use elements::pset::serialize::Serialize;
use hex_conservative::DisplayHex;
use leptos::{component, use_context, view, IntoView, SignalWith};
use simfony::elements;

use crate::components::copy_to_clipboard::CopyToClipboard;
use crate::components::program_window::Program;
use crate::components::run_window::TxEnv;

#[component]
pub fn TransactionButton() -> impl IntoView {
    let program = use_context::<Program>().expect("program should exist in context");
    let tx_env = use_context::<TxEnv>().expect("transaction environment should exist in context");

    let transaction = move || {
        tx_env.params.with(|params| match program.satisfied() {
            Ok(satisfied) => params
                .transaction(&satisfied)
                .serialize()
                .to_lower_hex_string(),
            Err(..) => "Invalid program".to_string(),
        })
    };
    view! {
        <CopyToClipboard content=transaction class="button" tooltip_below=true>
            <i class="fa-solid fa-right-left"></i>
            " Transaction"
        </CopyToClipboard>
    }
}
