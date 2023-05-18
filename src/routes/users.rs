use axum::Router;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use crate::repository::Repo;
use crate::routes::{ApiError, ApiResult};
use crate::models::user::User;

pub fn init_routes() -> Router {
    Router::new()
        .route("/me", get(user_current))
        .route("/:id", get(user_get));
}

pub async fn user_current() -> ApiResult<User> {
    todo!()
}

pub async fn user_get(
    repo: Data<Repo>,
    Path(id): Path<String>,
) -> ApiResult<User> {
    let user = repo.user.find_user(&id).await;

    if user.is_none() {
        return Err(ApiError::UserNotFound);
    }

    Ok(Json(user.unwrap()))
}
