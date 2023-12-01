use leptos::*;
use simplicity::jet::Elements;

use super::bit_machine::Stacks;
use crate::instruction::CachedRunner;
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    let (status, set_status) = create_signal("(Idle)".to_string());
    let (runner, set_runner) = create_signal::<Option<CachedRunner<Elements>>>(None);
    let past_instructions = Signal::<Option<Vec<String>>>::derive(move || {
        runner
            .get()
            .map(|r| r.past_instructions().map(|i| i.to_string()).collect())
    });
    let next_instructions = Signal::<Option<Vec<String>>>::derive(move || {
        runner
            .get()
            .map(|r| r.next_instructions().map(|i| i.to_string()).collect())
    });

    let update_program = move |human: String, tco: bool| match util::program_from_string(&human) {
        Ok(program) => {
            set_runner.set(Some(CachedRunner::for_program(program, tco)));
            set_status.set("(Let's start)".to_string());
        }
        Err(error) => {
            set_runner.set(None);
            set_status.set(format!("Error: {error}"));
        }
    };

    let run_next_step = move || {
        set_runner.update(|maybe_r| {
            if let Some(r) = maybe_r {
                match r.next() {
                    Ok(()) if r.next_instructions().next().is_none() => {
                        set_status.set("(Finished)".to_string())
                    }
                    Ok(()) => set_status.set("(Ok)".to_string()),
                    Err(error) => set_status.set(format!("Error: {error}")),
                };
            }
        })
    };

    view! {
        <button on:click=move |_| run_next_step()>
            "Next"
        </button>
        <p>
            {status}
        </p>
        <Stacks runner=runner/>
        <Instructions instructions=past_instructions highlight=false/>
        <Instructions instructions=next_instructions highlight=true/>
        <textarea
            on:input=move |event| update_program(event_target_value(&event), false)
            placeholder="Enter program text here"
            rows="10" cols="50"
        />
    }
}

#[component]
fn Instructions(instructions: Signal<Option<Vec<String>>>, highlight: bool) -> impl IntoView {
    let class = if highlight {
        "instructions next_instructions"
    } else {
        "instructions"
    };
    view! {
        <div class=class>
            {
                move || instructions.get().map(|is| is.iter().map(|i| view! { <span class="instruction">{i}</span> }).collect_view())
            }
        </div>
    }
}
