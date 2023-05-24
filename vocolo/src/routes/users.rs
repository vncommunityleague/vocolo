use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::models::user::User;
use crate::repository::model::ModelExt;
use crate::repository::{to_object_id, Repo};
use crate::routes::{ApiError, ApiResponse, ApiResult};

pub fn init_routes() -> Router<Repo> {
    Router::new()
        .route("/me", get(user_current))
        .route("/:id", get(user_get))
}

pub async fn user_current() -> ApiResult<User> {
    todo!()
}

pub async fn user_get(State(repo): State<Repo>, Path(id): Path<String>) -> ApiResult<User> {
    let user = User::find_by_id(repo.user.user_col, &to_object_id(&id))
        .await
        .map_err(ApiError::Database)?;

    let user = match user {
        Some(user) => user,
        None => return Err(ApiError::NotFound("user".to_string())),
    };

    Ok(ApiResponse::new().status_code(StatusCode::OK).body(user))
}
