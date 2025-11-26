use crate::components::cipher_form::CipherForm;
use cipher_factory::Algorithm;
use leptos::prelude::*;

#[component]
pub fn AesPage() -> impl IntoView {
    view! { <CipherForm algorithm=Algorithm::Aes /> }
}
