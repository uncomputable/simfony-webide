use itertools::Itertools;
use leptos::{
    component, create_node_ref, create_rw_signal, ev, html, spawn_local, use_context, view,
    IntoView, SignalGet, SignalSet, SignalUpdate, SignalWith,
};

use crate::components::program_window::Program;
use crate::components::run_window::{ExecutionOutput, TxEnv};
use crate::function::Runner;

#[component]
pub fn RunButton() -> impl IntoView {
    let program = use_context::<Program>().expect("program should exist in context");
    let tx_env = use_context::<TxEnv>().expect("transaction environment should exist in context");
    let output =
        use_context::<ExecutionOutput>().expect("execution output should exist in context");
    let run_succeeded = create_rw_signal::<Option<bool>>(None);

    let audio_ref = create_node_ref::<html::Audio>();

    let set_success = move |success: bool| {
        web_sys::window()
            .as_ref()
            .map(web_sys::Window::navigator)
            .map(|navigator| match success {
                true => navigator.vibrate_with_duration(200),
                false => navigator.vibrate_with_duration(500),
            });
        spawn_local(async move {
            run_succeeded.set(Some(success));
            gloo_timers::future::TimeoutFuture::new(500).await;
            run_succeeded.set(None);
        });
    };
    let run_program = move |_event: ev::MouseEvent| {
        let satisfied_program = match program.satisfied() {
            Ok(x) => x,
            Err(error) => {
                output.error.set(error);
                set_success(false);
                return;
            }
        };
        let mut runner = Runner::for_program(satisfied_program);
        let success = tx_env.lazy_env.with(|env| match runner.run(env) {
            Ok(..) => {
                output.error.update(String::clear);
                true
            }
            Err(error) => {
                output.error.set(error.to_string());
                let _promise = audio_ref.get().expect("<audio> should be mounted").play();
                false
            }
        });
        output
            .debug
            .set(runner.debug_output().into_iter().join("\n"));
        set_success(success);
    };

    let button_class = move || match run_succeeded.get() {
        None => "button run-button",
        Some(false) => "button run-button failure",
        Some(true) => "button run-button success",
    };

    view! {
        <button
            class=button_class
            on:click=run_program
        >
            <i class="fas fa-play"></i>
            " Run"
        </button>
        <audio
            preload="auto"
            node_ref=audio_ref
        >
          <source src="images/alarm.ogg" type="audio/ogg" />
        </audio>
    }
}
