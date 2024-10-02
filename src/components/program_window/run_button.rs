use itertools::Itertools;
use leptos::{
    component, create_rw_signal, ev, spawn_local, use_context, view, with, IntoView, RwSignal,
    SignalGet, SignalSet, SignalUpdate,
};
use simfony::witness::WitnessValues;

use crate::components::program_window::{Program, TxEnv};
use crate::components::run_window::ExecutionOutput;
use crate::function::Runner;

#[component]
pub fn RunButton() -> impl IntoView {
    let program = use_context::<RwSignal<Program>>().expect("program exist in context");
    let witness_values =
        use_context::<RwSignal<WitnessValues>>().expect("witness values should exist in context");
    let tx_env = use_context::<TxEnv>().expect("transaction environment should exist in context");
    let environment = tx_env.environment();
    let output =
        use_context::<ExecutionOutput>().expect("execution output should exist in context");
    let run_succeeded = create_rw_signal::<Option<bool>>(None);

    let set_success = move |success: bool| {
        spawn_local(async move {
            run_succeeded.set(Some(success));
            gloo_timers::future::TimeoutFuture::new(500).await;
            run_succeeded.set(None);
        });
    };
    let run_program = move |_event: ev::MouseEvent| {
        with!(|program, witness_values, environment| {
            let satisfied_program = match program.satisfy(witness_values) {
                Ok(x) => x,
                Err(error) => {
                    output.error.set(error);
                    set_success(false);
                    return;
                }
            };
            let mut runner = Runner::for_program(satisfied_program);
            let success = match runner.run(environment) {
                Ok(..) => {
                    output.error.update(String::clear);
                    true
                }
                Err(error) => {
                    output.error.set(error.to_string());
                    false
                }
            };
            output
                .debug
                .set(runner.debug_output().into_iter().join("\n"));
            set_success(success);
        });
    };

    let button_class = move || match run_succeeded.get() {
        None => "run-button",
        Some(false) => "run-button failure",
        Some(true) => "run-button success",
    };

    view! {
        <button
            class=button_class
            on:click=run_program
        >
            <i class="fas fa-play"></i>
            " Run"
        </button>
    }
}
