use leptos::{
    component, create_node_ref, create_rw_signal, ev, html, use_context, view, IntoView, NodeRef,
    RwSignal, SignalGetUntracked, SignalSet, SignalUpdate,
};
use simfony::witness::WitnessValues;
use simfony::{CompiledProgram, SatisfiedProgram};

use crate::components::apply_changes::ApplyChanges;
use crate::components::string_box::ErrorBox;

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub compiled: CompiledProgram,
    pub text: String,
}

impl Program {
    pub fn compile(text: String) -> Result<Self, String> {
        match CompiledProgram::new(text.as_str()) {
            Ok(compiled) => Ok(Self { compiled, text }),
            Err(error) => Err(error),
        }
    }

    pub fn satisfy(&self, witness_values: &WitnessValues) -> Result<SatisfiedProgram, String> {
        self.compiled.satisfy(witness_values)
    }
}

#[component]
pub fn ProgramTab() -> impl IntoView {
    let program = use_context::<RwSignal<Program>>().expect("program should exist in context");
    let parse_error = create_rw_signal("".to_string());
    let apply_changes = ApplyChanges::default();
    let textarea_ref: NodeRef<html::Textarea> = create_node_ref();

    let submit_program = move |event: ev::SubmitEvent| {
        event.prevent_default(); // stop page from reloading
        let textarea_value = textarea_ref
            .get()
            .expect("<textarea> should be mounted")
            .value();
        match Program::compile(textarea_value) {
            Ok(x) => {
                program.set(x);
                parse_error.update(String::clear);
                apply_changes.set_success(true);
            }
            Err(error) => {
                parse_error.set(error);
                apply_changes.set_success(false);
            }
        }
    };

    let program_text_initial_value = program.get_untracked().text;

    view! {
        <div class="tab-content">
            <form on:submit=submit_program>
                <textarea
                    class="program-input-field"
                    placeholder="Enter your program here"
                    rows="15"
                    cols="80"
                    spellcheck="false"
                    node_ref=textarea_ref
                >
                    {program_text_initial_value}
                </textarea>
                <ErrorBox error=parse_error />
                {apply_changes}
            </form>
        </div>
    }
}
