use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_garde::WithValidation;

use vocolo_database::{osu::Tournament, Database};
use vocolo_internal::{Result, UserConnections};
use vocolo_models::v1;

use crate::routes::AppState;

mod staff;
mod teams;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(tournament_create)).nest(
        "/:tournament_id",
        Router::new()
            .route(
                "/",
                get(tournament_fetch)
                    .patch(tournament_update)
                    .delete(tournament_delete),
            )
            .merge(teams::routes()),
    )
}

pub async fn tournament_fetch(
    State(db): State<Database>,
    Path(tournament_id): Path<String>,
) -> Result<Json<v1::osu::Tournament>> {
    let tournament = Tournament::fetch(&db, &tournament_id).await?;

    Ok(Json(tournament.into()))
}

pub async fn tournament_create(
    connections: UserConnections,
    State(db): State<Database>,
    WithValidation(data): WithValidation<Json<v1::osu::CreateTournamentRequest>>,
) -> Result<(StatusCode, Json<v1::osu::CreateTournamentResponse>)> {
    let data = data.into_inner();

    let tournament: Tournament = data.into();
    let id = tournament.create(&db).await?;

    Ok((
        StatusCode::CREATED,
        Json(v1::osu::CreateTournamentResponse {
            id,
            slug: tournament.slug,
            name: tournament.name,
        }),
    ))
}

pub async fn tournament_update(
    connections: UserConnections,
    State(db): State<Database>,
    Path(tournament_id): Path<String>,
    WithValidation(data): WithValidation<Json<v1::osu::UpdateTournamentRequest>>,
) -> Result<()> {
    let data = data.into_inner();

    let tournament = data.into();
    Tournament::update(&db, &tournament_id, &tournament).await?;

    Ok(())
}

pub async fn tournament_delete(
    connections: UserConnections,
    State(db): State<Database>,
    Path(tournament_id): Path<String>,
) -> Result<StatusCode> {
    Tournament::delete(&db, &tournament_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
