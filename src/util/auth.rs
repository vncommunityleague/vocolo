use std::collections::HashMap;
use std::str::FromStr;

use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use vocolo_entity::prelude::User;
use vocolo_entity::user::Column;

use crate::error::Error;
use crate::models::user::APIUser;
use crate::routes::AppState;
use crate::util;

pub struct SessionUser(pub APIUser);

#[axum::async_trait]
impl<S> FromRequestParts<S> for SessionUser
where
    AppState: FromRef<S>,
    S: Send + Sync
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let auth_url = format!("{}/sessions/me", util::env::var("AUTH_SERVER_URL"));

        // Hacky way to convert headers, wait for http 1.0 support in reqwest
        let mut headers = reqwest::header::HeaderMap::new();
        for (name, value) in parts.headers.iter() {
            let name = reqwest::header::HeaderName::from_str(name.as_str()).unwrap();
            headers.insert(name, value.as_bytes().try_into().unwrap());
        }
        
        let resp = reqwest::Client::new()
            .get(&auth_url)
            .headers(headers)
            .send()
            .await
            .map_err(|err| Error::Unauthorized(err.to_string()))?
            .json::<HashMap<String, String>>()
            .await
            .map_err(|err| Error::Unauthorized(err.to_string()))?;

        let id = resp.get("id").ok_or(Error::Unauthorized("No identity".to_string()))?;

        let user = User::find()
            .filter(Column::IdentityId.eq(id))
            .one(&state.database)
            .await?
            .ok_or(Error::NotFound("user".to_string()))?;

        Ok(Self(APIUser::from(user)))
    }
}