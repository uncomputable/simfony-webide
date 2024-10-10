use leptos::{
    component, ev, event_target_value, use_context, view, IntoView, RwSignal, SignalGetUntracked,
    SignalSet, SignalWith,
};
use simfony::parse::ParseFromStr;
use simfony::witness::WitnessValues;
use simfony::{CompiledProgram, SatisfiedProgram};

#[derive(Clone, Debug, Default)]
pub struct ProgramText(pub RwSignal<String>);

impl ProgramText {
    pub fn satisfy(&self) -> Result<SatisfiedProgram, String> {
        self.0.with(|text| {
            let compiled = CompiledProgram::new(text)?;
            let witness_values = WitnessValues::parse_from_str(text)?;
            compiled.satisfy(&witness_values)
        })
    }
}

#[component]
pub fn ProgramTab() -> impl IntoView {
    let program_text = use_context::<ProgramText>().expect("program text should exist in context");

    let update_program_text = move |event: ev::Event| {
        program_text.0.set(event_target_value(&event));
    };
    let program_text_initial_value = program_text.0.get_untracked();

    view! {
        <div class="tab-content">
            <textarea
                class="program-input-field"
                placeholder="Enter your program here"
                rows="25"
                cols="80"
                spellcheck="false"
                prop:value=program_text.0
                on:input=update_program_text
            >
                {program_text_initial_value}
            </textarea>
        </div>
    }
}
