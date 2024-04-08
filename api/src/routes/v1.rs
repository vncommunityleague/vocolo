use axum::Router;

use crate::routes::AppState;

pub mod osu;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/osu", osu::routes())
}
