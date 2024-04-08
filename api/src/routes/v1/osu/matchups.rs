use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{patch, post};
use axum::{Json, Router};
use axum_garde::WithValidation;

use vocolo_database::{
    osu::{Matchup, PartialMatchup},
    Database,
};
use vocolo_internal::Result;
use vocolo_models::v1;

use crate::routes::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(matchup_create)).nest(
        "/:match_id",
        Router::new().route(
            "/",
            patch(matchup_update)
                .get(matchup_fetch)
                .delete(matchup_delete),
        ),
    )
}

pub async fn matchup_create(
    State(db): State<Database>,
    WithValidation(data): WithValidation<Json<v1::osu::CreateMatchupRequest>>,
) -> Result<Json<v1::osu::CreateMatchupResponse>> {
    let data = data.into_inner();

    let matchup: Matchup = data.into();
    let id = matchup.create(&db).await?;

    Ok(Json(v1::osu::CreateMatchupResponse { id }))
}

pub async fn matchup_update(
    State(db): State<Database>,
    Path(match_id): Path<String>,
    WithValidation(data): WithValidation<Json<v1::osu::UpdateMatchupRequest>>,
) -> Result<()> {
    let data = data.into_inner();

    let matchup = PartialMatchup {
        date: data.date,
        ..Default::default()
    };

    Matchup::update(&db, &match_id, &matchup).await?;

    Ok(())
}

pub async fn matchup_fetch(
    State(db): State<Database>,
    Path(match_id): Path<String>,
) -> Result<Json<v1::osu::Matchup>> {
    let matchup = Matchup::fetch(&db, &match_id).await?;

    Ok(Json(matchup.into()))
}

pub async fn matchup_delete(
    State(db): State<Database>,
    Path(match_id): Path<String>,
) -> Result<StatusCode> {
    Matchup::delete(&db, &match_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn matchup_map_add() {}
