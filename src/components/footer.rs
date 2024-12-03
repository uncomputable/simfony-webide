use leptos::{component, view, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="footer center">
            <img class="footer-logo" src="images/simfony_logo.svg" />

            <h1 class="footer-title">Learn more</h1>
            
                <div class="footer-links">

                    <div class="footer-link">
                        "🚀 "
                        <a href="https://simfony-lang.com/" target="blank">Simfony Lander</a>
                    </div>
                    <div class="footer-link">
                        "📜 "
                        <a href="https://docs.rs/simfony-as-rust/latest/simfony_as_rust/jet/index.html" target="blank">Jet documentation</a>
                    </div>
                    <div class="footer-link">
                        "🛠️ "
                        <a href="https://github.com/uncomputable/simfony-webide" target="blank">Simfony web IDE GitHub repository</a>
                    </div>
                    <div class="footer-link">
                        "🛠️ "
                        <a href="https://github.com/BlockstreamResearch/simfony" target="blank">Simfony compiler GitHub repository</a>
                    </div>
                </div>
        </div>
    }
}
