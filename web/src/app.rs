use crate::pages::{aes::AesPage, des::DesPage, home::Home};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{A, Route, Router, Routes},
};
use std::fmt::Display;

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

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Light,
    Dark,
}

impl Theme {
    const fn inverse(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Light => "☀️ Light",
            Self::Dark => "🌙 Dark",
        };
        f.write_str(s)
    }
}

#[component]
// Provides context that manages stylesheets, titles, meta tags, etc.
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (theme, set_theme) = signal(Theme::Dark);

    let toggle_theme = move |_| {
        set_theme.update(|t| *t = t.inverse());

        if let Some(body) = document().body() {
            let class_list = body.class_list();
            match theme.get() {
                Theme::Light => {
                    let _ = class_list.remove_1("dark-theme");
                    let _ = class_list.add_1("light-theme");
                }
                Theme::Dark => {
                    let _ = class_list.remove_1("light-theme");
                    let _ = class_list.add_1("dark-theme");
                }
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
                        {move || theme.get().to_string()}
                    </button>
                </nav>
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("/") view=Home />
                        <Route path=StaticSegment("/des") view=DesPage />
                        <Route path=StaticSegment("/aes") view=AesPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
