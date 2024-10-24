use leptos::{component, view, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="container center intro">
            <h1 class="intro-title">Further Links</h1>
            <div>
                "🚀 "
                <a href="https://simfony-lang.com/" target="blank">Simfony Lander</a>
            </div>
            <div>
                "🛠️ "
                <a href="https://github.com/uncomputable/simfony-webide" target="blank">Simfony web IDE GitHub repository</a>
            </div>
            <div>
                "🛠️ "
                <a href="https://github.com/BlockstreamResearch/simfony" target="blank">Simfony compiler GitHub repository</a>
            </div>
        </div>
    }
}
