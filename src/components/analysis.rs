use leptos::*;
use std::str::FromStr;
use std::sync::Arc;

use crate::util::Expression;

#[component]
pub fn Analysis(program: Signal<Result<Arc<Expression>, String>>, program_success: ReadSignal<bool>, program_status_message: ReadSignal<String>) -> impl IntoView {
    view! {
        <div>
        {
            move || program.get().ok().map(|t| view! {
                <AnalysisInner expression=t program_success=program_success program_status_message=program_status_message />
            })
        }
         </div>
    }
}

const MILLISECONDS_PER_WU: f64 = 0.5 / 1000.0;

#[component]
fn AnalysisInner(expression: Arc<Expression>, program_success: ReadSignal<bool>, program_status_message: ReadSignal<String>) -> impl IntoView {
    let bounds = expression.bounds();
    // FIXME: Add conversion method to simplicity::Cost
    let milli_weight = u32::from_str(&bounds.cost.to_string()).unwrap();
    let weight = milli_weight.saturating_add(999) / 1000;
    let virtual_size = weight.div_ceil(4);
    let size = weight; // Simplicity programs are Taproot witness data
    let max_milliseconds = f64::from(weight) * MILLISECONDS_PER_WU;
    let max_bytes = bounds.extra_cells.div_ceil(8);

    view! {

        <Show when=move || {program_status_message.get() != ""} >
            <div class="analysis">
                <div class="flex analysis-header">
                    <h2 class="analysis-title">Program Analysis</h2>
                    <div class="program-status" class:is_error=move || !program_success.get() >
                        {program_status_message}
                    </div>
                </div>
                <div class="analysis-body">
                    <div class="analysis-item">
                        <div class="analysis-item-label">Size:</div>
                        <div class="analysis-item-data">{size}B</div>
                    </div>
                    <div class="analysis-item">
                        <div class="analysis-item-label">Virtual size:</div>
                        <div class="analysis-item-data">{virtual_size}vB</div>
                    </div>
                    <div class="analysis-item">
                        <div class="analysis-item-label">Maximum memory:</div>
                        <div class="analysis-item-data">{max_bytes}B</div>
                    </div>
                    <div class="analysis-item">
                        <div class="analysis-item-label">Weight:</div>
                        <div class="analysis-item-data">{weight}WU</div>
                    </div>
                    <div class="analysis-item">
                        <div class="analysis-item-label">Maximum runtime:</div>
                        <div class="analysis-item-data">{max_milliseconds}ms</div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
