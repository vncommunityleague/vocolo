use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::post;
use axum_garde::WithValidation;
use garde::Validate;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use vocolo_core::APIResponse;
use vocolo_models::user::{User, UserCreation, UserHandler};
use crate::error::Error;
use crate::route::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/_create_new_user", post(create_new_user))
        .route_layer(ValidateRequestHeaderLayer::bearer("Bearer"))
}

pub async fn create_new_user(
    State(state): State<AppState>,
    WithValidation(data): WithValidation<Json<UserCreation>>
) -> crate::Result<APIResponse<User>> {
    let user = UserHandler::create(&state.database, data.into_inner()).await.map_err(Error::Model)?;

    Ok(APIResponse::default().body(user))
}
