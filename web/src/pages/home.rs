use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home-container">
            <h1>"Cipher Workshop"</h1>
            <p>
                "Hello there! Select an algorithm (AES or DES) from the navigation bar to begin
                encrypting and decrypting data."
            </p>
        </div>
    }
}
