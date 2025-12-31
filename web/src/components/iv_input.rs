use js_sys::Uint8Array;
use leptos::{prelude::*, tachys::dom::event_target_value};

fn clean_hex_input(input: String) -> String {
    input
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .collect::<String>()
}

fn generate_random_bytes(len: usize) -> Option<Vec<u8>> {
    let window = web_sys::window()?;
    let crypto = window.crypto().ok()?;
    let array = Uint8Array::new_with_length(len as u32);
    crypto
        .get_random_values_with_array_buffer_view(&array)
        .ok()?;
    Some(array.to_vec())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

#[component]
pub fn IvInput(iv_input: ReadSignal<String>, set_iv_input: WriteSignal<String>) -> AnyView {
    let handle_hex_input = move |ev| {
        let val = event_target_value(&ev);
        let cleaned = clean_hex_input(val);
        set_iv_input(cleaned);
    };

    let generate_random_iv = move |_| {
        if let Some(bytes) = generate_random_bytes(16) {
            let hex = bytes_to_hex(&bytes);
            set_iv_input(hex);
        }
    };

    view! {
        <div class="form-group">
            <div class="label-header">
                <label>"Initialization Vector (IV)"</label>
                <div class="header-actions">
                    <span class="input-hint">"16 bytes (32 hex chars)"</span>
                    <button
                        type="button"
                        class="btn-generate"
                        on:click=generate_random_iv
                        title="Generate random IV"
                    >
                        "Random"
                    </button>
                </div>
            </div>
            <div class="input-wrapper hex-input">
                <span class="prefix">"0x"</span>
                <input
                    type="text"
                    placeholder="000102030405060708090A0B0C0D0E0F"
                    prop:value=move || iv_input.get()
                    on:input=handle_hex_input
                    spellcheck="false"
                    maxlength="32"
                />
            </div>
        </div>
    }
    .into_any()
}
