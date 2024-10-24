use crate::components::copy_to_clipboard::CopyToClipboard;
use crate::function::Runner;
use itertools::Itertools;
use leptos::{
    component, create_rw_signal, ev, event_target_value, html, spawn_local, use_context, view,
    with, IntoView, NodeRef, RwSignal, Signal, SignalGetUntracked, SignalSet, SignalUpdate,
    SignalWith,
};
use simfony::parse::ParseFromStr;
use simfony::simplicity::jet::elements::ElementsEnv;
use simfony::{elements, simplicity};
use simfony::{CompiledProgram, SatisfiedProgram, WitnessValues};
use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub struct Program {
    pub text: RwSignal<String>,
    cached_text: RwSignal<String>,
    pub lazy_cmr: RwSignal<Result<simplicity::Cmr, String>>,
    lazy_satisfied: RwSignal<Result<SatisfiedProgram, String>>,
}

impl Program {
    pub fn new(program_text: String) -> Self {
        Self {
            text: create_rw_signal(program_text),
            cached_text: create_rw_signal("".to_string()),
            lazy_cmr: create_rw_signal(Err("".to_string())),
            lazy_satisfied: create_rw_signal(Err("".to_string())),
        }
    }

    pub fn cmr(self) -> Result<simplicity::Cmr, String> {
        self.update_on_read();
        self.lazy_cmr.get_untracked()
    }

    pub fn satisfied(self) -> Result<SatisfiedProgram, String> {
        self.update_on_read();
        self.lazy_satisfied.get_untracked()
    }

    pub fn update_on_read(self) {
        let text = self.text;
        let cached_text = self.cached_text;
        let needs_update = with!(|text, cached_text| { text != cached_text });
        if !needs_update {
            return;
        }
        with!(|text| {
            self.cached_text.set(text.clone());
            let compiled = CompiledProgram::new(text);
            self.lazy_cmr
                .set(compiled.clone().map(|x| x.commit().cmr()));
            self.lazy_satisfied.set(compiled.and_then(|x| {
                let witness = WitnessValues::parse_from_str(text)?;
                x.satisfy(&witness)
            }));
        });
    }
}

impl Default for Program {
    fn default() -> Self {
        let text = crate::examples::get("✍️️ P2PK")
            .expect("P2PK example should exist")
            .program_text();
        Self::new(text.to_string())
    }
}

#[derive(Copy, Clone)]
pub struct Runtime {
    program: Program,
    env: Signal<ElementsEnv<Arc<elements::Transaction>>>,
    pub run_succeeded: RwSignal<Option<bool>>,
    pub debug_output: RwSignal<String>,
    pub error_output: RwSignal<String>,
    // This node ref needs to be mounted somewhere in order to work.
    pub alarm_audio_ref: NodeRef<html::Audio>,
}

impl Runtime {
    pub fn new(program: Program, env: Signal<ElementsEnv<Arc<elements::Transaction>>>) -> Self {
        Self {
            program,
            env,
            run_succeeded: Default::default(),
            debug_output: Default::default(),
            error_output: Default::default(),
            alarm_audio_ref: Default::default(),
        }
    }

    fn set_success(self, success: bool) {
        web_sys::window()
            .as_ref()
            .map(web_sys::Window::navigator)
            .map(|navigator| match success {
                true => navigator.vibrate_with_duration(200),
                false => navigator.vibrate_with_duration(500),
            });
        if !success {
            self.alarm_audio_ref.get().map(|audio| audio.play());
        }
        spawn_local(async move {
            self.run_succeeded.set(Some(success));
            gloo_timers::future::TimeoutFuture::new(500).await;
            self.run_succeeded.set(None);
        });
    }

    pub fn run(self) {
        let satisfied_program = match self.program.satisfied() {
            Ok(x) => x,
            Err(error) => {
                self.error_output.set(error);
                self.set_success(false);
                return;
            }
        };
        let mut runner = Runner::for_program(satisfied_program);
        let success = self.env.with(|env| match runner.run(env) {
            Ok(..) => {
                self.error_output.update(String::clear);
                true
            }
            Err(error) => {
                self.error_output.set(error.to_string());
                false
            }
        });
        self.debug_output
            .set(runner.debug_output().into_iter().join("\n"));
        self.set_success(success);
    }
}

#[component]
pub fn ProgramTab() -> impl IntoView {
    let program = use_context::<Program>().expect("program should exist in context");

    let update_program_text = move |event: ev::Event| {
        program.text.set(event_target_value(&event));
    };
    let program_text_initial_value = program.text.get_untracked();

    view! {
        <div class="tab-content">
            <div class="copy-program">
                <CopyToClipboard content=program.text class="copy-button" tooltip_below=true>
                    <i class="far fa-copy"></i>
                </CopyToClipboard>
            </div>
            <textarea
                class="program-input-field"
                placeholder="Enter your program here"
                rows="25"
                cols="80"
                spellcheck="false"
                prop:value=program.text
                on:input=update_program_text
            >
                {program_text_initial_value}
            </textarea>
        </div>
    }
}
