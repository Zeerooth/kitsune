use cynic::{http::SurfExt, MutationBuilder};
use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::components::Link;

use crate::{
    components::form::{FormButton, FormInput},
    schema::{Register, RegisterVariables},
    state::{AuthState, Homeserver},
};

pub fn AuthInfo(cx: Scope) -> Element {
    let state = use_shared_state::<AuthState>(cx).unwrap();

    match *state.read() {
        AuthState::Login => render! {
            Login {}
        },
        AuthState::Register => render! {
            Register {}
        },
        AuthState::LoggedIn(_) => todo!(),
    }
}

pub fn Login(cx: Scope) -> Element {
    let username = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    let auth_state = use_shared_state::<AuthState>(cx).unwrap();

    render! {
        div {
            class: "auth-page panel",
            div {
                class: "form",
                h1 {
                    class: "text-xs-center",
                    "Sign up"
                }
                p {
                    class: "text-xs-center",
                    a {
                        onclick: move |_| { *auth_state.write() = AuthState::Register },
                        "Don't have an account?"
                    }
                }
                br {}
                br {}

                form {
                    FormInput {
                        oninput: move |s: FormData| username.set(s.value),
                        placeholder: "Username".to_string()
                    }
                    FormInput {
                        oninput: move |s: FormData| password.set(s.value),
                        placeholder: "Password".to_string()
                    }
                    FormButton {
                        onclick: move |_: MouseEvent| {
                            println!(":: Login] button clicked. username: {}", username);
                            to_owned![username, password];
                            cx.spawn(async move {

                            });
                        },
                        label: "Login".to_string()
                    }
                }
            }
        }
    }
}

pub fn Register(cx: Scope) -> Element {
    let username = use_state(&cx, String::new);
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);
    let auth_state = use_shared_state::<AuthState>(cx).unwrap();
    let homeserver = use_shared_state::<Homeserver>(cx).unwrap();

    render! {
        div {
            class: "auth-page panel",
            div {
                class: "form",
                h1 {
                    class: "text-xs-center",
                    "Sign up"
                }
                p {
                    class: "text-xs-center",
                    a {
                        onclick: move |_| { *auth_state.write() = AuthState::Login },
                        "Have an account?"
                    }
                }
                br {}
                br {}

                form {
                    FormInput {
                        oninput: move |s: FormData| username.set(s.value),
                        placeholder: "Username".to_string()
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
                            println!(":: SignUpPage] button clicked. username: {} | email: {}", username, email);
                            let homeserver = homeserver.read();
                            to_owned![username, email, password, homeserver];
                            cx.spawn(async move {
                                let register_variables = RegisterVariables { captcha_token: None, email: &email, password: &password, username: &username };
                                let operation = Register::build(register_variables);
                                let response = surf::post(homeserver.endpoint)
                                    .run_graphql(operation)
                                    .await;
                                match response {
                                    // Parse data from here, such as storing a response token
                                    Ok(resp) => println!("Register successful! Account id: {:?}", resp.data.unwrap().register_user.id),

                                    //Handle any errors from the fetch here
                                    Err(_err) => {
                                        println!("Register failed")
                                    }
                                }
                            });
                        },
                        label: "Sign up".to_string()
                    }
                }
            }
        }
    }
}
