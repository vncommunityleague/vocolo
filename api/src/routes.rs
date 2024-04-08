use axum::{
    extract::FromRef, http::StatusCode, response::IntoResponse, routing::get, Json, Router,
};

use vocolo_database::Database;

mod v1;

pub fn init(state: AppState) -> Router {
    Router::new()
        .nest("/", v1::routes())
        .nest("/v1", v1::routes())
        .route("/", get(root))
        .with_state(state)
        .fallback(handle_404)
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub db: Database,
}

impl FromRef<AppState> for () {
    fn from_ref(_input: &AppState) -> Self {}
}

async fn root() -> &'static str {
    "Vocolo"
}

async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "error": "not_found",
            "description": "The requested resource was not found."
        })),
    )
        .into_response()
}
