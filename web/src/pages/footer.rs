use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="app-footer">
            <div class="footer-content">
                <p>
                    <strong>"🔐 Privacy First:"</strong>
                    " All encryption and decryption happens in your browser. "
                    "No data is transmitted to any server. You can verify this by disconnecting your internet connection."
                </p>
                <div class="footer-links">
                    <a href="https://github.com/kristoferssolo/cipher-workshop" target="_blank">
                        "View Source on GitHub"
                    </a>
                </div>
            </div>
        </footer>
    }
}
