use axum::Router;
use crate::repository::Repo;

mod tournaments;

pub fn init_routes() -> Router<Repo> {
    Router::new().nest("/tournaments", tournaments::init_routes())
}
