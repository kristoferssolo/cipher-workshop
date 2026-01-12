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
                        "This tool provides DES block encryption for educational purposes."
                    </p>
                </div>

                <div class="info-card">
                    <h3>"AES (Advanced Encryption Standard)"</h3>
                    <p>
                        "The modern standard for symmetric encryption. This tool offers "
                        <strong>"AES-128"</strong> " for single-block operations and "
                        <strong>"AES-128-CBC"</strong> " for encrypting arbitrary data with "
                        "PKCS#7 padding."
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
