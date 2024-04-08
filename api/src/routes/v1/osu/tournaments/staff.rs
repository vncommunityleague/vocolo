use axum::{
    routing::{get, post},
    Router,
};

use vocolo_internal::Result;

use crate::routes::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(tournament_staff_list))
        .route("/register", post(tournament_staff_register))
}

pub async fn tournament_staff_list() -> Result<()> {
    unimplemented!()
}

pub async fn tournament_staff_register() {
    unimplemented!()
}
