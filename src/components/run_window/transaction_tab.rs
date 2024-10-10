use leptos::{
    component, create_node_ref, create_rw_signal, ev, html, use_context, view, IntoView, NodeRef,
    RwSignal, Signal, SignalGet, SignalGetUntracked, SignalSet,
};
use simfony::{elements, simplicity};
use std::sync::Arc;

use crate::components::apply_changes::ApplyChanges;
use crate::components::string_box::ErrorBox;

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
    let parse_error = create_rw_signal("".to_string());
    let apply_changes = ApplyChanges::default();

    let lock_time_ref: NodeRef<html::Input> = create_node_ref();
    let sequence_ref: NodeRef<html::Input> = create_node_ref();

    let submit_transaction = move |event: ev::SubmitEvent| {
        event.prevent_default(); // stop page from reloading
        let lock_time_string = lock_time_ref
            .get()
            .expect("<input> should be mounted")
            .value();
        let lock_time = match lock_time_string.parse::<u32>() {
            Ok(x) => elements::LockTime::from_consensus(x),
            Err(error) => {
                parse_error.set(format!(
                    "Malformed nLockTime: `{lock_time_string}`: `{error}`"
                ));
                return;
            }
        };
        tx_env.lock_time.set(lock_time);

        let sequence_string = sequence_ref
            .get()
            .expect("<input> should be mounted")
            .value();
        let sequence = match sequence_string.parse::<u32>() {
            Ok(x) => elements::Sequence::from_consensus(x),
            Err(error) => {
                parse_error.set(format!(
                    "Malformed nSequence: `{lock_time_string}`: `{error}`"
                ));
                return;
            }
        };
        tx_env.sequence.set(sequence);

        apply_changes.set_success(true);
    };

    let lock_time_initial_value = tx_env.lock_time.get_untracked().to_string();
    let sequence_initial_value = tx_env.sequence.get_untracked().to_string();

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
            <form on:submit=submit_transaction>
                <div>
                    <div class="transaction-display-row">
                        <div class="display-row-label">"nLockTime (u32)"</div>
                        <input
                            class="input"
                            type="text"
                            inputmode="numeric"
                            pattern="\\d*"
                            value=lock_time_initial_value
                            node_ref=lock_time_ref
                        />
                    </div>
                    <div class="transaction-display-row">
                        <div class="display-row-label">"nSequence (u32)"</div>
                        <input
                            class="input"
                            type="text"
                            inputmode="numeric"
                            pattern="\\d*"
                            value=sequence_initial_value
                            node_ref=sequence_ref
                        />
                    </div>
                </div>
                <ErrorBox error=parse_error />
                <div class="transaction-tab-apply-button">
                    {apply_changes}
                </div>
            </form>
        </div>
    }
}
