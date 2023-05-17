use axum::Router;
use axum::routing::get;
use crate::repository::Repo;
use crate::routes::{ApiError, ApiResult};
use actix_web::web::{Data, ServiceConfig};
use actix_web::{get, web, HttpResponse};
use crate::models::user::User;

pub fn config(router: &Router) {
    router.route("/me", get(get_current_user));
    router.route("/{id}", get(get_user));
}

#[get("/me")]
pub async fn get_current_user() -> ApiResult<User> {
    todo!()
}

#[get("/{id}")]
pub async fn get_user(
    repo: Data<Repo>,
    info: web::Path<(String,)>,
) -> ApiResult<User> {
    let path = info.into_inner();
    let id = &path.0;

    let user = repo.user.find_user(id).await;

    if user.is_none() {
        return Err(ApiError::UserNotFound);
    }

    Ok(HttpResponse::Ok().json(user.unwrap()))
}
