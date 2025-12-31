use crate::components::{
    config_section::ConfigurationSection,
    error_box::ErrorBox,
    key_input::{KeyInput, KeySize},
    output_box::OutputBox,
    text_input::TextInput,
};
use cipher_factory::prelude::*;
use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn CipherForm(algorithm: Algorithm) -> AnyView {
    let (mode, set_mode) = signal(OperationMode::Encrypt);
    let (output_fmt, set_output_fmt) = signal(OutputFormat::Hex);

    let (key_input, set_key_input) = signal(String::new());
    let (text_input, set_text_input) = signal(String::new());

    let (output, set_output) = signal(String::new());
    let (error_msg, set_error_msg) = signal(String::new());

    let (copy_feedback, set_copy_feedback) = signal(false);

    let key_size = match algorithm {
        Algorithm::Des => KeySize::Des,
        Algorithm::Aes | Algorithm::AesCbc => KeySize::Aes128,
    };

    let handle_submit = move || {
        set_error_msg(String::new());
        set_output(String::new());

        let key = key_input.get();
        let raw_text = text_input.get();

        if key.is_empty() || raw_text.is_empty() {
            set_error_msg("Please enter both key and input text.".to_string());
            return;
        }

        let final_text = if mode.get() == OperationMode::Decrypt {
            format!("0x{raw_text}")
        } else {
            raw_text
        };

        let context = CipherContext::new(
            algorithm,
            mode.get(),
            key,
            None,
            final_text,
            output_fmt.get(),
        );
        match context.process() {
            Ok(out) => set_output(out),
            Err(e) => set_error_msg(e.to_string()),
        }
    };

    let copy_to_clipboard = move |content: String| {
        let clipboard = window().navigator().clipboard();
        let _ = clipboard.write_text(&content);
        set_copy_feedback(true);
        set_timeout(move || set_copy_feedback(false), Duration::from_secs(2));
    };

    let update_output = move |fmt| {
        set_output_fmt(fmt);
        if !output.get().is_empty() {
            handle_submit();
        }
    };

    view! {
        <div class="cipher-card">
            <div class="card-header">
                <h2>{algorithm.to_string()}</h2>
            </div>
            <ConfigurationSection
                mode=mode
                set_mode=set_mode
                output_fmt=output_fmt
                update_output=update_output
            />
            <KeyInput key_input=key_input set_key_input=set_key_input key_size=key_size />
            <TextInput mode=mode text_input=text_input set_text_input=set_text_input />

            <button class="btn-primary" on:click=move |_| handle_submit()>
                {move || format!("{} using {algorithm}", mode.get())}
            </button>

            <OutputBox
                output=output
                output_fmt=output_fmt
                copy_to_clipboard=copy_to_clipboard
                copy_feedback=copy_feedback
            />
            <ErrorBox error_msg=error_msg />

        </div>
    }
    .into_any()
}
