use actix_web::web::ServiceConfig;

mod tournaments;

pub fn config(cfg: &mut ServiceConfig) {
    tournaments::config(cfg);
}

pub fn init_routes() -> Router {
    Router::new()
        .nest("/tournaments", tournaments::init_routes())
}
