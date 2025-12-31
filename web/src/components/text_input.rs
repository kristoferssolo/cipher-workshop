use cipher_factory::prelude::OperationMode;
use leptos::{prelude::*, tachys::dom::event_target_value};

pub fn clean_hex_input(input: String) -> String {
    input
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .collect::<String>()
}

#[component]
pub fn TextInput(
    mode: ReadSignal<OperationMode>,
    text_input: ReadSignal<String>,
    set_text_input: WriteSignal<String>,
) -> AnyView {
    let handle_hex_input = move |ev| {
        let val = event_target_value(&ev);
        let cleaned = clean_hex_input(val);
        set_text_input(cleaned);
    };

    let handle_text_input = move |ev| {
        set_text_input(event_target_value(&ev));
    };

    view! {
        <div class="form-group">
            {move || {
                match mode.get() {
                    OperationMode::Encrypt => {
                        view! {
                            <div class="label-header">
                                <label>"Plaintext Input"</label>
                                <span class="input-hint">
                                    "Prefix: 0x (Hex), 0b (Bin), or nothing (Text)"
                                </span>
                            </div>
                            <div class="input-wrapper standard-input">
                                <input
                                    type="text"
                                    placeholder="Enter text..."
                                    on:input=handle_text_input
                                    spellcheck="false"
                                />
                            </div>
                        }
                            .into_any()
                    }
                    OperationMode::Decrypt => {
                        view! {
                            <div class="label-header">
                                <label>"Ciphertext Input"</label>
                            </div>
                            <div class="input-wrapper hex-input">
                                <span class="prefix">"0x"</span>
                                <input
                                    type="text"
                                    prop:value=move || text_input.get()
                                    placeholder="001122"
                                    on:input=handle_hex_input
                                    spellcheck="false"
                                />
                            </div>
                        }
                            .into_any()
                    }
                }
            }}
        </div>
    }
    .into_any()
}
