use actix_web::{get, HttpResponse};
use actix_web::web::ServiceConfig;
use crate::routes::ApiError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_staff_list);
}

#[get("{tournament_id}/staff")]
pub async fn get_staff_list() -> Result<HttpResponse, ApiError> {
    todo!();
}
