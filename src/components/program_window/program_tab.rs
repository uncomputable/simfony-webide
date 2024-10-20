use leptos::{
    component, create_rw_signal, ev, event_target_value, use_context, view, with, IntoView,
    RwSignal, SignalGetUntracked, SignalSet,
};
use simfony::parse::ParseFromStr;
use simfony::simplicity;
use simfony::{CompiledProgram, SatisfiedProgram, WitnessValues};

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

#[component]
pub fn ProgramTab() -> impl IntoView {
    let program = use_context::<Program>().expect("program should exist in context");

    let update_program_text = move |event: ev::Event| {
        program.text.set(event_target_value(&event));
    };
    let program_text_initial_value = program.text.get_untracked();

    view! {
        <div class="tab-content">
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
