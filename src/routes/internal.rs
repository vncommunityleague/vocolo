use axum::{Json, Router};
use axum::extract::State;
use axum::routing::post;
use axum_garde::WithValidation;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use tower_http::validate_request::ValidateRequestHeaderLayer;

use vocolo_entity::user::{ActiveModel, Model};

use crate::error::Error;
use crate::models::user::APIUser;
use crate::routes::AppState;
use crate::util;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/_create_new_user", post(create_new_user))
        .route_layer(ValidateRequestHeaderLayer::bearer(util::env::var("INTERNAL_TOKEN").as_str()))
}

#[derive(garde::Validate, Serialize, Deserialize)]
#[garde(allow_unvalidated)]
pub struct UserCreation {
    // Auth ID, provided by https://github.com/vncommunityleague/kazusa
    pub id: String,
    pub username: String,
}

pub async fn create_new_user(
    State(state): State<AppState>,
    WithValidation(data): WithValidation<Json<UserCreation>>,
) -> Result<Json<APIUser>, Error> {
    let data = data.into_inner();

    let identity_id =
        Uuid::parse_str(&data.id).map_err(|_| Error::Custom("Invalid UUID".to_string()))?;

    let user = ActiveModel {
        identity_id: Set(identity_id),
        ..Default::default()
    };

    let user: Model = user.insert(&state.database).await?;

    Ok(Json(APIUser::from(user)))
}
