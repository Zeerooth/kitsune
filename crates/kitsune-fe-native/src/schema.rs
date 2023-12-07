#[cynic::schema("kitsune")]
mod schema {}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "RootQuery")]
pub struct InstanceInfo {
    pub instance: Instance,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Instance {
    pub domain: String,
    pub character_limit: i32,
    pub description: String,
    pub local_post_count: i32,
    pub name: String,
    pub registrations_open: bool,
    pub user_count: i32,
    pub version: String,
    pub captcha: Option<CaptchaInfo>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct CaptchaInfo {
    pub backend: CaptchaBackend,
    pub key: String,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum CaptchaBackend {
    #[cynic(rename = "H_CAPTCHA")]
    HCaptcha,
    #[cynic(rename = "M_CAPTCHA")]
    MCaptcha,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct OauthVariables<'a> {
    pub name: &'a str,
    pub redirect_uri: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "RootMutation", variables = "OauthVariables")]
pub struct Oauth {
    #[arguments(name: $name, redirectUri: $redirect_uri)]
    pub register_oauth_application: Oauth2Application,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "OAuth2Application")]
pub struct Oauth2Application {
    pub id: Uuid,
    pub secret: String,
    pub redirect_uri: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct RegisterVariables<'a> {
    pub captcha_token: Option<&'a str>,
    pub email: &'a str,
    pub password: &'a str,
    pub username: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "RootMutation", variables = "RegisterVariables")]
pub struct Register {
    #[arguments(email: $email, password: $password, username: $username, captchaToken: $captcha_token)]
    pub register_user: User,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct User {
    pub id: Uuid,
}

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "UUID")]
pub struct Uuid(pub String);
