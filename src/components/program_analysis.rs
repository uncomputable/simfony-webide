use leptos::*;

#[component]
pub fn Analysis() -> impl IntoView {
    view! {
        <div class="analysis">
            <div class="flex analysis-header">
                <h2 class="analysis-title">Program Analysis</h2>
                <div class="program-status">
                    Program status
                </div>
            </div>
        </div>
    }
}