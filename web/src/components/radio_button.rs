use cipher_factory::prelude::OperationMode;
use leptos::prelude::*;

#[component]
pub fn RadioButton(
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
