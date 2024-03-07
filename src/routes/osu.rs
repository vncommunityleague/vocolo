use axum::Router;

use crate::routes::AppState;

pub mod tournament;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/tournaments", tournament::routes())
}
