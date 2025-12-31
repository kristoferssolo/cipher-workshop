use crate::components::cipher_form_cbc::CipherFormCbc;
use leptos::prelude::*;

#[component]
pub fn AesCbcPage() -> impl IntoView {
    view! {
        <CipherFormCbc />
    }
}
