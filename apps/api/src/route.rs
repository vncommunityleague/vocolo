use axum::extract::FromRef;
use axum::routing::get;
use axum::Router;
use sea_orm::DatabaseConnection;

mod osu;

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/osu", osu::routes())
        .with_state(state)
        .fallback(vocolo_core::handle_404)
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

impl axum::extract::FromRef<AppState> for () {
    fn from_ref(_: &AppState) {}
}

async fn root() -> &'static str {
    "Hello, World!"
}
