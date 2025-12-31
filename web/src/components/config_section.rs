use crate::components::radio_button::RadioButton;
use cipher_factory::prelude::*;
use leptos::{prelude::*, tachys::dom::event_target_value};
use std::str::FromStr;
use strum::IntoEnumIterator;
use web_sys::WheelEvent;

#[component]
pub fn ConfigurationSection(
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
