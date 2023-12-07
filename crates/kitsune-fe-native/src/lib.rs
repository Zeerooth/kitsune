#![allow(non_snake_case)]

use cynic::QueryBuilder;
use dioxus::prelude::{
    dioxus_elements, fc_to_builder, inline_props, render, use_shared_state_provider, Element,
    GlobalAttributes, Props, Scope,
};
use dioxus_router::{
    components::Router,
    prelude::{Link, Outlet, Routable},
    routable::ToRouteSegments,
};

pub mod components;
pub mod instance;
pub mod oauth;
pub mod pages;
pub mod schema;
pub mod state;

use pages::{NotFound, OAuthCallback, OAuthCallbackQuerySegments, Timeline};
use surf::Url;

use crate::{
    schema::InstanceInfo,
    state::{AuthState, Homeserver},
};

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[redirect("/", || Route::Timeline { timeline: String::from("public") })]
        #[route("/timelines/:timeline")]
        Timeline { timeline: String },
    #[end_layout]
    #[layout(OAuth)]
        #[route("/oauth-callback?:query_params")]
        OAuthCallback { query_params: OAuthCallbackQuerySegments },
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            div {
                class: "nav-inner",
                ul {
                    li { Link { to: Route::Timeline { timeline: String::from("home") }, "/home/" } }
                    li { Link { to: Route::Timeline { timeline: String::from("public") }, "/public/" } }
                }
            }
        }
        div {
            class: "main",
            div {
                class: "content-wrap",
                div {
                    class: "content",
                    Outlet::<Route> { }
                }
                div {
                    class: "sidebar",
                    components::auth::AuthInfo { }
                }
            }
        }
    }
}

fn OAuth(cx: Scope) -> Element {
    render! {
        Outlet::<Route> { }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct AppProps {
    pub default_host: Option<String>,
}

pub fn app(cx: Scope<AppProps>) -> Element {
    let auth_state = AuthState::new();
    use_shared_state_provider(cx, || auth_state);

    // TODO: here we assume that the default_host is always provided by the liveview
    // but in the future (e.g. for dekstop and mobile clients)
    // we want to ask for the homeserver first
    let homeserver =
        Url::parse(&cx.props.default_host.clone().unwrap()).expect("Invalid homeserver url");
    let endpoint = homeserver.join("graphql").expect("Invalid homeserver url");
    let homeserver = Homeserver { endpoint };
    use_shared_state_provider(cx, || homeserver);

    render! {
        Router::<Route> {}
    }
}
