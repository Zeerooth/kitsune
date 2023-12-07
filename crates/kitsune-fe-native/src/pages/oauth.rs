use std::fmt::Display;

use dioxus::prelude::{
    dioxus_elements, inline_props, render, to_owned, use_future, use_shared_state,
    use_shared_state_provider, Element, Props, Scope, ScopeState,
};
use dioxus_router::{prelude::use_navigator, routable::FromQuery};
use serde::{Deserialize, Serialize};

use crate::{
    oauth::{access_token, application_credentials},
    schema::Oauth2Application,
    state::Homeserver,
    Route,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuthCallbackQuerySegments {
    code: String,
}

impl Display for OAuthCallbackQuerySegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code={}", self.code)
    }
}

impl FromQuery for OAuthCallbackQuerySegments {
    fn from_query(query: &str) -> Self {
        serde_urlencoded::from_bytes::<OAuthCallbackQuerySegments>(query.as_bytes()).unwrap()
    }
}

#[inline_props]
pub fn OAuthCallback(cx: Scope, query_params: OAuthCallbackQuerySegments) -> Element {
    let nav = use_navigator(cx);
    let homeserver = use_shared_state::<Homeserver>(&cx).unwrap();
    cx.spawn(async move {
        use_shared_state_provider(&cx, || homeserver);
    });
    let access_token = use_future(cx, (), |_| access_token(cx.clone(), query_params.code));
    nav.push(Route::Timeline {
        timeline: String::from("home"),
    });
    render! {
        p { "This is a page just here to execute some code to finish authentication" }
    }
}
