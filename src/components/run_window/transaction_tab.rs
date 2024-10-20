use std::sync::Arc;

use leptos::{
    component, create_rw_signal, ev, event_target_value, use_context, view, with, IntoView,
    RwSignal, Signal, SignalGetUntracked, SignalSet, SignalUpdate,
};
use simfony::{elements, simplicity};
use simplicity::jet::elements::ElementsEnv;

use crate::components::program_window::Program;
use crate::components::string_box::ErrorBox;
use crate::transaction::TxParams;

#[derive(Copy, Clone, Debug)]
pub struct TxEnv {
    pub params: RwSignal<TxParams>,
    pub lazy_env: Signal<ElementsEnv<Arc<elements::Transaction>>>,
}

impl TxEnv {
    pub fn new(program: Program) -> Self {
        let params = create_rw_signal(TxParams::default());
        let lazy_cmr = program.lazy_cmr;
        let lazy_env = Signal::derive(move || {
            with!(|params, lazy_cmr| match lazy_cmr {
                Ok(cmr) => params.tx_env(*cmr),
                Err(..) => params.tx_env(simplicity::Cmr::unit()),
            })
        });
        Self { params, lazy_env }
    }
}

#[component]
pub fn TransactionTab() -> impl IntoView {
    let tx_env = use_context::<TxEnv>().expect("transaction environment should exist in context");
    let lock_time_parse_error = create_rw_signal("".to_string());
    let sequence_parse_error = create_rw_signal("".to_string());

    let update_lock_time = move |e: ev::Event| match event_target_value(&e).parse::<u32>() {
        Ok(lock_time) => {
            let lock_time = elements::LockTime::from_consensus(lock_time);
            tx_env.params.update(|x| x.lock_time = lock_time);
            lock_time_parse_error.update(String::clear);
        }
        Err(error) => lock_time_parse_error.set(error.to_string()),
    };
    let update_sequence = move |e: ev::Event| match event_target_value(&e).parse::<u32>() {
        Ok(sequence) => {
            let sequence = elements::Sequence::from_consensus(sequence);
            tx_env.params.update(|x| x.sequence = sequence);
            sequence_parse_error.update(String::clear);
        }
        Err(error) => sequence_parse_error.set(error.to_string()),
    };

    view! {
        <div class="tab-content transaction-tab">
            <h3 class="tab-title">
                Transaction Environment
            </h3>
            <p class="tab-description">
                "Currently, the runtime uses a "
                <a href="https://github.com/BlockstreamResearch/simfony/blob/master/src/dummy_env.rs">
                    dummy transaction environment</a>.
                " Only the lock time and sequence number can be changed. "
                "More customization will follow in future updates."
            </p>
            <div>
                <div class="transaction-display-row">
                    <div class="display-row-label">"nLockTime"</div>
                    <input
                        class="input"
                        type="number"
                        on:input=update_lock_time
                        min=0
                        value=tx_env.params.get_untracked().lock_time.to_string()
                    />
                </div>
                <ErrorBox error=lock_time_parse_error />
                <div class="transaction-display-row">
                    <div class="display-row-label">"nSequence"</div>
                    <input
                        class="input"
                        type="number"
                        on:input=update_sequence
                        min=0
                        value=tx_env.params.get_untracked().sequence.to_string()
                    />
                </div>
                <ErrorBox error=lock_time_parse_error />
            </div>
        </div>
    }
}
