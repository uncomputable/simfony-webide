use leptos::{
    component, create_rw_signal, ev, spawn_local, use_context, view, Fragment, IntoView, SignalGet,
    SignalSet, SignalUpdate,
};

use crate::components::app::{ProgramWrapper, WitnessWrapper};
use crate::components::error::ErrorBox;
use crate::function::Runner;

#[component]
pub fn RuntimeTab() -> impl IntoView {
    let program_text = use_context::<ProgramWrapper>().expect("program should exist in context");
    let witness_values = use_context::<WitnessWrapper>().expect("witness should exist in context");
    let run_error = create_rw_signal("".to_string());
    let run_succeeded = create_rw_signal::<Option<bool>>(None);

    let run_program = move |_event: ev::MouseEvent| {
        // TODO: Store simfony::witness::WitnessValues in witness signal
        let mut witness_values1 = simfony::witness::WitnessValues::empty();
        for (name, value) in witness_values.0.get() {
            witness_values1
                .insert(name, value)
                .expect("Same name cannot be assigned to two values");
        }

        // TODO: Store unsatisfied but compiled Simfony program in program signal
        let satisfied_program =
            match simfony::satisfy(program_text.0.get().as_str(), &witness_values1) {
                Ok(x) => x,
                Err(error) => {
                    run_error.set(error);
                    return;
                }
            };
        let mut runner = Runner::for_program(satisfied_program);
        let success = match runner.run() {
            Ok(..) => {
                run_error.update(String::clear);
                true
            }
            Err(error) => {
                run_error.set(error.to_string());
                false
            }
        };
        spawn_local(async move {
            run_succeeded.set(Some(success));
            gloo_timers::future::TimeoutFuture::new(500).await;
            run_succeeded.set(None);
        });
    };

    let run_button_view = move || -> Fragment {
        match run_succeeded.get() {
            None => view! {
                Run program
                <i class="fas fa-play"></i>
            },
            Some(true) => view! {
                Success
                <i class="fas fa-check"></i>
            },
            Some(false) => view! {
                Failure
                <i class="fas fa-times"></i>
            },
        }
    };

    view! {
        <div>
            <ErrorBox error=run_error />
            <button
                class="submit-button"
                on:click=run_program
            >
                {run_button_view}
            </button>
        </div>
    }
}
