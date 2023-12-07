use surf::Url;

use crate::oauth::OAuthTokenData;

#[derive(Debug, Clone)]
pub enum AuthState {
    Login,
    Register,
    LoggedIn(OAuthTokenData),
}

impl AuthState {
    pub fn new() -> Self {
        Self::Login
    }
}

#[derive(Debug, Clone)]
pub struct Homeserver {
    pub endpoint: Url,
}
