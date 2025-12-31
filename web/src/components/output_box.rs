use cipher_factory::prelude::OutputFormat;
use leptos::prelude::*;

#[component]
pub fn OutputBox(
    output: ReadSignal<String>,
    output_fmt: ReadSignal<OutputFormat>,
    copy_to_clipboard: impl Fn(String) + Copy + Send + 'static,
    copy_feedback: ReadSignal<bool>,
) -> AnyView {
    view! {
        {move || {
            if output.get().is_empty() {
                return view! { <span></span> }.into_any();
            }
            view! {
                <div class="result-box">
                    <div class="result-toolbar">
                        <strong>"Output ("{output_fmt.get().to_string()}")"</strong>
                        <button class="btn-copy" on:click=move |_| copy_to_clipboard(output.get())>
                            {move || {
                                if copy_feedback.get() { "Copied" } else { "Copy" }
                            }}
                        </button>
                    </div>
                    <code>{output.get()}</code>
                </div>
            }
                .into_any()
        }}
    }
    .into_any()
}
