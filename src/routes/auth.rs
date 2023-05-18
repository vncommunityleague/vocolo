use axum::Router;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::{Deserialize, Serialize};

use crate::repository::Repo;
use crate::routes::ApiError;
use crate::util::auth;
use crate::util::auth::AuthType;

pub fn init_routes() -> Router {
    Router::new()
        .route("/discord", get(discord_login))
        .route("/discord/callback", get(discord_login_callback))
        .route("/osu", get(osu_login))
        .route("/osu/callback", get(osu_login_callback))
}

#[derive(Serialize, Deserialize)]
pub struct Authorization {
    code: String,
    state: String,
}

// DISCORD
pub async fn discord_login() -> Result<HttpResponse, ApiError> {
    let (url, _csrf_token) = auth::DISCORD_CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("Location", &*url.to_string()))
        .finish())
}

pub async fn discord_login_callback(
    repo: Data<Repo>,
    info: Query<Authorization>,
) -> Result<HttpResponse, ApiError> {
    let token = auth::DISCORD_CLIENT
        .exchange_code(AuthorizationCode::new(info.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let user = auth::get_discord_user_from_token(token.access_token().secret())
        .await
        .unwrap();

    if repo.user.find_user_by_discord_id(&user.id).await.is_none() {
        repo.user.create(&user.id, &AuthType::Discord).await;
        return Ok(HttpResponse::Ok().json(user.id));
    }

    Ok(Json("Not found"))
}

// OSU
pub async fn osu_login() -> Result<HttpResponse, ApiError> {
    let (url, _csrf_token) = auth::OSU_CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("public".to_string()))
        .url();

    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("Location", &*url.to_string()))
        .finish())
}

pub async fn osu_login_callback(
    repo: Data<Repo>,
    info: Query<Authorization>,
) -> Result<HttpResponse, ApiError> {
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
        .find_user_by_osu_id(&user.id.to_string())
        .await
        .is_none()
    {
        repo.user
            .create(&*user.id.to_string().clone(), &AuthType::Osu)
            .await;
        return Ok(Json(user.id));
    }

    Ok(JSon(user.id))
}
