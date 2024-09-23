use leptos::{
    component, create_node_ref, create_rw_signal, ev, html, use_context, view, IntoView, NodeRef,
    SignalSet, SignalUpdate,
};

use crate::components::app::ProgramWrapper;
use crate::components::apply_changes::ApplyChanges;
use crate::components::error::ErrorBox;

#[component]
pub fn ProgramTab() -> impl IntoView {
    let program_text = use_context::<ProgramWrapper>().expect("program should exist in context");
    let parse_error = create_rw_signal("".to_string());
    let apply_changes = ApplyChanges::default();
    let textarea_ref: NodeRef<html::Textarea> = create_node_ref();

    let submit_program = move |event: ev::SubmitEvent| {
        event.prevent_default(); // stop page from reloading
        let textarea_value = textarea_ref
            .get()
            .expect("<textarea> should be mounted")
            .value();
        match simfony::compile(textarea_value.as_str()) {
            Ok(..) => {
                parse_error.update(String::clear);
                program_text.0.set(textarea_value);
                apply_changes.set_success(true);
            }
            Err(error) => {
                parse_error.set(error);
                apply_changes.set_success(false);
            }
        }
    };

    view! {
        <div>
            <form on:submit=submit_program>
                <textarea
                    class="program-input-field"
                    placeholder="Enter your program here"
                    rows="15"
                    cols="80"
                    spellcheck="false"
                    node_ref=textarea_ref
                >
                    {program_text.0}
                </textarea>
                <ErrorBox error=parse_error />
                {apply_changes}
            </form>
        </div>
    }
}
