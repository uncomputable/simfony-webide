use leptos::*;

use crate::instruction::Runner;
use crate::{exec, util};

fn default_program() -> &'static str {
    "
        not := comp (pair iden unit) (case (injr unit) (injl unit)) : 2 -> 2
        input := injl unit : 1 -> 2
        output := unit : 2 -> 1
        main := comp input (comp not output)
    "
}

#[component]
pub fn App() -> impl IntoView {
    let (step, set_step) = create_signal(0);
    let (output, set_output) = create_signal("(Start)".to_string());

    let mac = exec::BitMachine::for_program();
    let (mac, set_mac) = create_signal(mac);

    let program = util::program_from_string(default_program()).unwrap();
    let runner = Runner::for_program(program, false);
    let (_, set_runner) = create_signal(runner);

    let run_next_step = move || {
        set_output.update(|o| {
            set_mac.update(|m| {
                set_runner.update(|r| {
                    let new_output = match r.next(m) {
                        Ok(Some(x)) => x.to_string(),
                        Ok(None) => "(Done)".to_string(),
                        Err(error) => format!("Error: {error}"),
                    };
                    *o = new_output;
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
            {move || mac.get().to_string()}
        </p>
    }
}
