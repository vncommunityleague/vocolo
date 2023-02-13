use crate::constants::EnvironmentVariable;
use crate::helper::osu::OsuHelper;
use actix_web::web::{Data, Query, Redirect};
use actix_web::{delete, error::HttpError, get, patch, post, web, HttpResponse};
use lazy_static::lazy_static;
use rosu_v2::{Osu};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref DISCORD_REDIRECT_API: String = {
        format!(
            "{}/api/authorize/discord/callback",
            EnvironmentVariable::SERVER_PUBLIC_URL.value()
        )
    };
    static ref OSU_REDIRECT_API: String = {
        format!(
            "{}/api/authorize/osu/callback",
            EnvironmentVariable::SERVER_PUBLIC_URL.value()
        )
    };
}

#[derive(Deserialize)]
struct Authorization {
    code: String,
}

#[get("/osu")]
pub async fn osu_login() -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/osu/callback")]
pub async fn osu_login_callback(
    info: Query<Authorization>,
) -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().finish())
}
