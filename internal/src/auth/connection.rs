use std::env;

use axum::extract::FromRequestParts;
use http::{request::Parts, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct UserConnections {
    pub id: String,
    pub osu: OsuConnection,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct OsuConnection {
    pub id: i32,
    pub username: String,
    pub avatar_url: String,
}

impl UserConnections {
    pub async fn request(headers: HeaderMap<HeaderValue>) -> Result<UserConnections> {
        let kazusa_url = env::var("KAZUSA_URL").unwrap();
        let url = format!("{}/connections/me", kazusa_url);

        // Uhh
        let mut headers = headers;
        headers.remove(http::header::CONTENT_LENGTH);

        let client = reqwest::Client::new();
        let connections = client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json::<UserConnections>()
            .await?;

        Ok(connections)
    }
}

#[cfg(feature = "axum")]
#[axum::async_trait]
impl<S> FromRequestParts<S> for UserConnections
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let connections = UserConnections::request(parts.headers.clone())
            .await
            .map_err(|_| Error::Unauthorized)?;

        Ok(connections)
    }
}
