use leptos::*;
use simplicity::jet::Elements;

use crate::instruction::Runner;
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    let (status, set_status) = create_signal("(Idle)".to_string());
    let (prefix, set_prefix) = create_signal::<Vec<String>>(vec![]);
    let (suffix, set_suffix) = create_signal::<Vec<String>>(vec![]);

    let (runner, set_runner) = create_signal::<Option<Runner<Elements>>>(None);

    let update_program = move |human: String, tco: bool| match util::program_from_string(&human) {
        Ok(program) => {
            let runner = Runner::for_program(program, tco);
            let stack = runner.get_stack().iter().map(|x| x.to_string()).collect();
            set_runner.set(Some(runner));

            set_status.set("(Let's start)".to_string());
            set_prefix.set(vec![]);
            set_suffix.set(stack);
        }
        Err(error) => {
            set_status.set(format!("Error: {error}"));
        }
    };

    let run_next_step = move || {
        set_runner.update(|maybe_r| {
            if let Some(r) = maybe_r {
                match r.next() {
                    Ok(Some(instruction)) => {
                        set_prefix.update(|p| p.push(instruction.to_string()));
                        set_status.set("(Ok)".to_string());
                    }
                    Ok(None) => {
                        set_status.set("(Done)".to_string());
                    }
                    Err(error) => {
                        set_status.set(format!("Error: {error}"));
                    }
                };
                let stack = r.get_stack().iter().map(|x| x.to_string()).collect();
                set_suffix.set(stack);
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
        <BitMachine runner=runner/>
        <PastInstructions prefix=prefix/>
        <InstructionStack suffix=suffix/>
        <textarea
            on:input=move |event| update_program(event_target_value(&event), false)
            placeholder="Enter program text here"
            rows="10" cols="50"
        />
    }
}

#[component]
fn BitMachine(runner: ReadSignal<Option<Runner<Elements>>>) -> impl IntoView {
    view! {
        <p>
            {
                move || runner.get().map(|r| r.get_mac().to_string()).unwrap_or("(No machine)".to_string())
            }
        </p>
    }
}

#[component]
fn PastInstructions(prefix: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="instructions">
            {
                move || prefix.get().iter().map(|s| view! { <span class="instruction">{s}</span> }).collect_view()
            }
        </div>
    }
}

#[component]
fn InstructionStack(suffix: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="instructions instruction-stack">
            {
                move || suffix.get().iter().rev().map(|s| view! { <span class="instruction">{s}</span> }).collect_view()
            }
        </div>
    }
}
