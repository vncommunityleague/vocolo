use crate::models::user::User;
use crate::repository::Repo;
use crate::routes::{ApiError, ApiResult};
use axum::extract::State;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn init_routes() -> Router {
    Router::new()
        .route("/me", get(user_current))
        .route("/:id", get(user_get))
}

pub async fn user_current() -> ApiResult<User> {
    todo!()
}

pub async fn user_get(State(repo): State<Repo>, Path(id): Path<String>) -> ApiResult<User> {
    let user = repo.user.find_user(&id).await;

    if user.is_none() {
        return Err(ApiError::UserNotFound);
    }

    Ok(Json(user.unwrap()))
}
