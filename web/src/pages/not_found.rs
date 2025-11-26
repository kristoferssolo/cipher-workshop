use leptos::prelude::*;
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found-container">
            <div class="error-code">"404"</div>
            <h1>"Page Not Found"</h1>
            <p>"The data you are looking for has been encrypted into the void."</p>
            <div class="binary-decoration">
                "01000100 01000101 01000001 01000100 00100000 01001100 01001001 01001110 01001011"
            </div>
            <a href="/" class="btn-primary btn-link">
                "Return to Home"
            </a>
        </div>
    }
}
