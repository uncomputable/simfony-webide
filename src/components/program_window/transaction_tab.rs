use leptos::{
    component, create_node_ref, create_rw_signal, ev, html, use_context, view, IntoView, NodeRef,
    RwSignal, Signal, SignalGet, SignalGetUntracked, SignalSet,
};
use simfony::{elements, simplicity};
use std::sync::Arc;

use crate::components::apply_changes::ApplyChanges;

#[derive(Copy, Clone, Debug)]
pub struct TxEnv {
    pub lock_time: RwSignal<elements::LockTime>,
    pub sequence: RwSignal<elements::Sequence>,
}

impl Default for TxEnv {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl TxEnv {
    pub fn new(lock_time: u32, sequence: u32) -> Self {
        Self {
            lock_time: create_rw_signal(elements::LockTime::from_consensus(lock_time)),
            sequence: create_rw_signal(elements::Sequence::from_consensus(sequence)),
        }
    }

    pub fn environment(
        self,
    ) -> Signal<simplicity::jet::elements::ElementsEnv<Arc<elements::Transaction>>> {
        Signal::derive(move || {
            simfony::dummy_env::dummy_with(self.lock_time.get(), self.sequence.get())
        })
    }
}

#[component]
pub fn TransactionTab() -> impl IntoView {
    let tx_env = use_context::<TxEnv>().expect("tx environment should exist in context");
    let apply_changes = ApplyChanges::default();

    let lock_time_ref: NodeRef<html::Input> = create_node_ref();
    let sequence_ref: NodeRef<html::Input> = create_node_ref();

    let submit_transaction = move |event: ev::SubmitEvent| {
        event.prevent_default(); // stop page from reloading
        let lock_time = lock_time_ref
            .get()
            .expect("<input> should be mounted")
            .value()
            .parse::<u32>()
            .expect("<input> should be valid u32");
        let lock_time = elements::LockTime::from_consensus(lock_time);
        tx_env.lock_time.set(lock_time);

        let sequence = sequence_ref
            .get()
            .expect("<input> should be mounted")
            .value()
            .parse::<u32>()
            .expect("<input> should be valid u32");
        let sequence = elements::Sequence::from_consensus(sequence);
        tx_env.sequence.set(sequence);

        apply_changes.set_success(true);
    };

    let lock_time_initial_value = tx_env.lock_time.get_untracked().to_string();
    let sequence_initial_value = tx_env.sequence.get_untracked().to_string();

    view! {
        <div class="tab-content">
            <h3 class="program-title">
                Transaction Environment
            </h3>
            <p>
                "Currently, the runtime uses a "
                <a href="https://github.com/BlockstreamResearch/simfony/blob/master/src/dummy_env.rs">
                    dummy transaction environment
                </a>
                ". Only the lock time and sequence number can be changed. "
                "More customization will follow in future updates."
            </p>
            <form on:submit=submit_transaction>
                <div class="button-col">
                    <label>
                        nLockTime
                        <input
                            type="number"
                            min=0
                            max=2147483647
                            value=lock_time_initial_value
                            node_ref=lock_time_ref
                        />
                    </label>
                    <label>
                        nSequence
                        <input
                            type="number"
                            min=0
                            max=2147483647
                            value=sequence_initial_value
                            node_ref=sequence_ref
                        />
                    </label>
                </div>
                {apply_changes}
            </form>
        </div>
    }
}
