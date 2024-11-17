use std::sync::Arc;

use hex_conservative::DisplayHex;
use itertools::Itertools;
use leptos::{
    component, create_node_ref, create_rw_signal, ev, event_target_value, html, spawn_local,
    use_context, view, IntoView, NodeRef, RwSignal, Signal, SignalGetUntracked, SignalSet,
    SignalUpdate, SignalWith, SignalWithUntracked,
};
use simfony::elements::secp256k1_zkp as secp256k1;
use simfony::parse::ParseFromStr;
use simfony::simplicity::jet::elements::ElementsEnv;
use simfony::{elements, simplicity};
use simfony::{CompiledProgram, SatisfiedProgram, WitnessValues};

use crate::components::copy_to_clipboard::CopyToClipboard;
use crate::function::Runner;

#[derive(Copy, Clone, Debug)]
pub struct Program {
    pub text: RwSignal<String>,
    cached_text: RwSignal<String>,
    pub lazy_cmr: RwSignal<Result<simplicity::Cmr, String>>,
    lazy_satisfied: RwSignal<Result<SatisfiedProgram, String>>,
}

impl Program {
    pub fn new(key: secp256k1::XOnlyPublicKey) -> Self {
        let text = format!(
            r#"mod witness {{
    const SIG: Signature = 0x1d7d93f350e2db564f90da49fb00ee47294bb6d8f061929818b26065a3e50fdd87e0e8ab45eecd04df0b92b427e6d49a5c96810c23706566e9093c992e075dc5; // TODO: update this
}}

fn main() {{
    let pk: Pubkey = 0x{};
    let msg: u256 = jet::sig_all_hash();
    jet::bip_0340_verify((pk, msg), witness::SIG)
}}"#,
            key.serialize().as_hex()
        );

        let program = Self {
            text: create_rw_signal(text),
            cached_text: create_rw_signal("".to_string()),
            lazy_cmr: create_rw_signal(Err("".to_string())),
            lazy_satisfied: create_rw_signal(Err("".to_string())),
        };
        program.update_on_read();
        program
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
        let needs_update = self.text.with_untracked(|text| {
            self.cached_text
                .with_untracked(|cached_text| text != cached_text)
        });
        if !needs_update {
            return;
        }
        self.text.with_untracked(|text| {
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
        if !success {
            self.alarm_audio_ref.get().map(|audio| audio.play());
        }
        spawn_local(async move {
            self.run_succeeded.set(Some(success));
            gloo_timers::future::TimeoutFuture::new(500).await;
            self.run_succeeded.set(None);
        });
        web_sys::window()
            .as_ref()
            .map(web_sys::Window::navigator)
            .map(|navigator| match success {
                true => navigator.vibrate_with_duration(200),
                false => navigator.vibrate_with_duration(500),
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

const TAB_KEY: u32 = 9;
const ENTER_KEY: u32 = 13;

#[component]
pub fn ProgramTab() -> impl IntoView {
    let program = use_context::<Program>().expect("program should exist in context");
    let runtime = use_context::<Runtime>().expect("runtime should exist in context");
    let textarea_ref = create_node_ref::<html::Textarea>();

    let update_program_text = move |event: ev::Event| {
        program.text.set(event_target_value(&event));
    };
    let insert_4_spaces = move || {
        let element = textarea_ref.get().expect("<textarea> should be mounted");
        if let Ok(Some(start)) = element.selection_start() {
            let start_ = start as usize; // safety: 32-bit machine of higher
            program.text.update(|s| s.insert_str(start_, "    "));
            let _result = element.set_selection_range(start + 4, start + 4);
        }
    };
    let delete_4_spaces = move || {
        let element = textarea_ref.get().expect("<textarea> should be mounted");
        if let Ok(Some(start)) = element.selection_start() {
            let start_ = start as usize; // safety: 32-bit machine of higher
            if start < 4 || program.text.with(|s| &s[start_ - 4..start_] != "    ") {
                return;
            }
            program
                .text
                .update(|s| s.replace_range(start_ - 4..start_, ""));
            let _result = element.set_selection_range(start - 4, start - 4);
        }
    };
    let handle_keydown = move |event: ev::KeyboardEvent| {
        if event.ctrl_key() && event.key_code() == ENTER_KEY {
            runtime.run();
        } else if event.key_code() == TAB_KEY {
            event.prevent_default();
            match event.shift_key() {
                false => insert_4_spaces(),
                true => delete_4_spaces(),
            }
        }
    };

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
                on:keydown=handle_keydown
                node_ref=textarea_ref
            >
                {program.text.get_untracked()}
            </textarea>
        </div>
    }
}
