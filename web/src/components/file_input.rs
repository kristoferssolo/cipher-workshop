use js_sys::{ArrayBuffer, Uint8Array};
use leptos::prelude::*;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{DragEvent, Event, File, FileReader, HtmlInputElement};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputMode {
    Text,
    File,
}

fn read_file(
    file: File,
    set_file_name: WriteSignal<Option<String>>,
    set_file_data: WriteSignal<Option<Vec<u8>>>,
) {
    let name = file.name();
    set_file_name(Some(name));

    let Some(reader) = FileReader::new().ok() else {
        return;
    };
    let reader_clone = reader.clone();

    let onload = Closure::wrap(Box::new(move |_: web_sys::ProgressEvent| {
        if let Ok(result) = reader_clone.result()
            && let Some(array_buffer) = result.dyn_ref::<ArrayBuffer>()
        {
            let uint8_array = Uint8Array::new(array_buffer);
            set_file_data(Some(uint8_array.to_vec()));
        }
    }) as Box<dyn FnMut(_)>);

    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();

    let _ = reader.read_as_array_buffer(&file);
}

#[component]
fn InputModeToggle(
    input_mode: ReadSignal<InputMode>,
    set_input_mode: WriteSignal<InputMode>,
) -> impl IntoView {
    view! {
        <div class="input-mode-toggle">
            <button
                type="button"
                class=move || {
                    if input_mode.get() == InputMode::Text { "mode-btn active" } else { "mode-btn" }
                }
                on:click=move |_| set_input_mode(InputMode::Text)
            >
                "Text"
            </button>
            <button
                type="button"
                class=move || {
                    if input_mode.get() == InputMode::File { "mode-btn active" } else { "mode-btn" }
                }
                on:click=move |_| set_input_mode(InputMode::File)
            >
                "File"
            </button>
        </div>
    }
}

#[component]
fn TextAreaInput(
    text_content: ReadSignal<String>,
    set_text_content: WriteSignal<String>,
    is_decrypt_mode: Memo<bool>,
) -> impl IntoView {
    let handle_text_change = move |ev: Event| {
        if let Some(target) = ev.target() {
            let textarea: web_sys::HtmlTextAreaElement = target.unchecked_into();
            set_text_content(textarea.value());
        }
    };

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
                {move || format!("{} characters", text_content.get().len())}
            </div>
        </div>
    }
}

#[component]
fn FileDropZone(
    file_data: ReadSignal<Option<Vec<u8>>>,
    set_file_data: WriteSignal<Option<Vec<u8>>>,
    file_name: ReadSignal<Option<String>>,
    set_file_name: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (is_dragging, set_is_dragging) = signal(false);

    let handle_file_change = move |ev: Event| {
        if let Some(target) = ev.target() {
            let input: HtmlInputElement = target.unchecked_into();
            if let Some(files) = input.files()
                && let Some(file) = files.get(0)
            {
                read_file(file, set_file_name, set_file_data);
            }
        }
    };

    let handle_drag_over = move |ev: DragEvent| {
        ev.prevent_default();
        set_is_dragging(true);
    };

    let handle_drag_enter = move |ev: DragEvent| {
        ev.prevent_default();
        set_is_dragging(true);
    };

    let handle_drag_leave = move |ev: DragEvent| {
        ev.prevent_default();
        set_is_dragging(false);
    };

    let handle_drop = move |ev: DragEvent| {
        ev.prevent_default();
        set_is_dragging(false);

        if let Some(data_transfer) = ev.data_transfer()
            && let Some(files) = data_transfer.files()
            && let Some(file) = files.get(0)
        {
            read_file(file, set_file_name, set_file_data);
        }
    };

    view! {
        <div
            class="file-upload-area"
            on:dragover=handle_drag_over
            on:dragenter=handle_drag_enter
            on:dragleave=handle_drag_leave
            on:drop=handle_drop
        >
            <input
                type="file"
                id="file-input"
                on:change=handle_file_change
                class="file-input-hidden"
            />
            <label
                for="file-input"
                class=move || {
                    if is_dragging.get() {
                        "file-upload-label dragging"
                    } else {
                        "file-upload-label"
                    }
                }
            >
                {move || {
                    file_name
                        .get()
                        .map_or_else(
                            || view! { <FilePlaceholder /> }.into_any(),
                            |name| {
                                view! {
                                    <FileSelected
                                        name=name
                                        file_data=file_data
                                        set_file_name=set_file_name
                                        set_file_data=set_file_data
                                    />
                                }
                                    .into_any()
                            },
                        )
                }}
            </label>
        </div>
    }
}

#[component]
fn FileSelected(
    name: String,
    file_data: ReadSignal<Option<Vec<u8>>>,
    set_file_name: WriteSignal<Option<String>>,
    set_file_data: WriteSignal<Option<Vec<u8>>>,
) -> impl IntoView {
    let clear_file = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        set_file_name(None);
        set_file_data(None);
    };

    view! {
        <div class="file-selected">
            <span class="file-icon">"[FILE]"</span>
            <span class="file-name">{name}</span>
            <span class="file-size">
                {move || file_data.get().map_or(String::new(), |d| format!("({} bytes)", d.len()))}
            </span>
            <button type="button" class="btn-clear-file" on:click=clear_file title="Remove file">
                "X"
            </button>
        </div>
    }
}

#[component]
fn FilePlaceholder() -> impl IntoView {
    view! {
        <div class="file-placeholder">
            <span class="upload-icon">"[+]"</span>
            <span>"Click to select a file or drag and drop"</span>
        </div>
    }
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
) -> impl IntoView {
    view! {
        <div class="form-group">
            <div class="label-header">
                <label>
                    {move || {
                        if is_decrypt_mode.get() { "Ciphertext Input" } else { "Plaintext Input" }
                    }}
                </label>
                <InputModeToggle input_mode=input_mode set_input_mode=set_input_mode />
            </div>

            {move || match input_mode.get() {
                InputMode::Text => {
                    view! {
                        <TextAreaInput
                            text_content=text_content
                            set_text_content=set_text_content
                            is_decrypt_mode=is_decrypt_mode
                        />
                    }
                        .into_any()
                }
                InputMode::File => {
                    view! {
                        <FileDropZone
                            file_data=file_data
                            set_file_data=set_file_data
                            file_name=file_name
                            set_file_name=set_file_name
                        />
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
