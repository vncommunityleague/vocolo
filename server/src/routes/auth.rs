use actix_web::web::{Query, ServiceConfig};
use actix_web::{error::HttpError, get, web, HttpResponse};
use lazy_static::lazy_static;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::{Deserialize, Serialize};

use common::constants::EnvironmentVariable;

use crate::util::auth;

lazy_static! {
    static ref DISCORD_CLIENT: BasicClient = auth::create_client(
        EnvironmentVariable::DISCORD_CLIENT_ID.value(),
        EnvironmentVariable::DISCORD_CLIENT_SECRET.value(),
        "https://discord.com/api/oauth2/authorize",
        "https://discord.com/api/oauth2/token",
        format!(
            "{}/authorize/discord/callback",
            EnvironmentVariable::SERVER_PUBLIC_URL.value()
        )
    );
    static ref OSU_CLIENT: BasicClient = auth::create_client(
        EnvironmentVariable::OSU_CLIENT_ID.value(),
        EnvironmentVariable::OSU_CLIENT_SECRET.value(),
        "https://osu.ppy.sh/oauth/authorize",
        "https://osu.ppy.sh/oauth/token",
        format!(
            "{}/authorize/osu/callback",
            EnvironmentVariable::SERVER_PUBLIC_URL.value()
        )
    );
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("osu")
            .service(osu_login)
            .service(osu_login_callback),
    );

    cfg.service(
        web::scope("discord")
            .service(discord_login)
            .service(discord_login_callback),
    );
}

#[derive(Serialize, Deserialize)]
pub struct Authorization {
    code: String,
    state: String,
}

// DISCORD

#[get("/")]
pub async fn discord_login() -> Result<HttpResponse, HttpError> {
    let (url, _csrf_token) = DISCORD_CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("Location", &*url.to_string()))
        .finish())
}

#[get("/callback")]
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

#[get("/")]
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

#[get("/callback")]
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
