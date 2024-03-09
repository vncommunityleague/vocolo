use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::FromRef;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use sea_orm::DatabaseConnection;

mod internal;
mod osu;

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/internal", internal::routes())
        .nest("/osu", osu::routes())
        .with_state(state)
        .fallback(handle_404)
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
}

impl FromRef<AppState> for () {
    fn from_ref(_input: &AppState) -> Self {}
}

async fn root() -> &'static str {
    "hoaq, vu to!"
}

async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "error": "not_found",
            "error_description": "The requested resource was not found."
        })),
    )
        .into_response()
}
