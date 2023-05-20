use axum::Router;

mod tournaments;

pub fn init_routes() -> Router {
    Router::new().nest("/tournaments", tournaments::init_routes())
}
