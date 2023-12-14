use std::sync::Arc;

use leptos::*;
use simplicity::jet::Elements;
use simplicity::RedeemNode;

use crate::instruction::CachedRunner;
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    let (status, set_status) = create_signal("(Idle)".to_string());
    let (program, set_program) = create_signal::<Option<Arc<RedeemNode<Elements>>>>(None);

    let update_program = move |human: String| match util::program_from_string(&human) {
        Ok(program) => {
            set_program.set(Some(program));
            set_status.set("Ready to run".to_string());
        }
        Err(error) => {
            set_program.set(None);
            set_status.set(format!("Error: {error}"));
        }
    };
    let run_program = move || {
        let program = match program.get() {
            Some(program) => program,
            None => return,
        };
        let runner = CachedRunner::for_program(program, true);
        match runner.get_success() {
            Ok(()) => {
                set_status.set("✅ Program success".to_string());
            }
            Err(error) => {
                set_status.set(format!("❌ Program failure: {error}"));
            }
        }
    };

    view! {
        <p>
            {status}
        </p>
        <textarea
            on:input=move |event| update_program(event_target_value(&event))
            placeholder="Enter your program here"
            rows="10" cols="50"
        />
        <button on:click=move |_| run_program()>
            "Run program"
        </button>
    }
}
