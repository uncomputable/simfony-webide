use leptos::*;

use super::analysis::Analysis;
use super::examples::{ExampleProgramDescription, SelectExampleProgram};
use super::merkle::{Merkle, reload_graph};
use super::parser::ParseError;

use crate::function::Runner;
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    let (program_str, set_program_str) = create_signal("".to_string());
    let (run_result, set_run_result) = create_signal::<Option<Result<String, String>>>(None);
    let (name, set_name) = create_signal::<Option<String>>(None);

    let program = Signal::derive(move || util::program_from_string(&program_str.get()));
    let parse_error = Signal::derive(move || program.get().err());

    let update_program_str = move |s: String| {
        set_program_str.set(s);
        set_run_result.set(None);
        set_name.set(None);
    };
    let run_program = move || {
        let program = match program.get() {
            Ok(program) => program,
            Err(_) => return,
        };
        let mut runner = Runner::for_program(program.clone());
        match runner.run() {
            Ok(_) => {
                set_run_result.set(Some(Ok("Program success".to_string())));
                reload_graph(program);
            }
            Err(error) => {
                set_run_result.set(Some(Err(error.to_string())));
            }
        }
    };

    view! {
        <div class="input-page">
            <div class="page-header">
                <img class="header-icon" src="/images/simplicity_logo.svg" />
            </div>

            <div class="container center intro">
                <h1 class="intro-title">Simfony IDE</h1>
                <p class="intro-text text-grey">
                    <a href="https://github.com/BlockstreamResearch/simfony" target="blank">Simfony</a>
                    " is a high-level language for writing Bitcoin smart contracts."
                </p>
                <p class="intro-text text-grey">
                    "Simfony looks and feels like "
                    <a href="https://www.rust-lang.org" target="blank">Rust</a>
                    ". Just how Rust compiles down to assembly language, Simfony compiles down to "
                    <a href="https://github.com/BlockstreamResearch/simplicity" target="blank">Simplicity</a>
                    " bytecode. Developers write Simfony, full nodes execute Simplicity."
                </p>
            </div>

            <div class="container">
                <ParseError maybe_error=parse_error/>

                <div class="program-input">
                    <div class="program-input-header">
                        <div class="program-input-intro">
                            <h2>Program</h2>
                            <p>Select an example program or enter your own program below.</p>
                        </div>
                        <SelectExampleProgram update_program_str=update_program_str set_name=set_name/>
                    </div>

                    <textarea class="program-input-field"
                        prop:value=move || program_str.get()
                        on:input=move |event| update_program_str(event_target_value(&event))
                        placeholder="Enter your program here"
                        rows="15" cols="80"
                        spellcheck="false"
                    />

                    <div class="flex program-input-footer">
                        <ExampleProgramDescription name=name/>

                        <div class="run-button">
                            <button on:click=move |_| run_program()>
                                "Run program"
                            </button>
                        </div>
                    </div>
                </div>

                <Analysis program=program run_result=run_result/>

                <Merkle program=program/>
            </div>
        </div>
    }
}
