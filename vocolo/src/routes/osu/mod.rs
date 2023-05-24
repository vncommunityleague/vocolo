use crate::repository::Repo;
use axum::Router;

mod tournaments;

pub fn init_routes() -> Router<Repo> {
    Router::new().nest("/tournaments", tournaments::init_routes())
}
