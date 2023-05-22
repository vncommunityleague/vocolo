use lazy_static::lazy_static;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::routes::ApiError;
use crate::util::constants::EnvironmentVariable;

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

lazy_static! {
    pub static ref DISCORD_CLIENT: BasicClient = create_client(
        EnvironmentVariable::DiscordClientId.value(),
        EnvironmentVariable::DiscordClientSecret.value(),
        "https://discord.com/api/oauth2/authorize",
        "https://discord.com/api/oauth2/token",
        format!(
            "{}/authorize/discord/callback",
            EnvironmentVariable::ServerPublicUrl.value()
        )
    );
    pub static ref OSU_CLIENT: BasicClient = create_client(
        EnvironmentVariable::OsuClientId.value(),
        EnvironmentVariable::OsuClientSecret.value(),
        "https://osu.ppy.sh/oauth/authorize",
        "https://osu.ppy.sh/oauth/token",
        format!(
            "{}/authorize/osu_old/callback",
            EnvironmentVariable::ServerPublicUrl.value()
        )
    );
}

pub fn create_client(
    client_id: String,
    client_secret: String,
    auth_url: &str,
    token_url: &str,
    redirect_url: String,
) -> BasicClient {
    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url.to_string()).unwrap(),
        Some(TokenUrl::new(token_url.to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

pub enum AuthType {
    Discord,
    Osu,
}

impl AuthType {
    pub fn repo_path(&self) -> String {
        match self {
            AuthType::Discord => "link.discord",
            AuthType::Osu => "link.osu_old",
        }
        .to_string()
    }
}

pub async fn get_discord_user_from_token(access_token: &str) -> Result<DiscordUser, ApiError> {
    Ok(get_user_from_token("https://discord.com/api/users/@me", access_token).await)
}

pub async fn get_osu_user_from_token(access_token: &str) -> Result<OsuUser, ApiError> {
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
