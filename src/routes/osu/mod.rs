use actix_web::web::ServiceConfig;

mod tournaments;

pub fn config(cfg: &mut ServiceConfig) {
    tournaments::config(cfg);
}
