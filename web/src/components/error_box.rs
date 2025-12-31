use leptos::prelude::*;

#[component]
pub fn ErrorBox(error_msg: ReadSignal<String>) -> AnyView {
    view! {
        {move || {
            if error_msg.get().is_empty() {
                return view! { <span></span> }.into_any();
            }
            view! { <div class="error-box">{error_msg.get()}</div> }.into_any()
        }}
    }
    .into_any()
}
