use js_sys::Uint8Array;
use leptos::{prelude::*, tachys::dom::event_target_value};

fn generate_random_bytes(len: usize) -> Option<Vec<u8>> {
    let window = web_sys::window()?;
    let crypto = window.crypto().ok()?;
    let array = Uint8Array::new_with_length(len as u32);
    crypto.get_random_values_with_array_buffer_view(&array).ok()?;
    Some(array.to_vec())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

/// Key sizes in bytes for different algorithms
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeySize {
    /// DES: 8 bytes (64 bits, though only 56 are used)
    Des,
    /// AES-128: 16 bytes (128 bits)
    Aes128,
}

impl KeySize {
    const fn bytes(self) -> usize {
        match self {
            Self::Des => 8,
            Self::Aes128 => 16,
        }
    }
}

#[component]
pub fn KeyInput(
    key_input: ReadSignal<String>,
    set_key_input: WriteSignal<String>,
    #[prop(default = KeySize::Aes128)] key_size: KeySize,
) -> AnyView {
    let generate_random_key = move |_| {
        if let Some(bytes) = generate_random_bytes(key_size.bytes()) {
            let hex = format!("0x{}", bytes_to_hex(&bytes));
            set_key_input(hex);
        }
    };

    view! {
        <div class="form-group">
            <div class="label-header">
                <label>"Secret Key"</label>
                <div class="header-actions">
                    <span class="input-hint">"Prefix: 0x (Hex), 0b (Bin), or nothing (Text)"</span>
                    <button
                        type="button"
                        class="btn-generate"
                        on:click=generate_random_key
                        title="Generate random key"
                    >
                        "Random"
                    </button>
                </div>
            </div>
            <input
                type="text"
                placeholder="Enter key (e.g., 0x1A2B...)"
                prop:value=move || key_input.get()
                on:input=move |ev| set_key_input(event_target_value(&ev))
                spellcheck="false"
            />
        </div>
    }
    .into_any()
}
