use leptos::prelude::*;
use leptos_router::components::A;
use std::fmt::Display;

#[component]
pub fn Header() -> impl IntoView {
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
