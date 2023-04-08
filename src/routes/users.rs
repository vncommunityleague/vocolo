use actix_web::{get, HttpResponse, web};
use actix_web::web::{Data, ServiceConfig};
use crate::repository::Repo;
use crate::routes::ApiError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_current_user);
    cfg.service(get_user);
}

#[get("/me")]
pub async fn get_current_user() -> Result<HttpResponse, ApiError> {
    todo!()
}

#[get("/{id}")]
pub async fn get_user(
    repo: Data<Repo>,
    info: web::Path<(String,)>,
) -> Result<HttpResponse, ApiError> {
    let path = info.into_inner();
    let id = &path.0;

    let user = repo.user.find_user(id).await;

    if user.is_none() {
        return Err(ApiError::UserNotFound)
    }

    Ok(HttpResponse::Ok().json(user.unwrap()))
}
