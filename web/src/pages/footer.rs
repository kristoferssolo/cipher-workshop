use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="app-footer">
            <div class="footer-content">
                <p>
                    "🔒 " <strong>"Client-Side Security:"</strong>
                    " All encryption and decryption operations happen entirely in your browser. "
                    "No data is ever sent to a server. "
                    "You can verify this by disconnecting your internet."
                </p>
                <div class="footer-links">
                    <A href="https://github.com/kristoferssolo/cipher-workshop" target="_blank">
                        "View Source on GitHub"
                    </A>
                </div>
            </div>
        </footer>
    }
}
