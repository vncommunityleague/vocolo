use actix_web::{error::HttpError, get, HttpResponse};
use actix_web::web::Query;
use lazy_static::lazy_static;
use oauth2::{
    AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use serde::{Deserialize, Serialize};

use common::constants::EnvironmentVariable;

use crate::util::auth;

lazy_static! {
    static ref DISCORD_REDIRECT_API: String = {
        format!(
            "{}/authorize/discord/callback",
            EnvironmentVariable::SERVER_PUBLIC_URL.value()
        )
    };
    static ref OSU_REDIRECT_API: String = {
        format!(
            "{}/authorize/osu/callback",
            EnvironmentVariable::SERVER_PUBLIC_URL.value()
        )
    };
    static ref DISCORD_CLIENT: BasicClient = BasicClient::new(
        ClientId::new(EnvironmentVariable::DISCORD_CLIENT_ID.value()),
        Some(ClientSecret::new(
            EnvironmentVariable::DISCORD_CLIENT_SECRET.value()
        )),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap())
    )
    .set_redirect_uri(RedirectUrl::new(DISCORD_REDIRECT_API.to_string()).unwrap());
    static ref OSU_CLIENT: BasicClient = BasicClient::new(
        ClientId::new(EnvironmentVariable::OSU_CLIENT_ID.value()),
        Some(ClientSecret::new(
            EnvironmentVariable::OSU_CLIENT_SECRET.value()
        )),
        AuthUrl::new("https://osu.ppy.sh/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://osu.ppy.sh/oauth/token".to_string()).unwrap())
    )
    .set_redirect_uri(RedirectUrl::new(OSU_REDIRECT_API.to_string()).unwrap());
}

#[derive(Serialize, Deserialize)]
pub struct Authorization {
    code: String,
    state: String,
}

// DISCORD

#[get("/discord")]
pub async fn discord_login() -> Result<HttpResponse, HttpError> {
    let (url, _csrf_token) = DISCORD_CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("Location", &*url.to_string()))
        .finish())
}

#[get("/discord/callback")]
pub async fn discord_login_callback(info: Query<Authorization>) -> Result<HttpResponse, HttpError> {
    let token = DISCORD_CLIENT
        .exchange_code(AuthorizationCode::new(info.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let user = auth::get_discord_user_from_token(token.access_token().secret())
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(user.id))
}

// OSU

#[get("/osu")]
pub async fn osu_login() -> Result<HttpResponse, HttpError> {
    let (url, _csrf_token) = OSU_CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("public".to_string()))
        .url();

    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("Location", &*url.to_string()))
        .finish())
}

#[get("/osu/callback")]
pub async fn osu_login_callback(info: Query<Authorization>) -> Result<HttpResponse, HttpError> {
    let token = OSU_CLIENT
        .exchange_code(AuthorizationCode::new(info.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let user = auth::get_osu_user_from_token(token.access_token().secret())
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(user.id))
}
