use actix_web::error::HttpError;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OsuUser {
    pub id: u64,
    pub username: String,
}

pub async fn get_discord_user_from_token(access_token: &str) -> Result<DiscordUser, HttpError> {
    Ok(get_user_from_token("https://discord.com/api/users/@me", access_token).await)
}

pub async fn get_osu_user_from_token(access_token: &str) -> Result<OsuUser, HttpError> {
    Ok(get_user_from_token("https://osu.ppy.sh/api/v2/me", access_token).await)
}

pub async fn get_user_from_token<T: DeserializeOwned>(token_uri: &str, access_token: &str) -> T {
    reqwest::Client::new()
        .get(token_uri)
        .header(reqwest::header::USER_AGENT, "VCL")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {access_token}"),
        )
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
