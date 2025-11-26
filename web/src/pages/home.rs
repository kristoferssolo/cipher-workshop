use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home-container">
            <div class="hero-section">
                <h1>"Cipher Workshop"</h1>
                <p class="subtitle">
                    "A client-side cryptographic playground for learning and testing symmetric algorithms."
                </p>
            </div>

            <div class="info-grid">
                <div class="info-card">
                    <h3>"DES (Data Encryption Standard)"</h3>
                    <p>
                        "A legacy algorithm from the 1970s. While historically significant, "
                        "it is now considered insecure due to its short 56-bit key length. "
                        "This tool simulates " <strong>"DES-ECB"</strong>
                        " mode for educational comparison."
                    </p>
                </div>

                <div class="info-card">
                    <h3>"AES (Advanced Encryption Standard)"</h3>
                    <p>
                        "The modern standard for secure data transmission. This tool uses "
                        <strong>"AES-128-GCM"</strong>
                        ", which provides both confidentiality and data integrity. "
                        "It is widely used across the internet (HTTPS) and government communications."
                    </p>
                </div>
            </div>

            <div class="getting-started">
                <p>
                    "To get started, select an algorithm from the navigation bar above."
                    " You can generate output in Binary, Octal, Hex, or Text formats."
                </p>
            </div>
        </div>
    }
}
