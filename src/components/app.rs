use leptos::*;

use super::analysis::Analysis;
use super::examples::{ExampleProgramDescription, SelectExampleProgram};
use super::merkle::Merkle;

use crate::function::Runner;
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    let (program_str, set_program_str) = create_signal("".to_string());
    let (run_success, set_run_success) = create_signal("".to_string());
    let (name, set_name) = create_signal::<Option<String>>(None);

    let program = Signal::derive(move || util::program_from_string(&program_str.get()));
    let parse_error = move || program.get().err();

    let update_program_str = move |s: String| {
        set_program_str.set(s);
        set_run_success.set("".to_string());
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
                set_run_success.set("✅ Program success".to_string());
            }
            Err(error) => {
                set_run_success.set(format!("❌ {error}"));
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
            <pre>
                {parse_error}
            </pre>
            <p>
                {run_success}
            </p>
            <SelectExampleProgram update_program_str=update_program_str set_name=set_name/>
        </div>
        <textarea
            prop:value=move || program_str.get()
            on:input=move |event| update_program_str(event_target_value(&event))
            placeholder="Enter your program here"
            rows="15" cols="80"
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
