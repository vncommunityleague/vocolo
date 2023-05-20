use axum::Router;
use thiserror::Error;

mod tournaments;

pub fn init_routes() -> Router {
    Router::new().nest("/tournaments", tournaments::init_routes())
}
