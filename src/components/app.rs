use leptos::*;

use super::analysis::Analysis;
use super::examples::{ExampleProgramDescription, SelectExampleProgram};
use super::merkle::Merkle;

use crate::function::Runner;
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    let (human, set_human) = create_signal("".to_string());
    let (name, set_name) = create_signal::<Option<String>>(None);

    let (program_success, set_program_success) = create_signal(false);
    let (program_status_message, set_program_status_message) = create_signal("".to_string());

    let program = Signal::derive(move || util::program_from_string(&human.get()));
    let human_error = move || program.get().err().map(|error: String| format!("Error: {error}"));

    let update_human = move |new_human: String| {
        set_human.set(new_human);
        set_program_status_message.set("".to_string());
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
                set_program_status_message.set("Program success".to_string());
                set_program_success.set(true);
            }
            Err(error) => {
                set_program_status_message.set(format!("{error}"));
                set_program_success.set(false);
            }
        }
    };

    view! {
        <div class="input-page">
            <div class="page-header">
                <img class="header-icon" src="https://design.blockstream.com/assets/pages/simplicity/simplicity_logo_white_on_transparent_rgb.svg" />
            </div>

            <div class="container center">
                <h1>Simfony Plaground:<br /> Simplicity Frontend</h1>
                <p class="text-grey">Write and execute Simplicity programs in the browser!<br />
                "The IDE uses the "<a href="https://github.com/BlockstreamResearch/rust-simplicity/blob/master/src/human_encoding/README.md" target="blank">human encoding</a>" to serialize Simplicity."</p>
            </div>

            <div class="container">
                <div>
                    {human_error}
                </div>

                <div class="program-input">
                    <div class="program-input-header">
                        <div class="program-input-intro">
                            <h2>Program</h2>
                            <p>Select a program, upload a json, or enter your own program below.</p>
                        </div>
                        <SelectExampleProgram update_human=update_human set_name=set_name/>
                    </div>

                    <textarea class="program-input-field"
                        prop:value=move || human.get()
                        on:input=move |event| update_human(event_target_value(&event))
                        placeholder="Enter your program here"
                        rows="10" cols="80"
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

                <Analysis program=program program_success=program_success program_status_message=program_status_message />
                
                <Merkle program=program/>
            </div>
        </div>
    }
}
