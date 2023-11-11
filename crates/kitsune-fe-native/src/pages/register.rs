use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::components::Link;

use crate::components::form::{FormButton, FormInput};

pub fn Signup(cx: Scope) -> Element {
    let name = use_state(&cx, String::new);
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    render! {
        div {
            class: "auth-page",
            div {
                class: "container page",
                div {
                    class: "row",
                    div {
                        class: "col-md-6 offset-md-3 col-xs-12",
                        h1 {
                            class: "text-xs-center",
                            "Sign up"
                        }
                        p {
                            class: "text-xs-center",
                            Link { to: "/signin", "Have an account?" }
                        }
                        br {}
                        br {}

                        ul {
                            class: "error-messages",
                            li { "That email is already taken" }
                        }

                        form {
                            FormInput {
                                oninput: move |s: FormData| name.set(s.value),
                                placeholder: "Your Name".to_string()
                            }
                            FormInput {
                                oninput: move |s: FormData| email.set(s.value),
                                placeholder: "Email".to_string()
                            }
                            FormInput {
                                oninput: move |s: FormData| password.set(s.value),
                                placeholder: "Password".to_string()
                            }
                            FormButton {
                                onclick: move |_: MouseEvent| {
                                    println!(":: SignUpPage] button clicked. name: {} | email: {}", name, email);

                                },
                                label: "Sign up".to_string()
                            }
                        }
                    }
                }
            }
        }
    }
}
