use crate::components::cipher_form::CipherForm;
use leptos::prelude::*;

#[component]
pub fn DesPage() -> impl IntoView {
    let des_logic = Box::new(
        |encrypt: bool, key_str: String, text_str: String| -> (String, String) {
            (String::new(), String::new())
        },
    );

    view! { <CipherForm title="DES" logic=des_logic /> }
}
