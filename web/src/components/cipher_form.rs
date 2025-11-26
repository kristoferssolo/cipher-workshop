use cipher_factory::prelude::*;
use leptos::prelude::*;
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
        let text = text_input.get();

        if key.is_empty() || text.is_empty() {
            set_error_msg("Please enter both key and input text.".to_string());
            return;
        }

        let context = CipherContext::new(algorithm, mode.get(), key, text, output_fmt.get());
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
            <TextInput mode=mode set_text_input=set_text_input />

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
) -> impl IntoView {
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
}

#[component]
fn ConfigurationSection(
    mode: ReadSignal<OperationMode>,
    set_mode: WriteSignal<OperationMode>,
    output_fmt: ReadSignal<OutputFormat>,
    update_output: impl Fn(OutputFormat) + Copy + Send + 'static,
) -> impl IntoView {
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
    }
}

#[component]
fn KeyInput(set_key_input: WriteSignal<String>) -> impl IntoView {
    view! {
        <div class="form-group">
            <label>"Secret Key"</label>
            <input
                type="text"
                prop:key_input
                placeholder="Enter key..."
                on:input=move |ev| set_key_input(event_target_value(&ev))
            />
        </div>
    }
}

#[component]
fn TextInput(
    mode: ReadSignal<OperationMode>,
    set_text_input: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="form-group">
            <label>
                {move || {
                    match mode.get() {
                        OperationMode::Encrypt => "Plaintext Input",
                        OperationMode::Decrypt => "Ciphertext (Hex) Input",
                    }
                }}
            </label>
            <input
                type="text"
                prop:text_input
                placeholder="Enter text..."
                on:input=move |ev| set_text_input(event_target_value(&ev))
            />
        </div>
    }
}

#[component]
fn OutputBox(
    output: ReadSignal<String>,
    output_fmt: ReadSignal<OutputFormat>,
    copy_to_clipboard: impl Fn(String) + Copy + Send + 'static,
    copy_feedback: ReadSignal<bool>,
) -> impl IntoView {
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
}

#[component]
fn ErrorBox(error_msg: ReadSignal<String>) -> impl IntoView {
    view! {
        {move || {
            if error_msg.get().is_empty() {
                return view! { <span></span> }.into_any();
            }
            view! { <div class="error-box">{error_msg.get()}</div> }.into_any()
        }}
    }
}
