use crate::components::{
    config_section::ConfigurationSection,
    error_box::ErrorBox,
    file_input::{FileTextInput, InputMode},
    iv_input::IvInput,
    key_input::{KeyInput, KeySize},
};
use cipher_factory::prelude::*;
use js_sys::{Array, Uint8Array};
use leptos::prelude::*;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::{Blob, Url};

#[component]
pub fn CipherFormCbc() -> AnyView {
    let (mode, set_mode) = signal(OperationMode::Encrypt);
    let (output_fmt, set_output_fmt) = signal(OutputFormat::Hex);

    let (key_input, set_key_input) = signal(String::new());
    let (iv_input, set_iv_input) = signal(String::new());

    // Input mode and content
    let (input_mode, set_input_mode) = signal(InputMode::Text);
    let (text_content, set_text_content) = signal(String::new());
    let (file_data, set_file_data) = signal(Option::<Vec<u8>>::None);
    let (file_name, set_file_name) = signal(Option::<String>::None);

    // Output state
    let (output, set_output) = signal(String::new());
    let (output_bytes, set_output_bytes) = signal(Option::<Vec<u8>>::None);
    let (error_msg, set_error_msg) = signal(String::new());
    let (copy_feedback, set_copy_feedback) = signal(false);

    let is_decrypt_mode = Memo::new(move |_| mode.get() == OperationMode::Decrypt);

    let handle_submit = move || {
        set_error_msg(String::new());
        set_output(String::new());
        set_output_bytes(None);

        let key = key_input.get();
        let iv = iv_input.get();

        if key.is_empty() {
            set_error_msg("Please enter a secret key.".to_string());
            return;
        }

        if iv.is_empty() {
            set_error_msg("Please enter an initialization vector (IV).".to_string());
            return;
        }

        // Format IV with 0x prefix (key keeps user format, IV is always hex)
        let formatted_iv = format!("0x{iv}");

        // Get input data
        let input_data = match input_mode.get() {
            InputMode::Text => {
                let text = text_content.get();
                if text.is_empty() {
                    set_error_msg("Please enter input text or select a file.".to_string());
                    return;
                }
                if mode.get() == OperationMode::Decrypt {
                    // Parse hex input for decryption
                    match parse_hex_string(&text) {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            set_error_msg(e);
                            return;
                        }
                    }
                } else {
                    text.into_bytes()
                }
            }
            InputMode::File => match file_data.get() {
                Some(data) => data,
                None => {
                    set_error_msg("Please select a file.".to_string());
                    return;
                }
            },
        };

        // Process encryption/decryption
        match mode.get() {
            OperationMode::Encrypt => {
                match Algorithm::AesCbc.encrypt_cbc(&key, &formatted_iv, &input_data) {
                    Ok(ciphertext) => {
                        let hex_output = bytes_to_hex(&ciphertext);
                        set_output(hex_output);
                        set_output_bytes(Some(ciphertext));
                    }
                    Err(e) => set_error_msg(e.to_string()),
                }
            }
            OperationMode::Decrypt => {
                match Algorithm::AesCbc.decrypt_cbc(&key, &formatted_iv, &input_data) {
                    Ok(plaintext) => {
                        set_output_bytes(Some(plaintext.clone()));
                        let formatted = match output_fmt.get() {
                            OutputFormat::Text => {
                                String::from_utf8(plaintext).unwrap_or_else(|_| {
                                    set_error_msg(
                                        "Output contains invalid UTF-8. Try Hex format."
                                            .to_string(),
                                    );
                                    String::new()
                                })
                            }
                            OutputFormat::Hex => bytes_to_hex(&plaintext),
                            OutputFormat::Binary => bytes_to_binary(&plaintext),
                            OutputFormat::Octal => bytes_to_octal(&plaintext),
                        };
                        set_output(formatted);
                    }
                    Err(e) => set_error_msg(e.to_string()),
                }
            }
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

    let download_output = move |_| {
        if let Some(bytes) = output_bytes.get() {
            let original_name = file_name.get().unwrap_or_else(|| "output".to_string());
            let download_name = if mode.get() == OperationMode::Encrypt {
                format!("{original_name}.enc")
            } else {
                original_name
                    .strip_suffix(".enc")
                    .map_or_else(|| format!("{original_name}.dec"), String::from)
            };

            download_bytes(&bytes, &download_name);
        }
    };

    view! {
        <div class="cipher-card">
            <div class="card-header">
                <h2>"AES-CBC"</h2>
            </div>
            <ConfigurationSection
                mode=mode
                set_mode=set_mode
                output_fmt=output_fmt
                update_output=update_output
            />
            <KeyInput key_input=key_input set_key_input=set_key_input key_size=KeySize::Aes128 />
            <IvInput iv_input=iv_input set_iv_input=set_iv_input />

            <FileTextInput
                input_mode=input_mode
                set_input_mode=set_input_mode
                text_content=text_content
                set_text_content=set_text_content
                file_data=file_data
                set_file_data=set_file_data
                file_name=file_name
                set_file_name=set_file_name
                is_decrypt_mode=is_decrypt_mode
            />

            <button class="btn-primary" on:click=move |_| handle_submit()>
                {move || format!("{} using AES-CBC", mode.get())}
            </button>

            // Output section
            {move || {
                if output.get().is_empty() {
                    return view! { <span></span> }.into_any();
                }
                view! {
                    <div class="result-box">
                        <div class="result-toolbar">
                            <strong>"Output ("{output_fmt.get().to_string()}")"</strong>
                            <div class="result-actions">
                                <button
                                    class="btn-copy"
                                    on:click=move |_| copy_to_clipboard(output.get())
                                >
                                    {move || if copy_feedback.get() { "Copied" } else { "Copy" }}
                                </button>
                                {move || {
                                    if output_bytes.get().is_some() {
                                        view! {
                                            <button class="btn-download" on:click=download_output>
                                                "Download"
                                            </button>
                                        }
                                            .into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                }}
                            </div>
                        </div>
                        <div class="result-content">
                            <code>
                                {move || {
                                    let out = output.get();
                                    if out.len() > 1000 {
                                        format!("{}... ({} chars total)", &out[..1000], out.len())
                                    } else {
                                        out
                                    }
                                }}
                            </code>
                        </div>
                    </div>
                }
                    .into_any()
            }}
            <ErrorBox error_msg=error_msg />
        </div>
    }
    .into_any()
}

fn parse_hex_string(s: &str) -> Result<Vec<u8>, String> {
    let trimmed = s.trim();
    let s = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);

    // Remove whitespace and newlines
    let s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();

    if !s.len().is_multiple_of(2) {
        return Err("Hex string must have even length".to_string());
    }

    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| format!("Invalid hex at position {i}"))
        })
        .collect()
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

fn bytes_to_binary(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:08b}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn bytes_to_octal(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:03o}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn download_bytes(bytes: &[u8], filename: &str) {
    let uint8_array = Uint8Array::new_with_length(bytes.len() as u32);
    uint8_array.copy_from(bytes);

    let array = Array::new();
    array.push(&uint8_array.buffer());

    let Some(blob) = Blob::new_with_u8_array_sequence(&array).ok() else {
        return;
    };
    let Some(url) = Url::create_object_url_with_blob(&blob).ok() else {
        return;
    };

    let Some(document) = window().document() else {
        return;
    };
    let Some(a) = document.create_element("a").ok() else {
        return;
    };

    let _ = a.set_attribute("href", &url);
    let _ = a.set_attribute("download", filename);

    let a = a.unchecked_into::<web_sys::HtmlElement>();
    a.click();

    let _ = Url::revoke_object_url(&url);
}
