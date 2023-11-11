#![allow(non_snake_case)]

use dioxus::prelude::{
    dioxus_elements, fc_to_builder, render, rsx, use_shared_state_provider, use_state, Element,
    Scope,
};
use dioxus_router::{
    components::Router,
    prelude::{Link, Outlet, Routable},
    routable::ToRouteSegments,
};
use state::AppState;

pub mod components;
pub mod pages;
pub mod state;

use pages::{Home, NotFound, Signup};

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},

        #[route("/signup")]
        Signup {},
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            ul {
                li { Link { to: Route::Home {}, "Home" } }
                li { Link { to: Route::Signup {}, "Sign up" } }
            }
        }
        Outlet::<Route> {}
    }
}

pub fn app(cx: Scope) -> Element {
    let mut app_state = AppState::new();
    use_shared_state_provider(cx, || app_state);

    render! {
        Router::<Route> {}
    }
}
