use leptos::*;

use super::merkle::Merkle;
use crate::examples;
use crate::function::Runner;
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    let (human, set_human) = create_signal("".to_string());
    let (program_success, set_program_success) = create_signal("".to_string());

    let program = Signal::derive(move || util::program_from_string(&human.get()));
    let human_error = move || program.get().err().map(|error| format!("Error: {error}"));

    let update_human = move |new_human: String| {
        set_human.set(new_human);
        set_program_success.set("".to_string());
    };
    let select_example_program = move |name: String| {
        if let Some(new_human) = examples::get_program(&name) {
            update_human(new_human.to_string());
        }
    };
    let run_program = move || {
        let program = match program.get() {
            Ok(program) => program,
            Err(_) => return,
        };
        let mut runner = Runner::for_program(program);
        match runner.run() {
            Ok(_) => {
                set_program_success.set("✅ Program success".to_string());
            }
            Err(error) => {
                set_program_success.set(format!("❌ {error}"));
            }
        }
    };

    view! {
        <h1>Simplicity Web IDE</h1>
        <p>Write and execute Simplicity programs in the browser!</p>
        <p>"The IDE uses the "<a href="https://github.com/BlockstreamResearch/rust-simplicity/blob/master/src/human_encoding/README.md">human encoding</a>" to serialize Simplicity."</p>
        <div>
            <p>
                {human_error}
            </p>
            <p>
                {program_success}
            </p>
            <select
                on:input=move |event| select_example_program(event_target_value(&event))
            >
                <option value="" disabled selected>Example programs</option>
                {
                    examples::get_names()
                        .map(|name| view! { <option value={name}>{name}</option>})
                        .collect::<Vec<_>>()
                }
            </select>
        </div>
        <textarea
            prop:value=move || human.get()
            on:input=move |event| update_human(event_target_value(&event))
            placeholder="Enter your program here"
            rows="10" cols="80"
        />
        <div>
            <button on:click=move |_| run_program()>
                "Run program"
            </button>
        </div>
        <Merkle program=program/>
    }
}
