use leptos::*;

use crate::examples;

#[component]
pub fn SelectExampleProgram<F>(
    update_program_str: F,
    set_name: WriteSignal<Option<String>>,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let select_example_program = move |name: String| {
        if let Some(new_human) = examples::get_program_str(&name) {
            update_program_str(new_human.to_string());
            set_name.set(Some(name));
        }
    };

    // Select default example upon startup
    // The name must exist inside examples::EXAMPLES
    select_example_program("Welcome 💡".to_string());

    view! {
        <div class="example-program-select-container">
            <select
                class="example-program-select"
                on:input=move |event| select_example_program(event_target_value(&event))
            >
                <option value="" disabled selected>Example programs</option>
                {
                    examples::get_names()
                        .map(|name| view! { <option value={name}>{name}</option>})
                        .collect::<Vec<_>>()
                }
            </select>
            <i class="fas fa-angle-down"></i>
        </div>
    }
}

#[component]
pub fn ExampleProgramDescription(name: ReadSignal<Option<String>>) -> impl IntoView {
    view! {
        <div class="program-details">
        {
            move || name.get().map(|name| {
                let description = examples::get_description(name.as_str())
                    .unwrap_or("")
                    .to_string();

                view! {
                    <div>
                        <h3 class="program-title">{name}</h3>
                        <div
                            class="program-description"
                            inner_html={description}
                        >
                        </div>
                    </div>
                }
            })
        }
        </div>
    }
}
