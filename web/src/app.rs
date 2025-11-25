use crate::pages::{des::DesPage, home::Home};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{A, Route, Router, Routes},
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

    let (is_light, set_is_light) = signal(false);

    let toggle_theme = move |_| {
        set_is_light.update(|light| *light = !*light);

        if let Some(body) = document().body() {
            let class_list = body.class_list();
            if is_light.get() {
                let _ = class_list.add_1("light-theme");
            } else {
                let _ = class_list.remove_1("light-theme");
            }
        }
    };

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web2.css" />

        // sets the document title
        <Title text="Cipher Workshop" />

        // content for this welcome page
        <Router>
            <div class="app-containter">
                <nav class="main-nav">
                    <ul>
                        <li>
                            <A href="/">"Home"</A>
                        </li>
                        <li>
                            <A href="/des">"DES"</A>
                        </li>
                        <li>
                            <A href="/aes">"AES"</A>
                        </li>
                    </ul>
                    <button class="theme-toggle" on:click=toggle_theme>
                        {move || if is_light.get() { "🌙 Dark" } else { "☀️ Light" }}
                    </button>
                </nav>
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("/") view=Home />
                        <Route path=StaticSegment("/des") view=DesPage />
                        <Route path=StaticSegment("/aes") view=Home />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
