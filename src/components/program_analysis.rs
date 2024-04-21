use leptos::*;

#[component]
pub fn Analysis(program_success: ReadSignal<bool>, program_status_message: ReadSignal<String>) -> impl IntoView {

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
                        <div class="analysis-item-data">sizeTemp</div>
                    </div>
                    <div class="analysis-item">
                        <div class="analysis-item-label">Current fee rate:</div>
                        <div class="analysis-item-data">cfeeTemp</div>
                    </div>
                    <div class="analysis-item">
                        <div class="analysis-item-label">Virtual size:</div>
                        <div class="analysis-item-data">vsizeTemp</div>
                    </div>
                    <div class="analysis-item">
                        <div class="analysis-item-label">Maximum memory:</div>
                        <div class="analysis-item-data">mmemTemp</div>
                    </div>
                    <div class="analysis-item">
                        <div class="analysis-item-label">Weight:</div>
                        <div class="analysis-item-data">weightTemp</div>
                    </div>
                    <div class="analysis-item">
                        <div class="analysis-item-label">Maximum runtime:</div>
                        <div class="analysis-item-data">maxRunTemp</div>
                    </div>
                    <div class="analysis-item">
                        <div class="analysis-item-label">Current fee:</div>
                        <div class="analysis-item-data">cfeeTemp</div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

