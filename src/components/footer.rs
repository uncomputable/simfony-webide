use leptos::{component, view, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="footer">
            <div class="footer-left">
                <img class="footer-logo" src="images/simfony_logo.svg" />
                <p>Developers write Simfony, full nodes execute Simplicity.</p>
            </div>
            <div class="footer-right">
                <h1 class="footer-title">Learn more</h1>
                <div class="footer-links">
                    <a href="https://simfony-lang.com/" target="blank" class="footer-link">
                        <div class="footer-link-icon">"🚀"</div>
                        <div class="footer-link-text">Simfony Lander</div>
                        <div class="footer-link-url">"https://simfony-lang.com"</div>
                    </a>
                    <a href="https://docs.rs/simfony-as-rust/latest/simfony_as_rust/jet/index.html" target="blank" class="footer-link">
                        <div class="footer-link-icon">"📜"</div>
                        <div class="footer-link-text">Jet documentation</div>
                        <div class="footer-link-url">"https://docs.rs/simfony-as-rust"</div>
                    </a>
                    <a href="https://github.com/uncomputable/simfony-webide" target="blank" class="footer-link">
                        <div class="footer-link-icon">"🛠️"</div>
                        <div class="footer-link-text">Simfony web IDE GitHub repository</div>
                        <div class="footer-link-url">"https://github.com/uncomputable/simfony-webide"</div>
                    </a>
                    <a href="https://github.com/BlockstreamResearch/simfony" target="blank" class="footer-link">
                        <div class="footer-link-icon">"🛠️"</div>
                        <div class="footer-link-text">Simfony compiler GitHub repository</div>
                        <div class="footer-link-url">"https://github.com/BlockstreamResearch/simfony"</div>
                    </a>
                </div>
            </div>
        </div>
    }
}
