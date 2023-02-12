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
struct OsuCallbackInfo {
    code: String,
}

#[get("/osu")]
pub async fn osu_login() -> Result<HttpResponse, HttpError> {
    login(
        "https://osu.ppy.sh/oauth/authorize",
        &EnvironmentVariable::OSU_CLIENT_ID.value(),
        &OSU_REDIRECT_API,
        "identify, public",
    );
    Ok(HttpResponse::Ok().finish())
}

#[get("/osu/callback")]
pub async fn osu_login_callback(
    osu_helper: web::Data<OsuHelper>,
    info: Query<OsuCallbackInfo>,
) -> Result<HttpResponse, HttpError> {
    let osu: Osu = Osu::builder()
        .with_authorization(info.code.clone(), OSU_REDIRECT_API.clone())
        .build()
        .await
        .unwrap();

    Ok(HttpResponse::Ok().finish())
}

fn login(base_url: &str, client_id: &str, redirect_uri: &str, scope: &str) -> Redirect {
    Redirect::to(format!(
        "{}/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}",
        base_url, client_id, redirect_uri, scope
    ))
}
