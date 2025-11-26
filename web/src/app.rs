use crate::pages::{aes::AesPage, des::DesPage, footer::Footer, header::Header, home::Home};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
// Provides context that manages stylesheets, titles, meta tags, etc.
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web.css" />

        // sets the document title
        <Title text="Cipher Workshop" />

        // content for this welcome page
        <Router>
            <div class="app-containter">
                <Header />
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("/") view=Home />
                        <Route path=StaticSegment("/des") view=DesPage />
                        <Route path=StaticSegment("/aes") view=AesPage />
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}
