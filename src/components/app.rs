use leptos::*;

use super::merkle::Merkle;
use crate::examples;
use crate::function::Runner;
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    let (human, set_human) = create_signal("".to_string());
    let (program_success, set_program_success) = create_signal("".to_string());

    let program = move || util::program_from_string(&human.get());
    let human_error = move || program().err().map(|error| format!("Error: {error}"));

    let select_example_program = move |name: String| {
        if let Some(new_human) = examples::get_program(&name) {
            set_human.set(new_human.to_string());
            set_program_success.set("".to_string());
        }
    };
    let run_program = move || {
        let program = match program() {
            Ok(program) => program,
            Err(_) => return,
        };
        let mut runner = Runner::for_program(util::Expression(program));
        match runner.run() {
            Ok(_) => {
                set_program_success.set("✅ Program success".to_string());
            }
            Err(error) => {
                set_program_success.set(format!("❌ {error}"));
            }
        }
    };

    view! {
        <div>
            <p>
                {human_error}
            </p>
            <p>
                {program_success}
            </p>
            <select
                on:input=move |event| select_example_program(event_target_value(&event))
            >
                <option value="" disabled selected>Example programs</option>
                {
                    examples::get_names()
                        .map(|name| view! { <option value={name}>{name}</option>})
                        .collect::<Vec<_>>()
                }
            </select>
        </div>
        <textarea
            prop:value=move || human.get()
            on:input=move |event| set_human.set(event_target_value(&event))
            placeholder="Enter your program here"
            rows="10" cols="80"
        />
        <div>
            <button on:click=move |_| run_program()>
                "Run program"
            </button>
        </div>
        <div>
            {
                move || program().ok().map(|t| view! { <Merkle expression=t/> })
            }
        </div>
    }
}
