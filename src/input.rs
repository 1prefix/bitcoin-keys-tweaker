use crate::parse_secret_key;
use std::fmt;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
enum NotificationStatus {
    Success,
    Error,
}

impl fmt::Display for NotificationStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let status = match self {
            Self::Success => "success",
            Self::Error => "error",
        };
        write!(fmt, "{}", status)
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    pub label: String,

    #[prop_or_default]
    pub on_secret_key_change: Callback<Option<String>>,
}

#[function_component]
pub fn InputComponent(props: &InputProps) -> Html {
    let secret_key_value = use_state(|| String::new());
    let notification: UseStateHandle<Option<(String, NotificationStatus)>> = use_state(|| None);

    let on_input = {
        let secret_key_value_clone = secret_key_value.clone();
        let notification_clone = notification.clone();

        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let _ = secret_key_value_clone.set(input.value());
                let _ = notification_clone.set(None);
            }
        })
    };

    let on_change = {
        let notification_clone = notification.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let notification = match input.value().clone().len().gt(&0) {
                    true => match parse_secret_key(input.value().clone()).is_some() {
                        true => Some((
                            String::from("Valid Secret Key"),
                            NotificationStatus::Success,
                        )),
                        false => Some((
                            String::from("Invalid Secret Key"),
                            NotificationStatus::Error,
                        )),
                    },
                    false => None,
                };

                let _ = notification_clone.set(notification);
            }
        })
    };

    {
        let secret_key_value_clone = secret_key_value.clone();
        let on_secret_key_change_clone = props.on_secret_key_change.clone();
        let notification_clone = notification.clone();

        use_effect_with(notification.clone(), move |_| {
            let public_key = match (*notification_clone).clone() {
                Some((_, NotificationStatus::Success)) => Some((*secret_key_value_clone).clone()),
                _ => None,
            };

            let _ = on_secret_key_change_clone.emit(public_key);
        })
    }

    {
        let public_key_value_clone = secret_key_value.clone();
        let notification_clone = notification.clone();

        use_effect_with(secret_key_value.clone(), move |_| {
            if (*public_key_value_clone).clone().len().eq(&0) {
                let _ = notification_clone.set(None);
            }
        })
    }

    let clear_input = {
        let public_key_value_clone = secret_key_value.clone();
        move |_| {
            let _ = public_key_value_clone.set(String::new());
        }
    };

    html!(<>
        <div class="input_container">
            <p>{props.label.clone()}</p>
            <input
                onchange={on_change}
                oninput={on_input}
                autocomplete="off"
                value={(*secret_key_value).clone()}
            />
            {match (*secret_key_value).clone().len().gt(&0) {
                true => html!(<div class="clear_input" onclick={clear_input}>{"X"}</div>),
                false => html!()
            }}
        </div>
        <div class="input_notification_container">
            {match (*notification).clone() {
                Some((message, status)) => html!(<div class={format!("message {}", status)}>{message}</div>),
                None => html!()
            }}
        </div>
    </>)
}
