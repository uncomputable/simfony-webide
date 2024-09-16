use leptos::*;
use wasm_bindgen::prelude::*;

use super::analysis::Analysis;
use super::examples::{ExampleProgramDescription, SelectExampleProgram};
use super::merkle::{self, MerkleExplorer};
use super::parser::ParseError;

use crate::function::Runner;
use crate::util;

#[wasm_bindgen(module = "/src/assets/js/button_effects.js")]
extern "C" {
    fn button_success_animation();
    fn button_fail_animation();
}

#[wasm_bindgen(module = "/src/assets/js/copy_button.js")]
extern "C" {
    fn copy_program(text: &str);
}

#[wasm_bindgen(module = "/src/assets/js/badger.js")]
extern "C" {
    fn load_badger();
    fn laser_eyes();
    fn hide_badger(val: bool);
    fn hide_badger_timed();
}

#[component]
pub fn App() -> impl IntoView {
    create_effect(move |_| {
        load_badger();
    });

    let (program_str, set_program_str) = create_signal("".to_string());
    let (run_result, set_run_result) = create_signal::<Option<Result<String, String>>>(None);
    let (is_running, set_is_running) = create_signal(false);
    let (name, set_name) = create_signal::<Option<String>>(None);
    let (graph_toggle, set_graph_toggle) = create_signal(false);

    let program_result = Signal::derive(move || util::program_from_string(&program_str.get()));
    let program = Signal::derive(move || program_result.get().ok().map(|x| x.simplicity));
    let parse_error = Signal::derive(move || program_result.get().err());

    create_effect(move |_| match parse_error.get() {
        Some(_) => hide_badger(true),
        None => hide_badger(false),
    });

    let update_program_str = move |s: String| {
        set_program_str.set(s);
        set_run_result.set(None);
    };
    let run_program = move || {
        let program = match program_result.get() {
            Ok(program) => program,
            Err(..) => return,
        };
        set_is_running.set(true);
        let mut runner = Runner::for_program(program.simplicity.clone());
        match runner.run() {
            Ok(_) => {
                set_run_result.set(Some(Ok("Program success".to_string())));
                laser_eyes();
                button_success_animation();
                merkle::reload_graph(program.simplicity);
                set_is_running.set(false);
            }
            Err(error) => {
                hide_badger_timed();
                set_run_result.set(Some(Err(error.to_string())));
                button_fail_animation();
                set_is_running.set(false);
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
                        <canvas id="badger-canvas" width="3200" height="3200" style="width: 1600px; height: 1600px;"></canvas>
                        <div class="program-input-intro">
                            <h2>Program</h2>
                            <p>Select an example program or enter your own program below.</p>
                        </div>
                        <SelectExampleProgram update_program_str=update_program_str set_name=set_name/>
                    </div>

                    <div class="program-input-field-container">
                        <div class="copy-holder">
                            <span id="copy-button-success">Program copied</span>
                            <span class="copy-button" on:click=move |_| copy_program(&program_str.get())><i class="far fa-copy"></i></span>
                        </div>
                        <textarea class="program-input-field"
                            on:keydown=move |event: web_sys::KeyboardEvent| {
                                if event.ctrl_key() && event.key_code() == 13 { // 13 is the Enter key
                                    run_program();
                                }
                            }
                            prop:value=move || program_str.get()
                            on:input=move |event| update_program_str(event_target_value(&event))
                            placeholder="Enter your program here"
                            rows="15" cols="80"
                            spellcheck="false"
                        />
                    </div>

                    <div class="flex program-input-footer">
                        <ExampleProgramDescription name=name/>
                        <div class="run-button">
                            <button
                                on:click=move |_| run_program()
                                disabled=move || parse_error.get().is_some() | is_running.get()
                            >
                                "Run program"
                            </button>
                        </div>
                    </div>
                </div>

                <Analysis
                    program=program
                    run_result=run_result/>
                <MerkleExplorer
                    run_result=run_result
                    graph_toggle=graph_toggle
                    set_graph_toggle=set_graph_toggle/>
            </div>
        </div>
    }
}
