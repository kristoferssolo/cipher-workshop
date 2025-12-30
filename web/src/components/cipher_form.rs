use cipher_factory::prelude::*;
use leptos::{prelude::*, tachys::dom::event_target_value};
use std::{str::FromStr, time::Duration};
use strum::IntoEnumIterator;
use web_sys::WheelEvent;

#[component]
pub fn CipherForm(algorithm: Algorithm) -> impl IntoView {
    let (mode, set_mode) = signal(OperationMode::Encrypt);
    let (output_fmt, set_output_fmt) = signal(OutputFormat::Hex);

    let (key_input, set_key_input) = signal(String::new());
    let (text_input, set_text_input) = signal(String::new());

    let (output, set_output) = signal(String::new());
    let (error_msg, set_error_msg) = signal(String::new());

    let (copy_feedback, set_copy_feedback) = signal(false);

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

        let context = CipherContext::new(algorithm, mode.get(), key, final_text, output_fmt.get());
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
            <KeyInput set_key_input=set_key_input />
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
}

#[component]
fn RadioButton(
    value: OperationMode,
    current: ReadSignal<OperationMode>,
    set_current: WriteSignal<OperationMode>,
) -> AnyView {
    view! {
        <div class="radio-button">
            <label>
                <input
                    type="radio"
                    name="crypto-mode"
                    value=value.to_string()
                    prop:checked=move || current.get() == value
                    on:change=move |_| set_current.set(value)
                />
                {value.to_string()}
            </label>
        </div>
    }
    .into_any()
}

#[component]
fn ConfigurationSection(
    mode: ReadSignal<OperationMode>,
    set_mode: WriteSignal<OperationMode>,
    output_fmt: ReadSignal<OutputFormat>,
    update_output: impl Fn(OutputFormat) + Copy + Send + 'static,
) -> AnyView {
    let handle_format_change = move |ev| {
        let val = event_target_value(&ev);
        let fmt = OutputFormat::from_str(&val).unwrap_or_default();
        update_output(fmt);
    };

    let handle_format_wheel = move |ev: WheelEvent| {
        ev.prevent_default();

        let formats = OutputFormat::iter().collect::<Vec<_>>();
        let current_idx = formats
            .iter()
            .position(|f| *f == output_fmt.get())
            .unwrap_or(2);

        let next_idx = if ev.delta_y() > 0.0 {
            (current_idx + 1) % formats.len()
        } else if current_idx == 0 {
            formats.len() - 1
        } else {
            current_idx - 1
        };
        update_output(formats[next_idx]);
    };

    view! {
        <div class="form-group">
            <label>"Configuration"</label>
            <div class="controls-row">
                <div class="radio-group">
                    <RadioButton value=OperationMode::Encrypt current=mode set_current=set_mode />
                    <RadioButton value=OperationMode::Decrypt current=mode set_current=set_mode />
                </div>
                {move || {
                    if mode.get() != OperationMode::Decrypt {
                        return view! { <span></span> }.into_any();
                    }
                    view! {
                        <div class="format-controls-box">
                            <div class="format-controls">
                                <label>"Output format:"</label>
                                <select
                                    on:wheel=handle_format_wheel
                                    on:change=handle_format_change
                                    prop:value=move || output_fmt.get().to_string()
                                >
                                    {OutputFormat::iter()
                                        .map(|fmt| {
                                            view! {
                                                <option value=fmt.to_string()>{fmt.to_string()}</option>
                                            }
                                        })
                                        .collect_view()}
                                </select>
                            </div>
                        </div>
                    }
                        .into_any()
                }}
            </div>
        </div>
    }.into_any()
}

fn clean_hex_input(input: String) -> String {
    input
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .collect::<String>()
}

#[component]
fn KeyInput(set_key_input: WriteSignal<String>) -> AnyView {
    view! {
        <div class="form-group">
            <div class="lable-header">
                <label>"Secret Key"</label>
                <span class="input-hint">"Prefix: 0x (Hex), 0b (Bin), or nothing (Text)"</span>
            </div>
            <input
                type="text"
                placeholder="Enter key (e.g., 0x1A2B...)"
                on:input=move |ev| set_key_input(event_target_value(&ev))
            />
        </div>
    }
    .into_any()
}

#[component]
fn TextInput(
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
                            <div class="lable-header">
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
                            <div class="lable-header">
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

#[component]
fn OutputBox(
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
                                if copy_feedback.get() { "✔️ Copied" } else { "📋 Copy" }
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

#[component]
fn ErrorBox(error_msg: ReadSignal<String>) -> AnyView {
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
