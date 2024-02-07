use axum::Router;

use crate::route::AppState;

pub mod tournament;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/tournaments", tournament::routes())
}
