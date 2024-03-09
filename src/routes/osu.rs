use axum::Router;

use crate::routes::AppState;

pub mod tournament;
mod tournament_register;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/tournaments", tournament::routes())
}
