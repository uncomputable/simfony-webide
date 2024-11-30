use leptos::{component, ev, use_context, view, IntoView, SignalGet};

use crate::components::program_window::{Program, Runtime};
use crate::components::state::update_local_storage;

#[component]
pub fn RunButton() -> impl IntoView {
    let program = use_context::<Program>().expect("program should exist in context");
    let runtime = use_context::<Runtime>().expect("runtime should exist in context");
    let audio_ref = runtime.alarm_audio_ref;

    let run_program = move |_event: ev::MouseEvent| {
        program.add_default_modules();
        update_local_storage();
        runtime.run();
    };
    let button_class = move || match runtime.run_succeeded.get() {
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
