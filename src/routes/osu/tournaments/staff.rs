use crate::routes::ApiError;
use actix_web::web::ServiceConfig;
use actix_web::{get, HttpResponse};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_staff_list);
}

#[get("{tournament_id}/staff")]
pub async fn get_staff_list() -> Result<HttpResponse, ApiError> {
    todo!();
}
