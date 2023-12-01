use leptos::*;
use simplicity::jet::Jet;

use crate::exec;
use crate::instruction::CachedRunner;

#[component]
pub fn Stacks<J: Jet>(runner: ReadSignal<Option<CachedRunner<J>>>) -> impl IntoView {
    view! {
        {
            move || {
                runner.get().map(|r| {
                    let read_stack = r.get_mac().read_stack().to_vec();
                    let write_stack = r.get_mac().write_stack().to_vec();
                    view! {
                        <div class="stacks">
                            <Stack stack=read_stack name="Read"/>
                            <Stack stack=write_stack name="Write"/>
                        </div>
                    }
                })
            }
        }
    }
}

#[component]
pub fn Stack(stack: Vec<exec::Frame>, name: &'static str) -> impl IntoView {
    view! {
        <div class="named-stack">
            <p>{name}</p>
            <div class="stack">
                {
                    move || stack.iter().rev().map(|frame| view! { <Frame frame=frame.clone()/> }).collect_view()
                }
            </div>
        </div>
    }
}

#[component]
pub fn Frame(frame: exec::Frame) -> impl IntoView {
    view! {
        <div class="frame">
        {
            move || frame.cells().map(|cell| view! { <Cell cell=cell/> }).collect_view()
        }
        </div>
    }
}

#[component]
pub fn Cell(cell: exec::Cell) -> impl IntoView {
    let class = match cell {
        exec::Cell::Zero => "cell cell-zero",
        exec::Cell::One => "cell cell-one",
        exec::Cell::Cursor => "cell cell-cursor",
    };
    view! {
        <div class=class></div>
    }
}
