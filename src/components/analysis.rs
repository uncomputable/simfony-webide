use leptos::*;
use std::str::FromStr;
use std::sync::Arc;

use crate::util::Expression;

#[component]
pub fn Analysis(program: Signal<Result<Arc<Expression>, String>>) -> impl IntoView {
    view! {
        <div>
        {
            move || program.get().ok().map(|t| view! {
                <h2>Analysis</h2>
                <AnalysisInner expression=t/>
            })
        }
         </div>
    }
}

const MILLISECONDS_PER_WU: f64 = 0.5 / 1000.0;

#[component]
fn AnalysisInner(expression: Arc<Expression>) -> impl IntoView {
    let bounds = expression.bounds();
    // FIXME: Add conversion method to simplicity::Cost
    let milli_weight = u32::from_str(&bounds.cost.to_string()).unwrap();
    let weight = milli_weight.saturating_add(999) / 1000;
    let virtual_size = weight.div_ceil(4);
    let size = weight; // Simplicity programs are Taproot witness data
    let max_milliseconds = f64::from(weight) * MILLISECONDS_PER_WU;
    let max_bytes = bounds.extra_cells.div_ceil(8);

    view! {
        <ul>
            <li>"Size: "{size}B</li>
            <li>"Virtual size: "{virtual_size}vB</li>
            <li>"Weight: "{weight}WU</li>
            <li>"Max runtime: "{max_milliseconds}ms</li>
            <li>"Max memory: "{max_bytes}B</li>
        </ul>
    }
}
