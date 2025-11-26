use cipher_factory::prelude::*;
use leptos::prelude::*;
use std::str::FromStr;
use strum::IntoEnumIterator;

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
        set_copy_feedback(false);

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

    view! {
        <div class="cipher-card">
            <div class="card-header">
                <h2>{algorithm.to_string()}</h2>
            </div>

            <div class="form-group">
                <label>"Configuration"</label>
                <div class="controls-row">
                    <div class="radio-group">
                        <RadioButton
                            value=OperationMode::Encrypt
                            current=mode
                            set_current=set_mode
                        />
                        <RadioButton
                            value=OperationMode::Decrypt
                            current=mode
                            set_current=set_mode
                        />
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
                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            let fmt = OutputFormat::from_str(&val).unwrap_or_default();
                                            set_output_fmt(fmt);
                                            if !output.get().is_empty() {
                                                handle_submit();
                                            }
                                        }
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
            <div class="form-group">
                <label>"Secret Key"</label>
                <input
                    type="text"
                    prop:key_input
                    placeholder="Enter key..."
                    on:input=move |ev| set_key_input(event_target_value(&ev))
                />
            </div>
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

            <button class="btn-primary" on:click=move |_| handle_submit()>
                {move || format!("{} using {algorithm}", mode.get())}
            </button>

            // Output Section
            {move || {
                if output.get().is_empty() {
                    return view! { <span></span> }.into_any();
                }
                view! {
                    <div class="result-box">
                        <div class="result-toolbar">
                            <strong>"Output ("{output_fmt.get().to_string()}")"</strong>
                            <code>{output.get()}</code>
                        </div>
                    </div>
                }
                    .into_any()
            }}

            // Error Section
            {move || {
                if error_msg.get().is_empty() {
                    return view! { <span></span> }.into_any();
                }
                view! { <div class="error-box">{error_msg.get()}</div> }.into_any()
            }}
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
