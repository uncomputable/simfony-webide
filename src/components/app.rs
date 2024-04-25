use leptos::*;

use super::analysis::Analysis;
use super::examples::{ExampleProgramDescription, SelectExampleProgram};
use super::merkle::Merkle;

use crate::function::Runner;
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    let (human, set_human) = create_signal("".to_string());
    let (program_success, set_program_success) = create_signal("".to_string());
    let (name, set_name) = create_signal::<Option<String>>(None);

    let program = Signal::derive(move || util::program_from_string(&human.get()));
    let human_error = move || program.get().err().map(|error| format!("Error: {error}"));

    let update_human = move |new_human: String| {
        set_human.set(new_human);
        set_program_success.set("".to_string());
        set_name.set(None);
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
        <h1>Simfony Web IDE</h1>
        <p>
            <a href="https://github.com/BlockstreamResearch/simfony">Simfony</a>
            " is a high-level language for writing Bitcoin smart contracts."
        </p>
        <p>
            "Simfony looks and feels like "
            <a href="https://www.rust-lang.org">Rust</a>
            ". Just how Rust compiles down to assembly language, Simfony compiles down to "
            <a href="https://github.com/BlockstreamResearch/simplicity">Simplicity</a>
            " bytecode. Developers write Simfony, full nodes execute Simplicity."
        </p>
        <div>
            <p>
                {human_error}
            </p>
            <p>
                {program_success}
            </p>
            <SelectExampleProgram update_human=update_human set_name=set_name/>
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
        <ExampleProgramDescription name=name/>
        <Analysis program=program/>
        <Merkle program=program/>
    }
}
