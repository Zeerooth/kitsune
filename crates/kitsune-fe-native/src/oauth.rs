use std::collections::HashMap;

use cynic::{http::SurfExt, MutationBuilder};
use dioxus::prelude::{
    to_owned, use_shared_state, use_shared_state_provider, Scope, ScopeState, UseSharedState,
};
use iso8601_timestamp::{Duration, Timestamp};
use serde::{Deserialize, Serialize};
use surf::{http::Method, Request, Url};

use crate::{
    schema::{Oauth, Oauth2Application, OauthVariables},
    state::{AuthState, Homeserver},
};

const APPLICATION_NAME: &'static str = "Kitsune FE experimental";

#[derive(Debug, Clone)]
pub struct OAuthTokenData {
    token: String,
    refresh_token: String,
    expires_at: Timestamp,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenRequestBody {
    grant_type: String,
    code: String,
    redirect_uri: String,
}

pub async fn application_credentials(
    homeserver: &UseSharedState<Homeserver>,
) -> Result<Oauth2Application, surf::Error> {
    let redirect_uri = "/oauth-callback";
    let oauth_variables = OauthVariables {
        name: APPLICATION_NAME,
        redirect_uri,
    };
    let operation = Oauth::build(oauth_variables);
    let response = surf::post(homeserver.read().endpoint.clone())
        .run_graphql(operation)
        .await?;
    let oauth_app = response.data.unwrap().register_oauth_application;
    Ok(oauth_app)
}

pub async fn handle_oauth_response(cx: &ScopeState, response: OAuthResponse) -> OAuthTokenData {
    let expires_at = Timestamp::now_utc() + Duration::seconds(response.expires_in);
    let token_data = OAuthTokenData {
        token: response.access_token,
        refresh_token: response.refresh_token,
        expires_at,
    };
    use_shared_state_provider(&cx, || token_data.clone());
    token_data
}

pub async fn authorization_url(cx: &ScopeState) -> Result<Url, surf::Error> {
    let endpoint = use_shared_state::<Homeserver>(&cx)
        .unwrap()
        .read()
        .endpoint
        .clone();
    let credentials = application_credentials(cx).await?;
    let endpoint = endpoint.join(&format!(
        "/oauth/authorize?response_type=code&client_id={}&redirect_uri={}&scope=read+write",
        credentials.id.0, credentials.redirect_uri
    ));
    endpoint.map_err(|e| e.into())
}

pub async fn access_token(cx: &ScopeState, code: String) -> Result<OAuthTokenData, surf::Error> {
    let homeserver = use_shared_state::<Homeserver>(&cx).unwrap();
    let app_credentials = application_credentials(&cx).await?;
    let basic_auth_credentials = format!("{}:{}", app_credentials.id.0, app_credentials.secret);
    let base64 = base64_simd::STANDARD;
    let b64_auth_credentials = base64.encode_to_string(basic_auth_credentials);
    let request_body = TokenRequestBody {
        grant_type: String::from("authorization_code"),
        code,
        redirect_uri: app_credentials.redirect_uri,
    };
    let mut request = Request::new(Method::Post, homeserver.read().endpoint.clone());
    request.body_form(&request_body);
    request.insert_header("Authorization", format!("Basic {}", b64_auth_credentials));
    let mut response = surf::post(homeserver.read().endpoint.clone()).await?;
    Ok(handle_oauth_response(cx, response.body_json().await?).await)
}
