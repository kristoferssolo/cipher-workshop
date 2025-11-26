use leptos::prelude::*;
use leptos_router::components::A;
use std::fmt::Display;

#[component]
pub fn Header() -> impl IntoView {
    let (theme, set_theme) = signal(Theme::Dark);

    Effect::new(move |_| {
        if let Ok(Some(storage)) = window().local_storage()
            && let Ok(Some(saved)) = storage.get_item("theme")
        {
            let theme = Theme::from_local_storage(&saved);
            set_theme(theme);
            apply_theme(theme);
        }
    });

    let toggle_theme = move |_| {
        set_theme.update(|t| *t = t.inverse());

        if let Ok(Some(storage)) = window().local_storage() {
            let _ = storage.set_item("theme", theme.get().to_local_storage());
        }
        apply_theme(theme.get());
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

    fn from_local_storage(value: &str) -> Self {
        match value.trim().to_lowercase().as_str() {
            "light" => Self::Light,
            _ => Self::Dark,
        }
    }

    const fn to_local_storage(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
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

fn apply_theme(theme: Theme) {
    if let Some(body) = document().body() {
        let class_list = body.class_list();
        match theme {
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
}
