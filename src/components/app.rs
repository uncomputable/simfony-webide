use itertools::Itertools;
use leptos::*;
use simplicity::jet::Elements;

use crate::exec::BitMachine;
use crate::instruction::Runner;
use crate::util;

#[component]
pub fn App() -> impl IntoView {
    let (step, set_step) = create_signal(0);
    let (output, set_output) = create_signal("(No output)".to_string());
    let (mac, set_mac) = create_signal(Option::<BitMachine>::None);
    let (_, set_runner) = create_signal(Option::<Runner<Elements>>::None);

    let update_program = move |new_human: String| match util::program_from_string(&new_human) {
        Ok(program) => {
            set_mac.set(Some(BitMachine::for_program()));
            let runner = Runner::for_program(program, false);
            let stack = runner.get_stack().iter().map(|x| x.to_string()).join(" ");
            set_runner.set(Some(runner));
            set_output.set(stack);
        }
        Err(error) => {
            set_output.set(error);
        }
    };

    let display_mac = move || {
        mac.get()
            .map(|m| m.to_string())
            .unwrap_or("(No machine)".to_string())
    };

    let run_next_step = move || {
        set_output.update(|o| {
            set_mac.update(|maybe_m| {
                set_runner.update(|maybe_r| {
                    if let (Some(m), Some(r)) = (maybe_m, maybe_r) {
                        let status = match r.next(m) {
                            Ok(Some(x)) => x.to_string(),
                            Ok(None) => "(Done)".to_string(),
                            Err(error) => format!("Error: {error}"),
                        };
                        let stack = r.get_stack().iter().map(|x| x.to_string()).join(" ");
                        *o = format!("{stack} | {status}");
                    }
                })
            })
        });
        set_step.update(|n| *n += 1);
    };

    view! {
        <button
            on:click=move |_| run_next_step()
        >
            "Next"
        </button>
        <p>
            <strong>"Step: "</strong>
            {step}
        </p>
        <p>
            {output}
        </p>
        <p>
            {display_mac}
        </p>
        <textarea
            on:input=move |event| update_program(event_target_value(&event))
            placeholder="Enter program text here"
            rows="10" cols="50"
        />
    }
}
