use axum::extract::FromRef;
use axum::routing::get;
use axum::Router;
use sea_orm::DatabaseConnection;

mod osu;
mod internal;

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/internal", internal::routes())
        .nest("/osu", osu::routes())
        .with_state(state)
        .fallback(vocolo_core::handle_404)
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
}

impl FromRef<AppState> for () {
    fn from_ref(input: &AppState) -> Self { }
}

async fn root() -> &'static str {
    "Hello, World!"
}
