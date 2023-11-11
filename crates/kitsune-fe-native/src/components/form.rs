use dioxus::{events::MouseEvent, prelude::*};

#[derive(Props)]
pub struct FormButtonProps<'a> {
    onclick: EventHandler<'a, MouseEvent>,
    label: String,
}

pub fn FormButton<'a>(cx: Scope<'a, FormButtonProps<'a>>) -> Element {
    render! {
        button {
            class: "btn btn-lg btn-primary pull-xs-right",
            r#type: "button",
            onclick: move |evt| cx.props.onclick.call(evt),
            "{cx.props.label}"
        }
    }
}

#[derive(Props)]
pub struct FormInputProps<'a> {
    oninput: EventHandler<'a, FormData>,

    #[props(optional)]
    placeholder: Option<String>,
}

pub fn FormInput<'a>(cx: Scope<'a, FormInputProps<'a>>) -> Element {
    let ph = cx.props.placeholder.clone().unwrap_or_default();
    render! {
        fieldset {
            class: "form-group",
            input {
                class: "form-control form-control-lg",
                r#type: "text",
                oninput: move |evt| cx.props.oninput.call(evt.data.as_ref().clone()),
                placeholder: "{ph}",

            }
        }
    }
}

#[derive(Props)]
pub struct FormTextareaProps<'a> {
    oninput: EventHandler<'a, FormData>,

    #[props(optional)]
    rows: Option<u8>,

    #[props(optional)]
    placeholder: Option<String>,
}

pub fn FormTextarea<'a>(cx: Scope<'a, FormTextareaProps<'a>>) -> Element {
    let rows = cx.props.rows.unwrap_or(1);
    let ph = cx.props.placeholder.clone().unwrap_or_default();
    render! {
        fieldset {
            class: "form-group",
            textarea {
                class: "form-control form-control-lg",
                oninput: move |evt| cx.props.oninput.call(evt.data.as_ref().clone()),
                placeholder: "{ph}",
                rows: "{rows}"
            }
        }
    }
}
