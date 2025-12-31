use js_sys::{ArrayBuffer, Uint8Array};
use leptos::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Event, File, FileList, FileReader, HtmlInputElement};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputMode {
    Text,
    File,
}

#[component]
pub fn FileTextInput(
    input_mode: ReadSignal<InputMode>,
    set_input_mode: WriteSignal<InputMode>,
    text_content: ReadSignal<String>,
    set_text_content: WriteSignal<String>,
    file_data: ReadSignal<Option<Vec<u8>>>,
    set_file_data: WriteSignal<Option<Vec<u8>>>,
    file_name: ReadSignal<Option<String>>,
    set_file_name: WriteSignal<Option<String>>,
    is_decrypt_mode: Memo<bool>,
) -> AnyView {
    let handle_file_change = move |ev: Event| {
        let target = ev.target().unwrap();
        let input: HtmlInputElement = target.unchecked_into();

        if let Some(files) = input.files() {
            let files: FileList = files;
            if let Some(file) = files.get(0) {
                let file: File = file;
                let name = file.name();
                set_file_name(Some(name));

                let reader = FileReader::new().unwrap();
                let reader_clone = reader.clone();

                let onload = Closure::wrap(Box::new(move |_: web_sys::ProgressEvent| {
                    if let Ok(result) = reader_clone.result()
                        && let Some(array_buffer) = result.dyn_ref::<ArrayBuffer>()
                    {
                        let uint8_array = Uint8Array::new(array_buffer);
                        let data: Vec<u8> = uint8_array.to_vec();
                        set_file_data(Some(data));
                    }
                }) as Box<dyn FnMut(_)>);

                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();

                let _ = reader.read_as_array_buffer(&file);
            }
        }
    };

    let handle_text_change = move |ev: Event| {
        let target = ev.target().unwrap();
        let textarea: web_sys::HtmlTextAreaElement = target.unchecked_into();
        set_text_content(textarea.value());
    };

    view! {
        <div class="form-group">
            <div class="label-header">
                <label>{move || if is_decrypt_mode.get() { "Ciphertext Input" } else { "Plaintext Input" }}</label>
                <div class="input-mode-toggle">
                    <button
                        type="button"
                        class=move || if input_mode.get() == InputMode::Text { "mode-btn active" } else { "mode-btn" }
                        on:click=move |_| set_input_mode(InputMode::Text)
                    >
                        "Text"
                    </button>
                    <button
                        type="button"
                        class=move || if input_mode.get() == InputMode::File { "mode-btn active" } else { "mode-btn" }
                        on:click=move |_| set_input_mode(InputMode::File)
                    >
                        "File"
                    </button>
                </div>
            </div>

            {move || {
                match input_mode.get() {
                    InputMode::Text => {
                        view! {
                            <div class="textarea-wrapper">
                                <textarea
                                    rows="6"
                                    placeholder=move || {
                                        if is_decrypt_mode.get() {
                                            "Paste ciphertext here (hex format)..."
                                        } else {
                                            "Enter or paste your plaintext here..."
                                        }
                                    }
                                    prop:value=move || text_content.get()
                                    on:input=handle_text_change
                                    spellcheck="false"
                                ></textarea>
                                <div class="char-count">
                                    {move || {
                                        let len = text_content.get().len();
                                        format!("{} characters", len)
                                    }}
                                </div>
                            </div>
                        }.into_any()
                    }
                    InputMode::File => {
                        view! {
                            <div class="file-upload-area">
                                <input
                                    type="file"
                                    id="file-input"
                                    on:change=handle_file_change
                                    class="file-input-hidden"
                                />
                                <label for="file-input" class="file-upload-label">
                                    {move || {
                                        match file_name.get() {
                                            Some(name) => view! {
                                                <div class="file-selected">
                                                    <span class="file-icon">"[FILE]"</span>
                                                    <span class="file-name">{name}</span>
                                                    <span class="file-size">
                                                        {move || {
                                                            file_data.get().map(|d| format!("({} bytes)", d.len())).unwrap_or_default()
                                                        }}
                                                    </span>
                                                </div>
                                            }.into_any(),
                                            None => view! {
                                                <div class="file-placeholder">
                                                    <span class="upload-icon">"[+]"</span>
                                                    <span>"Click to select a file or drag and drop"</span>
                                                </div>
                                            }.into_any(),
                                        }
                                    }}
                                </label>
                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
    .into_any()
}
