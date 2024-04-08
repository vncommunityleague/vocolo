use axum::Router;

use super::AppState;

mod mappools;
mod matchups;
mod tournaments;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/mappools", mappools::routes())
        .nest("/matches", matchups::routes())
        .nest("/tournaments", tournaments::routes())
}
