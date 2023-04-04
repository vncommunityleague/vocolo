use actix_web::web::{Data, Query, ServiceConfig};
use actix_web::{error::HttpError, get, web, HttpResponse};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::{Deserialize, Serialize};

use crate::repository::Repo;

use crate::util::auth;
use crate::util::auth::AuthType;

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

#[get("")]
pub async fn discord_login() -> Result<HttpResponse, HttpError> {
    let (url, _csrf_token) = auth::DISCORD_CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("Location", &*url.to_string()))
        .finish())
}

#[get("/callback")]
pub async fn discord_login_callback(
    repo: Data<Repo>,
    info: Query<Authorization>,
) -> Result<HttpResponse, HttpError> {
    let token = auth::DISCORD_CLIENT
        .exchange_code(AuthorizationCode::new(info.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let user = auth::get_discord_user_from_token(token.access_token().secret())
        .await
        .unwrap();

    if repo.user.find_by_discord_id(&user.id).await.is_none() {
        repo.user.create(user.id.clone(), AuthType::Discord).await;
        return Ok(HttpResponse::Ok().json(user.id));
    }

    Ok(HttpResponse::Ok().json("Not found"))
}

// OSU

#[get("")]
pub async fn osu_login() -> Result<HttpResponse, HttpError> {
    let (url, _csrf_token) = auth::OSU_CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("public".to_string()))
        .url();

    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("Location", &*url.to_string()))
        .finish())
}

#[get("/callback")]
pub async fn osu_login_callback(
    repo: Data<Repo>,
    info: Query<Authorization>,
) -> Result<HttpResponse, HttpError> {
    let token = auth::OSU_CLIENT
        .exchange_code(AuthorizationCode::new(info.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let user = auth::get_osu_user_from_token(token.access_token().secret())
        .await
        .unwrap();

    if repo
        .user
        .find_by_osu_id(&user.id.to_string())
        .await
        .is_none()
    {
        repo.user
            .create(user.id.to_string().clone(), AuthType::Osu)
            .await;
        return Ok(HttpResponse::Ok().json(user.id));
    }

    Ok(HttpResponse::Ok().json(user.id))
}
