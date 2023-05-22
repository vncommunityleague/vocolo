use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::models::osu::tournaments::OsuTournament;
use crate::repository::model::ModelExt;
use crate::repository::{to_object_id, Repo};
use crate::routes::{ApiError, ApiResponse, ApiResult};

mod mappools;
mod matches;
// mod players;
mod staff;
// mod stages;
// mod teams;

pub fn init_routes() -> Router<Repo> {
    Router::new()
        .route("/", get(tournaments_list).post(tournaments_create))
        .route(
            "/:tournament_id",
            get(tournaments_get)
                .patch(tournaments_update)
                .delete(tournaments_delete),
        )
        .nest("/mappools", mappools::init_routes())
        .nest("/matches", matches::init_routes())
        // .nest("/:tournament_id", players::init_routes())
        .nest("/:tournament_id/staff", staff::init_routes())
    // .nest(":tournament_id/teams", teams::init_routes())
}

pub async fn tournaments_get(
    State(repo): State<Repo>,
    Path(tournament_id): Path<String>,
) -> ApiResult<OsuTournament> {
    let tournament = OsuTournament::find_by_id(
        repo.osu.tournaments.tournaments_col,
        &to_object_id(&tournament_id),
    )
    .await
    .map_err(ApiError::Database)?;

    let tournament = match tournament {
        Some(value) => value,
        None => return Err(ApiError::NotFound("tournament".to_string())),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(tournament))
}

// TODO: Add SearchConfig
pub async fn tournaments_list(State(repo): State<Repo>) -> ApiResult<Vec<OsuTournament>> {
    let tournaments = OsuTournament::list(repo.osu.tournaments.tournaments_col)
        .await
        .map_err(ApiError::Database)?;

    if tournaments.is_empty() {
        return Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT));
    }

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(tournaments))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentCreateRequest {
    pub title: String,
    pub slug: String,
}

pub async fn tournaments_create(
    State(repo): State<Repo>,
    Json(data): Json<TournamentCreateRequest>,
) -> ApiResult<OsuTournament> {
    let mut tournament = OsuTournament::default();

    tournament.info.title = data.title.clone();
    tournament.info.slug = data.slug.clone();

    let tournament = OsuTournament::create(repo.osu.tournaments.tournaments_col, tournament)
        .await
        .map_err(ApiError::Database)?;

    Ok(
        ApiResponse::new().status_code(StatusCode::CREATED), // .body(tournament)
    )
}

#[derive(Serialize, Deserialize)]
pub struct TournamentUpdateRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
}

pub async fn tournaments_update(
    State(repo): State<Repo>,
    Path(tournament_id): Path<String>,
    Json(data): Json<TournamentUpdateRequest>,
) -> ApiResult<OsuTournament> {
    let tournament = OsuTournament::find_one_and_update(
        repo.osu.tournaments.tournaments_col,
        doc! {"_id": to_object_id(&tournament_id)},
        doc! {"$set": bson::to_document(&data).unwrap()},
    )
    .await
    .map_err(ApiError::Database)?;

    let tournament = match tournament {
        Some(value) => value,
        None => return Err(ApiError::NotFound("tournament".to_string())),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(tournament))
}

pub async fn tournaments_delete(
    State(repo): State<Repo>,
    Path(tournament_id): Path<String>,
) -> ApiResult<()> {
    let tournament = OsuTournament::delete_one(
        repo.osu.tournaments.tournaments_col,
        doc! {"_id": to_object_id(&tournament_id)},
    )
    .await
    .map_err(ApiError::Database)?;

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}
