use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_garde::WithValidation;
use sea_orm::DatabaseConnection;

use vocolo_core::APIResponse;
use vocolo_models::osu::tournament::*;

use crate::route::AppState;
use crate::Result;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(tournament_create)).route(
        "/:id",
        get(tournament_get)
            .patch(tournament_edit)
            .delete(tournament_delete),
    )
}

pub async fn tournament_create(
    State(state): State<AppState>,
    WithValidation(data): WithValidation<Json<OsuTournamentCreation>>,
) -> Result<APIResponse<OsuTournament>> {
    let tournament = OsuTournamentHandler::create(&state.database, data.into_inner()).await?;

    Ok(APIResponse::default()
        .status_code(StatusCode::CREATED)
        .body(tournament))
}

pub async fn tournament_get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<APIResponse<OsuTournament>> {
    let tournament = if let Ok(id) = id.parse::<i32>() {
        OsuTournamentHandler::get_by_id(&state.database, id).await?
    } else {
        OsuTournamentHandler::get_by_slug(&state.database, &id).await?
    };

    Ok(APIResponse::default().body(tournament))
}

pub async fn tournament_edit(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    WithValidation(data): WithValidation<Json<OsuTournamentUpdate>>,
) -> Result<APIResponse<OsuTournament>> {
    let tournament = OsuTournamentHandler::update(&db, id, data.into_inner()).await?;

    Ok(APIResponse::default()
        .status_code(StatusCode::OK)
        .body(tournament))
}

pub async fn tournament_delete(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<APIResponse<()>> {
    OsuTournamentHandler::delete(&db, id).await?;

    Ok(APIResponse::default().status_code(StatusCode::NO_CONTENT))
}
