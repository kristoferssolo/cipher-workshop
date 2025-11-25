use leptos::prelude::*;
type LogicFn = Box<dyn Fn(bool, String, String) -> (String, String)>;

#[component]
pub fn CipherForm(title: &'static str, logic: LogicFn) -> impl IntoView {
    let (mode, set_mode) = signal("Encrypt".to_string());
    let (key_input, set_key_input) = signal(String::new());
    let (text_input, set_text_input) = signal(String::new());
    let (output, set_output) = signal(String::new());
    let (error_msg, set_error_msg) = signal(String::new());

    let handle_submit = move |_| {
        set_error_msg(String::new());
        set_output(String::new());

        let is_encrypt = mode.get() == "Encrypt";
        let key = key_input.get();
        let text = text_input.get();

        if key.is_empty() || text.is_empty() {
            set_error_msg("Please enter both key and text/hex.".to_string());
            return;
        }

        let (res_out, res_err) = logic(is_encrypt, key, text);

        if !res_err.is_empty() {
            set_error_msg(res_err);
            return;
        }
        set_output(res_out);
    };

    view! {
        <div class="cipher-card">
            <h2>{title} " Encryption"</h2>
            <div class="form-group">
                <label>"Operation Mdoe"</label>
                <div class="radio-group">
                    <label>
                        <input
                            type="radio"
                            name="mode"
                            value="Encrypt"
                            checked=move || mode.get() == "Encrypt"
                            on:change=move |ev| set_mode(event_target_value(&ev))
                        />
                        "Encrypt"
                    </label>
                    <label>
                        <input
                            type="radio"
                            name="mode"
                            value="Decrypt"
                            checked=move || mode.get() == "Decrypt"
                            on:change=move |ev| set_mode(event_target_value(&ev))
                        />
                        "Decrypt"
                    </label>
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
                        if mode.get() == "Encrypt" {
                            "Plaintext Input"
                        } else {
                            "Ciphertext (Hex) Input"
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

            <button class="btn-primary" on:click=handle_submit>
                {move || format!("Run {title} {}", mode.get())}
            </button>

            {move || {
                if error_msg.get().is_empty() {
                    view! { <span></span> }.into_any()
                } else {
                    view! { <div class="error-box">{error_msg.get()}</div> }.into_any()
                }
            }}

            {move || {
                if output.get().is_empty() {
                    view! { <span></span> }.into_any()
                } else {
                    view! {
                        <div class="result-box">
                            <strong>"Output:"</strong>
                            <code>{output.get()}</code>
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
